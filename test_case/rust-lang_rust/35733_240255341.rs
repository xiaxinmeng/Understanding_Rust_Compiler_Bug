 rust
struct IntoIter<T> {
  buf: Shared<T>,
  cap: usize,
  head: Shared<T>,
  tail: Shared<T>,
}
