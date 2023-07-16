rust
fn merry_christmas(x: Box<Into<__m256i>>) {
    if !cfg_target_feature!("avx") { return; }
    unsafe { _mm256_castsi256_ps(x.into()) }; // unsound ?
}
