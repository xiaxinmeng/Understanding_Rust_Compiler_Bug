rust
use core::mem::MaybeUninit;

#[repr(transparent)]
struct NoNiche<T>(MaybeUninit<T>);

impl<T> NoNiche<T> {
    pub fn new(value: T) -> Self {
        Self(MaybeUninit::new(value))
    }
    pub fn into_inner(self) -> T {
        // Safety: we never store an uninitialized MaybeUninit
        unsafe { self.0.assume_init() }
    }
    pub fn get(&self) -> &T {
        // Safety: we never store an uninitialized MaybeUninit
        unsafe { self.0.assume_init_ref() }
    }
    pub fn get_mut(&mut self) -> &mut T {
        // Safety: we never store an uninitialized MaybeUninit
        unsafe { self.0.assume_init_mut() }
    }
}
