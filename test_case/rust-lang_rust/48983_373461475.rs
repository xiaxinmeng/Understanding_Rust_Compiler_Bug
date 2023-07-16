
[00:59:00] ---- [compile-fail] compile-fail/simd-intrinsic-generic-reduction.rs stdout ----
[00:59:00] 	
[00:59:00] error: compiler encountered internal error
[00:59:00] status: exit code: 101
[00:59:00] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/simd-intrinsic-generic-reduction.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/simd-intrinsic-generic-reduction.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/simd-intrinsic-generic-reduction.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:59:00] stdout:
[00:59:00] ------------------------------------------
[00:59:00] 
[00:59:00] ------------------------------------------
[00:59:00] stderr:
[00:59:00] ------------------------------------------
[00:59:00] {"message":"librustc_trans/builder.rs:966: LLVMRustBuildVectorReduceFAdd is not available in LLVM version < 5.0","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: librustc_trans/builder.rs:966: LLVMRustBuildVectorReduceFAdd is not available in LLVM version < 5.0\n\n"}
[00:59:00] thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:540:9
[00:59:00] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:59:00] 
[00:59:00] note: the compiler unexpectedly panicked. this is a bug.
[00:59:00] 
[00:59:00] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:59:00] 
[00:59:00] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:59:00] 
[00:59:00] note: compiler flags: -Z ui-testing -Z miri -Z unstable-options -C prefer-dynamic -C rpath
[00:59:00] 
[00:59:00] 
[00:59:00] ------------------------------------------
[00:59:00] 
[00:59:00] thread '[compile-fail] compile-fail/simd-intrinsic-generic-reduction.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2903:9
[00:59:00] 
[00:59:00] 
[00:59:00] failures:
[00:59:00]     [compile-fail] compile-fail/simd-intrinsic-generic-reduction.rs
[00:59:00] 
[00:59:00] test result: FAILED. 2304 passed; 1 failed; 15 ignored; 0 measured; 0 filtered out
[00:59:00] 
[00:59:00] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:478:22
