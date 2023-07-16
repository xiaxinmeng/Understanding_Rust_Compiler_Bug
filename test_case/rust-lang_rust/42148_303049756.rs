rust
#![feature(core_intrinsics)]
extern crate core;
use core::intrinsics;

struct Zst;

fn main() {
    unsafe { intrinsics::volatile_store(1 as *mut Zst, Zst); }
}
