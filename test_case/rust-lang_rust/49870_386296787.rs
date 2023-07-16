plain
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:57:47] 
[00:57:47] running 50 tests
[00:57:58] ERROR 2018-05-03T13:32:55Z: compiletest::runtest: None
[00:58:12] ...........................F......................
[00:58:12] 
[00:58:12] ---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
[00:58:12] ---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
[00:58:12]  thread '[mir-opt] mir-opt/match_false_edges.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:58:12] Current block: None
[00:58:12] Actual Line: "        _7 = discriminant(_2);"
[00:58:12] Expected Line: "     _6 = discriminant(_2);"
[00:58:12] Test Name: rustc.full_tested_match.QualifyAndPromoteConstants.after.mir
[00:58:12] Expected:
[00:58:12] ... (elided)
[00:58:12]  bb0: {
[00:58:12] ... (elided)
[00:58:12]      _2 = std::option::Option<i32>::Some(const 42i32,);
[00:58:12]      _3 = discriminant(_2);
[00:58:12]      _6 = discriminant(_2);
[00:58:12]      switchInt(move _6) -> [0isize: bb6, 1isize: bb4, otherwise: bb8];
[00:58:12]  }
[00:58:12]  bb1: {
[00:58:12]      resume;
[00:58:12]  }
[00:58:12]  bb2: {  // arm1
[00:58:12]      StorageLive(_8);
[00:58:12]      _8 = _4;
[00:58:12]      _1 = (const 1i32, move _8);
[00:58:12]      StorageDead(_8);
[00:58:12]      goto -> bb13;
[00:58:12]  }
[00:58:12]  bb3: { // binding3(empty) and arm3
[00:58:12]      _1 = (const 3i32, const 3i32);
[00:58:12]      goto -> bb13;
[00:58:12]  }
[00:58:12]  bb4: {
[00:58:12]      falseEdges -> [real: bb9, imaginary: bb5]; //pre_binding1
[00:58:12]  }
[00:58:12]  bb5: {
[00:58:12]      falseEdges -> [real: bb12, imaginary: bb6]; //pre_binding2
[00:58:12]  }
[00:58:12]  bb6: {
[00:58:12]      falseEdges -> [real: bb3, imaginary: bb7]; //pre_binding3
[00:58:12]  }
[00:58:12]  bb7: {
[00:58:12]      unreachable;
[00:58:12]  }
[00:58:12]  bb8: {
[00:58:12]      unreachable;
[00:58:12]  }
[00:58:12]  bb9: { // binding1 and guard
[00:58:12]      StorageLive(_4);
[00:58:12]      _4 = ((_2 as Some).0: i32);
[00:58:12]      StorageLive(_7);
[00:58:12]      _7 = const guard() -> [return: bb10, unwind: bb1];
[00:58:12]  }
[00:58:12]  bb10: { // end of guard
[00:58:12]      switchInt(move _7) -> [false: bb11, otherwise: bb2];
[00:58:12]  }
[00:58:12]  bb11: { // to pre_binding2
[00:58:12]      falseEdges -> [real: bb5, imaginary: bb5];
[00:58:12]  }
[00:58:12]  bb12: { // bindingNoLandingPads.before.mir2 and arm2
[00:58:12]      StorageLive(_5);
[00:58:12]      _5 = ((_2 as Some).0: i32);
[00:58:12]      StorageLive(_9);
[00:58:12]      _9 = _5;
[00:58:12]      _1 = (const 2i32, move _9);
[00:58:12]      StorageDead(_9);
[00:58:12]      goto -> bb13;
[00:58:12]  }
[00:58:12]  bb13: {
[00:58:12] ... (elided)
[00:58:12]      return;
[00:58:12]  }
[00:58:12] Actual:
[00:58:12] fn full_tested_match() -> (){
[00:58:12]     let mut _0: ();
[00:58:12]     scope 1 {
[00:58:12]         let _4: i32;
[00:58:12]         let _5: &'<empty> i32;
[00:58:12]     scope 2 {
[00:58:12]         let _6: i32;
[00:58:12]     }
[00:58:12]     }
[00:58:12]     let mut _1: (i32, i32);
[00:58:12]     let mut _2: std::option::Option<i32>;
[00:58:12]     let mut _3: isize;
[00:58:12]     let mut _7: isize;
[00:58:12]     let mut _8: bool;
[00:58:12]     let mut _9: i32;
[00:58:12]     let mut _10: i32;
[00:58:12]     bb0: {                              
[00:58:12]         StorageLive(_1);
[00:58:12]         StorageLive(_2);
[00:58:12]         _2 = std::option::Option<i32>::Some(const 42i32,);
[00:58:12]         _3 = discriminant(_2);
[00:58:12]         _7 = discriminant(_2);
[00:58:12]         switchInt(move _7) -> [0isize: bb6, 1isize: bb4, otherwise: bb8];
[00:58:12]     bb1: {
[00:58:12]         resume;
[00:58:12]     }
[00:58:12]     }
[00:58:12]     bb2: {                              
[00:58:12]         StorageLive(_9);
[00:58:12]         _9 = _4;
[00:58:12]         _1 = (const 1i32, move _9);
[00:58:12]         StorageDead(_9);
[00:58:12]         goto -> bb13;
[00:58:12]     }
[00:58:12]     bb3: {                              
[00:58:12]         _1 = (const 3i32, const 3i32);
[00:58:12]         goto -> bb13;
[00:58:12]     }
[00:58:12]     bb4: {                              
[00:58:12]         falseEdges -> [real: bb9, imaginary: bb5];
[00:58:12]     }
[00:58:12]     bb5: {                              
[00:58:12]         falseEdges -> [real: bb12, imaginary: bb6];
[00:58:12]     }
[00:58:12]     bb6: {                              
[00:58:12]         falseEdges -> [real: bb3, imaginary: bb7];
[00:58:12]     }
[00:58:12]     bb7: {                              
[00:58:12]         unreachable;
[00:58:12]     }
[00:58:12]     bb8: {                              
[00:58:12]         unreachable;
[00:58:12]     }
[00:58:12]     bb9: {                              
[00:58:12]         StorageLive(_5);
[00:58:12]         _5 = &((_2 as Some).0: i32);
[00:58:12]         StorageLive(_8);
[00:58:12]         _8 = const guard() -> [return: bb10, unwind: bb1];
[00:58:12]     }
[00:58:12]     bb10: {                             
[00:58:12]         StorageLive(_4);
[00:58:12]         _4 = ((_2 as Some).0: i32);
[00:58:12]         switchInt(move _8) -> [false: bb11, otherwise: bb2];
[00:58:12]     }
[00:58:12]     bb11: {                             
[00:58:12]         falseEdges -> [real: bb5, imaginary: bb5];
[00:58:12]     }
[00:58:12]     bb12: {                             
[00:58:12]         StorageLive(_6);
[00:58:12]         _6 = ((_2 as Some).0: i32);
[00:58:12]         StorageLive(_10);
[00:58:12]         _10 = _6;
[00:58:12]         _1 = (const 2i32, move _10);
[00:58:12]         StorageDead(_10);
[00:58:12]         goto -> bb13;
[00:58:12]     }
[00:58:12]     bb13: {                             
[00:58:12]         StorageDead(_6);
[00:58:12]         StorageDead(_4);
[00:58:12]         StorageDead(_8);
[00:58:12]         StorageDead(_5);
[00:58:12]         StorageDead(_1);
[00:58:12]         StorageDead(_2);
[00:58:12]         _0 = ();
[00:58:12]         return;
[00:58:12] }', tools/compiletest/src/runtest.rs:2735:13
[00:58:12] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:58:12] 
[00:58:12] 
---
[00:58:12] 
[00:58:12] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:58:12] 
[00:58:12] 
[00:58:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:58:12] 
[00:58:12] 
[00:58:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:58:12] Build completed unsuccessfully in 0:17:20
[00:58:12] Build completed unsuccessfully in 0:17:20
[00:58:12] Makefile:58: recipe for target 'check' failed
[00:58:12] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:163a6649
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
