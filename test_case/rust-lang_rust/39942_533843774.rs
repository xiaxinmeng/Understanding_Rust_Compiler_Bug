
error[E0277]: can't compare `&{integer}` with `{integer}`
 --> src/main.rs:6:10
  |
6 |     if a == b {
  |          ^^ no implementation for `&{integer} == {integer}`
  |
  = help: the trait `std::cmp::PartialEq<{integer}>` is not implemented for `&{integer}`
