
error[E0277]: `[i32]` is not an iterator
 --> src/lib.rs:2:14
  |
2 |     for i in x[1..] {
  |              ^^^^^^ expected an implementor of trait `IntoIterator`
  |
  = note: the trait bound `[i32]: IntoIterator` is not satisfied
  = note: required because of the requirements on the impl of `IntoIterator` for `[i32]`
help: consider borrowing here
  |
2 |     for i in &x[1..] {
  |              +
2 |     for i in &mut x[1..] {
  |              ++++
