
[00:52:50] ---- [run-fail] run-fail/simd-intrinsic-float-minmax.rs stdout ----
[00:52:50] 	
[00:52:50] error: compilation failed!
[00:52:50] status: exit code: 101
[00:52:50] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-fail/simd-intrinsic-float-minmax.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/simd-intrinsic-float-minmax.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-fail/simd-intrinsic-float-minmax.stage2-x86_64-unknown-linux-gnu.aux"
[00:52:50] stdout:
[00:52:50] ------------------------------------------
[00:52:50] 
[00:52:50] ------------------------------------------
[00:52:50] stderr:
[00:52:50] ------------------------------------------
[00:52:50] error: internal compiler error: librustc_trans/builder.rs:918: LLVMRustBuildMinNum is not available in LLVM version < 6.0
[00:52:50] 
[00:52:50] thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:543:9
[00:52:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:52:50] 
[00:52:50] note: the compiler unexpectedly panicked. this is a bug.
[00:52:50] 
[00:52:50] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:52:50] 
[00:52:50] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:52:50] 
[00:52:50] note: compiler flags: -Z miri -Z unstable-options -C prefer-dynamic -C rpath
[00:52:50] 
[00:52:50] 
[00:52:50] ------------------------------------------
[00:52:50] 
[00:52:50] thread '[run-fail] run-fail/simd-intrinsic-float-minmax.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2903:9
[00:52:50] 
[00:52:50] 
[00:52:50] failures:
[00:52:50]     [run-fail] run-fail/simd-intrinsic-float-minmax.rs
