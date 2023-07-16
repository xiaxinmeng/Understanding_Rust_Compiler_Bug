
error[E0277]: the trait bound `(): Bar` is not satisfied
 --> src/main.rs:4:13
  |
4 | fn baz() -> impl Bar {
  |             ^^^^^^^^ the trait `Bar` is not implemented for `()`
5 |     true;
  |         - consider removing this semicolon
  |
  = note: the return type of a function must have a statically known size
