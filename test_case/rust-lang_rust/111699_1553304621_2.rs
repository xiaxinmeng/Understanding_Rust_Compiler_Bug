rust
#![feature(core_intrinsics)]
use std::intrinsics::floorf32;

fn main() {
    unsafe { floorf32(true); }
}
