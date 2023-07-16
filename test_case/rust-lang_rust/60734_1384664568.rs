
error[E0277]: the trait bound `&T: Foo` is not satisfied
 --> src/main.rs:9:9
  |
9 |     foo(&t)
  |     --- ^^ the trait `Foo` is not implemented for `&T`
  |     |
  |     required by a bound introduced by this call
  |
note: required by a bound in `foo`
 --> src/main.rs:5:11
  |
5 | fn foo<T: Foo>(t: &T) {
  |           ^^^ required by this bound in `foo`
help: consider removing the leading `&`-reference
  |
9 -     foo(&t)
9 +     foo(t)
  |
