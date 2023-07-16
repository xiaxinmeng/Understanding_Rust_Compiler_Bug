
error: 2 positional arguments in format string, but no arguments were given
 --> src/test/ui/issues/issue-66065.rs:2:15
  |
2 |     println!("{:.*}");
  |               ^^--^
  |                 |
  |                 this precision flag adds an extra required argument at position 0, which is why there are 2 arguments expected
  |
  = note: positional arguments are zero-based
  = note: for information about formatting flags, visit https://doc.rust-lang.org/std/fmt/index.html
