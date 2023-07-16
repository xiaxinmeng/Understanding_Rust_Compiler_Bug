
~/Downloads $ rustc test.rs
@ref771 = internal unnamed_addr constant %str_slice { i8* getelementptr inbounds ([0 x i8]* @str770, i32 0, i32 0), i64 0 }
{ i8*, i8* } undef
error: internal compiler error: const expr(8: &*"") of type &'static str has size 8 instead of 16
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/beta-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:209


~/Downloads $ rustc --version
rustc 1.0.0-beta.2 (e9080ec39 2015-04-16) (built 2015-04-16)
