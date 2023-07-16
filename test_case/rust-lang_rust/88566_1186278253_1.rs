rust
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
#[target_feature(enable = "lzcnt")]
#[target_feature(enable = "bmi2")]
#[allow(unused_unsafe)]
/// This isn't in the hot path, so prefer dynamic dispatch over a generic `Write` output.
/// This is an AVX2-optimized newline search function that searches a 32-byte (256-bit) window
/// instead of scanning character-by-character (once aligned). This is a *safe* function, but must
/// be adorned with `unsafe` to guarantee it's not called without first checking for AVX2 support.
///
/// We need to explicitly enable lzcnt support for u32::leading_zeros() to use the `lzcnt`
/// instruction instead of an extremely slow combination of branching + BSR. We do not need to test
/// for lzcnt support before calling this method as lzcnt was introduced by AMD alongside SSE4a, long
/// before AVX2, and by Intel on Haswell.
///
/// BMI2 is explicitly opted into to inline the BZHI instruction; otherwise a call to the intrinsic
/// function is added and not inlined.
unsafe fn search256<W: Write>(bytes: &[u8], mut output: &mut W) -> Result<(), std::io::Error> {
