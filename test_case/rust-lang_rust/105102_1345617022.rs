rust
impl Copy for Adt {
  fn copy(&self) -> Self {
    Adt {
      field0: field0.copy(),
      field1: field1.copy(),
      [...]
    }
  }
}
