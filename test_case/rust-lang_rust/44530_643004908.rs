
error[E0277]: the trait bound `{integer}: T` is not satisfied
 --> src/lib.rs:6:13
  |
6 | fn foo() -> impl T {
  |             ^^^^^^ the trait `T` is not implemented for `{integer}`
7 |     return 1;
  |            - this returned value is of type `{integer}`
  |
  = note: the return type of a function must have a statically known size
