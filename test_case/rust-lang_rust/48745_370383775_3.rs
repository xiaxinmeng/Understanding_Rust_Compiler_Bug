rust
#![feature(const_fn, untagged_unions, stdsimd)]
use std::arch::x86_64::*;
use std::simd::u32x8;

// Marker trait for safe bitwise conversions
pub unsafe trait SafeFromBits<Other> {}
unsafe impl SafeFromBits<u32x8> for __m256i {}

mod simd {
    #[allow(unions_with_drop_fields)]
    union U<A, B> {
        a: A,
        b: B,
    }
    pub const fn transmute<A, B: ::SafeFromBits<A>>(x: A) -> B {
        unsafe { U::<A, B> { a: x }.b }
    }
}

static FOO: __m256i = simd::transmute(u32x8::new(1, 2, 3, 4, 5, 6, 7, 8));

fn main() {
    println!("{:?}", FOO);
}
