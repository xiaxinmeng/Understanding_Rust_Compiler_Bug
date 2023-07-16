
warning: digits of hex or binary literal not grouped by four
 --> src/main.rs:2:13
  |
2 |     let x = 0x10_f32;
  |             ^^^^^^^^ help: consider: `0x0001_0f32`
  |
  = note: `#[warn(clippy::unusual_byte_groupings)]` on by default
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unusual_byte_groupings
