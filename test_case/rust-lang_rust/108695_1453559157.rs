rustc
warning: function `print_message` is never used
 --> src\utils\messagebox.rs:2:8
  |
2 | pub fn print_message(msg: &str) -> Result<i32, ()> {
  |        ^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default
