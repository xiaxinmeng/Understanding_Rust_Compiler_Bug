
error[E0277]: cannot add `std::option::Option<i8>` to `i8`
 --> src/main.rs:5:13
  |
5 | let sum = x + y;
  |             ^ no implementation for `i8 + std::option::Option<i8>`
  |
  = help: the trait `std::ops::Add<std::option::Option<i8>>` is not implemented for `i8`

error[E0369]: cannot add `i8` to `std::option::Option<i8>`
 --> src/main.rs:5:13
  |
5 | let sum = y + x;
  |           - ^ - i8
  |           | |
  |           | no implementation for `std::option::Option<i8> + i8`
  |           std::option::Option<i8>
  |
  = note: an implementation of `std::ops::Add<i8>` might be missing for `std::option::Option<i8>`
