
error[E0277]: the trait bound `&str: std::iter::Iterator` is not satisfied
 --> src/main.rs:2:14
  |
2 |     for c in "foobarbaz" {
  |              ^^^^^^^^^^^ `&str` is not an iterator; try calling `.chars()` or `.bytes()`
  |
  = help: the trait `std::iter::Iterator` is not implemented for `&str`
  = note: required by `std::iter::IntoIterator::into_iter`
