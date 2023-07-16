
error[E0774]: `derive` may only be applied to `struct`s, `enum`s and `union`s
 --> src/main.rs:3:5
  |
3 |     #[derive(Debug)]
  |     ^^^^^^^^^^^^^^^^ not applicable here
4 |     pub type Bar;
  |     ------------- not a `struct`, `enum` or `union`
  