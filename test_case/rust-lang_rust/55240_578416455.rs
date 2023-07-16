
warning: unused `std::result::Result` that must be used
 --> /home/sibi/foo.rs:7:9
  |
7 |         write!(fmt, "ignored result\n");
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_must_use)]` on by default
  = note: this `Result` may be an `Err` variant, which should be handled
  = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

warning: unused `std::result::Result` that must be used
  --> /home/sibi/foo.rs:11:9
   |
11 |         closure(); // also ignoring here
   |         ^^^^^^^^^^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
