rust
trait TryIterator {
  type Item;
  fn try_next(&mut self) -> Result<Option<Self::Item>>;
}

impl<T> Iterator for T
where T: TryIterator {
  type Item = T::Item;
  fn next(&mut self) -> Option<Result<Self::Item>> {
    self.try_next().transpose()
  }
}
