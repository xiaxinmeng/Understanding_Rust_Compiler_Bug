rust
#[allow(unreachable_code)]
fn search_auto<W: Write>(bytes: &[u8], mut output: &mut W) -> Result<(), std::io::Error> {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if is_x86_feature_detected!("avx2") {
        return unsafe { search256(bytes, &mut output) };
    }

    #[cfg(all(feature = "nightly", target_arch = "aarch64"))]
    return search128(bytes, &mut output);

    search(bytes, &mut output)
}
