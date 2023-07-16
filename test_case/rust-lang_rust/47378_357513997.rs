rust
// test.rs
#![feature(repr_align)]
#![feature(attr_literals)]

#[repr(align(16))]
pub struct Foo(i32);

#[no_mangle]
pub extern "C" fn foo(x: Foo) {}

fn main() {}
