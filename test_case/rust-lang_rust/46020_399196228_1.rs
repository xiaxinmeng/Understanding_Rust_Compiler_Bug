
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:6:5
  |
5 |     let x = 0;
  |         -   - first assignment to `x`
  |         |
  |         consider changing this to `mut x`
6 |     x = 1;
  |     ^^^^^ cannot assign twice to immutable variable

error[E0596]: cannot borrow immutable item `x` as mutable
 --> src/main.rs:7:23
  |
5 |     let x = 0;
  |         - help: consider changing this to be mutable: `mut x`
6 |     x = 1;
7 |     std::mem::replace(&mut x, 2);
  |                       ^^^^^^ cannot borrow as mutable
