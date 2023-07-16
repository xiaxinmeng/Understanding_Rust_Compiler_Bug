
error[E0308]: mismatched types
 --> src/main.rs:4:11
  |
4 |         f(s);
  |           ^
  |           |
  |           expected `usize`, found `&{integer}`
  |           help: consider dereferencing the borrow: `*s`
