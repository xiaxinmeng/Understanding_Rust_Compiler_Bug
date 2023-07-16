
error[E0277]: the trait bound `(): Bar` is not satisfied
 --> src/main.rs:9:5
  |
9 |     requires_foo::<()>();
  |     ^^^^^^^^^^^^^^^^^^ the trait `Bar` is not implemented for `()`
  |
  = note: required because of the requirements on the impl of `Foo` for `()`
note: required by `requires_foo`
