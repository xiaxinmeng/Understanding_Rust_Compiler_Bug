text
error: literal out of range for i8
 --> src/main.rs:4:28
  |
4 |     println!("{} {}", i == 0x80, j == 0x80);
  |                            ^^^^
  |
  = note: `#[deny(overflowing_literals)]` on by default
  = note: the literal `0x80` (decimal `128`) does not fit into an `i8` and will become `-128i8`
  = help: consider using `u8` instead
