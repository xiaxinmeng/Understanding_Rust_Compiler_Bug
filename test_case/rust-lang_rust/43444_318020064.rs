
[00:58:11] thread 'main' panicked at 'Some tests failed', /checkout/src/tools/compiletest/src/main.rs:323:21
[00:58:11] ---- [run-pass] run-pass/backtrace.rs stdout ----
[00:58:11] 	
[00:58:11] error: test run failed!
[00:58:11] status: exit code: 101
[00:58:11] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/remote-test-client run /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/backtrace.stage2-arm-linux-androideabi
[00:58:11] stdout:
[00:58:11] ------------------------------------------
[00:58:11] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/backtrace.stage2-arm-linux-androideabi", waiting for result
[00:58:11] 
[00:58:11] ------------------------------------------
[00:58:11] stderr:
[00:58:11] ------------------------------------------
[00:58:11] thread 'main' panicked at 'bad output: thread 'main' panicked at 'explicit panic', /checkout/src/test/run-pass/backtrace.rs:23:8
[00:58:11] stack backtrace:
[00:58:11]    0: <unknown>
[00:58:11]    1: <unknown>
[00:58:11]    2: <unknown>
[00:58:11]    3: <unknown>
[00:58:11]    4: <unknown>
[00:58:11]    5: <unknown>
[00:58:11]    6: <unknown>
[00:58:11]    7: <unknown>
[00:58:11]    8: <unknown>
[00:58:11]    9: <unknown>
[00:58:11]   10: <unknown>
[00:58:11] ', /checkout/src/test/run-pass/backtrace.rs:58:4
[00:58:11] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:11] 
[00:58:11] ------------------------------------------
[00:58:11] 
[00:58:11] thread '[run-pass] run-pass/backtrace.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2479:8
[00:58:11] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:11] 
[00:58:11] ---- [run-pass] run-pass/backtrace-debuginfo.rs stdout ----
[00:58:11] 	
[00:58:11] error: test run failed!
[00:58:11] status: exit code: 101
[00:58:11] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/remote-test-client run /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/backtrace-debuginfo.stage2-arm-linux-androideabi
[00:58:11] stdout:
[00:58:11] ------------------------------------------
[00:58:11] uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/backtrace-debuginfo.stage2-arm-linux-androideabi", waiting for result
[00:58:11] 
[00:58:11] ------------------------------------------
[00:58:11] stderr:
[00:58:11] ------------------------------------------
[00:58:11] thread 'main' panicked at 'trace does not match position list: test case 0
[00:58:11] thread 'main' panicked at 'explicit panic', /checkout/src/test/run-pass/backtrace-debuginfo.rs:79:4
[00:58:11] stack backtrace:
[00:58:11]    0: 0xb6e609f7 - <unknown>
[00:58:11]    1: 0xb6e5a9f7 - <unknown>
[00:58:11]    2: 0xb6e6c50f - <unknown>
[00:58:11]    3: 0xb6e6c1ff - <unknown>
[00:58:11]    4: 0xb6e6cb23 - <unknown>
[00:58:11]    5: 0xb6f2c2cf - <unknown>
[00:58:11]    6: 0xb6f2e6f7 - <unknown>
[00:58:11]    7: 0xb6f2e7b7 - <unknown>
[00:58:11]    8: 0xb6f2eeb3 - <unknown>
[00:58:11]    9: 0xb6eaf61b - <unknown>
[00:58:11]   10: 0xb6e6d92b - <unknown>
[00:58:11]   11: 0xb6d9fb4f - <unknown>
[00:58:11] 
[00:58:11] ---
[00:58:11] backtrace-debuginfo.rs:120
[00:58:11] backtrace-debuginfo.rs:168
[00:58:11] ', /checkout/src/test/run-pass/backtrace-debuginfo.rs:134:4
[00:58:11] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:11] 
[00:58:11] ------------------------------------------
[00:58:11] 
[00:58:11] thread '[run-pass] run-pass/backtrace-debuginfo.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2479:8
[00:58:11] 
[00:58:11] 
[00:58:11] failures:
[00:58:11]     [run-pass] run-pass/backtrace-debuginfo.rs
[00:58:11]     [run-pass] run-pass/backtrace.rs
[00:58:11] 
[00:58:11] test result: [31mFAILED(B[m. 2700 passed; 2 failed; 19 ignored; 0 measured; 0 filtered out
