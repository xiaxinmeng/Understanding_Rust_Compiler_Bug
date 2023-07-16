
error[E0595]: closure cannot modify immutable argument `x`
 --> src/main.rs:4:6
  |
1 | fn foo(x: &mut i32) {
  |        - consider changing this to `mut x`
...
4 |     (|| {
  |      ^^ cannot borrow mutably
5 |        process(&mut x);
  |                     - `x` used here
