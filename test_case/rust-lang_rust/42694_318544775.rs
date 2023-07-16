
error[E0277]: the trait bound `(): std::ops::Try` is not satisfied
 --> file.rs:6:5
  |
6 |     std::fs::File::open("foo")?;
  |     ---------------------------
  |     |
  |     the `?` operator can only be used in a function that returns `Result` (or another type that implements `std::ops::Try`)
  |     in this macro invocation
  |
  = help: the trait `std::ops::Try` is not implemented for `()`
  = note: required by `std::ops::Try::from_error`
