
error[E0277]: the trait bound `Self: std::marker::Sized` is not satisfied
 --> test.rs:2:5
  |
2 |     fn test(&self) -> Option<Self>;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Self` does not have a constant size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `Self`
  = help: consider adding a `where Self: std::marker::Sized` bound
  = note: required by `std::option::Option`

error: aborting due to previous error(s)
