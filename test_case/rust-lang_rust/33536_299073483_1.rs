
warning: variable `a` is assigned to, but never used
 --> test.rs:2:9
  |
2 |     let mut a = 10;
  |         ^^^^^
  |
  = note: #[warn(unused_variables)] on by default

warning: value assigned to `a` is never read
 --> test.rs:3:5
  |
3 |     a = 15;
  |     ^
  |
  = note: #[warn(unused_assignments)] on by default
