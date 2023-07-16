
error: range endpoint is out of range for `u8`
 --> src/main.rs:4:14
  |
4 |     for i in 0..256 {
  |              ^^^^^^ help: use an inclusive range instead: `0..=255`
  |
  = note: #[deny(overflowing_literals)] on by default
