plain
 finished in 0.664 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 179 tests
i.......F...............FF.............i....F.F..F............F......................... 88/179
.......i.......F....F..........F........F...F.............i....F........................ 176/179
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] src/test/mir-opt/const-promotion-extern-static.rs stdout ----
---- [mir-opt] src/test/mir-opt/const-promotion-extern-static.rs stdout ----
17 -         StorageLive(_4);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:32: 9:34
18 -         StorageLive(_5);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:33: 9:34
19 -         _5 = const {alloc1: &i32};       // scope 0 at $DIR/const-promotion-extern-static.rs:9:33: 9:34
- +         _6 = const BAR::promoted[0];     // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
+ +         _6 = const _;                    // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:44
21                                            // mir::Constant
22 -                                          // + span: $DIR/const-promotion-extern-static.rs:9:33: 9:34
23 -                                          // + literal: Const { ty: &i32, val: Value(Scalar(alloc1)) }

thread '[mir-opt] src/test/mir-opt/const-promotion-extern-static.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_promotion_extern_static.BAR.PromoteTemps.diff', src/tools/compiletest/src/runtest.rs:3466:25

---- [mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs stdout ----
25           StorageLive(_1);                 // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:9: 5:10
26           StorageLive(_2);                 // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35
27           StorageLive(_3);                 // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35
-           _9 = const main::promoted[0];    // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35
+           _9 = const _;                    // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35
29                                            // mir::Constant
30                                            // + span: $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35
31                                            // + literal: Const { ty: &[i32; 3], val: Unevaluated(main, [], Some(promoted[0])) }

thread '[mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.main.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/const_prop/const_prop_fails_gracefully.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/const_prop_fails_gracefully.rs stdout ----
19           _3 = const FOO;                  // scope 0 at $DIR/const_prop_fails_gracefully.rs:7:13: 7:16
20                                            // mir::Constant
21                                            // + span: $DIR/const_prop_fails_gracefully.rs:7:13: 7:16
-                                            // + literal: Const { ty: &i32, val: Unevaluated(FOO, [], None) }
+                                            // + literal: Const { ty: &i32, val: Unevaluated(FOO, [], ()) }
23           _2 = &raw const (*_3);           // scope 0 at $DIR/const_prop_fails_gracefully.rs:7:13: 7:16
24           _1 = move _2 as usize (PointerExposeAddress); // scope 0 at $DIR/const_prop_fails_gracefully.rs:7:13: 7:39
25           StorageDead(_2);                 // scope 0 at $DIR/const_prop_fails_gracefully.rs:7:38: 7:39

thread '[mir-opt] src/test/mir-opt/const_prop/const_prop_fails_gracefully.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/const_prop_fails_gracefully.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/const_prop/ref_deref_project.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/ref_deref_project.rs stdout ----
14 -         StorageLive(_3);                 // scope 0 at $DIR/ref_deref_project.rs:5:8: 5:14
15 -         _3 = (const 4_i32, const 5_i32); // scope 0 at $DIR/ref_deref_project.rs:5:8: 5:14
16 -         _2 = &(_3.1: i32);               // scope 0 at $DIR/ref_deref_project.rs:5:6: 5:17
- +         _4 = const main::promoted[0];    // scope 0 at $DIR/ref_deref_project.rs:5:6: 5:17
+ +         _4 = const _;                    // scope 0 at $DIR/ref_deref_project.rs:5:6: 5:17
18 +                                          // mir::Constant
19 +                                          // + span: $DIR/ref_deref_project.rs:5:6: 5:17
20 +                                          // + literal: Const { ty: &(i32, i32), val: Unevaluated(main, [], Some(promoted[0])) }

thread '[mir-opt] src/test/mir-opt/const_prop/ref_deref_project.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/ref_deref_project.main.PromoteTemps.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/const_prop/ref_deref.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/ref_deref.rs stdout ----
14 -         StorageLive(_3);                 // scope 0 at $DIR/ref_deref.rs:5:8: 5:9
15 -         _3 = const 4_i32;                // scope 0 at $DIR/ref_deref.rs:5:8: 5:9
16 -         _2 = &_3;                        // scope 0 at $DIR/ref_deref.rs:5:6: 5:10
- +         _4 = const main::promoted[0];    // scope 0 at $DIR/ref_deref.rs:5:6: 5:10
+ +         _4 = const _;                    // scope 0 at $DIR/ref_deref.rs:5:6: 5:10
18 +                                          // mir::Constant
19 +                                          // + span: $DIR/ref_deref.rs:5:6: 5:10
20 +                                          // + literal: Const { ty: &i32, val: Unevaluated(main, [], Some(promoted[0])) }

thread '[mir-opt] src/test/mir-opt/const_prop/ref_deref.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/ref_deref.main.PromoteTemps.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/const_prop/slice_len.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/slice_len.rs stdout ----
19           StorageLive(_2);                 // scope 0 at $DIR/slice_len.rs:5:5: 5:30
20           StorageLive(_3);                 // scope 0 at $DIR/slice_len.rs:5:6: 5:19
21           StorageLive(_4);                 // scope 0 at $DIR/slice_len.rs:5:6: 5:19
-           _9 = const main::promoted[0];    // scope 0 at $DIR/slice_len.rs:5:6: 5:19
+           _9 = const _;                    // scope 0 at $DIR/slice_len.rs:5:6: 5:19
23                                            // mir::Constant
24                                            // + span: $DIR/slice_len.rs:5:6: 5:19
25                                            // + literal: Const { ty: &[u32; 3], val: Unevaluated(main, [], Some(promoted[0])) }

thread '[mir-opt] src/test/mir-opt/const_prop/slice_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/slice_len.main.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/derefer_complex_case.rs stdout ----
28       bb0: {
28       bb0: {
29           StorageLive(_1);                 // scope 0 at $DIR/derefer_complex_case.rs:4:17: 4:26
30           StorageLive(_2);                 // scope 0 at $DIR/derefer_complex_case.rs:4:17: 4:26
-           _14 = const main::promoted[0];   // scope 0 at $DIR/derefer_complex_case.rs:4:17: 4:26
+           _14 = const _;                   // scope 0 at $DIR/derefer_complex_case.rs:4:17: 4:26
32                                            // mir::Constant
33                                            // + span: $DIR/derefer_complex_case.rs:4:17: 4:26
34                                            // + literal: Const { ty: &[i32; 2], val: Unevaluated(main, [], Some(promoted[0])) }

thread '[mir-opt] src/test/mir-opt/derefer_complex_case.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/derefer_complex_case.main.Derefer.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs stdout ----
50 -     bb2: {
50 -     bb2: {
51 +                                          // + span: $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
52 +                                          // + user_ty: UserType(0)
- +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Unevaluated(alloc::raw_vec::RawVec::<T>::NEW, [u32], None) }
+ +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Unevaluated(alloc::raw_vec::RawVec::<T>::NEW, [u32], ()) }
54 +         Deinit((*_9));                   // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
55 +         ((*_9).0: alloc::raw_vec::RawVec<u32>) = move _10; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
56 +         ((*_9).1: usize) = const 0_usize; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL

thread '[mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_into_box_place.main.Inline.64bit.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/inline/inline-retag.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/inline-retag.rs stdout ----
32         _2 = _1;                         // scope 1 at $DIR/inline-retag.rs:12:5: 12:6
33         StorageLive(_3);                 // scope 1 at $DIR/inline-retag.rs:12:7: 12:9
34         StorageLive(_4);                 // scope 1 at $DIR/inline-retag.rs:12:7: 12:9
-         _10 = const bar::promoted[1];    // scope 1 at $DIR/inline-retag.rs:12:7: 12:9
+         _10 = const _;                   // scope 1 at $DIR/inline-retag.rs:12:7: 12:9
36                                          // mir::Constant
37                                          // + span: $DIR/inline-retag.rs:12:7: 12:9
38                                          // + literal: Const { ty: &i32, val: Unevaluated(bar, [], Some(promoted[1])) }

43         Retag(_3);                       // scope 1 at $DIR/inline-retag.rs:12:7: 12:9
44         StorageLive(_6);                 // scope 1 at $DIR/inline-retag.rs:12:11: 12:14
45         StorageLive(_7);                 // scope 1 at $DIR/inline-retag.rs:12:11: 12:14
-         _9 = const bar::promoted[0];     // scope 1 at $DIR/inline-retag.rs:12:11: 12:14
+         _9 = const _;                    // scope 1 at $DIR/inline-retag.rs:12:11: 12:14
47                                          // mir::Constant
48                                          // + span: $DIR/inline-retag.rs:12:11: 12:14
49                                          // + literal: Const { ty: &i32, val: Unevaluated(bar, [], Some(promoted[0])) }

thread '[mir-opt] src/test/mir-opt/inline/inline-retag.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_retag.bar.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/issue-73223.rs stdout ----
---- [mir-opt] src/test/mir-opt/issue-73223.rs stdout ----
83           StorageLive(_10);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
84           _10 = &_1;                       // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
85           StorageLive(_11);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _28 = const main::promoted[0];   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _28 = const _;                   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
87                                            // mir::Constant
88                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
89                                            // + literal: Const { ty: &i32, val: Unevaluated(main, [], Some(promoted[0])) }

thread '[mir-opt] src/test/mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.64bit.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/lower_intrinsics.rs stdout ----
---- [mir-opt] src/test/mir-opt/lower_intrinsics.rs stdout ----
44           StorageLive(_5);                 // scope 0 at $DIR/lower_intrinsics.rs:75:5: 75:45
45           StorageLive(_6);                 // scope 0 at $DIR/lower_intrinsics.rs:75:42: 75:44
46           StorageLive(_7);                 // scope 0 at $DIR/lower_intrinsics.rs:75:42: 75:44
-           _19 = const discriminant::<T>::promoted[2]; // scope 0 at $DIR/lower_intrinsics.rs:75:42: 75:44
+           _19 = const _;                   // scope 0 at $DIR/lower_intrinsics.rs:75:42: 75:44
48                                            // mir::Constant
49                                            // + span: $DIR/lower_intrinsics.rs:75:42: 75:44
50                                            // + literal: Const { ty: &i32, val: Unevaluated(discriminant, [T], Some(promoted[2])) }

65           StorageLive(_9);                 // scope 0 at $DIR/lower_intrinsics.rs:76:5: 76:46
66           StorageLive(_10);                // scope 0 at $DIR/lower_intrinsics.rs:76:42: 76:45
67           StorageLive(_11);                // scope 0 at $DIR/lower_intrinsics.rs:76:42: 76:45
-           _18 = const discriminant::<T>::promoted[1]; // scope 0 at $DIR/lower_intrinsics.rs:76:42: 76:45
+           _18 = const _;                   // scope 0 at $DIR/lower_intrinsics.rs:76:42: 76:45
69                                            // mir::Constant
70                                            // + span: $DIR/lower_intrinsics.rs:76:42: 76:45
71                                            // + literal: Const { ty: &(), val: Unevaluated(discriminant, [T], Some(promoted[1])) }

86           StorageLive(_13);                // scope 0 at $DIR/lower_intrinsics.rs:77:5: 77:48
87           StorageLive(_14);                // scope 0 at $DIR/lower_intrinsics.rs:77:42: 77:47
88           StorageLive(_15);                // scope 0 at $DIR/lower_intrinsics.rs:77:42: 77:47
-           _17 = const discriminant::<T>::promoted[0]; // scope 0 at $DIR/lower_intrinsics.rs:77:42: 77:47
+           _17 = const _;                   // scope 0 at $DIR/lower_intrinsics.rs:77:42: 77:47
90                                            // mir::Constant
91                                            // + span: $DIR/lower_intrinsics.rs:77:42: 77:47
92                                            // + literal: Const { ty: &E, val: Unevaluated(discriminant, [T], Some(promoted[0])) }

thread '[mir-opt] src/test/mir-opt/lower_intrinsics.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/lower_intrinsics.discriminant.LowerIntrinsics.diff', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/match_false_edges.rs stdout ----
51 
52     bb5: {
52     bb5: {
53         StorageLive(_6);                 // scope 0 at $DIR/match_false_edges.rs:14:14: 14:15
-         _11 = const full_tested_match::promoted[0]; // scope 0 at $DIR/match_false_edges.rs:14:14: 14:15
+         _11 = const _;                   // scope 0 at $DIR/match_false_edges.rs:14:14: 14:15
55                                          // mir::Constant
56                                          // + span: $DIR/match_false_edges.rs:14:14: 14:15
57                                          // + literal: Const { ty: &Option<i32>, val: Unevaluated(full_tested_match, [], Some(promoted[0])) }

thread '[mir-opt] src/test/mir-opt/match_false_edges.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/match_false_edges.full_tested_match.PromoteTemps.after.mir', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/retag.rs stdout ----
---- [mir-opt] src/test/mir-opt/retag.rs stdout ----
147         Retag(_20);                      // scope 7 at $DIR/retag.rs:47:5: 47:24
148         StorageLive(_22);                // scope 7 at $DIR/retag.rs:47:21: 47:23
149         StorageLive(_23);                // scope 7 at $DIR/retag.rs:47:21: 47:23
-         _28 = const main::promoted[0];   // scope 7 at $DIR/retag.rs:47:21: 47:23
+         _28 = const _;                   // scope 7 at $DIR/retag.rs:47:21: 47:23
151                                          // mir::Constant
152                                          // + span: $DIR/retag.rs:47:21: 47:23
153                                          // + literal: Const { ty: &i32, val: Unevaluated(main, [], Some(promoted[0])) }

thread '[mir-opt] src/test/mir-opt/retag.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/retag.main.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3466:25

failures:
    [mir-opt] src/test/mir-opt/const-promotion-extern-static.rs
    [mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs
