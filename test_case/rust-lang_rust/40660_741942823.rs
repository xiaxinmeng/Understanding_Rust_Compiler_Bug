
error[E0308]: mismatched types
 --> src/main.rs:4:13
  |
4 |     if x == y {
  |             ^
  |             |
  |             expected `usize`, found `&usize`
  |             help: consider dereferencing the borrow: `*y`

error[E0277]: can't compare `&usize` with `usize`
 --> src/main.rs:7:10
  |
7 |     if y == x {
  |          ^^ no implementation for `&usize == usize`
  |
  = help: the trait `PartialEq<usize>` is not implemented for `&usize`

error[E0277]: can't compare `{integer}` with `&{integer}`
  --> src/main.rs:10:10
   |
10 |     if 3 == &3 {
   |          ^^ no implementation for `{integer} == &{integer}`
   |
   = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`

error[E0277]: can't compare `&{integer}` with `{integer}`
  --> src/main.rs:13:11
   |
13 |     if &3 == 3 {
   |           ^^ no implementation for `&{integer} == {integer}`
   |
   = help: the trait `PartialEq<{integer}>` is not implemented for `&{integer}`
