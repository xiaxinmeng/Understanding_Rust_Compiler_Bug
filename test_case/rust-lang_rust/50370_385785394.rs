plain
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:59:50] 
[00:59:50] running 50 tests
[01:00:11] ..................................FFF.............
[01:00:11] 
[01:00:11] ---- [mir-opt] mir-opt/nll/region-liveness-basic.rs stdout ----
[01:00:11] ---- [mir-opt] mir-opt/nll/region-liveness-basic.rs stdout ----
[01:00:11]  thread '[mir-opt] mir-opt/nll/region-liveness-basic.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:00:11] Expected Line: "| \'_#2r    | {bb2[0..=1], bb3[0..=1]}"
[01:00:11] Expected:
[01:00:11] ... (elided)
[01:00:11] | '_#2r    | {bb2[0..=1], bb3[0..=1]}
[01:00:11] | '_#3r    | {bb2[1], bb3[0..=1]}
[01:00:11] ... (elided)
[01:00:11]             let _2: &'_#3r usize;
[01:00:11] Actual:
[01:00:11] | Free Region Mapping
[01:00:11] | '_#0r    | Global   | ['_#0r, '_#1r]
[01:00:11] | '_#1r    | Local    | ['_#1r]
[01:00:11] | Inferred Region Values
[01:00:11] | Inferred Region Values
[01:00:11] | '_#0r    | {'_#0r, bb0[0..=7], bb1[0], bb2[0..=1], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=3]}
[01:00:11] | '_#1r    | {'_#1r, bb0[0..=7], bb1[0], bb2[0..=1], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=3]}
[01:00:11] | '_#2r    | {bb2[1], bb3[0..=1]}
[01:0               | Live variables on entry to bb3[1]: [_2]
[01:00:11]         _7 = (*_2);
[01:00:11]                                          | Live variables on entry to bb3[2]: [_7]
[01:00:11]         _6 = const use_x(move _7) -> [return: bb5, unwind: bb1];
[01:00:11]     }
[01:00:11]     | Live variables on entry to bb4: []
[01:00:11]     bb4: {                              
[01:00:11]                                          | Live variables on entry to bb4[0]: []
[01:00:11]         _8 = const use_x(const 22usize) -> [return: bb6, unwind: bb1];
[01:00:11]     }
[01:00:11]     | Live variables on entry to bb5: []
[01:00:11]     bb5: {                              
[01:00:11]                                          | Live variables on entry to bb5[0]: []
[01:00:11]         StorageDead(_7);
[01:00:11]                                          | Live variables on entry to bb5[1]: []
[01:00:11]         _0 = ();
[01:00:11]                                          | Live variables on entry to bb5[2]: []
[01:00:11]         goto -> bb7;
[01:00:11]     }
[01:00:11]     | Live variables on entry to bb6: []
[01:00:11]     bb6: {                              
[01:00:11]                                          | Live variables on entry to bb6[0]: []
[01:00:11]         _0 = ();
[01:00:11]                                          | Live variables on entry to bb6[1]: []
[01:00:11]         goto -> bb7;
[01:00:11]     }
[01:00:11]     | Live variables on entry to bb7: []
[01:00:11]     bb7: {                              
[01:00:11]                                          | Live variables on entry to bb7[0]: []
[01:00:11]         nop;
[01:00:11]                                          | Live variables on entry to bb7[1]: []
[01:00:11]         StorageDead(_2);
[01:00:11]                                          | Live variables on entry to bb7[2]: []
[01:00:11]         StorageDead(_1);
[01:00:11]                                          | Live variables on entry to bb7[3]: []
[01:00:11]         return;
[01:00:11]     }
[01:00:11] }', tools/compiletest/src/runtest.rs:2735:13
[01:00:11] 
[01:00:11] ---- [mir-opt] mir-opt/nll/region-liveness-two-disjoint-uses.rs stdout ----
[01:00:11] ---- [mir-opt] mir-opt/nll/region-liveness-two-disjoint-uses.rs stdout ----
[01:00:11]  thread '[mir-opt] mir-opt/nll/region-liveness-two-disjoint-uses.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:00:11] Expected Line: "| \'_#2r    | {bb2[0..=1], bb3[0..=1]}"
[01:00:11] Expected:
[01:00:11] ... (elided)
[01:00:11] | '_#2r    | {bb2[0..=1], bb3[0..=1]}
[01:00:11] ... (elided)
[01:00:11] | '_#4r    | {bb8[1..=4]}
[01:00:11] | '_#5r    | {bb2[1], bb3[0..=1], bb8[2..=4]}
[01:00:11] ... (elided)
[01:00:11] let mut _2: &'_#5r usize;
[01:00:11] ... (elided)
[01:00:11] _2 = &'_#2r _1[_3];
[01:00:11] ... (elided)
[01:00:11] _2 = &'_#4r (*_10);
[01:00:11] Actual:
[01:00:11] | Free Region Mapping
[01:00:11] | '_#0r    | Global   | ['_#0r, '_#1r]
[01:00:11] | '_#1r    | Local    | ['_#1r]
[01:00:11] | Inferred Region Values
[01:00:11] | Inferred Region Values
[01:00:11] | '_#0r    | {'_#0r, bb0[0..=7], bb1[0], bb2[0..=1], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5], bb8[0..=5], bb9[0..=5]}
[01:00:11] | '_#1r    | {'_#1r, bb0[0..=7], bb1[0], bb2[0..=1], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5], bb8[0..=5], bb9[0..=5]}
[01:00:11] | '_#2r    | {bb2[1], bb3[0..=1]}
[01:00:11] | '_#3r    | {bb8[1..=4]}
[01:00:11] | '_#4r    | {bb8[2..=4]}
[01:00:11] | '_#5r    | {bb2[1], bb3[0..=1], bb8[2..=4]}
[01:00:11] | '_#6r    | {bb8[1..=4]}
[01:00:11] | Inference Constraints
[01:00:11] | Inference Constraints
[01:00:11] | '_#0r live at {'_#0r, bb0[0..=7], bb1[0], bb2[0..=1], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5], bb8[0..=5], bb9[0..=5]}
[01:00:11] | '_#1r live at {'_#1r, bb0[0..=7], bb1[0], bb2[0..=1], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5], bb8[0..=5], bb9[0..=5]}
[01:00:11] | '_#5r live at {bb2[1], bb3[0..=1], bb8[2..=4]}
[01:00:11] | '_#6r live at {bb8[1]}
[01:00:11] | '_#2r: '_#5r @ bb2[1] due to /checkout/src/test/mir-opt/nll/region-liveness-two-disjoint-uses.rs:26:17: 26:22
[01:00:11] | '_#3r: '_#6r @ bb8[1] due to /checkout/src/test/mir-opt/nll/region-liveness-two-disjoint-uses.rs:33:9: 33:14
[01:00:11] | '_#4r: '_#5r @ bb8[2] due to /checkout/src/test/mir-opt/nll/region-liveness-two-disjoint-uses.rs:33:5: 33:14
[01:00:11] | '_#6r: '_#4r @ bb8[2] due to /checkout/src/test/mir-opt/nll/region-liveness-two-disjoint-uses.rs:33:5: 33:14
[01:00:11] fn main() -> (){
[01:00:11]     let mut _0: ();
[01:00:11]     scope 1 {
[01:00:11]         let mut _1: [usize; 3];
[01:00:11]         scope 3 {
[01:00:11]             let mut _2: &'_#5r usize;
[01:00:11]         scope 4 {
[01:00:11]         }
[01:00:11]     }
[01:00:11]     scope 2 {
[01:00:11]     scope 2 {
[01:00:11]     }
[01:00:11]     let mut _3: usize;
[01:0the len is move _4 but the index is _3") -> [success: bb2, unwind: bb1];
[01:00:11]     }
[01:00:11]     | Live variables on entry to bb1: []
[01:00:11]     bb1: {
[01:00:11]                                          | Live variables on entry to bb1[0]: []
[01:00:11]         resume;
[01:00:11]     }
[01:00:11]     | Live variables on entry to bb2: [_1, _3]
[01:00:11]     bb2: {                              
[01:00:11]                                          | Live variables on entry to bb2[0]: [_1, _3]
[01:00:11]         _2 = &'_#2r _1[_3];
[01:00:11]                                          | Live variables on entry to bb2[1]: [_1, _2]
[01:00:11]         switchInt(const true) -> [false: bb4, otherwise: bb3];
[01:00:11]     }
[01:00:11]     | Live variables on entry to bb3: [_1, _2]
[01:00:11]     bb3: {                              
[01:00:11]                                          | Live variables on entry to bb3[0]: [_1, _2]
[01:00:11]         StorageLive(_8);
[01:00:11]                                          | Live variables on entry to bb3[1]: [_1, _2]
[01:00:11]         _8 = (*_2);
[01:00:11]                                          | Live variables on entry to bb3[2]: [_1, _8]
[01:00:11]         _7 = const use_x(move _8) -> [return: bb5, unwind: bb1];
[01:00:11]     }
[01:00:11]     | Live variables on entry to bb4: [_1]
[01:00:11]     bb4: {                              
[01:00:11]                                          | Live variables on entry to bb4[0]: [_1]
[01:00:11]         _9 = const use_x(const 22usize) -> [return: bb6, unwind: bb1];
[01:00:11]     }
[01:00:11]     | Live variables on entry to bb5: [_1]
[01:00:11]     bb5: {                              
[01:00:11]                                          | Live variables on entry to bb5[0]: [_1]
[01:00:11]         StorageDead(_8);
[01:00:11]                                          | Live variables on entry to bb5[1]: [_1]
[01:00:11]         _6 = ();
[01:00:11]                                          | Live variables on entry to bb5[2]: [_1]
[01:00:11]         goto -> bb7;
[01:00:11]     }
[01:00:11]     | Live variables on entry to bb6: [_1]
[01:00:11]     bb6: {                              
[01:00:11]                                          | Live variables on entry to bb6[0]: [_1]
[01:00:11]         _6 = ();
[01:00:11]                                          | Live variables on entry to bb6[1]: [_1]
[01:00:11]         goto -> bb7;
[01:00:11]     }
[01:00:11]     | Live variables on entry to bb7: [_1]
[01:00:11]     bb7: {                              
[01:00:11]                                          | Live variables on entry to bb7[0]: [_1]
[01:00:11]         StorageLive(_10);
[01:00:11]                                          | Live variables on entry to bb7[1]: [_1]
[01:00:11]         StorageLive(_11);
[01:00:11]                                          | Live variables on entry to bb7[2]: [_1]
[01:00:11]         _11 = const 1usize;
[01:00:11]                                          | Live variables on entry to bb7[3]: [_1, _11]
[01:00:11]         _12 = Len(_1);
[01:00:11]                                          | Live variables on entry to bb7[4]: [_1, _11, _12]
[01:00:11]         _13 = Lt(_11, _12);
[01:00:11]                                          | Live variables on entry to bb7[5]: [_1, _11, _12, _13]
[01:00:11]         assert(move _13, "index out of bounds: the len is move _12 but the index is _11") -> [success: bb8, unwind: bb1];
[01:00:11]     }
[01:00:11]     | Live variables on entry to bb8: [_1, _11]
[01:00:11]     bb8: {                              
[01:00:11]                                          | Live variables on entry to bb8[0]: [_1, _11]
[01:00:11]         _10 = &'_#3r _1[_11];
[01:00:11]                                          | Live variables on entry to bb8[1]: [_10]
[01:00:11]         _2 = &'_#4r (*_10);
[01:00:11]                                          | Live variables on entry to bb8[2]: [_2]
[01:00:11]         StorageDead(_10);
[01:00:11]                                          | Live variables on entry to bb8[3]: [_2]
[01:00:11]         StorageLive(_15);
[01:00:11]                                          | Live variables on entry to bb8[4]: [_2]
[01:00:11]         _15 = (*_2);
[01:00:11]                                          | Live variables on entry to bb8[5]: [_15]
[01:00:11]         _14 = const use_x(move _15) -> [return: bb9, unwind: bb1];
[01:00:11]     }
[01:00:11]     | Live variables on entry to bb9: []
[01:00:11]     bb9: {                              
[01:00:11]                                          | Live variables on entry to bb9[0]: []
[01:00:11]         StorageDead(_15);
[01:00:11]                                          | Live variables on entry to bb9[1]: []
[01:00:11]         _0 = ();
[01:00:11]                                          | Live variables on entry to bb9[2]: []
[01:00:11]         nop;
[01:00:11]                                          | Live variables on entry to bb9[3]: []
[01:00:11]         StorageDead(_2);
[01:00:11]                                          | Live variables on entry to bb9[4]: []
[01:00:11]         StorageDead(_1);
[01:00:11]                                          | Live variables on entry to bb9[5]: []
[01:00:11]         return;
[01:00:11]     }
[01:00:11] }', tools/compiletest/src/runtest.rs:2735:13
[01:00:11] ---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
[01:00:11] ---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
[01:00:11]  thread '[mir-opt] mir-opt/nll/region-subtyping-basic.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:00:11] Expected Line: "| \'_#2r    | {bb2[0..=6], bb3[0..=1]}"
[01:00:11] Expected:
[01:00:11] ... (elided)
[01:00:11] | '_#2r    | {bb2[0..=6], bb3[0..=1]}
[01:00:11] | '_#3r    | {bb2[1..=6], bb3[0..=1]}
[01:00:11] | '_#4r    | {bb2[5..=6], bb3[0..=1]}
[01:00:11] Actual:
[01:00:11] | Free Region Mapping
[01:00:11] | '_#0r    | Global   | ['_#0r, '_#1r]
[01:00:11] | '_#1r    | Local    | ['_#1r]
[01:00:11] | Inferred Region Values
[01:00:11] | Inferred Region Values
[01:00:11] | '_#0r    | {'_#0r, bb0[0..=7], bb1[0], bb2[0..=6], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5]}
[01:00:11] | '_#1r    | {'_#1r, bb0[0..=7], bb1[0], bb2[0..=6], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5]}
[01:00:11] | '_#2r    | {bb2[1..=6], bb3[0..=1]}
[01:00:11] | '_#3r    | {bb2[1..=6], bb3[0..=1]}
[01:00:11] | '_#4r    | {bb2[5..=6], bb3[0..=1]}
[01:00:11] | '_#5r    | {bb2[4..=6], bb3[0..=1]}
[01:00:11] | '_#6r    | {bb2[3..=6], bb3[0..=1]}
[01:00:11] | Inference Constraints
[01:00:11] | Inference Constraints
[01:00:11] | '_#0r live at {'_#0r, bb0[0..=7], bb1[0], bb2[0..=6], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5]}
[01:00:11] | '_#1r live at {'_#1r, bb0[0..=7], bb1[0], bb2[0..=6], bb3[0..=2], bb4[0], bb5[0..=2], bb6[0..=1], bb7[0..=5]}
[01:00:11] | '_#3r live at {bb2[1..=3]}
[01:00:11] | '_#4r live at {bb2[5..=6], bb3[0..=1]}
[01:00:11] | '_#5r live at {bb2[4]}
[01:00:11] | '_#2r: '_#3r @ bb2[1] due to /checkout/src/test/mir-opt/nll/region-subtyping-basic.rs:24:13: 24:18
[01:00:11] | '_#3r: '_#5r @ bb2[4] due to /checkout/src/test/mir-opt/nll/region-subtyping-basic.rs:25:13: 25:14
[01:00:11] | '_#3r: '_#6r @ bb2[3] due to /checkout/src/test/mir-opt/nll/region-subtyping-basic.rs:25:13: 25:14
[01:00:11] | '_#5r: '_#4r @ bb2[5] due to /checkout/src/test/mir-opt/nll/region-subtyping-basic.rs:25:13: 25:14
[01:00:11] | '_#6r: '_#3r @ bb2[3] due to /checkout/src/test/mir-opt/nll/region-subtyping-basic.rs:25:13: 25:14
[01:00:11] fn main() -> (){
[01:00:11]     let mut _0: ();
[01:00:11]     scope 1 {
[01:00:11]         let mut _1: [usize; 3];
[01:00:11]         scope 3 {
[01:00:11]             let _2: &'_#3r usize;
[01:00:11]             scope 5 {
[01:00:11]                 let _6: &'_#4r usize;
[01:00:11]             scope 6 {
[01:00:11]             }
[01:00:11]         }
[01:00:11]         scope 4 {
[01:00:11]         scope 4 {
[01:00:11]         }
[01:00:11]     }
[01:00:11]     scope 2 {
[01:00:11]     }
[01:00:11]     let mut _3: usize;
[01:00:11]     let mut _4: usize;
[01:00:11try to bb7[2]: []
[01:00:11]         nop;
[01:00:11]                                          | Live variables on entry to bb7[3]: []
[01:00:11]         StorageDead(_2);
[01:00:11]                                          | Live variables on entry to bb7[4]: []
[01:00:11]         StorageDead(_1);
[01:00:11]                                          | Live variables on entry to bb7[5]: []
[01:00:11]         return;
[01:00:11]     }
[01:00:11] }', tools/compiletest/src/runtest.rs:2735:13
[01:00:11] 
[01:00:11] failures:
[01:00:11]     [mir-opt] mir-opt/nll/region-liveness-basic.rs
[01:00:11]     [mir-opt] mir-opt/nll/region-liveness-two-disjoint-uses.rs
[01:00:11]     [mir-opt] mir-opt/nll/region-liveness-two-disjoint-uses.rs
[01:00:11]     [mir-opt] mir-opt/nll/region-subtyping-basic.rs
[01:00:11] 
[01:00:11] test result: FAILED. 47 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
[01:00:11] 
[01:00:11] 
[01:00:11] 
[01:00:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:00:11] 
[01:00:11] 
[01:00:11] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[01:00:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:00:11] Build completed unsuccessfully in 0:17:46
[01:00:11] make: *** [check] Error 1
[01:00:11] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0224b284
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
