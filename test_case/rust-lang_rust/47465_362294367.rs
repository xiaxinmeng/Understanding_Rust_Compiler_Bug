
[01:27:46]    Compiling style_traits v0.0.1 (file:///checkout/obj/build/ct/servo/components/style_traits)
[01:27:48] thread 'rustc' panicked at 'byte index 13 is out of bounds of `MallocSizeOf`', libcore/str/mod.rs:2221:9
[01:27:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:27:48] 
[01:27:48] error: internal compiler error: unexpected panic
[01:27:48] 
[01:27:48] note: the compiler unexpectedly panicked. this is a bug.
[01:27:48] 
[01:27:48] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:27:48] 
[01:27:48] note: rustc 1.25.0-dev running on x86_64-unknown-linux-gnu
[01:27:48] 
[01:27:48] error: Could not compile `style_traits`.
[01:27:48] warning: build failed, waiting for other jobs to finish...
[01:27:48] error: build failed
[01:27:48] thread 'main' panicked at 'tests failed for https://github.com/servo/servo', tools/cargotest/main.rs:100:9
[01:27:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:27:48] 
[01:27:48] 
[01:27:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/cargotest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/ct"
[01:27:48] expected success, got: exit code: 101
