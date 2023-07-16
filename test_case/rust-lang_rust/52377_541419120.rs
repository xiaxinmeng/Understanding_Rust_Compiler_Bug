
   Compiling hello v0.1.0 (file:///Z:/hello)
error: incremental compilation: error canonicalizing path `Z:\hello\target\debug\incremental\hello-2tm11x4hww51g`: 传递给系统调用的数据区域太小。 (os error 122)

thread 'rustc' panicked at 'librustc\session\mod.rs:727: Trying to get session directory from IncrCompSession `NotInitialized`', librustc\session\mod.rs:1180:26
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.25.0 (84203cac6 2018-03-25) running on x86_64-pc-windows-msvc

error: Could not compile `hello`.

To learn more, run the command again with --verbose.
