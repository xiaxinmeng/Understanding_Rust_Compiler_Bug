
$ cargo check
    Checking temp v0.1.0 (/tmp/temp.fCUY)
thread 'main' panicked at 'index out of bounds: the len is 4129 but the index is 4129', /rustc/0b9f19dff1347e29bf4362ab5a8fab84b43023b5/src/libcore/slice/mod.rs:2453:14
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.32.0-nightly (0b9f19dff 2018-11-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `temp`.

To learn more, run the command again with --verbose.
