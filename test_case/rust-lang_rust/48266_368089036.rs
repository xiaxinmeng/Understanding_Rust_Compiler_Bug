
$ RUSTFLAGS="-Z treat-err-as-bug" cargo +stage1 build
   Compiling foo v0.1.0 (file:///home/pietro/r/github/rust-lang/rust/pietro)
[...]

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.25.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z treat-err-as-bug -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `pietro`.

To learn more, run the command again with --verbose.
