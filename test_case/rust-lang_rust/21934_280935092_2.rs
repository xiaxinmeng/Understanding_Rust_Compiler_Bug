
error[E0277]: the trait bound `(): foo::Foo` is not satisfied
 --> src/main.rs:6:5
  |
6 |     foo::<()>();
  |     ^^^^^^^^^ the trait `foo::Foo` is not implemented for `()`
  |
  = note: required by `foo::foo`
