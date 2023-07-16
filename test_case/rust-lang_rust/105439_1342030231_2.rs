rust
#![feature(repr_simd)]
#![feature(platform_intrinsics)]

extern crate core;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(simd)]
pub struct i32x4([i32; 4]);

extern "platform-intrinsic" {
    pub(crate) fn simd_add<T>(x: T, y: T) -> T;
}

#[inline(always)]
fn to_array(x: i32x4) -> [i32; 4] {
    x.0
}

pub fn check(a: i32x4) -> bool {
    let b: i32x4 = unsafe { simd_add(a, a) };
    to_array(b) == [0; 4]
}
