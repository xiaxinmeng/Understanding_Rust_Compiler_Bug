
warning: panic message is not a string literal
 --> src/main.rs:3:25
  |
3 |     assert!(red >= 0.0, red <= 1.0);
  |                         ^^^^^^^^^^
  |
  = note: `#[warn(non_fmt_panic)]` on by default
  = note: this is no longer accepted in Rust 2021
help: add a "{}" format string to Display the message
  |
3 |     assert!(red >= 0.0, "{}", red <= 1.0);
  |                         ^^^^^
