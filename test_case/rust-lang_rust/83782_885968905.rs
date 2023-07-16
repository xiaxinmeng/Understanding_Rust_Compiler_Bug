
error[E0277]: the trait bound `A: T` is not satisfied
 --> src/lib.rs:7:11
  |
3 | trait T {
  | ------- required by this bound in `T`
...
7 | fn foo(a: <A as T>::Item) {
  |           ^^^^^^^^^^^^^^ the trait `T` is not implemented for `A`
