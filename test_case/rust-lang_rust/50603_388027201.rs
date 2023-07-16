plain
[00:52:10] ..................test [run-pass] run-pass/mir_heavy_promoted.rs has been running for over 60 seconds
[00:52:33] ..................................................................................
[00:52:52] ..................................................................................ii................
[00:53:42] ..............................................i....................................................i
[00:53:54] .ii.............................test [run-pass] run-pass/saturating-float-casts.rs has been running for over 60 seconds
[00:54:43] .......iiiiiii......................................................................................
[00:55:03] ....................................................................................................
[00:55:22] ....................................................................................................
[00:55:40] ................................................................................
---
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:58:57] 
[00:58:57] running 50 tests
[00:59:07] ERROR 2018-05-10T11:25:42Z: compiletest::runtest: Some(" bb9: { // binding1 and guard")
[00:59:18]      _9 = _4;
[00:59:18]      _9 = _4;
[00:59:18]      _1 = (const 1i32, move _9);
[00:59:18]      StorageDead(_9);
[00:59:18]      goto -> bb13;
[00:59:18]  }
[00:59:18]  bb3: { // binding3(empty) and arm3
[00:59:18]      _1 = (const 3i32, const 3i32);
[00:59:18]      goto -> bb13;
[00:59:18]  }
[00:59:18]  bb4: {
[00:59:18]      falseEdges -> [real: bb9, imaginary: bb5]; //pre_binding1
[00:59:18]  }
[00:59:18]  bb5: {
[00:59:18]      falseEdges -> [real: bb12, imaginary: bb6]; //pre_binding2
[00:59:18]  }
[00:59:18]  bb6: {
[00:59:18]      falseEdges -> [real: bb3, imaginary: bb7]; //pre_binding3
[00:59:18]  }
[00:59:18]  bb7: {
[00:59:18]      unreachable;
[00:59:18]  }
[00:59:18]  bb8: {
[00:59:18]      unreachable;
[00:59:18]  }
[00:59:18]  bb9: { // binding1 and guard
[00:59:18]      StorageLive(_5);
[00:59:18]      _5 = &((_2 as Some).0: i32);
[00:59:18]      StorageLive(_8);
[00:59:18]      _8 = const guard() -> [return: bb10, unwind: bb1];
[00:59:18]  }
[00:59:18]  bb10: { // end of guard
[00:59:18]      StorageLive(_4);
[00:59:18]      _4 = ((_2 as Some).0: i32);
[00:59:18]      switchInt(move _8) -> [false: bb11, otherwise: bb2];
[00:59:18]  }
[00:59:18]  bb11: { // to pre_binding2
[00:59:18]      falseEdges -> [real: bb5, imaginary: bb5];
[00:59:18]  }
[00:59:18]  bb12: { // bindingNoLandingPads.before.mir2 and arm2
[00:59:18]      StorageLive(_6);
[00:59:18]      _6 = ((_2 as Some).0: i32);
[00:59:18]      StorageLive(_10);
[00:59:18]      _10 = _6;
[00:59:18]      _1 = (const 2i32, move _10);
[00:59:18]      StorageDead(_10);
[00:59:18]      goto -> bb13;
[00:59:18]  }
[00:5 -> [real: bb9, imaginary: bb5];
[00:59:18]     }
[00:59:18]     bb5: {                              
[00:59:18]         falseEdges -> [real: bb12, imaginary: bb6];
[00:59:18]     }
[00:59:18]     bb6: {                              
[00:59:18]         falseEdges -> [real: bb3, imaginary: bb7];
[00:59:18]     }
[00:59:18]     bb7: {                              
[00:59:18]         unreachable;
[00:59:18]     }
[00:59:18]     bb8: {                              
[00:59:18]         unreachable;
[00:59:18]     }
[00:59:18]     bb9: {                              
[00:59:18]         StorageLive(_5);
[00:59:18]         _11 = promoted[0];
[00:59:18]         _5 = &(((*_11) as Some).0: i32);
[00:59:18]         StorageLive(_8);
[00:59:18]         _8 = const guard() -> [return: bb10, unwind: bb1];
[00:59:18]     }
[00:59:18]     bb10: {                             
[00:59:18]         StorageLive(_4);
[00:59:18]         _4 = ((_2 as Some).0: i32);
[00:59:18]         switchInt(move _8) -> [false: bb11, otherwise: bb2];
[00:59:18]     }
[00:59:18]     bb11: {                             
[00:59:18]         falseEdges -> [real: bb5, imaginary: bb5];
[00:59:18]     }
[00:59:18]     bb12: {                             
[00:59:18]         StorageLive(_6);
[00:59:18]         _6 = ((_2 as Some).0: i32);
[00:59:18]         StorageLive(_10);
[00:59:18]         _10 = _6;
[00:59:18]         _1 = (const 2i32, move _10);
[00:59:18]         StorageDead(_10);
[00:59:18]         goto -> bb13;
[00:59:18]     }
[00:59:18]     bb13: {                             
[00:59:18]         StorageDead(_6);
[00:59:18]         StorageDead(_4);
[00:59:18]         StorageDead(_8);
[00:59:18]         StorageDead(_5);
[00:59:18]         StorageDead(_1);
[00:59:18]         StorageDead(_2);
[00:59:18]         _0 = ();
[00:59:18]         return;
[00:59:18] }', tools/compiletest/src/runtest.rs:2801:13
[00:59:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:59:18] 
[00:59:18] 
[00:59:18] 
[00:59:18] failures:
[00:59:18]     [mir-opt] mir-opt/match_false_edges.rs
[00:59:18] 
[00:59:18] test result: FAILED. 49 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[00:59:18] 
[00:59:18] 
[00:59:18] 
[00:59:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:59:18] 
[00:59:18] 
[00:59:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:59:18] Build completed unsuccessfully in 0:17:06
[00:59:18] Build completed unsuccessfully in 0:17:06
[00:59:18] Makefile:58: recipe for target 'check' failed
[00:59:18] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:002d674e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
