rust
#![feature(ptr_metadata)]

use std::ptr::Pointee;

unsafe fn ice<T: ?Sized + 'static>(ptr: *const T) -> <T as Pointee>::Metadata {
    std::ptr::metadata(ptr)
}
