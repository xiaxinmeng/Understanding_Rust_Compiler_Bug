
   Compiling playground v0.0.1 (file:///playground)
error[E0277]: the trait bound `&str: std::cmp::PartialEq<str>` is not satisfied
  --> src/main.rs:20:5
   |
20 |     Contains::contains(&v, a);
   |     ^^^^^^^^^^^^^^^^^^ can't compare `&str` with `str`
   |
   = help: the trait `std::cmp::PartialEq<str>` is not implemented for `&str`
   = note: required by `Contains::contains`
