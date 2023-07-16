plain
[00:44:44] 
[00:44:44] running 2965 tests
[00:44:59] ....................................................................................................
[00:45:14] .............................................i......................................................
[00:45:31] .................................................................F..................................
[00:46:00] ....................................................................................................
[00:46:21] ....................................................................................................
[00:46:36] ....................................................................................................
[00:46:51] ....................................................................................................
---
[00:55:14] 
[00:55:14] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:55:14] 
[00:55:14] 
[00:55:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:55:14] 
[00:55:14] 
[00:55:14] failed 6_64-unknown-linux-gnu/release/incremental
111128 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
111128 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
111124 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
107240 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
102808 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
101924 ./obj/build/bootstrap/debug/incremental/bootstrap-2s8ik7x786gic
101920 ./obj/build/bootstrap/debug/incremental/bootstrap-2s8ik7x786gic/s-f0enuo18a0-1d1jek5-8chh8umqc096
89216 ./obj/build/x86_64-unknown-linux-gnu/stage1
89192 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
88056 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
88056 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
88052 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz/s-f0ens9tbq7-p0tlls-3by8jin3pp2v3
84832 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
81104 ./obj/build/x86_64-unknown-linux-gnu/doc/std
78700 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot
78696 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib
