plain
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:03:31] 
[01:03:31] running 50 tests
[01:03:52] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[01:03:52] ...................................F..............
[01:03:52] 
[01:03:52] ---- [mir-opt] mir-opt/nll/region-liveness-two-disjoint-uses.rs stdout ----
[01:03:52] ---- [mir-opt] mir-opt/nll/region-liveness-two-disjoint-uses.rs stdout ----
[01:03:52]  thread '[mir-opt] mir-opt/nll/region-liveness-two-disjoint-uses.rs' panicked at 'Did not find expected line, error: ran out of mir dump to match against
[01:03:52] Expected Line: "| \'_#2r    | {bb2[0..=1], bb3[0..=1]}"
[01:03:52] Test Name: rustc.main.nll.0.mir
[01:03:52] Expected:
[01:03:52] ... (elided)
[01:03:52] | '_#2r    | {bb2[0..=1], bb3[0..=1]}
[01:03:52] ... (elided)
[01:03:52] | '_#4r    | {bb8[1..=4]}
[01:03:52] | '_#5r    | {bb2[1], bb3[0..=1], bb8[2..=4]}
[01:03:52] ... (elided)
[01:03:52] let mut _2: &'_#5r usize;
[01:03:52] ... (elided)
[01:03:52] _2 = &'_#2r _1[_3];
[01:03:52] ... (elided)
[01:03:52] _2 = &'_#4r (*_10);
[01:03:52] Actual:
[01:03:52] | Free Region Mapping
[01:03:52] | '_#0r    | Global   | ['_#0r, '_#1r]
[01:03:52] | '_#1r    | Local    | ['_#1r]
                                 | Live variables on entry to bb0[5]: [_1, _3]
[01:03:52]         _4 = Len(_1);
[01:03:52]                                          | Live variables on entry to bb0[6]: [_1, _3, _4]
[01:03:52]         _5 = Lt(_3, _4);
[01:03:52]                                          | Live variables on entry to bb0[7]: [_1, _3, _4, _5]
[01:03:52]         assert(move _5, "index out of bounds: the len is move _4 but the index is _3") -> [success: bb2, unwind: bb1];
[01:03:52]     }
[01:03:52]     | Live variables on entry to bb1: []
[01:03:52]     bb1: {
[01:03:52]                                          | Live variables on entry to bb1[0]: []
[01:03:52]         resume;
[01:03:52]     }
[01:03:52]     | Live variables on entry to bb2: [_1, _3]
[01:03:52]     bb2: {                              
[01:03:52]                                          | Live variables on entry to bb2[0]: [_1, _3]
[01:03:52]         _2 = &'_#2r _1[_3];
[01:03:52]                                          | Live variables on entry to bb2[1]: [_1, _2]
[01:03:52]         switchInt(const true) -> [false: bb4, otherwise: bb3];
[01:03:52]     }
[01:03:52]     | Live variables on entry to bb3: [_1, _2]
[01:03:52]     bb3: {                              
[01:03:52]                                          | Live variables on entry to bb3[0]: [_1, _2]
[01:03:52]         StorageLive(_8);
[01:03:52]                                          | Live variables on entry to bb3[1]: [_1, _2]
[01:03:52]         _8 = (*_2);
[01:03:52]                                          | Live variables on entry to bb3[2]: [_1, _8]
[01:03:52]         _7 = const use_x(move _8) -> [return: bb5, unwind: bb1];
[01:03:52]     }
[01:03:52]     | Live variables on entry to bb4: [_1]
[01:03:52]     bb4: {                              
[01:03:52]                                          | Live variables on entry to bb4[0]: [_1]
[01:03:52]         _9 = const use_x(const 22usize) -> [return: bb6, unwind: bb1];
[01:03:52]     }
[01:03:52]     | Live variables on entry to bb5: [_1]
[01:03:52]     bb5: {                              
[01:03:52]                                          | Live variables on entry to bb5[0]: [_1]
[01:03:52]         StorageDead(_8);
[01:03:52]                                          | Live variables on entry to bb5[1]: [_1]
[01:03:52]         _6 = ();
[01:03:52]                                          | Live variables on entry to bb5[2]: [_1]
[01:03:52]         goto -> bb7;
[01:03:52]     }
[01:03:52]     | Live variables on entry to bb6: [_1]
[01:03:52]     bb6: {                              
[01:03:52]                                          | Live variables on entry to bb6[0]: [_1]
[01:03:52]         _6 = ();
[01:03:52]                                          | Live variables on entry to bb6[1]: [_1]
[01:03:52]         goto -> bb7;
[01:03:52]     }
[01:03:52]     | Live variables on entry to bb7: [_1]
[01:03:52]     bb7: {                              
[01:03:52]                                          | Live variables on entry to bb7[0]: [_1]
[01:03:52]         StorageLive(_10);
[01:03:52]                                          | Live variables on entry to bb7[1]: [_1]
[01:03:52]         StorageLive(_11);
[01:03:52]                                          | Live variables on entry to bb7[2]: [_1]
[01:03:52]         _11 = const 1usize;
[01:03:52]                                          | Live variables on entry to bb7[3]: [_1, _11]
[01:03:52]         _12 = Len(_1);
[01:03:52]                                          | Live variables on entry to bb7[4]: [_1, _11, _12]
[01:03:52]         _13 = Lt(_11, _12);
[01:03:52]                                          | Live variables on entry to bb7[5]: [_1, _11, _12, _13]
[01:03:52]         assert(move _13, "index out of bounds: the len is move _12 but the index is _11") -> [success: bb8, unwind: bb1];
[01:03:52]     }
[01:03:52]     | Live variables on entry to bb8: [_1, _11]
[01:03:52]     bb8: {                              
[01:03:52]                                          | Live variables on entry to bb8[0]: [_1, _11]
[01:03:52]         _10 = &'_#3r _1[_11];
[01:03:52]                                          | Live variables on entry to bb8[1]: [_10]
[01:03:52]         _2 = &'_#4r (*_10);
[01:03:52]                                          | Live variables on entry to bb8[2]: [_2]
[01:03:52]         StorageDead(_10);
[01:03:52]                                          | Live variables on entry to bb8[3]: [_2]
[01:03:52]         StorageLive(_15);
[01:03:52]                                          | Live variables on entry to bb8[4]: [_2]
[01:03:52]         _15 = (*_2);
[01:03:52]                                          | Live variables on entry to bb8[5]: [_15]
[01:03:52]         _14 = const use_x(move _15) -> [return: bb9, unwind: bb1];
[01:03:52]     }
[01:03:52]     | Live variables on entry to bb9: []
[01:03:52]     bb9: {                              
[01:03:52]                                          | Live variables on entry to bb9[0]: []
[01:03:52]         StorageDead(_15);
[01:03:52]                                          | Live variables on entry to bb9[1]: []
[01:03:52]         _0 = ();
[01:03:52]                                          | Live variables on entry to bb9[2]: []
[01:03:52]         nop;
[01:03:52]                                          | Live variables on entry to bb9[3]: []
[01:03:52]         StorageDead(_2);
[01:03:52]                                          | Live variables on entry to bb9[4]: []
[01:03:52]         StorageDead(_1);
[01:03:52]                                          | Live variables on entry to bb9[5]: []
[01:03:52]         return;
[01:03:52] }', tools/compiletest/src/runtest.rs:2801:13
[01:03:52] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:03:52] 
[01:03:52] 
[01:03:52] 
[01:03:52] failures:
[01:03:52]     [mir-opt] mir-opt/nll/region-liveness-two-disjoint-uses.rs
[01:03:52] 
[01:03:52] test result: FAILED. 49 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:03:52] 
[01:03:52] 
[01:03:52] 
[01:03:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu
