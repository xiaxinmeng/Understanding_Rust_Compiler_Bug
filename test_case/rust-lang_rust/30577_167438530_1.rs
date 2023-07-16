
{ i8*, void (i8*)** } { i8* null, void (i8*)** getelementptr inbounds ({ void (i8*)*, i64, i64 }, { void (i8*)*, i64, i64 }* @vtable2631, i32 0, i32 0) }
{} undef
error: internal compiler error: const expr(18: &mut DangerDummy as *mut DangerDummy) of type DangerDummy has size 16 instead of 0
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', ../src/libsyntax/diagnostic.rs:253

playpen: application terminated with error code 101
