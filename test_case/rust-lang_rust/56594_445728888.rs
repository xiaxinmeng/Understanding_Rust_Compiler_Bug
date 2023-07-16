rust
use std::ffi::c_void;

pub struct Foo(c_void);

pub extern "C" fn meh1(m: *mut Foo) -> *mut Foo { m }

#[repr(C)]
pub struct Bar {
    v: *mut Foo,
}

pub extern "C" fn meh2(m: *mut Bar) -> *mut Bar { m }
