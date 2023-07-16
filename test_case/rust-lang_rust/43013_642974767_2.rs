
error[E0271]: type mismatch resolving `<&T as std::ops::FnOnce<(char,)>>::Output == bool`
 --> src/lib.rs:7:17
  |
7 |         if line.contains(query) {
  |                 ^^^^^^^^ expected `()`, found `bool`
  |
  = note: required because of the requirements on the impl of `std::str::pattern::Pattern<'_>` for `&T`
