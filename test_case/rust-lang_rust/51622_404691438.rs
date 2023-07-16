plain
[01:15:59] failures:
[01:15:59] 
[01:15:59] ---- [codegen] codegen/issue-45222.rs stdout ----
[01:15:59] 
[01:15:59] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:15:59] error: verification with 'FileCheck' failed
[01:15:59] status: exit code: 1
[01:15:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll" "/checkout/src/test/codegen/issue-45222.rs"
[01:15:59] ------------------------------------------
[01:15:59] 
[01:15:59] ------------------------------------------
[01:15:59] stderr:
[01:15:59] stderr:
[01:15:59] ------------------------------------------
[01:15:59] /checkout/src/test/codegen/issue-45222.rs:52:12: error: expected string not found in input
[01:15:59]  // CHECK: ret i64 5000050000
[01:15:59]            ^
[01:15:59] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:16:31: note: scanning from here
[01:15:59] define i64 @check_triangle_inc() unnamed_addr #0 personality i32 (i32, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
[01:15:59]                               ^
[01:15:59] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:45:2: note: possible intended match here
[01:15:59]  ret i64 %count.0.i
[01:15:59] 
[01:15:59] ------------------------------------------
[01:15:59] 
[01:15:59] thread '[codegen] codegen/issue-45222.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
---
[01:15:59] test result: FAILED. 77 passed; 1 failed; 20 ignored; 0 measured; 0 filtered out
[01:15:59] 
[01:15:59] 
[01:15:59] 
[01:15:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-arm-unknown-linux-gnueabihf" "--mode" "codegen" "--target" "arm-unknown-linux-gnueabihf" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-linux-gnueabihf-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "7.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:15:59] 
[01:15:59] 
[01:15:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-unknown-linux-gnueabihf
[01:15:59] Build completed unsuccessfully in 1:12:53
---
travis_time:end:0915c3b7:start=1531442260441499410,finish=1531442260448266148,duration=6766738
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:23d4bc00
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:27da8068
$ dmesg | grep -i kill
