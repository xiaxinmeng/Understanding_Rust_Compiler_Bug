rust
// Lifetime-invariant ZST token. Probably doesn't even need the invariance.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct InterruptsDisabled<'a>(PhantomData<&'a mut &'a ()>);

// Public field type to get around the lack of stable `const fn`.
pub struct InterruptLock<T: ?Sized + Send>(pub UnsafeCell<T>);

// AFAIK this should behave like Mutex.
unsafe impl<T: ?Sized + Send> Sync for InterruptLock<T> {}

impl<T: ?Sized + Send> InterruptLock<T> {
    // This gives access to the `T` that's `Send` but maybe not `!Sync`,
    // for *at most* the duration that the interrupts are disabled for.
    // Note: I wanted to implement `Index` but that can't constrain lifetimes.
    fn get<'a>(&'a self, _: InterruptsDisabled<'a>) -> &'a T {
        unsafe { &*self.0.get() }
    }
}

// Note that you bake any of these into `InterruptLock` without `const fn`,
// because you would need a private field to ensure nobody can touch it.
// OTOH, `UnsafeCell<T>` makes *only constructing* the field public.
pub type InterruptCell<T: ?Sized + Send> = InterruptLock<Cell<T>>;
pub type InterruptRefCell<T: ?Sized + Send> = InterruptLock<RefCell<T>>;
