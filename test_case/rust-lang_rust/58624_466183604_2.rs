
error[E0277]: the trait bound `!: Foo` is not satisfied
 --> src/lib.rs:5:13
  |
5 | fn foo() -> impl Foo {
  |             ^^^^^^^^ the trait `Foo` is not implemented for `!`
  |
  = note: the return type of a function must have a statically known size
