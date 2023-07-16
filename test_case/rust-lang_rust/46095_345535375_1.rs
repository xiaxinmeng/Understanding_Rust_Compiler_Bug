
error[E0596]: cannot borrow immutable local variable `a` as mutable
 --> src/main.rs:8:5
  |
6 |     let a = [a1];
  |         - consider changing this to `mut a`
7 |     a[0].bar(); //compilation ok
8 |     a[j % 2].bar();
  |     ^ cannot borrow mutably
