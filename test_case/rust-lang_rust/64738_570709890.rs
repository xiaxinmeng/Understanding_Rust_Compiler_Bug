rust
extern "platform-intrinsic" {
    crate fn simd_insert<T, U>(x: T, idx: u32, val: U) -> T;
    crate fn simd_extract<T, U>(x: T, idx: u32) -> U;
}
