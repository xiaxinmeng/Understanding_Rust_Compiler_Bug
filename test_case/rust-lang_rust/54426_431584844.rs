plain
[00:48:40] .................................................................................................... 2200/4658
[00:48:44] ....................................i............................................................... 2300/4658
[00:48:48] .................................................................................................... 2400/4658
[00:48:51] .................................................................................................... 2500/4658
[00:48:55] ...................................................iiiiiiiii........................................ 2600/4658
[00:49:01] .................................................................................................... 2800/4658
[00:49:04] .................................................................................................... 2900/4658
[00:49:08] .................................................................................i.................. 3000/4658
[00:49:10] .................................................................................................... 3100/4658
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:51] 
[01:01:51] running 111 tests
[01:01:54] i..ii...iii.......i...i.........i.Fiii...........i....Fi.....ii...i.i.ii....F.........i...ii..ii.i.. 100/111
[01:01:54] ..iiii.....
[01:01:54] 
[01:01:54] ---- [codegen] codegen/issue-32031.rs stdout ----
[01:01:54] 
[01:01:54] 
[01:01:54] error: verification with 'FileCheck' failed
[01:01:54] status: exit code: 1
[01:01:54] command: "/usr/lib/llvm-5.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-32031/issue-32031.ll" "/checkout/src/test/codegen/issue-32031.rs"
[01:01:54] ------------------------------------------
[01:01:54] 
[01:01:54] ------------------------------------------
[01:01:54] stderr:
[01:01:54] stderr:
[01:01:54] ------------------------------------------
[01:01:54] /checkout/src/test/codegen/issue-32031.rs:18:11: error: expected string not found in input
[01:01:54] // CHECK: define float @add_newtype_f32(float %a, float %b)
[01:01:54]           ^
[01:01:54] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-32031/issue-32031.ll:1:1: note: scanning from here
[01:01:54] ; ModuleID = 'issue_32031.3a1fbbbh-cgu.0'
[01:01:54] ^
[01:01:54] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-32031/issue-32031.ll:7:1: note: possible intended match here
[01:01:54] define float @add_newtype_f32(float, float) unnamed_addr #0 {
[01:01:54] 
[01:01:54] ------------------------------------------
[01:01:54] 
[01:01:54] thread '[codegen] codegen/issue-32031.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:01:54] thread '[codegen] codegen/issue-32031.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:01:54] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:01:54] 
[01:01:54] ---- [codegen] codegen/move-val-init.rs stdout ----
[01:01:54] 
[01:01:54] error: verification with 'FileCheck' failed
[01:01:54] status: exit code: 1
[01:01:54] command: "/usr/lib/llvm-5.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/move-val-init/move-val-init.ll" "/checkout/src/test/codegen/move-val-init.rs"
[01:01:54] ------------------------------------------
[01:01:54] 
[01:01:54] ------------------------------------------
[01:01:54] stderr:
[01:01:54] stderr:
[01:01:54] ------------------------------------------
[01:01:54] /checkout/src/test/codegen/move-val-init.rs:27:12: error: expected string not found in input
[01:01:54]  // CHECK: call void %make_big(%Big*{{[^%]*}} %target)
[01:01:54]            ^
[01:01:54] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/move-val-init/move-val-init.ll:9:22: note: scanning from here
[01:01:54] define void @test_mvi(%Big* %target, void (%Big*)* nonnull %make_big) unnamed_addr #0 {
[01:01:54]                      ^
[01:01:54] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/move-val-init/move-val-init.ll:14:2: note: possible intended match here
[01:01:54]  call void %make_big(%Big* noalias nocapture sret dereferenceable(65536) %0)
[01:01:54] 
[01:01:54] ------------------------------------------
[01:01:54] 
[01:01:54] thread '[codegen] codegen/move-val-init.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:01:54] thread '[codegen] codegen/move-val-init.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[01:01:54] 
[01:01:54] ---- [codegen] codegen/scalar-pair-bool.rs stdout ----
[01:01:54] 
[01:01:54] error: verification with 'FileCheck' failed
[01:01:54] status: exit code: 1
[01:01:54] command: "/usr/lib/llvm-5.0/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/scalar-pair-bool/scalar-pair-bool.ll" "/checkout/src/test/codegen/scalar-pair-bool.rs"
[01:01:54] ------------------------------------------
[01:01:54] 
[01:01:54] ------------------------------------------
[01:01:54] stderr:
[01:01:54] stderr:
[01:01:54] ------------------------------------------
[01:01:54] /checkout/src/test/codegen/scalar-pair-bool.rs:33:11: error: expected string not found in input
[01:01:54] // CHECK: define { i8, i8 } @pair_and_or(i1 zeroext %arg0.0, i1 zeroext %arg0.1)
[01:01:54]           ^
[01:01:54] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/scalar-pair-bool/scalar-pair-bool.ll:34:68: note: scanning from here
[01:01:54] define { i32, i8 } @pair_i32_bool(i32 %pair.0, i1 zeroext %pair.1) unnamed_addr #0 {
[01:01:54]                                                                    ^
[01:01:54] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/scalar-pair-bool/scalar-pair-bool.ll:43:1: note: possible intended match here
[01:01:54] define { i8, i8 } @pair_and_or(i1 zeroext, i1 zeroext) unnamed_addr #0 {
[01:01:54] 
[01:01:54] ------------------------------------------
[01:01:54] 
[01:01:54] thread '[codegen] codegen/scalar-pair-bool.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
---
[01:01:54] 
[01:01:54] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:01:54] 
[01:01:54] 
[01:01:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:54] 
[01:01:54] 
[01:01:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:54] Build completed unsuccessfully in 0:17:50
[01:01:54] Build completed unsuccessfully in 0:17:50
[01:01:54] Makefile:58: recipe for target 'check' failed
[01:01:54] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1900825c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
