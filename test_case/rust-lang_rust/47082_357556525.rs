rust
enum Size {
  Unknown,
  Bounded(usize, usize),
  BoundedLower(usize),
  LargerThanUsize,
  Unbounded,
}
