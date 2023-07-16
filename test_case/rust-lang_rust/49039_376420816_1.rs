rust
error[E0277]: the trait bound `std::error::Error: std::marker::Sized` is not satisfied
 --> src/lib.rs:4:5
  |
4 |     Box::new(x)
  |     ^^^^^^^^ `std::error::Error` does not have a constant size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `std::error::Error`
  = note: required by `<std::boxed::Box<T>>::new`
