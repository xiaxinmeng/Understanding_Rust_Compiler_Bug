rust
impl<T> *const T {
  const unsafe fn offset_from(self, origin: *const T) -> isize
}
