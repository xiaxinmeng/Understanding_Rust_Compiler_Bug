
warning: casting the result of `i32::abs()` to u8
 --> src/main.rs:4:15
  |
4 |     let res = (l as i32 - r as i32).abs() as u8;
  |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: replace with: `(l as i32 - r as i32).unsigned_abs()`
  |
  = note: `#[warn(clippy::cast_abs_to_unsigned)]` on by default
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#cast_abs_to_unsigned

