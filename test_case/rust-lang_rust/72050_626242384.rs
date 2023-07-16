
error[E0507]: cannot move out of a shared reference
 --> src/main.rs:4:20
  |
4 |     for &(a, _) in &tuples {
  |         -------    ^^^^^^^
  |         | |
  |         | data moved here
  |         | move occurs because `a` has type `S`, which does not implement the `Copy` trait
  |         help: consider removing the `&`: `(a, _)`
