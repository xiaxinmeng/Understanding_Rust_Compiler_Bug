
error[E0277]: the trait bound `S: T` is not satisfied
 --> src/lib.rs:6:13
  |
6 | fn foo() -> impl T {
  |             ^^^^^^ the trait `T` is not implemented for `S`
7 |     S
  |     - this returned value is of type `S`
  |
  = note: the return type of a function must have a statically known size
