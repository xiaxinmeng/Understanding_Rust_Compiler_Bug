rust
use std::ops::Deref;

pub struct Unique<T: ?Sized> {
    pointer: *const T,
}

impl<T:?Sized> Deref for Unique<T> {
    type Target = *mut T;

    #[inline]
    fn deref<'a>(&'a self) -> &'a *mut T {
        unsafe { mem::transmute(&*self.pointer) }
    }
}
