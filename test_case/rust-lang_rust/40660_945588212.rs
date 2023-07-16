
error[E0308]: mismatched types
  --> f16.rs:18:13
   |
18 |     if x == y {
   |             ^ expected `usize`, found `&usize`
   |
help: consider dereferencing the borrow
   |
18 |     if x == *y {
   |             +

error[E0369]: can't compare `&usize` with `usize`
  --> f16.rs:21:10
   |
21 |     if y == x {
   |        - ^^ - usize
   |        |
   |        &usize
   |
help: `==` can be applied on `&usize`
   |
21 |     if y == &x {
   |             +
help: `==` can be used on `usize`, you can dereference `y`
   |
21 |     if *y == x {
   |        +

error[E0277]: can't compare `{integer}` with `&{integer}`
  --> f16.rs:24:10
   |
24 |     if 3 == &3 {
   |          ^^ no implementation for `{integer} == &{integer}`
   |
   = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`

error[E0277]: can't compare `&{integer}` with `{integer}`
  --> f16.rs:27:11
   |
27 |     if &3 == 3 {
   |           ^^ no implementation for `&{integer} == {integer}`
   |
   = help: the trait `PartialEq<{integer}>` is not implemented for `&{integer}`
