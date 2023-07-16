
$ cargo new foo
$ cd foo
$ echo 'libc = "0.2"' >> Cargo.toml
$ rustup run nightly cargo build
...
$ rustup run stable cargo build
   Compiling libc v0.2.11
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: Utf8Error { valid_up_to: 1 }', ../src/libcore/result.rs:746
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: Could not compile `libc`.

To learn more, run the command again with --verbose.
