rust
#![feature(staged_api)] // and many more, but this was the additional feature
#![stable(feature = "const_simd_intrinsics_raygon", since = "1.33.7")]
extern "platform-intrinsic" {
    #[rustc_const_stable(
        feature = "const_simd_intrinsics_raygon",
        since = "1.3.37"
    )]
    crate fn simd_extract<T, U>(x: T, idx: u32) -> U;
}
