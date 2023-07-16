rust
#![feature(platform_intrinsics, repr_simd)]

extern "platform-intrinsic" {
    fn simd_shuffle2<T, U>(x: T, y: T, idx: [u32; 2]) -> U;
}

#[repr(simd)]
#[derive(Debug, PartialEq)]
struct Simd2(u8, u8);

fn main() {
    unsafe {
        let _: Simd2 = inline_me(); 
    }
}

#[inline(always)]
unsafe fn inline_me() -> Simd2 {
    simd_shuffle2(Simd2(10, 11), Simd2(12, 13), [0, 1])
}
