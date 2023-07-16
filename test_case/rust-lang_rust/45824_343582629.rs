
[00:52:35] failures:
[00:52:35] 
[00:52:35] ---- [run-pass] run-pass/msvc-opt-minsize.rs stdout ----
[00:52:35] 	
[00:52:35] error: compilation failed!
[00:52:35] status: exit code: 101
[00:52:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/msvc-opt-minsize.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/msvc-opt-minsize.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Copt-level=z" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/msvc-opt-minsize.stage2-x86_64-unknown-linux-gnu.aux"
[00:52:35] stdout:
[00:52:35] ------------------------------------------
[00:52:35] 
[00:52:35] ------------------------------------------
[00:52:35] stderr:
[00:52:35] ------------------------------------------
[00:52:35] error: -O and -C opt-level both provided
[00:52:35] 
[00:52:35] 
[00:52:35] ------------------------------------------
[00:52:35] 
[00:52:35] thread '[run-pass] run-pass/msvc-opt-minsize.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2498:8
[00:52:35] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:52:35] 
[00:52:35] 
[00:52:35] failures:
[00:52:35]     [run-pass] run-pass/msvc-opt-minsize.rs
[00:52:35] 
[00:52:35] test result: FAILED. 2808 passed; 1 failed; 14 ignored; 0 measured; 0 filtered out
[00:52:35] 
[00:52:35] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:329:21
[00:52:35] 
