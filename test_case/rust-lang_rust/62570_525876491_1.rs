
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
 --> src/main.rs:3:9
  |
3 |         Err(5)?;
  |         ^^^^^^^ cannot use the `?` operator in a function that returns `{integer}`
  |
  = help: the trait `std::ops::Try` is not implemented for `{integer}`
  = note: required by `std::ops::Try::from_error`
