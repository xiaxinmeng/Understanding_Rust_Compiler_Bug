 Rust
use core::mem;
use std::ops::{Deref, DerefMut};
use std::ptr::drop_in_place;

#[repr(transparent)]
pub struct Box1<T: 'static> {
    ptr: &'static mut T,
}

impl<T> Box1<T> {
    pub fn new(ptr: &'static mut T) -> Self {
        Self { ptr: ptr }
    }

    #[inline]
    pub fn as_ptr(self) -> &'static mut T {
        let ref_ = self.ptr as *mut T;
        mem::forget(self);
        unsafe { &mut *ref_ }
    }
}

impl<T: 'static> Deref for Box1<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        self.ptr
    }
}

impl<T: 'static> DerefMut for Box1<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        self.ptr
    }
}

impl<T> Drop for Box1<T> {
    fn drop(&mut self) {
        //Call destructor of T (if exists)
        let ptr = self.deref_mut();
        unsafe {
            drop_in_place(ptr);
        }
    }
}

fn main() {

}
