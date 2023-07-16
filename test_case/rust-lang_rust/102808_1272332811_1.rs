
    Checking playground v0.0.1 (/playground)
error: mistyped literal suffix
 --> src/main.rs:2:13
  |
2 |     let x = 0x10_32;
  |             ^^^^^^^ help: did you mean to write: `0x0010_i32`
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#mistyped_literal_suffixes
  = note: `#[deny(clippy::mistyped_literal_suffixes)]` on by default

error: could not compile `playground` due to previous error
