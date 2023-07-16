
Compiling playground v0.0.1 (/playground)
error: this arithmetic operation will overflow
 --> src/main.rs:3:18
  |
3 |     let coordx = 960+x as u32;
  |                  ^^^^^^^^^^^^ attempt to compute `960_u32 + 4294967116_u32`, which would overflow
  |
  = note: `#[deny(arithmetic_overflow)]` on by default

error: could not compile `playground` due to previous error
