rust
#![feature(allocator_api)]

use std::alloc::Global;
use std::mem::MaybeUninit;

fn crash(_: &[MaybeUninit<()>]) {}

fn main() {
    crash(&Box::new_uninit_slice_in(0, &Global));
}
