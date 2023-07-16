rust
#![feature(portable_simd)]
#![feature(avx512_target_feature)]

use std::simd::{i64x8, f64x8};
use std::arch::asm;

#[target_feature(enable = "avx512f")]
pub unsafe fn convert(a: i64x8) -> f64x8{
    let converted: f64x8;
    unsafe {
        asm!(
        "vcvtqq2pd {converted}, {a}",
        a = in(zmm_reg) a,
        converted = out(zmm_reg) converted,
        );
    }
    converted
}
