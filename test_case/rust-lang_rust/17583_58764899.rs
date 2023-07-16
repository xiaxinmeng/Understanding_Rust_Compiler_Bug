 rust
use std::cell::UnsafeCell;

// Genric structure representing a slot in TLS, its definition changes
// per-platform.
//
// Note that the inner field is private, and this would require some form of
// macro hygiene to allow the macros below to initialize the fields without
// allowing access to them. This is already highly desired for other structures
// like, for example, UnsafeCell, Cell, and RefCell.
pub struct Tls<T> { inner: T }

// If a platform didn't support #[thread_local], the definition would look more
// like:
pub struct Tls<T> {
    init: T,         // bit pattern to initialize thread statics with
    key: AtomicUint, // init 0, lazily initialized
}

// These would be appropriately modified for platfors which didn't support
// #[thread_local] as just normal `static`/`static mut` globals.
macro_rules! tls(
    (static $name:ident: $t:ty = $init:expr) => (
        #[thread_local]
        static $name: Tls<$t> = Tls { inner: $init };
    );
    (static mut $name:ident: $t:ty = $init:expr) => (
        #[thread_local]
        static mut $name: Tls<$t> = Tls { inner: $init };
    );
)
macro_rules! cell(($e:expr) => (Cell { inner: UnsafeCell { value: $e } }))
macro_rules! refcell(($e:expr) => (...))

impl<T> Tls<T> {
    pub fn get(&'static self) -> TlsRef<T> {
        // On platforms which don't support #[thread_local], this would perform
        // lazy initialization of the corresponding OS-based TLS key.
        TlsRef { inner: &self.inner }
    }

    // This is a safe function, it should be unsafe to get a mutable reference
    // in the first place.
    pub fn get_mut(&'static mut self) -> TlsRefMut<T> {
        TlsRefMut { inner: &self.inner }
    }
}

pub struct TlsRef<T> { inner: &'static T }
pub struct TlsRefMut<T> { inner: &'static mut T }

impl<T> Deref<T> for TlsRef<T> {
    fn deref<'a>(&'a self) -> &'a T { self.inner }
}
impl<T> Deref<T> for TlsRefMut<T> {
    fn deref<'a>(&'a self) -> &'a T { &*self.inner }
}
impl<T> DerefMut<T> for TlsRefMut<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T { &mut *self.inner }
}

// Note that this is stored in a `static`, which normally requires `Sync`. The
// compiler would understand that a `#[thread_local]` static does not require
// `Sync`.
tls!(static FOO: Cell<uint> = cell!(1));
tls!(static BAR: RefCell<int> = refcell!(2));

fn main() {
    let foo = FOO.get();
    assert_eq!(foo.get(), 1);
    foo.set(1);

    let mut bar = BAR.get();
    assert_eq!(2, *bar.borrow());
    *bar.borrow_mut() = 3;
}
