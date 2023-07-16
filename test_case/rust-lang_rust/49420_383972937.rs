plain
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:36] 
[00:57:36] running 75 tests
[00:57:40] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:57:40] i...i..ii.F..i.............ii.....F...iii......i..i...i...ii..i..i..ii.....
[00:57:40] 
[00:57:40] ---- [codegen] codegen/align-struct.rs stdout ----
[00:57:40]  
[00:57:40]  
[00:57:40] error: verification with 'FileCheck' failed
[00:57:40] status: exit code: 1
[00:57:40] command: "/usr/lib/llvm-3.9/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-struct.ll" "/checkout/src/test/codegen/align-struct.rs"
[00:57:40] ------------------------------------------
[00:57:40] 
[00:57:40] ------------------------------------------
[00:57:40] stderr:
[00:57:40] stderr:
[00:57:40] ------------------------------------------
[00:57:40] /checkout/src/test/codegen/align-struct.rs:61:11: error: expected string not found in input
[00:57:40] // CHECK: %e4 = alloca { i32, i32 }, align 4
[00:57:40]           ^
[00:57:40] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-struct.ll:78:18: note: scanning from here
[00:57:40] define i64 @enum4(i32 %a) unnamed_addr #0 {
[00:57:40]                  ^
[00:57:40] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-struct.ll:81:2: note: possible intended match here
[00:57:40]  %e4 = alloca %Enum4, align 4
[00:57:40]  ^
[00:57:40] ------------------------------------------
[00:57:40] 
[00:57:40] thread '[codegen] codegen/align-struct.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2956:9
[00:57:40] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:40] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:40] 
[00:57:40] ---- [codegen] codegen/lifetime_start_end.rs stdout ----
[00:57:40]  
[00:57:40] error: verification with 'FileCheck' failed
[00:57:40] status: exit code: 1
[00:57:40] command: "/usr/lib/llvm-3.9/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/lifetime_start_end.ll" "/checkout/src/test/codegen/lifetime_start_end.rs"
[00:57:40] ------------------------------------------
[00:57:40] 
[00:57:40] ------------------------------------------
[00:57:40] stderr:
[00:57:40] stderr:
[00:57:40] ------------------------------------------
[00:57:40] /checkout/src/test/codegen/lifetime_start_end.rs:28:11: error: expected string not found in input
[00:57:40] // CHECK: [[S_b:%[0-9]+]] = bitcast { i32, i32 }** %b to i8*
[00:57:40]           ^
[00:57:40] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/lifetime_start_end.ll:18:2: note: scanning from here
[00:57:40]  store i32 0, i32* %a, align 4
[00:57:40]  ^
[00:57:40] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/lifetime_start_end.ll:34:2: note: possible intended match here
[00:57:40]  %9 = bitcast i32* %c to i8*
[00:57:40]  ^
[00:57:40] ------------------------------------------
[00:57:40] 
[00:57:40] thread '[codegen] codegen/lifetime_start_end.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2956:9
[00:57:40] 
---
[00:57:40] test result: FAILED. 54 passed; 2 failed; 19 ignored; 0 measured; 0 filtered out
[00:57:40] 
[00:57:40] 
[00:57:40] 
[00:57:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:57:40] 
[00:57:40] 
[00:57:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:57:40] Build completed unsuccessfully in 0:17:32
[00:57:40] Build completed unsuccessfully in 0:17:32
[00:57:40] Makefile:58: recipe for target 'check' failed
[00:57:40] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b5e1899
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
