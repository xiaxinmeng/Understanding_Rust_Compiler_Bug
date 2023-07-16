
[00:53:07] ---- [run-pass] run-pass/match-arm-statics.rs stdout ----
[00:53:07] 	
[00:53:07] error: compilation failed!
[00:53:07] status: exit code: 101
[00:53:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/match-arm-statics.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/match-arm-statics.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/match-arm-statics.stage2-x86_64-unknown-linux-gnu.aux"
[00:53:07] stdout:
[00:53:07] ------------------------------------------
[00:53:07] 
[00:53:07] ------------------------------------------
[00:53:07] stderr:
[00:53:07] ------------------------------------------
[00:53:07] error: internal compiler error: librustc_mir/hair/pattern/mod.rs:873: discriminant 4 not found in [
[00:53:07]     Discr {
[00:53:07]         val: 0,
[00:53:07]         ty: isize
[00:53:07]     },
[00:53:07]     Discr {
[00:53:07]         val: 1,
[00:53:07]         ty: isize
[00:53:07]     },
[00:53:07]     Discr {
[00:53:07]         val: 2,
[00:53:07]         ty: isize
[00:53:07]     },
[00:53:07]     Discr {
[00:53:07]         val: 3,
[00:53:07]         ty: isize
[00:53:07]     }
[00:53:07] ]
[00:53:07] 
[00:53:07] thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:543:9
[00:53:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:53:07] error: aborting due to previous error
[00:53:07] 
[00:53:07] 
[00:53:07] note: the compiler unexpectedly panicked. this is a bug.
[00:53:07] 
[00:53:07] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:07] 
[00:53:07] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:53:07] 
[00:53:07] note: compiler flags: -Z miri -Z unstable-options -C prefer-dynamic -C rpath
[00:53:07] 
[00:53:07] 
[00:53:07] ------------------------------------------
[00:53:07] 
[00:53:07] thread '[run-pass] run-pass/match-arm-statics.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:53:07] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:53:07] 
[00:53:07] 
[00:53:07] failures:
[00:53:07]     [run-pass] run-pass/match-arm-statics.rs
