plain
[00:50:42] .............................................................................ii.....................
[00:51:29] .........................................i....................................................i.ii..
[00:51:40] .............................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:52:09] .......................................................................
[00:52:25] ..iiiiiii...........................................................................................
[00:53:04] ....................................................................................................
[00:53:20] ..........................................................................
[00:53:20] test result: ok. 2955 passed; 0 failed; 19 ignored; 0 measured; 0 filtered out
[00:53:20] 
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:56:57] 
[00:56:57] running 77 tests
[00:57:01] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:57:01] i...ii..ii....i.............ii........Fiii......i..i...i...ii..i..i...ii.....
[00:57:01] 
[00:57:01] ---- [codegen] codegen/link_section.rs stdout ----
[00:57:01]  
[00:57:01]  
[00:57:01] error: verification with 'FileCheck' failed
[00:57:01] status: exit code: 1
[00:57:01] command: "/usr/lib/llvm-3.9/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/link_section.ll" "/checkout/src/test/codegen/link_section.rs"
[00:57:01] ------------------------------------------
[00:57:01] 
[00:57:01] ------------------------------------------
[00:57:01] stderr:
[00:57:01] stderr:
[00:57:01] ------------------------------------------
[00:57:01] /checkout/src/test/codegen/link_section.rs:15:11: error: expected string not found in input
[00:57:01] // CHECK: @VAR1 = constant <{ [4 x i8] }> <{ [4 x i8] c"\01\00\00\00" }>, section ".test_one"
[00:57:01]           ^
[00:57:01] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/link_section.ll:1:1: note: scanning from here
[00:57:01] ; ModuleID = 'link_section0-8787f43e282added376259c1adb08b80.rs'
[00:57:01] ^
[00:57:01] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/link_section.ll:8:7: note: possible intended match here
[00:57:01] @VAR3 = constant <{ [8 x i8] }> <{ [8 x i8] c"\01\00\00\00\00\00\80?" }>, section ".test_three", align 4
[00:57:01] 
[00:57:01] ------------------------------------------
[00:57:01] 
[00:57:01] thread '[codegen] codegen/link_section.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
---
[00:57:01] test result: FAILED. 56 passed; 1 failed; 20 ignored; 0 measured; 0 filtered out
[00:57:01] 
[00:57:01] 
[00:57:01] 
[00:57:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:01] 
[00:57:01] 
[00:57:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:01] Build completed unsuccessfully in 0:16:43
[00:57:01] Build completed unsuccessfully in 0:16:43
[00:57:01] Makefile:58: recipe for target 'check' failed
[00:57:01] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:00a6b4a0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
