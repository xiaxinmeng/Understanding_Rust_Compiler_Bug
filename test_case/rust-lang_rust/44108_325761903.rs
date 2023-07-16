
[00:49:52] ---- [compile-fail] compile-fail/feature-gate-match_beginning_vert.rs stdout ----
[00:49:52] 	
[00:49:52] error: compile-fail test compiled successfully!
[00:49:52] status: exit code: 0

[00:49:52] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/feature-gate-match_beginning_vert.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/feature-gate-match_beginning_vert.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/feature-gate-match_beginning_vert.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux" "-A" "unused"
[00:49:52] stdout:
[00:49:52] ------------------------------------------
[00:49:52] 
[00:49:52] ------------------------------------------
[00:49:52] stderr:
[00:49:52] ------------------------------------------
[00:49:52] 
[00:49:52] ------------------------------------------
[00:49:52] 
[00:49:52] thread '[compile-fail] compile-fail/feature-gate-match_beginning_vert.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2435:8
[00:49:52] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:52] 
[00:49:52] 
[00:49:52] failures:
[00:49:52]     [compile-fail] compile-fail/feature-gate-match_beginning_vert.rs
[00:49:52] 
[00:49:52] test result: FAILED. 2724 passed; 1 failed; 13 ignored; 0 measured; 0 filtered out
[00:49:52] 
