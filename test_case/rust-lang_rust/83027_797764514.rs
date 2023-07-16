rust
#[target_feature(enable = "avx2")]
pub unsafe fn _mm256_add_epi32(a: __m256i, b: __m256i) -> __m256i { ... }
