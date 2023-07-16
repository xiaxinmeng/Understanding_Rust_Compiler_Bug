
error[E0369]: binary operation `+` cannot be applied to type `std::option::Option<i8>`
 --> src/main.rs:6:11
  |
6 | let sum = y + x;
  |           ^^^^^
  |
  = note: an implementation of `std::ops::Add` might be missing for `std::option::Option<i8>`
