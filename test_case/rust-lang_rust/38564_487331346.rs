
error[E0277]: cannot add `std::option::Option<i8>` to `i8`
 --> src/main.rs:5:13
  |
5 | let sum = x + y;
  |             ^ no implementation for `i8 + std::option::Option<i8>`
  |
  = help: the trait `std::ops::Add<std::option::Option<i8>>` is not implemented for `i8`
