rust
pub unsafe fn _mm256_extract_epi64<const INDEX: i32>(a: __m256i) -> i64 {
    static_assert_imm2!(INDEX);
    simd_extract(a.as_i64x4(), INDEX as u32)
}
