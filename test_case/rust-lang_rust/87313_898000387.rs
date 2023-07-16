
warning: panic message is not a string literal
 --> src/main.rs:8:12
  |
8 |     panic!(f);
  |            ^
  |
  = note: `#[warn(non_fmt_panics)]` on by default
  = note: this usage of panic!() is deprecated; it will be a hard error in Rust 2021
  = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/panic-macro-consistency.html>
help: add a "{}" format string to Display the message
  |
8 |     panic!("{}", f);
  |            +++++
help: or use std::panic::panic_any instead
  |
8 |     std::panic::panic_any(f);
  |     ~~~~~~~~~~~~~~~~~~~~~
