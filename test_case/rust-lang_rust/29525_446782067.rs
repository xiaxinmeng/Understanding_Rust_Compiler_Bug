
error: local variables in const fn are unstable
 --> src/lib.rs:1:13
  |
1 | const fn i((a, b): (u32, u32)) -> u32 { a + b }
  |             ^
