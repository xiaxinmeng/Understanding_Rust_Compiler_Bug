
error: internal compiler error: src/librustc/dep_graph/graph.rs:674: try_mark_green() - Forcing the DepNode should have set its color

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:590:9
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.33.0-nightly (a7be40c65 2018-12-26) running on x86_64-apple-darwin

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `project`.

To learn more, run the command again with --verbose.
