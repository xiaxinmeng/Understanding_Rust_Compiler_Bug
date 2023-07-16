
warning: casting function pointer `Thing::Item` to `u32`, which truncates the value
 --> src/main.rs:8:25
  |
8 |     let uninitialized = Thing::Item as u32;
  |                         ^^^^^^^^^^^^^^^^^^ help: try: `Thing::Item as usize`
  |
  = note: `#[warn(clippy::fn_to_numeric_cast_with_truncation)]` on by default
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#fn_to_numeric_cast_with_truncation
