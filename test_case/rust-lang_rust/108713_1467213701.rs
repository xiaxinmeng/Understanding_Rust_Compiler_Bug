
error[E0599]: cannot write into '&'static str'
 --> src/macros/mod.rs:484:12
  |
6 |     write!("{}_{}", x, y);
  |     -------^^^^^^^------- method not found in `&str`
  |
note: must implement 'io::Write', 'fmt::Write' or have a 'write_fmt' method
 --> src/macros/mod.rs:484:12
  |
6 |     write!("{}_{}", x, y);
  |            ^^^^^^^
help: you might want to insert a writer in front of this format string
 --> src/macros/mod.rs:484:12
  |
6 |     write!("{}_{}", x, y);
  |            ^

error: aborting due to 2 previous errors
