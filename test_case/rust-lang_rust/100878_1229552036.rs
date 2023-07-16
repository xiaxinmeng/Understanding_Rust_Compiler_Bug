`
error: this arithmetic operation will overflow
 --> fixed/100878.rs:3:5
  |
3 |     data[0] << 8
  |     ^^^^^^^^^^^^ attempt to shift left by `8_i32`, which would overflow
  |
  = note: `#[deny(arithmetic_overflow)]` on by default

error: aborting due to previous error
