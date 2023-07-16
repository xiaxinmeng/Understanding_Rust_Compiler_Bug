
error[E0277]: Adding incompatible types
 -->
  |
7 | let sum = x + y;
  |           ^^^^^ can't add i8 and std::option::Option<i8>
  |
= note: trait `std::ops::Add<std::option::Option<i8>>` is not implemented for `i8`
