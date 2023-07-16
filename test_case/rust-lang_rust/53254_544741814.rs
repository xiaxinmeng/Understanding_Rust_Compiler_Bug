rust
#![feature(core_intrinsics, link_llvm_intrinsics, repr_simd, simd_ffi, stdsimd)]
#![allow(non_camel_case_types)]
extern crate core;
use core::mem::transmute;

#[repr(simd)]
struct __m64(i64);

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.pow.f32"]
    fn pow_f32(f: f32, h: f32) -> f32;
    #[link_name = "llvm.x86.mmx.pmovmskb"]
    fn pmovmskb(a: __m64) -> i32;
}

#[repr(simd)]
#[derive(Copy, Clone)]
struct f32x2(f32, f32);

pub fn main() {
    unsafe {
        let _ = pmovmskb(transmute(0_i64));         
        //println!("{}", pow_f32(2.0, 1.0));
        if 2.0.ne(&pow_f32(2.0, 1.0)) {
            core::intrinsics::abort();
        };
    }
}

