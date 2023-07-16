rust
pub unsafe fn _mm256_shuffle_epi32<const MASK: i32>(a: __m256i) -> __m256i {
    static_assert_imm8!(MASK);
    let r: i32x8 = simd_shuffle8_param!(
        a.as_i32x8(),
        a.as_i32x8(),
        <const MASK: i32> [
            MASK as u32 & 0b11,
            (MASK as u32 >> 2) & 0b11,
            (MASK as u32 >> 4) & 0b11,
            (MASK as u32 >> 6) & 0b11,
            (MASK as u32 & 0b11) + 4,
            ((MASK as u32 >> 2) & 0b11) + 4,
            ((MASK as u32 >> 4) & 0b11) + 4,
            ((MASK as u32 >> 6) & 0b11) + 4,
        ],
    );
    transmute(r)
}
