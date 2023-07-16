 rust
use std::libc::*;
pub struct Union_Foo {
    data: [u64, ..1u],
}
impl Union_Foo {
    pub fn x(&mut self) -> *mut c_int {
        unsafe { ::std::cast::transmute(::std::ptr::to_mut_unsafe_ptr(self)) }
    }
    pub fn y(&mut self) -> *mut c_schar {
        unsafe { ::std::cast::transmute(::std::ptr::to_mut_unsafe_ptr(self)) }
    }
    pub fn z(&mut self) -> *mut c_double {
        unsafe { ::std::cast::transmute(::std::ptr::to_mut_unsafe_ptr(self)) }
    }
}
