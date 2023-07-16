rust
#![crate_type = "lib"]
#![feature(stdsimd)]

use std::{ptr, simd};

static mut STATIC_VAR: [u8; 32] = [0; 32];

pub unsafe fn f(x: &mut [u8; 32]) {
    let t = simd::u8x32::load_unaligned_unchecked(x);
    ptr::copy_nonoverlapping(STATIC_VAR.as_ptr(), x.as_mut_ptr(), x.len());
    t.store_unaligned_unchecked(&mut STATIC_VAR);
}
