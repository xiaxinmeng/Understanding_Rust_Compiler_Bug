
error: internal compiler error: src/librustc/middle/region.rs:1037: Encountered greater count 20 at span src/main.rs:7:9: 7:26 - expected no greater than 12

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:905:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0 (5e1a79984 2020-01-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: aborting due to previous error
