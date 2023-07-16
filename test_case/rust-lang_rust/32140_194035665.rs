 rust
#![feature(repr_simd, platform_intrinsics)]

use std::mem::transmute;

#[repr(simd)]
#[derive(Debug, PartialEq)]
struct F64x2(f64, f64); 

#[repr(simd)]
#[derive(Debug, PartialEq)]
struct F64x4(f64, f64, f64, f64);

#[repr(simd)]
#[derive(Debug, PartialEq)]
struct F32x4(f32, f32, f32, f32);

#[repr(simd)]
#[derive(Debug, PartialEq)]
struct F32x8(f32, f32, f32, f32, f32, f32, f32, f32);

#[repr(simd)]
#[derive(Debug, PartialEq)]
struct I32x4(i32, i32, i32, i32);

#[repr(simd)]
#[derive(Debug, PartialEq)]
struct I32x8(i32, i32, i32, i32, i32, i32, i32, i32);

extern "platform-intrinsic" {
    fn x86_mm256_broadcast_pd(ptr: *const i8) -> F64x4;
    fn x86_mm256_broadcast_ps(ptr: *const i8) -> F32x8;
    fn x86_mm256_cvtepi32_pd(x: I32x4) -> F64x4;
    fn x86_mm256_cvtepi32_ps(x: I32x8) -> F32x8;
    fn x86_mm256_cvtpd_epi32(x: F64x4) -> I32x4;
    fn x86_mm256_cvtpd_ps(x: F64x4) -> F32x4;
    fn x86_mm256_cvtps_epi32(x: F32x8) -> I32x8;
    fn x86_mm256_cvtps_pd(x: F32x4) -> F64x4;
    fn x86_mm256_cvttpd_epi32(x: F64x4) -> I32x4;
    fn x86_mm256_cvttps_epi32(x: F32x8) -> I32x8;
}

fn main() {
    let a = F64x2(1.0, 2.0);
    let b = unsafe { x86_mm256_broadcast_pd(transmute(&a)) };
    let c = F64x4(1.0, 2.0, 1.0, 2.0);
    assert_eq!(b, c);

    let a = F32x4(1.0, 2.0, 3.0, 4.0);
    let b = unsafe { x86_mm256_broadcast_ps(transmute(&a)) };
    let c = F32x8(1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0);
    assert_eq!(b, c);

    let a = I32x4(1, -2, 3, -4);
    let b = unsafe { x86_mm256_cvtepi32_pd(a) };
    let c = F64x4(1.0, -2.0, 3.0, -4.0);
    assert_eq!(b, c);

    let a = I32x8(1, -2, 3, -4, 5, -6, 7, -8);
    let b = unsafe { x86_mm256_cvtepi32_ps(a) };
    let c = F32x8(1.0, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -8.0);
    assert_eq!(b, c);

    let a = F64x4(1.0, -2.0, 3.0, -4.0);
    let b = unsafe { x86_mm256_cvtpd_epi32(a) };
    let c = I32x4(1, -2, 3, -4);
    assert_eq!(b, c);

    let a = F64x4(1.0, -2.0, 3.0, -4.0);
    let b = unsafe { x86_mm256_cvtpd_ps(a) };
    let c = F32x4(1.0, -2.0, 3.0, -4.0);
    assert_eq!(b, c);

    let a = F32x8(1.0, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -8.0);
    let b = unsafe { x86_mm256_cvtps_epi32(a) };
    let c = I32x8(1, -2, 3, -4, 5, -6, 7, -8);
    assert_eq!(b, c);

    let a = F32x4(1.0, -2.0, 3.0, -4.0);
    let b = unsafe { x86_mm256_cvtps_pd(a) };
    let c = F64x4(1.0, -2.0, 3.0, -4.0);
    assert_eq!(b, c);

    let a = F64x4(1.6, -2.0, 3.0, -4.0);
    let b = unsafe { x86_mm256_cvttpd_epi32(a) };
    let c = I32x4(1, -2, 3, -4);
    assert_eq!(b, c);

    let a = F32x8(1.6, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -8.0);
    let b = unsafe { x86_mm256_cvttps_epi32(a) };
    let c = I32x8(1, -2, 3, -4, 5, -6, 7, -8);
    assert_eq!(b, c);
}
