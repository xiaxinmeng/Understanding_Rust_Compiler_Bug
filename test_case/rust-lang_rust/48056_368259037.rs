
[01:52:21] 
[01:52:21] ---- [pretty] run-pass/macro-comma-support.rs stdout ----
[01:52:21] 	
[01:52:21] error in revision `std`: pretty-printed source does not typecheck
[01:52:21] status: exit code: 101
[01:52:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "-" "-Zno-trans" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-comma-support.pretty-out" "--target=x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-comma-support.stage2-x86_64-unknown-linux-gnu.pretty.aux" "--cfg" "std" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes"
[01:52:21] stdout:
[01:52:21] ------------------------------------------
[01:52:21] 
[01:52:21] ------------------------------------------
[01:52:21] stderr:
[01:52:21] ------------------------------------------
[01:52:21] thread 'rustc' panicked at 'cannot resolve relative path in non-file source `<anon>`', libsyntax/ext/source_util.rs:202:22
[01:52:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:52:21] 
[01:52:21] error: internal compiler error: unexpected panic
[01:52:21] 
[01:52:21] note: the compiler unexpectedly panicked. this is a bug.
[01:52:21] 
[01:52:21] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:52:21] 
[01:52:21] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[01:52:21] 
[01:52:21] 
[01:52:21] ------------------------------------------
[01:52:21] 
[01:52:21] thread '[pretty] run-pass/macro-comma-support.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2892:9
[01:52:21] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:52:21] 
[01:52:21] 
[01:52:21] failures:
[01:52:21]     [pretty] run-pass/macro-comma-support.rs
[01:52:21] 
[01:52:21] test result: [31mFAILED(B[m. 2876 passed; 1 failed; 45 ignored; 0 measured; 0 filtered out
[01:52:21] 
[01:52:21] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:476:22
[01:52:21] 
