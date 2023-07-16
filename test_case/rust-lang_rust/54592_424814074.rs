plain
[00:55:36] ....................................................................................................
[00:55:39] ..............................................................i.....................................
[00:55:42] ....................................................................................................
[00:55:45] ....................................................................................................
[00:55:48] ...........iiiiiiiii................................................................................
[00:55:53] ....................................................................................................
[00:55:57] ...............................................................................................i....
[00:56:00] ....................................................................................................
[00:56:03] .......................................................i.i..ii......................................
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:04:25] 
[01:04:25] running 107 tests
[01:04:28] i..ii...iii....i...i............iii...........i....Fi....ii...i.i.ii..............i...ii..ii.i....ii
[01:04:28] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[01:04:28] failures:
[01:04:28] 
[01:04:28] ---- [codegen] codegen/naked-functions.rs stdout ----
[01:04:28] 
[01:04:28] 
[01:04:28] error: verification with 'FileCheck' failed
[01:04:28] status: exit code: 1
[01:04:28] command: "/usr/lib/llvm-5.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-functions/naked-functions.ll" "/checkout/src/test/codegen/naked-functions.rs"
[01:04:28] ------------------------------------------
[01:04:28] 
[01:04:28] ------------------------------------------
[01:04:28] stderr:
[01:04:28] stderr:
[01:04:28] ------------------------------------------
[01:04:28] /checkout/src/test/codegen/naked-functions.rs:18:11: error: expected string not found in input
[01:04:28] // CHECK: Function Attrs: naked uwtable
[01:04:28]           ^
[01:04:28] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-functions/naked-functions.ll:1:1: note: scanning from here
[01:04:28] ; ModuleID = 'naked_functions.3a1fbbbh-cgu.0'
[01:04:28] ^
[01:04:28] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-functions/naked-functions.ll:6:3: note: possible intended match here
[01:04:28] ; Function Attrs: naked nonlazybind uwtable
[01:04:28] 
[01:04:28] ------------------------------------------
[01:04:28] 
[01:04:28] thread '[codegen] codegen/naked-functions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
---
[01:04:28] test result: FAILED. 77 passed; 1 failed; 29 ignored; 0 measured; 0 filtered out
[01:04:28] 
[01:04:28] 
[01:04:28] 
[01:04:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:28] 
[01:04:28] 
[01:04:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:28] Build completed unsuccessfully in 0:17:44
[01:04:28] Build completed unsuccessfully in 0:17:44
[01:04:28] Makefile:58: recipe for target 'check' failed
[01:04:28] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d3ba4a0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0204f06f:start=1537985051202083688,finish=1537985051358059111,duration=155975423
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1243b65a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:23d227fe
$ dmesg | grep -i kill
