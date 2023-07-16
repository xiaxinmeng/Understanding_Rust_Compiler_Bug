
[01:00:56] ---- [rustdoc] rustdoc/issue-12834.rs stdout ----
[01:00:56] 
[01:00:56] error: rustdoc failed!
[01:00:56] status: exit code: 1
[01:00:56] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-12834/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-12834" "/checkout/src/test/rustdoc/issue-12834.rs"
[01:00:56] stdout:
[01:00:56] ------------------------------------------
[01:00:56] 
[01:00:56] ------------------------------------------
[01:00:56] stderr:
[01:00:56] ------------------------------------------
[01:00:56] warning: Backing out of syntax highlighting
[01:00:56]   |
[01:00:56]   = note: You probably did not intend to render this as a rust code-block
[01:00:56] 
[01:00:56] error: internal compiler error: Error constructed but not emitted
[01:00:56] 
[01:00:56] thread '<unnamed>' panicked at 'explicit panic', librustc_errors/diagnostic_builder.rs:295:13
[01:00:56] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:00:56] 
[01:00:56] error: internal compiler error: unexpected panic
[01:00:56] 
[01:00:56] note: the compiler unexpectedly panicked. this is a bug.
[01:00:56] 
[01:00:56] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:00:56] 
[01:00:56] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[01:00:56] 
[01:00:56] 
[01:00:56] ------------------------------------------
[01:00:56] 
[01:00:56] thread '[rustdoc] rustdoc/issue-12834.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[01:00:56] 
[01:00:56] 
[01:00:56] failures:
[01:00:56]     [rustdoc] rustdoc/issue-12834.rs
