rust
#[cfg(all(target_feature = "avx", target_feature = "avx2"))]
pub mod avx;
