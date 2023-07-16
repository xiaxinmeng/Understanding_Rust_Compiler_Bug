
error[E0308]: mismatched types
 --> src/main.rs:4:13
  |
4 |     if x == y {
  |             ^
  |             |
  |             expected usize, found &usize
  |             help: consider dereferencing the borrow: `*y`
  |
  = note: expected type `usize`
             found type `&usize`

error[E0277]: the trait bound `&usize: std::cmp::PartialEq<usize>` is not satisfied
 --> src/main.rs:7:10
  |
7 |     if y == x {
  |          ^^ can't compare `&usize` with `usize`
  |
  = help: the trait `std::cmp::PartialEq<usize>` is not implemented for `&usize`

error[E0277]: the trait bound `{integer}: std::cmp::PartialEq<&{integer}>` is not satisfied
  --> src/main.rs:10:10
   |
10 |     if 3 == &3 {
   |          ^^ can't compare `{integer}` with `&{integer}`
   |
   = help: the trait `std::cmp::PartialEq<&{integer}>` is not implemented for `{integer}`

error[E0277]: the trait bound `&{integer}: std::cmp::PartialEq<{integer}>` is not satisfied
  --> src/main.rs:13:11
   |
13 |     if &3 == 3 {
   |           ^^ can't compare `&{integer}` with `{integer}`
   |
   = help: the trait `std::cmp::PartialEq<{integer}>` is not implemented for `&{integer}`
