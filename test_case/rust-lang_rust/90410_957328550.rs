rust
impl slice {
  fn split_at_mid(&self, mid: usize) -> Option<(&[T], &[T])>;
  fn split_at_mid_mut(&mut self, mid: usize) -> Option<(&mut [T], &mut [T])>;
}

impl str {
  fn split_at_mid(&self, mid: usize) -> Option<(&str, &str)>;
  fn split_at_mid_mut(&mut self, mid: usize) -> Option<(&mut str, &mut str)>;
}
