
error: closure cannot assign to immutable local variable `x`
 --> test.rs:5:5
  |
4 |     let x = 0;
  |         - consider changing this to `mut x`
5 |     || set(&mut x);
  |     ^^ cannot borrow mutably
