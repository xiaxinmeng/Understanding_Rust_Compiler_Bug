
error[E0277]: the trait bound `(): Foo` is not satisfied
 --> src/lib.rs:3:13
  |
3 | fn foo() -> impl Foo {
  |             ^^^^^^^^ the trait `Foo` is not implemented for `()`
  |
  = note: the return type of a function must have a statically known size
