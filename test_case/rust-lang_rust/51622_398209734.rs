plain
[00:49:45] ....................................................................................................
[00:50:01] ............................................................i.......................................
[00:50:18] ...........................................................i........................................
[00:50:46] ....................................................................................................
[00:50:50] ......F.........test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:51:36] ...........................ii............................................................i....ii....
[00:51:55] ................................................i.ii................................................
[00:52:24] ........................................................iiiiiii.....................................
[00:52:42] ....................................................................................................
---
[00:53:33] ------------------------------------------
[00:53:33] stderr:
[00:53:33] ------------------------------------------
[00:53:33] thread 'main' panicked at 'assertion failed: `(left == right)`
[00:53:33]   left: `42..=42`,
[00:53:33]  right: `1..=0`', /checkout/src/test/run-pass/range_inclusive.rs:83:5
[00:53:33] 
[00:53:33] ------------------------------------------
[00:53:33] 
[00:53:33] thread '[run-pass] run-pass/range_inclusive.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
---
[00:53:33] 
[00:53:33] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:53:33] 
[00:53:33] 
[00:53:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:53:33] 
[00:53:33] 
[00:53:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:53:33] Build completed unsuccessfully in 0:11:57
[00:53:33] Build completed unsuccessfully in 0:11:57
[00:53:33] Makefile:58: recipe for target 'check' failed
[00:53:33] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1644cf3a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:004fac2e:start=1529359291815234785,finish=1529359291821791310,duration=6556525
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0020817c
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:31a42802
$ dmesg | grep -i kill
