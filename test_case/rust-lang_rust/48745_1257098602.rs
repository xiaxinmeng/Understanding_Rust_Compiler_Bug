rust
use std::arch::x86_64::*;
use std::mem::transmute;

pub const FOO: __m256i = unsafe { transmute([1, 2, 3, 4, 5, 6, 7, 8]) };

#[target_feature(enable = "avx2")]
pub unsafe fn test() -> u32 {
    _mm256_extract_epi32(FOO, 3) as u32
}
