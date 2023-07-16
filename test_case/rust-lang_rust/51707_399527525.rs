
   Compiling playground v0.0.1 (file:///playground)
error: internal compiler error: librustc_mir/hair/cx/expr.rs:554: invalid loop id for continue: not inside loop scope

thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:554:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.27.0 (3eda71b00 2018-06-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `playground`.

To learn more, run the command again with --verbose.
