 rust

enum Foo {
  A,
  B,
  C
}

match (x, y) {
  (_, A) => {},
  (A, _) => {} // We know that the `_` can't be A here because otherwise the earlier case would have been taken
}
