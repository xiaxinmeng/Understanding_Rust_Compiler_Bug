
error[E0277]: Adding incompatible types
 -->
  |
7 | let sum = x + y;
  |           ^^^^^ can't add i8 and Option<i8>
  |
= note: trait `ops::Add<Option<i8>>` is not implemented for `i8`
