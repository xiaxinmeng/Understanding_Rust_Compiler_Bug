
#![crate_type = "lib"]
#![feature(core_simd)]

use std::simd::f32x4;

pub fn foo(x: f32x4) -> f32x4 {
        f32x4(x.0, x.2, x.3, x.1)
}
