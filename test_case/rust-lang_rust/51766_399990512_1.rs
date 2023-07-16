rust
impl<T> Add<*const T> for *const T {
  #[inline]
  fn add(self, off : *const T) -> Self {
    ((self as usize) + (off as usize)) as Self
  }
}

pub unsafe fn buggy(pos : *const u16) -> *const u16 {
  pos + (2 as _)
}
