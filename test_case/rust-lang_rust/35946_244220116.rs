
error[E0277]: the trait bound `(): std::ops::Carrier` is not satisfied
 --> test.rs:4:5
  |
4 |     Ok::<()>(42)?;
  |     ^^^^^^^^^^^^^ trait `(): std::ops::Carrier` not satisfied
  |
  = note: the return type `()` cannot be used with the `?` operator
  = note: required by `std::ops::Carrier::from_error`
