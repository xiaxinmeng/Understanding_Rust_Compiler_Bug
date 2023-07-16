rust
error[E0308]: mismatched types
 --> src/main.rs:3:21
  |
3 |     let x: &[i32] = v[2..3];
  |            ------   ^^^^^^^
  |            |        |
  |            |        expected `&[i32]`, found slice `[i32]`
  |            |        help: consider borrowing here: `&v[2..3]`
  |            expected due to this
  