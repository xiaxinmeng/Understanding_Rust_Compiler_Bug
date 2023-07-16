
[00:50:33] failures:
[00:50:33] 
[00:50:33] ---- [ui] ui/test-should-panic-attr.rs stdout ----
[00:50:33] 	
[00:50:33] error: test run failed!
[00:50:33] status: exit code: 3
[00:50:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/remote-test-client" "run" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-should-panic-attr.stage2-arm-linux-androideabi"
[00:50:33] stdout:
[00:50:33] ------------------------------------------
[00:50:33] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-should-panic-attr.stage2-arm-linux-androideabi", waiting for result
[00:50:33] 
[00:50:33] running 5 tests
[00:50:33] test test2 ... ok
[00:50:33] died due to signal 11
[00:50:33] 
[00:50:33] ------------------------------------------
[00:50:33] stderr:
[00:50:33] ------------------------------------------
[00:50:33] 
[00:50:33] ------------------------------------------
[00:50:33] 
[00:50:33] thread '[ui] ui/test-should-panic-attr.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2425:8
[00:50:33] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:33] 
[00:50:33] 
[00:50:33] failures:
[00:50:33]     [ui] ui/test-should-panic-attr.rs
[00:50:33] 
[00:50:33] test result: FAILED. 421 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out
