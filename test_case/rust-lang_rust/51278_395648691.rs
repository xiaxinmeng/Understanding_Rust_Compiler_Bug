plain
[00:46:34] ...i..............................................................................i.................
[00:46:39] ....................................................................................................
[00:46:45] ....................................................................................................
[00:46:51] ....................................................................................................
[00:46:56] ...............i.................iiiiiiiii...................................................
[00:46:56] 
[00:46:56] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:47:47] ...i..............................................................................i.................
[00:47:51] ....................................................................................................
[00:47:56] ....................................................................................................
[00:48:02] ....................................................................................................
[00:48:07] ...............i.................iiiiiiiii...................................................
[00:48:07] 
[00:48:07]  finished in 70.725
[00:48:07] travis_fold:end:test_ui_nll

---
[00:52:27] .....................................................................test [run-pass] run-pass/issue-29227.rs has been running for over 60 seconds
[00:52:36] ...............................
[00:53:01] ....................................................................................................
[00:53:13] ....................................................................................................
[00:53:30] ...................................................F....i...........................................
[00:54:17] ....................................................................................................
[00:54:18] .test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:54:33] ...................................................................................................
[00:55:10] .......................ii...........................................................i....ii.........
---
[00:57:08] ---- [run-pass] run-pass/mod_dir_path_canonicalized.rs stdout ----
[00:57:08] 
[00:57:08] error: compilation failed!
[00:57:08] status: exit code: 101
[00:57:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/mod_dir_path_canonicalized.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mod_dir_path_canonicalized/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/mod_dir_path_canonicalized/auxiliary"
[00:57:08] ------------------------------------------
[00:57:08] 
[00:57:08] ------------------------------------------
[00:57:08] stderr:
[00:57:08] stderr:
[00:57:08] ------------------------------------------
[00:57:08] error: couldn't read "/checkout/src/test/run-pass/mod_dir_simple\\test.rs": No such file or directory (os error 2)
[00:57:08]   --> /checkout/src/test/run-pass/mod_dir_path_canonicalized.rs:24:5
[00:57:08]    |
[00:57:08] 24 | mod gravy;
[00:57:08] 
[00:57:08] error: aborting due to previous error
[00:57:08] 
[00:57:08] 
---
[00:57:08] 
[00:57:08] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:57:08] 
[00:57:08] 
[00:57:08] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:08] 
[00:57:08] 
[00:57:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:08] Build completed unsuccessfully in 0:12:53
[00:57:08] Build completed unsuccessfully in 0:12:53
[00:57:08] Makefile:58: recipe for target 'check' failed
[00:57:08] make: *** [check] Error 1

The comma3916684 .
2642188 ./obj
2642156 ./obj/build
---
147120 ./.git/modules/src
143060 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
143056 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
123264 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
123260 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f1s7n6pasb-x83fbl-t4oe0he9btpl
107456 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
103816 .
