plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:07] 
[00:58:07] running 1400 tests
[00:58:12] ..................................................................................i.................
[00:58:20] ................................i..............................................................F....
[00:58:28] ....................................................................................................
[00:58:32] ....................................................................................................
[00:58:37] ....................................................................................................
[00:58:43] ....................................................................................................
---
[00:59:34] failures:
[00:59:34] 
[00:59:34] ---- [ui] ui/edition-lint-paths.rs stdout ----
[00:59:34]  
[00:59:34] error: ui test compiled successfully!
[00:59:34] status: exit code: 0
[00:59:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/edition-lint-paths.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-fedition-lint-paths.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[00:59:34] 
[00:59:34] 
[00:59:34] failures:
[00:59:34]     [ui] ui/edition-lint-paths.rs
[00:59:34]     [ui] ui/edition-lint-paths.rs
[00:59:34] 
[00:59:34] test result: FAILED. 1392 passed; 1 failed; 7 ignored; 0 measured; 0 filtered out
[00:59:34] 
[00:59:34] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[00:59:34] 
[00:59:34] 
[00:59:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:34] 
[00:59:34] 
[00:59:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:34] Build completed unsuccessfully in 0:03:03
[00:59:34] Build completed unsuccessfully in 0:03:03
[00:59:34] make: *** [check] Error 1
[00:59:34] Makefile:58: recipe for target 'check' failed
112616 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
112612 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
108620 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
103728 ./obj/build/bootstrap/debug/incremental/bootstrap-1x7l2oj4p22hp
103728 ./obj/build/bootstrap/debug/incremental/bootstrap-1x7l2oj4p22hp
103724 ./obj/build/bootstrap/debug/incremental/bootstrap-1x7l2oj4p22hp/s-f0y16e0z56-4wftwn-1nozvcbv9keul
98508 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental
93332 ./obj/build/x86_64-unknown-linux-gnu/stage1
93308 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
90832 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90832 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90828 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz/s-f0y136m9r0-au7odn-5nj5h7hrw8mg
87816 ./obj/build/x86_64-unknown-linux-gnu/doc/core
81368 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
80984 ./obj/build/x86_64-unknown-linux-gnu/doc/std
79044 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot
