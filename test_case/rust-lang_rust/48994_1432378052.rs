rust
trait Iterator {
  // unsable, doc-hidden etc
  fn expected_size_lower_bound(&self) -> usize {
    self.size_hint().0
  }
}
