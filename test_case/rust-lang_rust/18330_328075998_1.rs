
error[E0595]: closure cannot assign to immutable argument `x`
 --> test.rs:2:3
  |
1 | fn f(x: Option<()>) {
  |      - consider changing this to `mut x`
2 |     (|| x.take())();
  |   ^^ cannot borrow mutably

error: aborting due to previous error
