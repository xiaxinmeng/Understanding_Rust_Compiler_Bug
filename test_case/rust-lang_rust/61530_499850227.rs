rust
#![feature(
    platform_intrinsics,
    repr_simd,
)]

extern "platform-intrinsic" {
    pub fn simd_shuffle2<T, U>(x: T, y: T, idx: [u32; 2]) -> U;
}

#[repr(simd)]
#[derive(Copy, Clone)]
pub struct T(f32, f32);

pub unsafe fn foo(b: T) -> T {
    simd_shuffle2(b, b, [0, 1])
}
pub unsafe fn bar(b: T) -> T {
    simd_shuffle2(b, b, [0, 1])
}
