rust
pub fn simd_shuffle32<T, U, IDX: const [u32; 32]>(x: T, y: T) -> U { 
  extern "platform-intrinsic" {
    fn simd_shuffle32<T, U>(x: T, y: T, idx: [u32; 32]) -> U;
  }
  simd_shuffle32(x, y, IDX)
}
