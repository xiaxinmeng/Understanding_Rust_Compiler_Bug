
trait ExpectedSize: Iterator {
  fn expected_size_lower_bound(&self) -> usize;
}
