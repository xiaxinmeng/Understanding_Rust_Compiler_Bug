
error[E0265]: recursive constant
  --> src/test/compile-fail/issue-23302.rs:14:5
   |
14 |     A = X::A as isize, //~ ERROR E0265
   |         ^^^^^^^^^^^^^ recursion not allowed in constant
