
$ cargo clean
$ cargo +1.29.0 build
   Compiling xz v0.1.0 (file:///private/tmp/xz)
warning: unused `std::result::Result` which must be used
 --> src/main.rs:4:5
  |
4 |     (&mut example).write_fmt(format_args!("{}", 42));
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_must_use)] on by default
  = note: this `Result` may be an `Err` variant, which should be handled

    Finished dev [unoptimized + debuginfo] target(s) in 0.56s
