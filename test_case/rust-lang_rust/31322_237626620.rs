
failures:

---- [run-pass] run-pass/allocator-default.rs stdout ----

error: compilation failed!
status: exit code: 101
command: /home/jirutjak/rust/build/x86_64-unknown-linux-musl/stage2/bin/rustc /home/jirutjak/rust/src/test/run-pass/allocator-default.rs -L /home/jirutjak/rust/build/x86_64-unknown-linux-musl/test/run-pass --target=x86_64-unknown-linux-musl -L /home/jirutjak/rust/build/x86_64-unknown-linux-musl/test/run-pass/allocator-default.stage2-x86_64-unknown-linux-musl.run-pass.libaux -C prefer-dynamic -o /home/jirutjak/rust/build/x86_64-unknown-linux-musl/test/run-pass/allocator-default.stage2-x86_64-unknown-linux-musl -Crpath -O -Lnative=/home/jirutjak/rust/build/x86_64-unknown-linux-musl/rust-test-helpers
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/home/jirutjak/rust/src/test/run-pass/allocator-default.rs:14:1: 14:29 error: can't find crate for `alloc_jemalloc` [E0463]
/home/jirutjak/rust/src/test/run-pass/allocator-default.rs:14 extern crate alloc_jemalloc;
                                                              ^~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error

------------------------------------------

thread '[run-pass] run-pass/allocator-default.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2353
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    [run-pass] run-pass/allocator-default.rs

test result: FAILED. 2468 passed; 1 failed; 5 ignored; 0 measured

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:293


command did not execute successfully: "/home/jirutjak/rust/build/x86_64-unknown-linux-musl/stage2-tools/x86_64-unknown-linux-musl/release/compiletest" "--compile-lib-path" "/home/jirutjak/rust/build/x86_64-unknown-linux-musl/stage2/lib" "--run-lib-path" "/home/jirutjak/rust/build/x86_64-unknown-linux-musl/stage2/lib/rustlib/x86_64-unknown-linux-musl/lib" "--rustc-path" "/home/jirutjak/rust/build/x86_64-unknown-linux-musl/stage2/bin/rustc" "--rustdoc-path" "/home/jirutjak/rust/build/x86_64-unknown-linux-musl/stage2/bin/rustdoc" "--src-base" "/home/jirutjak/rust/src/test/run-pass" "--build-base" "/home/jirutjak/rust/build/x86_64-unknown-linux-musl/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-musl" "--mode" "run-pass" "--target" "x86_64-unknown-linux-musl" "--host" "x86_64-unknown-linux-musl" "--llvm-filecheck" "/usr/lib/llvm-3.7/bin/FileCheck" "--host-rustcflags" "-Crpath -O" "--target-rustcflags" "-Crpath -O -Lnative=/home/jirutjak/rust/build/x86_64-unknown-linux-musl/rust-test-helpers" "--docck-python" "python" "--lldb-python" "python" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp" "--android-cross-path" ""
expected success, got: exit code: 101


make: *** [Makefile:45: check] Error 1
