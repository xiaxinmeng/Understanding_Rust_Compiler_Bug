
error[E0277]: the trait bound `str: std::marker::Sized` is not satisfied
 --> src/main.rs:3:5
  |
3 | let v = s[..2];                              // strange error message
  |     ^   ------ help: considering borrowing the slice: `&s[..2]`
  |     |
  |     `str` does not have a constant size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `str`
  = note: all local variables must have a statically known size
help: considering borrowing the slice
