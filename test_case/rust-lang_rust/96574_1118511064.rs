rust
fn missing_avx_support() -> bool {
    !std::is_x86_feature_detected!("avx")
}

#[test]
#[ignore(if = missing_avx_support, reason = "missing AVX support")]
fn test_memcpy_avx() {
    // ...
}
