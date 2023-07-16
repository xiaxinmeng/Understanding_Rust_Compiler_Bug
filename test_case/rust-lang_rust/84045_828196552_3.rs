rust
error[E0308]: mismatched types
 --> src/main.rs:5:9
  |
2 |     const Z: f64 = 1.00000011;
  |     -------------------------- constant defined here
3 |     let x: f32 = 1.0000001;
4 |     let y = match x {
  |                   - this expression has type `f32`
5 |         Z => true,
  |         ^
  |         |
  |         expected `f32`, found `f64`
