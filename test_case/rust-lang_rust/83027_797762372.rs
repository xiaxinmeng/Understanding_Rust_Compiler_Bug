rust
#[cfg(target_arch = "x86")]
use std::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

extern "C" {
    fn black_box(a: *const u8);
}

pub fn foo() {
    #[target_feature(enable = "avx2")]
    unsafe fn test() {
        let a = _mm256_set_epi32(1, 1, 1, 1, 1, 1, 1, 1);
        let b = _mm256_set_epi32(2, 2, 2, 2, 2, 2, 2, 2);

        let e = _mm256_set_epi32(3, 3, 3, 3, 3, 3, 3, 3);
        let r = _mm256_add_epi32(a, b);

        assert_eq_m256i(e, r);
    }

    if is_x86_feature_detected!("avx2") {
        unsafe { test() }
    } else {
        loop {}
    }
}

#[target_feature(enable = "avx")]
pub unsafe fn assert_eq_m256i(a: __m256i, b: __m256i) {
    black_box(&a as *const _ as *const _);
    black_box(&b as *const _ as *const _);
}
