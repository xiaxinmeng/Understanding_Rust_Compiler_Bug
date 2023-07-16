rust
#![feature(platform_intrinsics, repr_simd)]
#[allow(non_camel_case_types)]

#[repr(simd)]
pub struct u8x32([u8; 32]);

extern "platform-intrinsic" {
    fn simd_shuffle32<T, U>(x: T, y: T, idx: [u32; 32]) -> U;
}

pub fn right_shift_1(left: u8x32, right: u8x32) -> u8x32 {
    const IDX: [u32; 32] = {
        let mut a = [31u32; 32];
        let mut n: u32 = 0;
        while n < 32 {
            a[n as usize] += n;
            n +=1;
        }
        a
    };
    unsafe { simd_shuffle32(left, right, IDX) }
}
