plain
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:34] 
[00:58:34] running 75 tests
[00:58:37] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:58:37] i...i..ii....i.............ii.....F...iii......i..i...i...ii..i..i..ii.....
[00:58:37] 
[00:58:37] ---- [codegen] codegen/lifetime_start_end.rs stdout ----
[00:58:37]  
[00:58:37]  
[00:58:37] error: verification with 'FileCheck' failed
[00:58:37] status: exit code: 1
[00:58:37] command: "/usr/lib/llvm-3.9/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/lifetime_start_end.ll" "/checkout/src/test/codegen/lifetime_start_end.rs"
[00:58:37] ------------------------------------------
[00:58:37] 
[00:58:37] ------------------------------------------
[00:58:37] stderr:
[00:58:37] stderr:
[00:58:37] ------------------------------------------
[00:58:37] /checkout/src/test/codegen/lifetime_start_end.rs:28:11: error: expected string not found in input
[00:58:37] // CHECK: [[S_b:%[0-9]+]] = bitcast %"core::option::Option<i32>"** %b to i8*
[00:58:37]           ^
[00:58:37] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/lifetime_start_end.ll:17:2: note: scanning from here
[00:58:37]  store i32 0, i32* %a, align 4
[00:58:37]  ^
[00:58:37] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/lifetime_start_end.ll:23:29: note: possible intended match here
[00:58:37]  %4 = bitcast { i32, i32 }* %_4 to %"core::option::Option<i32>::Some"*
[00:58:37] 
[00:58:37] ------------------------------------------
[00:58:37] 
[00:58:37] thread '[codegen] codegen/lifetime_start_end.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2956:9
---
[00:58:37] test result: FAILED. 55 passed; 1 failed; 19 ignored; 0 measured; 0 filtered out
[00:58:37] 
[00:58:37] 
[00:58:37] 
[00:58:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:37] 
[00:58:37] 
[00:58:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:37] Build completed unsuccessfully in 0:17:29
[00:58:37] Build completed unsuccessfully in 0:17:29
[00:58:37] Makefile:58: recipe for target 'check' failed
[00:58:37] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01806598
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
