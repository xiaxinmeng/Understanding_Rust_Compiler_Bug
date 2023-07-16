rust
[00:21:41] [0m[0m[1m[32m   Compiling[0m chalk-engine v0.6.0
[00:21:41] thread 'main' panicked at 'assertion failed: self.lifetimes_to_define.is_empty()', librustc/hir/lowering.rs:658:9
[00:21:41] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:21:41] 
[00:21:41] error: internal compiler error: unexpected panic
[00:21:41] 
[00:21:41] note: the compiler unexpectedly panicked. this is a bug.
[00:21:41] 
[00:21:41] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:21:41] 
[00:21:41] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:21:41] 
[00:21:41] note: compiler flags: -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:21:41] 
[00:21:41] note: some of the compiler flags provided by cargo are hidden
[00:21:41] 
[00:21:41] [0m[0m[1m[31merror:[0m Could not compile `chalk-engine`.
