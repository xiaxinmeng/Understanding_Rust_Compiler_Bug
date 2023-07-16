
error: internal compiler error: src/librustc_mir/util/pretty.rs:612: Unexpected def description Some(Variant(DefId(0/1:9 ~ playground[5604]::E[0]::V[0])))

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:620:9
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.2 (6c2484dc3 2019-05-13) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden
