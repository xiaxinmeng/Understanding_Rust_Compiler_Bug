rust
#![feature(
    crate_visibility_modifier,
    link_llvm_intrinsics,
    platform_intrinsics,
    repr_simd,
    stdsimd
)]
#![allow(non_camel_case_types)]
extern crate core;
use core::arch;
use core::mem;

extern "platform-intrinsic" {
    crate fn simd_ne<d, e>(f: d, h: d) -> e;
}

extern "C" {
    #[link_name = "llvm.pow.f32"]
    fn pow_f32(f: f32, h: f32) -> f32;
}

#[repr(simd)]
#[derive(Copy, Clone)]
struct f32x2(f32, f32);
#[repr(simd)]
#[derive(Copy, Clone)]
struct i32x2(i32, i32);

impl f32x2 {
    pub fn ne(self, aq: Self) -> bool {
        let z: i32x2 = unsafe { simd_ne(self, aq) };
        use arch::x86_64::_mm_movemask_pi8;
        unsafe { _mm_movemask_pi8(mem::transmute(z)) != 0 }
    }
}

macro_rules! assert_eq_ {
    ($a:expr, $b:expr) => {
        if $a.ne($b) {
            panic!()
        }
    };
}
pub fn main() {
    let o = f32x2(1.0, 1.0);
    assert_eq_!(o, o);
    assert_eq_!(2.0, unsafe {&pow_f32(2.0, 1.0)});
    //println!("{}", unsafe {&pow_f32(2.0, 1.0)});
}
