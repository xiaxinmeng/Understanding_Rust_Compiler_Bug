rust
error[E0308]: mismatched types
 --> src/main.rs:5:9
  |
4 |     assert!(match x {
  |                   - this expression has type `f32`
5 |         const { 1.00000011 } => true,
  |         ^^^^^^^^^^^^^^^^^^^^ expected `f32`, found `f64`

