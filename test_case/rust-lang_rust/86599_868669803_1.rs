
error: format argument must be a string literal
 --> test.rs:2:14
  |
2 |     println!(include_str("format"), 42);
  |              ^^^^^^^^^^^^^^^^^^^^^
  |
help: you might be missing a string literal to format with
  |
2 |     println!("{} {}", include_str("format"), 42);
  |              ^^^^^^^^
