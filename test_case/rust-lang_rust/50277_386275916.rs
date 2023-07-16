plain
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:46] 
[01:02:46] running 78 tests
------------------
[01:02:50] /checkout/src/test/codegen/function-arguments.rs:154:11: error: expected string not found in input
[01:02:50] // CHECK: { i1, i8 } @enum_id_2(i1 zeroext %x.0, i8 %x.1)
[01:02:50]           ^
[01:02:50] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments.ll:400:52: note: scanning from here
[01:02:50] define { i16, i16 } @enum_id_1(i16 %x.0, i16 %x.1) unnamed_addr #0 {
[01:02:50]                                                    ^
[01:02:50] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments.ll:402:19: note: possible intended match here
[01:02:50]  %0 = insertvalue { i16, i16 } undef, i16 %x.0, 0
[01:02:50] 
[01:02:50] ------------------------------------------
[01:02:50] 
[01:02:50] thread '[codegen] codegen/function-arguments.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[01:02:50] thread '[codegen] codegen/function-arguments.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[01:02:50] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:02:50] 
[01:02:50] ---- [codegen] codegen/repeat-trusted-len.rs stdout ----
[01:02:50]  
[01:02:50] error: verification with 'FileCheck' failed
[01:02:50] status: exit code: 1
[01:02:50] command: "/usr/lib/llvm-3.9/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len.ll" "/checkout/src/test/codegen/repeat-trusted-len.rs"
[01:02:50] ------------------------------------------
[01:02:50] 
[01:02:50] ------------------------------------------
[01:02:50] stderr:
[01:02:50] stderr:
[01:02:50] ------------------------------------------
[01:02:50] /checkout/src/test/codegen/repeat-trusted-len.rs:26:11: error: expected string not found in input
[01:02:50] // CHECK: call void @llvm.memset.p0i8.[[USIZE]](i8* {{(nonnull )?}}%{{[0-9]+}}, i8 42, [[USIZE]] 100000, i32 1, i1 false)
[01:02:50]           ^
[01:02:50] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len.ll:36:33: note: scanning from here
[01:02:50] define void @repeat_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture sret dereferenceable(24)) unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
[01:02:50]                                 ^
[01:02:50] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len.ll:36:33: note: with variable "USIZE" equal to "i64"
[01:02:50] define void @repeat_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture sret dereferenceable(24)) unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
[01:02:50]                                 ^
[01:02:50] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len.ll:36:33: note: with variable "USIZE" equal to "i64"
[01:02:50] define void @repeat_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture sret dereferenceable(24)) unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
[01:02:50] 
[01:02:50] ------------------------------------------
[01:02:50] 
[01:02:50] thread '[codegen] codegen/repeat-trusted-len.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
---
[01:02:50] test result: FAILED. 56 passed; 2 failed; 20 ignored; 0 measured; 0 filtered out
[01:02:50] 
[01:02:50] 
[01:02:50] 
[01:02:50] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:02:50] 
[01:02:50] 
[01:02:50] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:02:50] Build completed unsuccessfully in 0:17:56
[01:02:50] Build completed unsuccessfully in 0:17:56
[01:02:50] make: *** [check] Error 1
[01:02:50] Makefile:58: recipe for target 'check' failed
az72e5d/s-f0oudboefg-1fcypi6-1ujokh614sjd2
112944 ./obj/build/x86_64-unknown-linux-gnu/test/mir-opt
111764 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
111760 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
107788 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
107788 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
102840 ./obj/build/bootstrap/debug/incremental/bootstrap-2wettvttcntnm
102836 ./obj/build/bootstrap/debug/incremental/bootstrap-2wettvttcntnm/s-f0ovk3dgxi-1iewviq-3sijyc9m9vnmm
98300 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental
92672 ./obj/build/x86_64-unknown-linux-gnu/stage1
92648 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
90800 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90800 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90796 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz/s-f0ovhl2rss-bssmga-df2ow3h1s6v7
86668 ./obj/build/x86_64-unknown-linux-gnu/doc/core
81076 ./obj/build/x86_64-unknown-linux-gnu/doc/std
80744 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
79068 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot
