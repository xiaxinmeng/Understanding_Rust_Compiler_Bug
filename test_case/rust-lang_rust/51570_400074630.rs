plain
travis_time:start:test_run-pass
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:47:19] 
[00:47:19] running 3031 tests
[00:47:34] ..................................................F.................................................
[00:48:05] ....................................................................................................
[00:48:20] ....................................................................................................
[00:48:32] ....................................................................................................
[00:48:53] ....................................................................................................
---
[00:56:19] ---- [run-pass] run-pass/associated-const-cross-crate-defaults.rs stdout ----
[00:56:19] 
[00:56:19] error: compilation failed!
[00:56:19] status: exit code: 101
[00:56:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/associated-const-cross-crate-defaults.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-const-cross-crate-defaults/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/associated-const-cross-crate-defaults/auxiliary"
[00:56:19] ------------------------------------------
[00:56:19] 
[00:56:19] ------------------------------------------
[00:56:19] stderr:
[00:56:19] stderr:
[00:56:19] ------------------------------------------
[00:56:19] error: internal compiler error: librustc_metadata/decoder.rs:797: impossible case reached
[00:56:19] thread 'main' panicked at 'Box<Any>', librustc_errors/lib.rs:554:9
[00:56:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:56:19] error: aborting due to previous error
[00:56:19] 
[00:56:19] 
[00:56:19] 
[00:56:19] note: the compiler unexpectedly panicked. this is a bug.
[00:56:19] 
[00:56:19] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:56:19] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[00:56:19] 
[00:56:19] 
[00:56:19] note: compiler flags: -Z unstable-options -C prefer-dynamic -C rpath
[00:56:19] 
[00:56:19] ------------------------------------------
[00:56:19] 
[00:56:19] thread '[run-pass] run-pass/associated-const-cross-crate-defaults.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
---
[00:56:19] 
[00:56:19] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:56:19] 
[00:56:19] 
[00:56:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:19] 
[00:56:19] 
[00:56:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:19] Build completed unsuccessfully in 0:12:18
[00:56:19] Build completed unsuccessfully in 0:12:18
[00:56:19] make: *** [check] Error 1
[00:56:19] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:006faf10
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
