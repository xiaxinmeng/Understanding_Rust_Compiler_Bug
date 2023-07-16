
...
test [run-pass] run-pass/issue-47364.rs ... FAILED
test [run-pass] run-pass/rfc-1937-termination-trait/termination-trait-for-result.rs ... ok

failures:

---- [run-pass] run-pass/issue-47364.rs stdout ----
	
error: test run failed!
status: exit code: 101
command: "/home/brad/Development/rust/rust/build/x86_64-unknown-linux-gnu/test/run-pass/issue-47364.stage2-x86_64-unknown-linux-gnu"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 2', /home/brad/Development/rust/rust/src/test/run-pass/issue-47364.rs:61:17
note: Run with `RUST_BACKTRACE=1` for a backtrace.

------------------------------------------

thread '[run-pass] run-pass/issue-47364.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2884:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [run-pass] run-pass/issue-47364.rs

test result: FAILED. 3 passed; 1 failed; 2917 ignored; 0 measured; 0 filtered out

thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:476:22


command did not execute successfully: "/home/brad/Development/rust/rust/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/home/brad/Development/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/home/brad/Development/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/home/brad/Development/rust/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/home/brad/Development/rust/rust/src/test/run-pass" "--build-base" "/home/brad/Development/rust/rust/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/home/brad/Development/rust/rust/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Zmiri -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zmiri -Zunstable-options  -Lnative=/home/brad/Development/rust/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python" "--lldb-python" "/usr/bin/python" "--gdb" "/usr/bin/gdb" "--llvm-version" "4.0.1\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" ""
expected success, got: exit code: 101


failed to run: /home/brad/Development/rust/rust/build/bootstrap/debug/bootstrap test
Build completed unsuccessfully in 0:00:40
