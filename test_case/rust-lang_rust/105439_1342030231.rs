rust
#![feature(portable_simd)]

extern crate core;

use core::simd::*;

pub fn check(a: i32x4) -> bool {
    (a + a).to_array() == [0, 0, 0, 0]
}
