
error[E0277]: the trait bound `T: std::ops::Fn<(char,)>` is not satisfied
 --> src/main.rs:5:17
  |
5 |         if line.contains(query) {
  |                 ^^^^^^^^ the trait `std::ops::Fn<(char,)>` is not implemented for `T`
  |
  = help: consider adding a `where T: std::ops::Fn<(char,)>` bound
  = note: required because of the requirements on the impl of `std::ops::FnOnce<(char,)>` for `&T`
  = note: required because of the requirements on the impl of `std::str::pattern::Pattern<'_>` for `&T`
