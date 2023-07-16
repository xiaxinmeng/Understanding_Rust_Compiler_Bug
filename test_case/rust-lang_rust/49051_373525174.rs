
[02:08:41] ---- [run-pass] run-pass/simd-intrinsic-generic-reduction.rs stdout ----
[02:08:41] 	
[02:08:41] error: compilation failed!
[02:08:41] status: exit code: 101
[02:08:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/simd-intrinsic-generic-reduction.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=asmjs-unknown-emscripten" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-generic-reduction.stage2-asmjs-unknown-emscripten.js" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/asmjs-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/simd-intrinsic-generic-reduction.stage2-asmjs-unknown-emscripten.aux"
[02:08:41] stdout:
[02:08:41] ------------------------------------------
[02:08:41] 
[02:08:41] ------------------------------------------
[02:08:41] stderr:
[02:08:41] ------------------------------------------
[02:08:41] warning: type `u32x4` should have a camel case name such as `U32x4`
[02:08:41]   --> /checkout/src/test/run-pass/simd-intrinsic-generic-reduction.rs:24:1
[02:08:41]    |
[02:08:41] 24 | struct u32x4(pub u32, pub u32, pub u32, pub u32);
[02:08:41]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[02:08:41]    |
[02:08:41]    = note: #[warn(non_camel_case_types)] on by default
[02:08:41] 
[02:08:41] warning: type `f32x4` should have a camel case name such as `F32x4`
[02:08:41]   --> /checkout/src/test/run-pass/simd-intrinsic-generic-reduction.rs:28:1
[02:08:41]    |
[02:08:41] 28 | struct f32x4(pub f32, pub f32, pub f32, pub f32);
[02:08:41]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[02:08:41] 
[02:08:41] warning: type `b8x4` should have a camel case name such as `B8x4`
[02:08:41]   --> /checkout/src/test/run-pass/simd-intrinsic-generic-reduction.rs:32:1
[02:08:41]    |
[02:08:41] 32 | struct b8x4(pub i8, pub i8, pub i8, pub i8);
[02:08:41]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[02:08:41] 
[02:08:41] warning: type `b8x16` should have a camel case name such as `B8x16`
[02:08:41]   --> /checkout/src/test/run-pass/simd-intrinsic-generic-reduction.rs:36:1
[02:08:41]    |
[02:08:41] 36 | / struct b8x16(
[02:08:41] 37 | |     pub i8, pub i8, pub i8, pub i8,
[02:08:41] 38 | |     pub i8, pub i8, pub i8, pub i8,
[02:08:41] 39 | |     pub i8, pub i8, pub i8, pub i8,
[02:08:41] 40 | |     pub i8, pub i8, pub i8, pub i8
[02:08:41] 41 | | );
[02:08:41]    | |__^
[02:08:41] 
[02:08:41] error: internal compiler error: librustc_trans/builder.rs:991: LLVMRustBuildVectorReduceAdd is not available in LLVM version < 5.0
[02:08:41] 
[02:08:41] thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:540:9
[02:08:41] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[02:08:41] 
[02:08:41] note: the compiler unexpectedly panicked. this is a bug.
[02:08:41] 
[02:08:41] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[02:08:41] 
[02:08:41] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[02:08:41] 
[02:08:41] note: compiler flags: -Z miri -Z unstable-options -C prefer-dynamic -C rpath
[02:08:41] 
[02:08:41] 
[02:08:41] ------------------------------------------
[02:08:41] 
[02:08:41] thread '[run-pass] run-pass/simd-intrinsic-generic-reduction.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2903:9
[02:08:41] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[02:08:41] 
[02:08:41] 
[02:08:41] failures:
[02:08:41]     [run-pass] run-pass/simd-intrinsic-generic-reduction.rs
[02:08:41] 
[02:08:41] test result: [31mFAILED(B[m. 2798 passed; 1 failed; 150 ignored; 0 measured; 0 filtered out
