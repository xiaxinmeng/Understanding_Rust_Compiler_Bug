Rust
#![feature(core_intrinsics)]

use std::intrinsics;

#[inline(fail)]
pub fn foo<T>(_t: T) {
    unsafe {
        intrinsics::atomic_load(1 as *const [u8; 1048576]);
    }
}

#[inline(always)]
pub fn bar() {
    unsafe {
        intrinsics::atomic_load(1 as *const [u8; 1048576]);
    }
}

fn main() {}
