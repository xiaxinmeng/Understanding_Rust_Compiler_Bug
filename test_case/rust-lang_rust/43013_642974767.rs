
error[E0277]: expected a `std::ops::Fn<(char,)>` closure, found `T`
 --> src/lib.rs:7:26
  |
7 |         if line.contains(query) {
  |                          ^^^^^ expected an `Fn<(char,)>` closure, found `T`
  |
  = help: the trait `std::ops::Fn<(char,)>` is not implemented for `T`
  = note: required because of the requirements on the impl of `std::ops::FnOnce<(char,)>` for `&T`
  = note: required because of the requirements on the impl of `std::str::pattern::Pattern<'_>` for `&T`
help: consider restricting type parameter `T`
  |
3 | fn search<'a, T: std::ops::Fn<(char,)>>(query: &T, contents: &'a str) -> Vec<&'a str> {
  |                ^^^^^^^^^^^^^^^^^^^^^^^
