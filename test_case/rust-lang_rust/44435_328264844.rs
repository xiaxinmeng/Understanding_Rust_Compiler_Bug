
[00:17:57] error: internal compiler error: unexpected panic
[00:17:57] 
[00:17:57] note: the compiler unexpectedly panicked. this is a bug.
[00:17:57] 
[00:17:57] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:17:57] 
[00:17:57] note: rustc 1.22.0-dev running on x86_64-unknown-linux-gnu
[00:17:57] 
[00:17:57] thread 'rustc' panicked at '/checkout/src/librustc/hir/map/mod.rs:330: local_def_id: no entry for `291005`, which has a map of `Some(EntryLifetime(NodeId(192109), DepNodeIndex { index: 4294967295 }, lifetime(291005: )))`', /checkout/src/librustc/session/mod.rs:889:25
[00:17:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:17:57] 
[00:17:57] error: Could not compile `core`.
