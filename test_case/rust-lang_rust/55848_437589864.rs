rust
#![cfg_attr(not(dox), feature(cfg_target_feature, target_feature, stdsimd))]
#[cfg(not(dox))]
#[macro_use]
extern crate stdsimd;
fn main() {
    let mut dst = [0];
    add_quickly(&[1], &[2], &mut dst);
    assert_eq!(dst[0], 3);
}
fn add_quickly(a: &[u8], b: &[u8], c: &mut [u8]) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        // Note that this `unsafe` block is safe because we're testing
        // that the `avx2` feature is indeed available on our CPU.
        if is_x86_feature_detected!("avx2") {
            return unsafe { add_quickly_avx2(a, b, c) };
        }
    }
    add_quickly_fallback(a, b, c)
}
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn add_quickly_avx2(a: &[u8], b: &[u8], c: &mut [u8]) {
    add_quickly_fallback(a, b, c) // the function below is inlined here
}
fn add_quickly_fallback(a: &[u8], b: &[u8], c: &mut [u8]) {
    for ((a, b), c) in a.iter().zip(b).zip(c) {
        *c = *a + *b;
    }
}

/// 