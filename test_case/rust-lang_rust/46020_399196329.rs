
error[E0596]: cannot borrow immutable local variable `x` as mutable
 --> src/main.rs:6:28
  |
4 |     let x = 0;
  |         - consider changing this to `mut x`
5 |     x = 1;
6 |     std::mem::replace(&mut x, 2);
  |                            ^ cannot borrow mutably

error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:5:5
  |
4 |     let x = 0;
  |         - first assignment to `x`
5 |     x = 1;
  |     ^^^^^ cannot assign twice to immutable variable
