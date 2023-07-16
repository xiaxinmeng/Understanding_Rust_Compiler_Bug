
error[E0277]: the trait bound `(): Foo` is not satisfied
 --> src/main.rs:9:5
  |
9 |     requires_foo::<()>();
  |     ^^^^^^^^^^^^^^^^^^ the trait `Foo` is not implemented for `()`
  |
note: required by `requires_foo`
