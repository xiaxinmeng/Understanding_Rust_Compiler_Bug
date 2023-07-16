plain
[00:55:25] ............................................................................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:55:31] ........................
[00:55:45] ....................................................................................................
[00:56:29] ......ii..............................................................i.............................
[00:56:51] .......................i.ii....................................................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:57:28] ...............................iiiiiii..............................................................
[00:57:47] ....................................................................................................
[00:58:01] ....................................................................................................
[00:58:19] ....................................................................................................
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:00:49] 
[01:00:49] running 50 tests
[01:01:06] ERROR 2018-05-19T11:13:39Z: compiletest::runtest: None
[01:01:11] ...........................................F......
[01:01:11] 
[01:01:11] ---- [mir-opt] mir-opt/validate_2.rs stdout ----
[01:01:11] ---- [mir-opt] mir-opt/validate_2.rs stdout ----
[01:01:11] thread '[mir-opt] mir-opt/validate_2.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:01:11] Current block: None
[01:01:11] Actual Line: "        Validate(Acquire, [_2: std::boxed::Box<[i32; 3], std::heap::Global>]);"
[01:01:11] Expected Line: "        Validate(Acquire, [_2: std::boxed::Box<[i32; 3]>]);"
[01:01:11] Test Name: rustc.main.EraseRegions.after.mir
[01:01:11] Expected:
[01:01:11] ... (elided)
[01:01:11] fn main() -> () {
[01:01:11] ... (elided)
[01:01:11]     bb1: {
[01:01:11]         Validate(Acquire, [_2: std::boxed::Box<[i32; 3]>]);
[01:01:11]         Validate(Release, [_2: std::boxed::Box<[i32; 3]>]);
[01:01:11]         _1 = move _2 as std::boxed::Box<[i32]> (Unsize);
[01:01:11]         Validate(Acquire, [_1: std::boxed::Box<[i32]>]);
[01:01:11]         StorageDead(_2);
[01:01:11]         StorageDead(_3);
[01:01:11]         _0 = ();
[01:01:11]         Validate(Release, [_1: std::boxed::Box<[i32]>]);
[01:01:11]         drop(_1) -> [return: bb2, unwind: bb3];
[01:01:11]     }
[01:01:11] ... (elided)
[01:01:11] }
[01:01:11] Actual:
[01:01:11] fn main() -> (){
[01:01:11]     let mut _0: ();
[01:01:11]     scope 1 {
[01:01:11]         let _1: std::boxed::Box<[i32], std::heap::Global>;
[01:01:11]     scope 2 {
[01:01:11]     }
[01:01:11]     }
[01:01:11]     let mut _2: std::boxed::Box<[i32; 3], std::heap::Global>;
[01:01:11]     let mut _3: [i32; 3];
[01:01:11]     bb0: {                              
[01:01:11]         StorageLive(_1);
[01:01:11]         StorageLive(_2);
[01:01:11]         StorageLive(_3);
[01:01:11]         _3 = [const 1i32, const 2i32, const 3i32];
[01:01:11]         Validate(Release, [_2: std::boxed::Box<[i32; 3], std::heap::Global>, _3: [i32; 3]]);
[01:01:11]         _2 = const <std::boxed::Box<T, std::heap::Global>>::new(move _3) -> bb1;
[01:01:11]     }
[01:01:11]     bb1: {                              
[01:01:11]         Validate(Acquire, [_2: std::boxed::Box<[i32; 3], std::heap::Global>]);
[01:01:11]         Validate(Release, [_2: std::boxed::Box<[i32; 3], std::heap::Global>]);
[01:01:11]         _1 = move _2 as std::boxed::Box<[i32], std::heap::Global> (Unsize);
[01:01:11]         Validate(Acquire, [_1: std::boxed::Box<[i32], std::heap::Global>]);
[01:01:11]         StorageDead(_2);
[01:01:11]         StorageDead(_3);
[01:01:11]         _0 = ();
[01:01:11]         Validate(Release, [_1: std::boxed::Box<[i32], std::heap::Global>]);
[01:01:11]         drop(_1) -> [return: bb2, unwind: bb3];
[01:01:11]     }
[01:01:11]     bb2: {                              
[01:01:11]         StorageDead(_1);
[01:01:11]         return;
[01:01:11]     bb3: {
[01:01:11]         resume;
[01:01:11]     }
[01:01:11] }', tools/compiletest/src/runtest.rs:2776:13
---
[01:01:11] 
[01:01:11] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:01:11] 
[01:01:11] 
[01:01:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:01:11] 
[01:01:11] 
[01:01:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:01:11] Build completed unsuccessfully in 0:15:04
[01:01:11] Build completed unsuccessfully in 0:15:04
[01:01:11] Makefile:58: recipe for target 'check' failed
[01:01:11] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3146d628
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
