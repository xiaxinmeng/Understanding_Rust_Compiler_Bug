 rust
pub enum RangeInclusive<T> {
  Empty {
    at: T,
  },
  NonEmpty {
    start: T,
    end: T,
  }
}
