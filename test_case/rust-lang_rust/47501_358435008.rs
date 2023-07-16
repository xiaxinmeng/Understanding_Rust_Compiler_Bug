
[00:53:57] ---- [run-pass] run-pass/rustc-rust-log.rs stdout ----
[00:53:57] 	
[00:53:57] error: compilation failed!
[00:53:57] status: exit code: 101
[00:53:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/rustc-rust-log.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rustc-rust-log.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rustc-rust-log.stage2-x86_64-unknown-linux-gnu.aux"
[00:53:57] stdout:
[00:53:57] ------------------------------------------
[00:53:57] 
[00:53:57] ------------------------------------------
[00:53:57] stderr:
[00:53:57] ------------------------------------------
<logs snipped>
[00:53:57] error: internal compiler error: unexpected panic
[00:53:57] 
[00:53:57] note: the compiler unexpectedly panicked. this is a bug.
[00:53:57] 
[00:53:57] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:57] 
[00:53:57] note: rustc 1.25.0-dev running on x86_64-unknown-linux-gnu
[00:53:57] 
[00:53:57] thread 'rustc' panicked at 'already borrowed: BorrowMutError', libcore/result.rs:916:5
[00:53:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:53:57] 
[00:53:57] 
[00:53:57] ------------------------------------------
[00:53:57] 
[00:53:57] thread '[run-pass] run-pass/rustc-rust-log.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2884:9
[00:53:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:53:57] 
[00:53:57] 
[00:53:57] failures:
[00:53:57]     [run-pass] run-pass/rustc-rust-log.rs
[00:53:57] 
[00:53:57] test result: FAILED. 2875 passed; 1 failed; 16 ignored; 0 measured; 0 filtered out
