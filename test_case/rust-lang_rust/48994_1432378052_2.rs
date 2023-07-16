
trait ExpectedSize {
  fn expected_size_lower_bound(&self) -> usize;
}

impl<I: Iterator> ExpectedSize for I {
  fn expected_size_lower_bound(&self) -> usize {
    self.size_hint().0
  }
}

// And here I get compilation errors.
impl<I, R> ExpectedSize for GenericShunt<I, R> { ... }
