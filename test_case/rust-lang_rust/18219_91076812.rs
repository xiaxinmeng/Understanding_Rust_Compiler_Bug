
#![feature(core)]

fn main () {
    extern crate core;
    let _ = core::cell::UnsafeCell::new(42i32);
}
