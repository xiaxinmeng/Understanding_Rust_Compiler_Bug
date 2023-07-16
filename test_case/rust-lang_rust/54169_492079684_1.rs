
Compiling playground v0.0.1 (/playground)
error: internal compiler error: src/librustc/ty/subst.rs:489: Region parameter out of range when substituting in region 'a (root type=Some(ResultFut::<'a, 'a>)) (index=1)

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:572:9
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: aborting due to previous error

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.36.0-nightly (a9ec99f42 2019-05-13) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
