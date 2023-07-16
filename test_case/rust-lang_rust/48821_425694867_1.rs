console
error[E0658]: statements in constants are unstable (see issue #48821)
 --> src/main.rs:7:5
  |
7 |     s.0 = 1;
  |     ^^^^^^^
  |
  = help: add #![feature(const_let)] to the crate attributes to enable
