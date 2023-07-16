
warning: unused arithmetic operation that must be used
 --> src/main.rs:4:5
  |
4 |     1 + 2;
  |     ^^^^^ the arithmetic operation produces a value
  |
  = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
  |
4 |     let _ = 1 + 2;
  |     +++++++
