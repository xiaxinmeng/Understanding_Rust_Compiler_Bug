
error: invalid reference to positional argument 1 (no arguments were given)
 --> src/main.rs:3:21
  |
3 |     println!("Hello {test:1$}!");
  |                     ^^^^^^--^
  |                           |
  |                           this width flag expects an `usize` argument at position 1, but no arguments were given
  |
  = note: positional arguments are zero-based
  = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
  