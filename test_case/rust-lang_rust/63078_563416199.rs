
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
 --> src/lib.rs:2:5
  |
1 | / fn bar() {
2 | |     foo()?;
  | |     ^^^^^^ cannot use the `?` operator in a function that returns `()`
3 | | }
  | |_- this function should return `Result` or `Option` to accept `?`
  |
  = help: the trait `std::ops::Try` is not implemented for `()`
  = note: required by `std::ops::Try::from_error`
