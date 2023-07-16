
#[runtime_target_feature("+avx")]
pub fn sum(input: &[u32]) -> u32 {
    #[cfg(target_feature = "avx")]
    { /* write some assembly using avx */ }

    #[cfg(not(target_feature = "avx"))]
    { /* fallback code */ input.iter().sum()}
}
