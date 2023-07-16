rust
enum ItemId {
    Real(DefId),
    Fake(usize),
}

impl Item {
  fn stability() -> Option<Stability> { ... }
}
