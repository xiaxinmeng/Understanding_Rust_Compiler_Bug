rust
#![feature(
    no_core,
    lang_items,
    intrinsics,
    decl_macro,
    start,
    auto_traits,
    arbitrary_self_types
)]
#![no_core]

#[start]
fn start(_: isize, _: *const *const u8) -> isize {
    let future = async {};
    let future = future.map();

    0
}

#[lang = "sized"]
trait Sized {}

#[lang = "receiver"]
trait Receiver {}

#[lang = "copy"]
unsafe trait Copy {}

unsafe impl Copy for u8 {}

#[lang = "phantom_data"]
struct PhantomData<T>;

#[lang = "fn_once"]
trait FnOnce<Args> {
    #[lang = "fn_once_output"]
    type Output;
}

#[lang = "panic"]
fn panic_bounds_check(_index: usize) -> ! {
    intrinsics::abort();
}

#[lang = "drop_in_place"]
fn drop_in_place<T>() {}

#[lang = "deref"]
trait Deref {
    #[lang = "deref_target"]
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}

mod intrinsics {
    extern "rust-intrinsic" {
        #[rustc_safe_intrinsic]
        pub fn abort() -> !;
    }
}

mod libc {
    #[link(name = "c")]
    extern "C" {}
}

#[lang = "panic_location"]
struct PanicLocation {
    file: &'static str,
    line: u32,
    column: u32,
}

#[lang = "pin"]
struct Pin<P> {
    pointer: P,
}

#[lang = "Poll"]
enum Poll<T> {
    Ready(T),
    Pending,
}

#[lang = "future_trait"]
trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;

    fn map(self) -> Self
    where
        Self: Sized,
    {
        loop {}
    }
}

#[lang = "Context"]
struct Context;

impl<T: ?Sized> Deref for &T {
    type Target = T;

    fn deref(&self) -> &T {
        *self
    }
}

impl<T: ?Sized> Deref for &mut T {
    type Target = T;

    fn deref(&self) -> &T {
        *self
    }
}

impl<P: Deref> Deref for Pin<P> {
    type Target = P::Target;
    fn deref(&self) -> &P::Target {
        &*self.pointer
    }
}

#[lang = "get_context"]
unsafe fn get_context<'a>(cx: ResumeTy) -> &'a Context {
    &*(cx.0 as *mut Context)
}

#[lang = "deref_mut"]
trait DerefMut: Deref {
    fn deref_mut(&mut self) -> &mut Self::Target;
}

#[lang = "ResumeTy"]
struct ResumeTy(*const ());

#[lang = "unpin"]
pub auto trait Unpin {}
