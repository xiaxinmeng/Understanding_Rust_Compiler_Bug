rust
#![feature(avx512_target_feature)]
#![feature(stdsimd)]
#![feature(portable_simd)]

use std::arch::{asm, x86_64::{__m512d, __m512i}};
use std::simd::{f64x8, i64x8};

#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512dq")]
pub unsafe fn convert(a: i64x8) -> f64x8 {
    let a: __m512i = a.into();
    let converted: __m512d;
    unsafe {
        asm! {
            "vcvtqq2pd {converted} {a}",
            a = in(zmm_reg) a,
            converted = out(zmm_reg) converted,
        };
    }
    converted.into()
}
