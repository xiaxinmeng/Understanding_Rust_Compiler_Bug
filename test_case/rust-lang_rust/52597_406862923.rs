plain
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:12] 
[00:54:12] running 46 tests
[00:54:17] ERROR 2018-07-22T12:26:19Z: compiletest::runtest: None
[00:54:20] ERROR 2018-07-22T12:26:22Z: compiletest::runtest: None
     _14 = promoted[1];
[00:54:29]      _4 = &(*_14);
[00:54:29]      _9 = discriminant(_2);
[00:54:29]      switchInt(move _9) -> [0isize: bb5, 1isize: bb3, otherwise: bb7];
[00:54:29]  }
[00:54:29]  bb1: {
[00:54:29]      resume;
[00:54:29]  }
[00:54:29]  bb2: {  // arm1
[00:54:29]      _1 = (const 3i32, const 3i32);
[00:54:29]      goto -> bb13;
[00:54:29]  }
[00:54:29]  bb3: { // binding3(empty) and arm3
[00:54:29]      ReadForMatch(_4);
[00:54:29]      falseEdges -> [real: bb8, imaginary: bb4]; //pre_binding1
[00:54:29]  }
[00:54:29]  bb4: {
[00:54:29]      ReadForMatch(_4);
[00:54:29]      falseEdges -> [real: bb12, imaginary: bb5]; //pre_binding2
[00:54:29]  }
[00:54:29]  bb5: {
[00:54:29]      ReadForMatch(_4);
[00:54:29]      falseEdges -> [real: bb2, imaginary: bb6]; //pre_binding3
[00:54:29]  }
[00:54:29]  bb6: {
[00:54:29]  }
[00:54:29]  bb7: {
[00:54:29]      unreachable;
[00:54:29]  }
[00:54:29]  }
[00:54:29]  bb8: { // binding1 and guard
[00:54:29]      StorageLive(_7);
[00:54:29]      _13 = promoted[0];
[00:54:29]      _7 = &(((*_13) as Some).0: i32);
[00:54:29]      StorageLive(_10);
[00:54:29]      _10 = const guard() -> [return: bb9, unwind: bb1];
[00:54:29]  }
[00:54:29]  bb9: {
[00:54:29]      switchInt(move _10) -> [false: bb10, otherwise: bb11];
[00:54:29]  }
[00:54:29]  bb10: { // to pre_binding2
[00:54:29]      falseEdges -> [real: bb4, imaginary: bb4];
[00:54:29]  }
[00:54:29]  bb11: { // bindingNoLandingPads.before.mir2 and arm2
[00:54:29]      StorageLive(_5);
[00:54:29]      _5 = ((_2 as Some).0: i32);
[00:54:29]      StorageLive(_11);
, const 3i32);
[00:54:29]         goto -> bb13;
[00:54:29]     }
[00:54:29]     bb3: {                              
[00:54:29]         ReadForMatch(_4);
[00:54:29]         falseEdges -> [real: bb8, imaginary: bb4];
[00:54:29]     }
[00:54:29]     bb4: {                              
[00:54:29]         ReadForMatch(_4);
[00:54:29]         falseEdges -> [real: bb12, imaginary: bb5];
[00:54:29]     }
[00:54:29]     bb5: {                              
[00:54:29]         ReadForMatch(_4);
[00:54:29]         falseEdges -> [real: bb2, imaginary: bb6];
[00:54:29]     }
[00:54:29]     bb6: {                              
[00:54:29]     }
[00:54:29]     }
[00:54:29]     bb7: {                              
[00:54:29]     }
[00:54:29]     }
[00:54:29]     bb8: {                              
[00:54:29]         StorageLive(_7);
[00:54:29]         _7 = &(((promoted[0]: std::option::Option<i32>) as Some).0: i32);
[00:54:29]         StorageLive(_10);
[00:54:29]         _10 = const guard() -> [return: bb9, unwind: bb1];
[00:54:29]     }
[00:54:29]     bb9: {                              
[00:54:29]         switchInt(move _10) -> [false: bb10, otherwise: bb11];
[00:54:29]     }
[00:54:29]     bb10: {                             
[00:54:29]         falseEdges -> [real: bb4, imaginary: bb4];
[00:54:29]     }
[00:54:29]     bb11: {                             
[00:54:29]         StorageLive(_5);
[00:54:29]         _5 = ((_2 as Some).0: i32);
[00:54:29]         StorageLive(_11);
[00:54:29]         _11 = _5;
[00:54:29]         _1 = (const 1i32, move _11);
[00:54:29]         StorageDead(_11);
[00:54:29]         goto -> bb13;
[00:54:29]     }
[00:54:29]     bb12: {                             
[00:54:29]         StorageLive(_8);
[00:54:29]         _8 = ((_2 as Some).0: i32);
[00:54:29]         StorageLive(_12);
[00:54:29]         _12 = _8;
[00:54:29]         _1 = (const 2i32, move _12);
[00:54:29]         StorageDead(_12);
[00:54:29]         goto -> bb13;
[00:54:29]     }
[00:54:29]     bb13: {                             
[00:54:29]         StorageDead(_8);
[00:54:29]         StorageDead(_5);
[00:54:29]         StorageDead(_10);
[00:54:29]         StorageDead(_7);
[00:54:29]         StorageDead(_1);
[00:54:29]         StorageDead(_2);
[00:54:29]         _0 = ();
[00:54:29]         return;
[00:54:29] }', tools/compiletest/src/runtest.rs:2813:13
[00:54:29] 
[00:54:29] 
[00:54:29] failures:
[00:54:29] failures:
[00:54:29]     [mir-opt] mir-opt/end_region_destruction_extents_1.rs
[00:54:29]     [mir-opt] mir-opt/match_false_edges.rs
[00:54:29] 
[00:54:29] test result: FAILED. 44 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[00:54:29] 
[00:54:29] 
[00:54:29] 
[00:54:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:54:29] 
[00:54:29] 
[00:54:29] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:54:29] Build completed unsuccessfully in 0:09:44
[00:54:29] Build completed unsuccessfully in 0:09:44
[00:54:29] make: *** [check] Error 1
[00:54:29] Makefile:58: recipe for target 'check' failed
121696 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
113392 ./obj/build/x86_64-unknown-linux-gnu/test/mir-opt
111276 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
107600 ./src/llvm/test/CodeGen
