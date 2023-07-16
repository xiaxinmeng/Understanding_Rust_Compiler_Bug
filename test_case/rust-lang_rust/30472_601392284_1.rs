rust
struct BoolVec(Vec<bool>);

impl Iterable for BoolVec {
  type Item<'a> = &'a bool;
  type Iter<'a> = core::slice::Iter<'a, bool>;

  fn iter<'a>(&'a self) -> Self::Iter<'a> {
    self.0.iter()
  }
}
