command
> rustc +1.0.0-gnu -C help | echo
ECHO is on.
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread '<main>' panicked at 'failed printing to stdout: The pipe is being closed.
 (os error 232)', C:/bot/slave/stable-dist-rustc-win-64/build/src/libstd\io\stdio.rs:416
