shell
error: literal out of range for i8
 --> src/main.rs:2:14
  |
2 |     let _x = 0b1110_0000i8;
  |              ^^^^^^^^^^^^^ help: consider using `u8` instead: `0b1110_0000u8`
  |
  = note: #[deny(overflowing_literals)] on by default
  = note: the literal `0b1110_0000i8` (decimal `224`) does not fit into an `i8` and will become `-32i8`
