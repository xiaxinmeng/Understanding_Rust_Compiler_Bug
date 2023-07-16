rust
use std::ffi::c_void;

#[repr(C)]
pub struct Foo(c_void);

extern "C" {
  fn meh1(m: *mut Foo) -> *mut Foo;
}
