rust
extern "rust-intrinsic" {
    fn simd_insert_new<T, U, const N: u32>(t: T, u: U) -> T
}
