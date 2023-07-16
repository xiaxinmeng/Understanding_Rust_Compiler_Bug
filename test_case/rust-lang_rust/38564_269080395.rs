plain
error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is not satisfied
 -->
  |
7 | let sum = x + y;
  |           ^   - `std::option::Option<i8>`
  |           |   
  |          `i8`
