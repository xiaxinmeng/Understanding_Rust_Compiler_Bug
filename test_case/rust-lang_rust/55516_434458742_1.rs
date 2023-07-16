
$ cargo +1.28.0 build
   Compiling xz v0.1.0 (file:///private/tmp/xz)
warning: unused `std::result::Result` which must be used
 --> src/main.rs:4:5
  |
4 |     write!(&mut example, "{}", 42);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_must_use)] on by default
  = note: this `Result` may be an `Err` variant, which should be handled
  = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

    Finished dev [unoptimized + debuginfo] target(s) in 1.02s

$ cargo clean
$ cargo +1.29.0 build
   Compiling xz v0.1.0 (file:///private/tmp/xz)
    Finished dev [unoptimized + debuginfo] target(s) in 1.09s
