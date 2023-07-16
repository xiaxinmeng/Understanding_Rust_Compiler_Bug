
rustc 1.19.0-dev (5dfcd85fd 2017-05-19)
error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is not satisfied
 --> test.rs:5:11
  |
5 | let sum = x + y;
  |           ^^^^^ the trait `std::ops::Add<std::option::Option<i8>>` is not implemented for `i8`
  |
  = note: no implementation for `i8 + std::option::Option<i8>`

error: aborting due to previous error
