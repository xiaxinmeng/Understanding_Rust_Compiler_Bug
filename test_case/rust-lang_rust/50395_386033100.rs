plain
[00:54:34] .............................................................................ii.....................
[00:55:25] .........................................i....................................................i.ii..
[00:55:32] .............test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:56:09] .......................................................................................
[00:56:29] ..iiiiiii...........................................................................................
[00:57:11] ....................................................................................................
[00:57:29] ...........................................................................
[00:57:29] test result: ok. 2956 passed; 0 failed; 19 ignored; 0 measured; 0 filtered out
[00:57:29] 
---
[00:58:51] ....................................................................................................
[00:59:00] ....................................................................................................
[00:59:12] ....................................................................................................
[00:59:21] ...i................................................................................................
[00:59:28] .i..ii...............................................................test [compile-fail] compile-fail/issue-22638.rs has been running for over 60 seconds
[00:59:40] ....................................................................................................
[00:59:48] ....................................................................................................
[00:59:56] ...................................................................i................................
[01:00:06] ............i.......................................................................................
---
[01:01:16] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[01:01:16] .F................................................
[01:01:16] failures:
[01:01:16] 
[01:01:16] ---- [mir-opt] mir-opt/basic_assignment.rs stdout ----
[01:01:16]  thread '[mir-opt] mir-opt/basic_assignment.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:16] Current block: None
[01:01:16] Actual Line: "        UserAssertTy(Canonical { variables: [], value: std::option::Option<std::boxed::Box<u32>> }, _4);"
[01:01:16] Expected Line: "        UserAssertTy(Canonical { variables: Slice([]), value: std::option::Option<std::boxed::Box<u32>> }, _4);"
[01:01:16] Expected:
[01:01:16] ... (elided)
[01:01:16]     bb0: {
[01:01:16]         StorageLive(_1);
[01:01:16]         _1 = const false;
[01:01:16]         StorageLive(_2);
[01:01:16]         StorageLive(_3);
[01:01:16]         _3 = _1;
[01:01:16]         _2 = move _3;
[01:01:16]         StorageDead(_3);
[01:01:16]         StorageLive(_4);
[01:01:16]         UserAssertTy(Canonical { variables: Slice([]), value: std::option::Option<std::boxed::Box<u32>> }, _4);
[01:01:16]         _4 = std::option::Option<std::boxed::Box<u32>>::None;
[01:01:16]         StorageLive(_5);
[01:01:16]         StorageLive(_6);
[01:01:16]         _6 = move _4;
[01:01:16]         replace(_5 <-move _6) -> [return: bb2, unwind: bb5];
[01:01:16]     bb1: {
[01:01:16]         resume;
[01:01:16]     }
[01:01:16]     bb2: {
[01:01:16]     bb2: {
[01:01:16]         drop(_6) -> [return: bb6, unwind: bb4];
[01:01:16]     bb3: {
[01:01:16]     bb3: {
[01:01:16]         drop(_4) -> bb1;
[01:01:16]     bb4: {
[01:01:16]     bb4: {
[01:01:16]         drop(_5) -> bb3;
[01:01:16]     bb5: {
[01:01:16]     bb5: {
[01:01:16]         drop(_6) -> bb4;
[01:01:16]     bb6: {
[01:01:16]     bb6: {
[01:01:16]         StorageDead(_6);
[01:01:16]         _0 = ();
[01:01:16]         drop(_5) -> [return: bb7, unwind: bb3];
[01:01:16]     bb7: {
[01:01:16]     bb7: {
[01:01:16]         StorageDead(_5);
[01:01:16]         drop(_4) -> [return: bb8, unwind: bb1];
[01:01:16]     bb8: {
[01:01:16]     bb8: {
[01:01:16]         StorageDead(_4);
[01:01:16]         StorageDead(_2);
[01:01:16]         StorageDead(_1);
[01:01:16]         return;
[01:01:16] Actual:
[01:01:16] Actual:
[01:01:16] fn main() -> (){
[01:01:16]     let mut _0: ();
[01:01:16]     scope 1 {
[01:01:16]         let _1: bool;
[01:01:16]         scope 3 {
[01:01:16]             let _2: bool;
[01:01:16]             scope 5 {
[01:01:16]                 let _4: std::option::Option<std::boxed::Box<u32>>;
[01:01:16]                 scope 7 {
[01:01:16]                     let _5: std::option::Option<std        
[01:01:16]         StorageDead(_6);
[01:01:16]         _0 = ();
[01:01:16]         drop(_5) -> [return: bb7, unwind: bb3];
[01:01:16]     }
[01:01:16]     bb7: {                              
[01:01:16]         StorageDead(_5);
[01:01:16]         drop(_4) -> [return: bb8, unwind: bb1];
[01:01:16]     }
[01:01:16]     bb8: {                              
[01:01:16]         StorageDead(_4);
[01:01:16]         StorageDead(_2);
[01:01:16]         StorageDead(_1);
[01:01:16]         return;
[01:01:16] }', tools/compiletest/src/runtest.rs:2735:13
[01:01:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:01:16] 
[01:01:16] 
[01:01:16] 
[01:01:16] failures:
[01:01:16]     [mir-opt] mir-opt/basic_assignment.rs
[01:01:16] 
[01:01:16] test result: FAILED. 49 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:01:16] 
[01:01:16] 
[01:01:16] 
[01:01:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:16] 
[01:01:16] 
[01:01:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:16] Build completed unsuccessfully in 0:17:49
[01:01:16] Build completed unsuccessfully in 0:17:49
[01:01:16] make: *** [check] Error 1
[01:01:16] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:093be0f8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
