rust
mod intrinsics {
  #[inline(always)]
  pub unsafe fn add_with_overflow<T>(x: T, y: T) -> (T, bool) {
    extern "rust-intrinsic" { fn add_with_overflow<T>(x: T, y: T) -> (T, bool); }
    add_with_overflow(x, y)
  } 
}
