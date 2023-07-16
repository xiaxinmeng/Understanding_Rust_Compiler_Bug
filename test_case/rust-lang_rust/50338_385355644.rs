plain
[00:47:12] .................................i..................................................................
[00:47:24] ..................................................................i.................................
[00:47:37] .....................................................i..............................................
[00:47:54] ....................................................................................................
[00:48:12] ........................................................F...........................................
[00:48:50] ...............i....................................................................................
[00:49:18] .............i......................................................................................
[00:49:25] ..............test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:49:49] ......................................................................................
---
[00:52:55] failures:
[00:52:55] 
[00:52:55] ---- [run-pass] run-pass/macro-comma-behavior.rs stdout ----
[00:52:55]  
[00:52:55] error in revision `core`: test run failed!
[00:52:55] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/macro-comma-behavior.stage2-x86_64-unknown-linux-gnu"
[00:52:55] stdout:
[00:52:55] ------------------------------------------
[00:52:55] 
[00:52:55] 
[00:52:55] running 3 tests
[00:52:55] test debug_assert_1arg ... FAILED
[00:52:55] test to_format_or_not_to_format ... ok
[00:52:55] test assert_1arg ... FAILED
[00:52:55] failures:
[00:52:55] failures:
[00:52:55n-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:52:55] 
[00:52:55] 
[00:52:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:52:55] Build completed unsuccessfully in 0:13:18
[00:52:55] Build completed unsuccessfully in 0:13:18
[00:52:55] Makefile:58: recipe for target 'check' failed
[00:52:55] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental
111308 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
111308 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
111304 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
107420 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
102820 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
102808 ./obj/build/bootstrap/debug/incremental/bootstrap-2wettvttcntnm
102804 ./obj/build/bootstrap/debug/incremental/bootstrap-2wettvttcntnm/s-f0lgpm65kh-1jc4cti-v7hez0f2m5v0
90744 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90744 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90740 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz/s-f0lgn98ibl-4n1z2x-q6snowoabdxz
89896 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
89684 ./src/llvm/test/CodeGen
86672 ./obj/build/x86_64-unknown-linux-gnu/doc/core
84688 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
