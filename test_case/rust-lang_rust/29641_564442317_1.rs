rust
error: to use a constant of type `Foo` in a pattern, `Foo` must be annotated with `#[derive(PartialEq, Eq)]`
  --> src/lib.rs:12:12
   |
12 |     if let FOUR = x {}
   |            ^^^^
