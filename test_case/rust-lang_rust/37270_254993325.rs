
---- [compile-fail] compile-fail/coherence-projection-conflict-orphan.rs stdout ----

error: compile-fail test compiled successfully!
status: exit code: 0
command: /build/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /build/src/test/compile-fail/coherence-projection-conflict-orphan.rs -L /build/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -L /build/build/x86_64-unknown-linux-gnu/test/compile-fail/coherence-projection-conflict-orphan.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /build/build/x86_64-unknown-linux-gnu/test/compile-fail/coherence-projection-conflict-orphan.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/build/build/x86_64-unknown-linux-gnu/rust-test-helpers
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------

------------------------------------------

thread '[compile-fail] compile-fail/coherence-projection-conflict-orphan.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2377
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [compile-fail] compile-fail/coherence-projection-conflict-orphan.rs

test result: FAILED. 2591 passed; 1 failed; 12 ignored; 0 measured

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:298


command did not execute successfully: "/build/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/compiletest" "--compile-lib-path" "/build/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/build/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/build/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/build/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/build/src/test/compile-fail" "--build-base" "/build/build/x86_64-unknown-linux-gnu/test/compile-fail" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "compile-fail" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.7/bin/FileCheck" "--host-rustcflags" "-Crpath -O" "--target-rustcflags" "-Crpath -O -Lnative=/build/build/x86_64-unknown-linux-gnu/rust-test-helpers" "--docck-python" "python" "--lldb-python" "python" "--gdb-version" "GNU gdb (Ubuntu 7.11.1-0ubuntu1~16.04) 7.11.1" "--llvm-version" "3.7.1\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp" "--android-cross-path" ""
expected success, got: exit code: 101

