
error[E0277]: the trait bound `(): Bar` is not satisfied
 --> src/lib.rs:3:13
  |
3 | fn foo() -> impl Bar {
  |             ^^^^^^^^ the trait `Bar` is not implemented for `()`
4 |     5;
  |      - consider removing this semicolon
  |
  = note: the return type of a function must have a statically known size
