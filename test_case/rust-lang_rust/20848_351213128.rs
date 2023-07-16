
error[E0277]: the trait bound `str: std::marker::Sized` is not satisfied
 --> src/main.rs:3:9
  |
3 |     let x = y[1..2];
  |         ^   ------- help: consider borrowing here: `&y[1..2]`
  |         |
  |         `str` does not have a constant size known at compile-time
  |
  = help: the trait `std::marker::Sized` is not implemented for `str`
  = note: all local variables must have a statically known size
