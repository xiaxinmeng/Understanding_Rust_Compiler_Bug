
[01:02:27] ---- [debuginfo-gdb] debuginfo/multi-byte-chars.rs stdout ----
[01:02:27] 	NOTE: compiletest thinks it is using GDB without native rust support
[01:02:27] 
[01:02:27] error: compilation failed!
[01:02:27] status: exit code: 101
[01:02:27] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/debuginfo/multi-byte-chars.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/multi-byte-chars.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/multi-byte-chars.stage2-x86_64-unknown-linux-gnu.gdb.aux"
[01:02:27] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:476:22
[01:02:27] stdout:
[01:02:27] ------------------------------------------
[01:02:27] 
[01:02:27] ------------------------------------------
[01:02:27] stderr:
[01:02:27] ------------------------------------------
[01:02:27] error: internal compiler error: unexpected panic
[01:02:27] 
[01:02:27] note: the compiler unexpectedly panicked. this is a bug.
[01:02:27] 
[01:02:27] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:02:27] 
[01:02:27] note: rustc 1.25.0-dev running on x86_64-unknown-linux-gnu
[01:02:27] 
[01:02:27] thread 'rustc' panicked at 'assertion failed: bpos.to_usize() >= mbc.pos.to_usize() + mbc.bytes', libsyntax/codemap.rs:613:17
[01:02:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:02:27] 
[01:02:27] 
[01:02:27] ------------------------------------------
[01:02:27] 
[01:02:27] thread '[debuginfo-gdb] debuginfo/multi-byte-chars.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2884:9
[01:02:27] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:02:27] 
[01:02:27] 
[01:02:27] failures:
[01:02:27]     [debuginfo-gdb] debuginfo/multi-byte-chars.rs
[01:02:27] 
[01:02:27] test result: FAILED. 84 passed; 1 failed; 24 ignored; 0 measured; 0 filtered out
