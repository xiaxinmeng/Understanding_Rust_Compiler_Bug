rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub struct TwoVecs(__m256i, __m256i);

#[target_feature(enable = "avx2")]
pub unsafe fn avx2_any_upper_bit_set(vecs: &TwoVecs) -> bool {
    inlined_non_avx2(&vecs)
}

#[inline]
unsafe fn inlined_non_avx2(
    vecs: &TwoVecs
) -> bool {
    non_inlined_non_avx2(vecs)
}

#[inline(never)]
unsafe fn non_inlined_non_avx2(
    vecs: &TwoVecs,
) -> bool {
    let eq = _mm256_or_si256(vecs.0, vecs.1);
    _mm256_movemask_epi8(eq) != 0
}

pub fn main() {
    unsafe {
        let vecs = TwoVecs(_mm256_set1_epi8(0x00_u8 as i8), _mm256_set1_epi8(0x00_u8 as i8));
        assert!(!avx2_any_upper_bit_set(&vecs));
    }
}
