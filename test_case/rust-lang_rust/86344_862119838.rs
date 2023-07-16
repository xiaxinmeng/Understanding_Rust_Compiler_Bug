rust
impl<T> MaybeUninit<T> {
  pub fn write(&mut self, val: T) -> &mut T
}
