
error[E0277]: the `?` operator can only be applied to values that implement `std::ops::Try`
 --> src/main.rs:2:5
  |
2 |     42?;
  |     ^^^ the `?` operator cannot be applied to type `{integer}`
  |
  = help: the trait `std::ops::Try` is not implemented for `{integer}`
  = note: required by `std::ops::Try::into_result`
