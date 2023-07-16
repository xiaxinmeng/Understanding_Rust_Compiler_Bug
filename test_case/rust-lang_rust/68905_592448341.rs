rust
// Detection code based on the std_detect crate
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn avx512f_avx512vl_detected() -> bool {
    use std::arch::x86_64::*;

    // Check for avx2 to ensure all needed CPUID features are available.
    if !is_x86_feature_detected!("avx2") {
        return false;
    }

    unsafe {
        // Check that the OS supports AVX512
        let xcr0 = _xgetbv(0);
        let os_avx512_support = xcr0 & 224 == 224;
        if !os_avx512_support {
            return false;
        }

        // Check that the CPU supports AVX512F and AVX512VL
        let CpuidResult { ebx, .. } = __cpuid(7);
        let avx512f = ebx & 1 << 16 != 0;
        let avx512vl = ebx & 1 << 31 != 0;
        avx512f && avx512vl
    }
}
