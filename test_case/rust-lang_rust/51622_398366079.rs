plain
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:03] 
[00:57:03] running 96 tests
[00:57:09] i...ii..ii....i...i..........i.i.......F.iii......i..i...i...ii.............i...ii..i...iii.....
[00:57:09] 
[00:57:09] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:57:09] ---- [codegen] codegen/issue-45222.rs stdout ----
[00:57:09] 
[00:57:09] 
[00:57:09] error: verification with 'FileCheck' failed
[00:57:09] status: exit code: 1
[00:57:09] command: "/usr/lib/llvm-3.9/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll" "/checkout/src/test/codegen/issue-45222.rs"
[00:57:09] ------------------------------------------
[00:57:09] 
[00:57:09] ------------------------------------------
[00:57:09] stderr:
[00:57:09] stderr:
[00:57:09] ------------------------------------------
[00:57:09] /checkout/src/test/codegen/issue-45222.rs:51:12: error: expected string not found in input
[00:57:09]  // CHECK: ret i64 5000050000
[00:57:09]            ^
[00:57:09] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:16:31: note: scanning from here
[00:57:09] define i64 @check_triangle_inc() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
[00:57:09]                               ^
[00:57:09] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:30:2: note: possible intended match here
[00:57:09]  ret i64 %2
[00:57:09]  ^
[00:57:09] /checkout/src/test/codegen/issue-45222.rs:71:12: error: expected string not found in input
[00:57:09]  // CHECK: ret i64 500005000000000
[00:57:09]            ^
[00:57:09] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:34:24: note: scanning from here
[00:57:09] define i64 @check_foo3r() unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
[00:57:09]                        ^
[00:57:09] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-45222/issue-45222.ll:93:25: note: possible intended match here
[00:57:09]  %exitcond.i.i.3 = icmp eq i64 %21, 100000
[00:57:09] 
[00:57:09] ------------------------------------------
[00:57:09] 
[00:57:09] thread '[codegen] codegen/issue-45222.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
---
[00:57:09] test result: FAILED. 71 passed; 1 failed; 24 ignored; 0 measured; 0 filtered out
[00:57:09] 
[00:57:09] 
[00:57:09] 
[00:57:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" 

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1277d6e4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
