
error[E0599]: cannot write into '&'static str'
 --> src/macros/mod.rs:484:12
  |
6 |     write!("{}_{}", x, y);
  |     -------^^^^^^^------- method not found in `&str`
  |
  = note: "{}_{}" must implement 'io::Write', 'fmt::Write' or have a 'write_fmt' method
  = help: you might want to insert a writer in front of this format string
