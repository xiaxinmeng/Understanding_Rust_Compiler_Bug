 rust
error[E0308]: mismatched types
 --> foo.rs:2:19
  |
2 |     let y: &i32 = 5.0f64;
  |                   ^^^^^^ expected &i32, found f64
  |
  = note: expected type `&i32`
  = note:    found type `f64`
