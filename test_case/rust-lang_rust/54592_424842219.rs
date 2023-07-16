plain
[00:58:24] ....................................................................................................
[00:58:27] ..............................................................i.....................................
[00:58:30] ....................................................................................................
[00:58:33] ....................................................................................................
[00:58:36] ............iiiiiiiii...............................................................................
[00:58:42] ....................................................................................................
[00:58:46] ...............................................................................................i....
[00:58:49] ....................................................................................................
[00:58:52] .......................................................i.i..ii......................................
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:32] 
[01:07:32] running 107 tests
[01:07:35] i..ii...iii....i...i............iii...........i.....iF...ii...i.i.ii..............i...ii..ii.i....ii
[01:07:35] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[01:07:35] failures:
[01:07:35] 
[01:07:35] ---- [codegen] codegen/naked-functions.rs stdout ----
[01:07:35] 
[01:07:35] 
[01:07:35] error: verification with 'FileCheck' failed
[01:07:35] status: exit code: 1
[01:07:35] command: "/usr/lib/llvm-5.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-functions/naked-functions.ll" "/checkout/src/test/codegen/naked-functions.rs"
[01:07:35] ------------------------------------------
[01:07:35] 
[01:07:35] ------------------------------------------
[01:07:35] stderr:
[01:07:35] stderr:
[01:07:35] ------------------------------------------
[01:07:35] /checkout/src/test/codegen/naked-functions.rs:18:11: error: expected string not found in input
[01:07:35] // CHECK: Function Attrs: naked uwtable
[01:07:35]           ^
[01:07:35] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-functions/naked-functions.ll:1:1: note: scanning from here
[01:07:35] ; ModuleID = 'naked_functions.3a1fbbbh-cgu.0'
[01:07:35] ^
[01:07:35] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/naked-functions/naked-functions.ll:6:3: note: possible intended match here
[01:07:35] ; Function Attrs: naked nonlazybind uwtable
[01:07:35] 
[01:07:35] ------------------------------------------
[01:07:35] 
[01:07:35] thread '[codegen] codegen/naked-functions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3238:9
---
[01:07:35] test result: FAILED. 77 passed; 1 failed; 29 ignored; 0 measured; 0 filtered out
[01:07:35] 
[01:07:35] 
[01:07:35] 
[01:07:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:35] 
[01:07:35] 
[01:07:35] failed to run: /checkout/obj/build/756 ./src/tools/lldb/www
37080 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
---
travis_time:end:0c6f72ba:start=1537990447136677365,finish=1537990447141035866,duration=4358501
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:017849be
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then print
