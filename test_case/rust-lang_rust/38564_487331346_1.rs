
error[E0369]: binary operation `+` cannot be applied to type `std::option::Option<i8>`
 --> src/main.rs:5:13
  |
5 | let sum = y + x;
  |           - ^ - i8
  |           |
  |           std::option::Option<i8>
  |
  = note: an implementation of `std::ops::Add` might be missing for `std::option::Option<i8>`
