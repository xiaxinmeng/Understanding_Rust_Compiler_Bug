rust
  trait Factory {
      type Item = impl Debug;
      fn make() -> Self::Item;
  }
  