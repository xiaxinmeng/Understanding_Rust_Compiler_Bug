
error[E0277]: the `?` operator can only be used in a function that returns `Result` (or another type that implements `std::ops::Try`)
 --> src/main.rs:3:5
  |
3 |     std::fs::File::open("foo")?;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a function that returns `()`
  |
  = help: the trait `std::ops::Try` is not implemented for `()`
  = note: required by `std::ops::Try::from_error`
