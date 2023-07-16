plain
Suite("src/test/mir-opt") not skipped for "bootstrap::test::MirOpt" -- not in ["src/tools/tidy"]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 159 tests
.........F.........F......F.......i....FF.......F...............................i...........F....... 100/159
.....F.........FF.........i........F.......................
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] mir-opt/const-promotion-extern-static.rs stdout ----
---- [mir-opt] mir-opt/const-promotion-extern-static.rs stdout ----
22 -                                          // + ty: &i32
23 -                                          // + val: Value(Scalar(alloc0))
24 +                                          // + ty: &[&i32; 1]
- +                                          // + val: Unevaluated(WithOptConstParam { did: DefId(0:6 ~ const_promotion_extern_static[317d]::BAR), const_param_did: None }, [], Some(promoted[0]))
+ +                                          // + val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:6 ~ const_promotion_extern_static[317d]::BAR), const_param_did: None }, substs: [], promoted: Some(promoted[0]) })
26                                            // mir::Constant
27 -                                          // + span: $DIR/const-promotion-extern-static.rs:9:33: 9:34
28 -                                          // + literal: Const { ty: &i32, val: Value(Scalar(alloc0)) }

30 -         _3 = [move _4];                  // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
31 -         _2 = &_3;                        // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
32 +                                          // + span: $DIR/const-promotion-extern-static.rs:9:31: 9:35
- +                                          // + literal: Const { ty: &[&i32; 1], val: Unevaluated(WithOptConstParam { did: DefId(0:6 ~ const_promotion_extern_static[317d]::BAR), const_param_did: None }, [], Some(promoted[0])) }
+ +                                          // + literal: Const { ty: &[&i32; 1], val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:6 ~ const_promotion_extern_static[317d]::BAR), const_param_did: None }, substs: [], promoted: Some(promoted[0]) }) }
34 +         _2 = &(*_6);                     // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
35           _1 = move _2 as &[&i32] (Pointer(Unsize)); // scope 0 at $DIR/const-promotion-extern-static.rs:9:31: 9:35
36 -         StorageDead(_4);                 // scope 0 at $DIR/const-promotion-extern-static.rs:9:34: 9:35

thread '[mir-opt] mir-opt/const-promotion-extern-static.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_promotion_extern_static.BAR.PromoteTemps.diff', src/tools/compiletest/src/runtest.rs:3516:25

---- [mir-opt] mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs stdout ----
---- [mir-opt] mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs stdout ----
28           _9 = const main::promoted[0];    // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35
29                                            // ty::Const
30                                            // + ty: &[i32; 3]
-                                            // + val: Unevaluated(WithOptConstParam { did: DefId(0:3 ~ bad_op_unsafe_oob_for_slices[317d]::main), const_param_did: None }, [], Some(promoted[0]))
+                                            // + val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ bad_op_unsafe_oob_for_slices[317d]::main), const_param_did: None }, substs: [], promoted: Some(promoted[0]) })
32                                            // mir::Constant
33                                            // + span: $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35
-                                            // + literal: Const { ty: &[i32; 3], val: Unevaluated(WithOptConstParam { did: DefId(0:3 ~ bad_op_unsafe_oob_for_slices[317d]::main), const_param_did: None }, [], Some(promoted[0])) }
+                                            // + literal: Const { ty: &[i32; 3], val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ bad_op_unsafe_oob_for_slices[317d]::main), const_param_did: None }, substs: [], promoted: Some(promoted[0]) }) }
35           _3 = _9;                         // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35
36           _2 = &raw const (*_3);           // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35
37           _1 = move _2 as *const [i32] (Pointer(Unsize)); // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35

thread '[mir-opt] mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.main.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3516:25
---- [mir-opt] mir-opt/const_prop/const_prop_fails_gracefully.rs stdout ----
---- [mir-opt] mir-opt/const_prop/const_prop_fails_gracefully.rs stdout ----
19           _3 = const FOO;                  // scope 0 at $DIR/const_prop_fails_gracefully.rs:7:13: 7:16
20                                            // ty::Const
21                                            // + ty: &i32
-                                            // + val: Unevaluated(WithOptConstParam { did: DefId(0:5 ~ const_prop_fails_gracefully[317d]::main::FOO), const_param_did: None }, [], None)
+                                            // + val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:5 ~ const_prop_fails_gracefully[317d]::main::FOO), const_param_did: None }, substs: [], promoted: None })
23                                            // mir::Constant
24                                            // + span: $DIR/const_prop_fails_gracefully.rs:7:13: 7:16
-                                            // + literal: Const { ty: &i32, val: Unevaluated(WithOptConstParam { did: DefId(0:5 ~ const_prop_fails_gracefully[317d]::main::FOO), const_param_did: None }, [], None) }
+                                            // + literal: Const { ty: &i32, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:5 ~ const_prop_fails_gracefully[317d]::main::FOO), const_param_did: None }, substs: [], promoted: None }) }
26           _2 = &raw const (*_3);           // scope 0 at $DIR/const_prop_fails_gracefully.rs:7:13: 7:16
27           _1 = move _2 as usize (Misc);    // scope 0 at $DIR/const_prop_fails_gracefully.rs:7:13: 7:39
28           StorageDead(_2);                 // scope 0 at $DIR/const_prop_fails_gracefully.rs:7:38: 7:39

thread '[mir-opt] mir-opt/const_prop/const_prop_fails_gracefully.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/const_prop_fails_gracefully.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3516:25
---- [mir-opt] mir-opt/const_prop/ref_deref_project.rs stdout ----
---- [mir-opt] mir-opt/const_prop/ref_deref_project.rs stdout ----
17 +         _4 = const main::promoted[0];    // scope 0 at $DIR/ref_deref_project.rs:5:6: 5:17
18 +                                          // ty::Const
19 +                                          // + ty: &(i32, i32)
- +                                          // + val: Unevaluated(WithOptConstParam { did: DefId(0:3 ~ ref_deref_project[317d]::main), const_param_did: None }, [], Some(promoted[0]))
+ +                                          // + val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ ref_deref_project[317d]::main), const_param_did: None }, substs: [], promoted: Some(promoted[0]) })
21 +                                          // mir::Constant
22 +                                          // + span: $DIR/ref_deref_project.rs:5:6: 5:17
- +                                          // + literal: Const { ty: &(i32, i32), val: Unevaluated(WithOptConstParam { did: DefId(0:3 ~ ref_deref_project[317d]::main), const_param_did: None }, [], Some(promoted[0])) }
+ +                                          // + literal: Const { ty: &(i32, i32), val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ ref_deref_project[317d]::main), const_param_did: None }, substs: [], promoted: Some(promoted[0]) }) }
24 +         _2 = &((*_4).1: i32);            // scope 0 at $DIR/ref_deref_project.rs:5:6: 5:17
25           _1 = (*_2);                      // scope 0 at $DIR/ref_deref_project.rs:5:5: 5:17
26 -         StorageDead(_3);                 // scope 0 at $DIR/ref_deref_project.rs:5:17: 5:18

thread '[mir-opt] mir-opt/const_prop/ref_deref_project.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/ref_deref_project.main.PromoteTemps.diff', src/tools/compiletest/src/runtest.rs:3516:25
---- [mir-opt] mir-opt/const_prop/ref_deref.rs stdout ----
---- [mir-opt] mir-opt/const_prop/ref_deref.rs stdout ----
17 +         _4 = const main::promoted[0];    // scope 0 at $DIR/ref_deref.rs:5:6: 5:10
18 +                                          // ty::Const
19 +                                          // + ty: &i32
- +                                          // + val: Unevaluated(WithOptConstParam { did: DefId(0:3 ~ ref_deref[317d]::main), const_param_did: None }, [], Some(promoted[0]))
+ +                                          // + val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ ref_deref[317d]::main), const_param_did: None }, substs: [], promoted: Some(promoted[0]) })
21 +                                          // mir::Constant
22 +                                          // + span: $DIR/ref_deref.rs:5:6: 5:10
- +                                          // + literal: Const { ty: &i32, val: Unevaluated(WithOptConstParam { did: DefId(0:3 ~ ref_deref[317d]::main), const_param_did: None }, [], Some(promoted[0])) }
+ +                                          // + literal: Const { ty: &i32, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ ref_deref[317d]::main), const_param_did: None }, substs: [], promoted: Some(promoted[0]) }) }
24 +         _2 = &(*_4);                     // scope 0 at $DIR/ref_deref.rs:5:6: 5:10
25           _1 = (*_2);                      // scope 0 at $DIR/ref_deref.rs:5:5: 5:10
26 -         StorageDead(_3);                 // scope 0 at $DIR/ref_deref.rs:5:10: 5:11

thread '[mir-opt] mir-opt/const_prop/ref_deref.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/ref_deref.main.PromoteTemps.diff', src/tools/compiletest/src/runtest.rs:3516:25
---- [mir-opt] mir-opt/const_prop/slice_len.rs stdout ----
---- [mir-opt] mir-opt/const_prop/slice_len.rs stdout ----
21           _9 = const main::promoted[0];    // scope 0 at $DIR/slice_len.rs:5:6: 5:19
22                                            // ty::Const
23                                            // + ty: &[u32; 3]
-                                            // + val: Unevaluated(WithOptConstParam { did: DefId(0:3 ~ slice_len[317d]::main), const_param_did: None }, [], Some(promoted[0]))
+                                            // + val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ slice_len[317d]::main), const_param_did: None }, substs: [], promoted: Some(promoted[0]) })
25                                            // mir::Constant
26                                            // + span: $DIR/slice_len.rs:5:6: 5:19
-                                            // + literal: Const { ty: &[u32; 3], val: Unevaluated(WithOptConstParam { did: DefId(0:3 ~ slice_len[317d]::main), const_param_did: None }, [], Some(promoted[0])) }
+                                            // + literal: Const { ty: &[u32; 3], val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ slice_len[317d]::main), const_param_did: None }, substs: [], promoted: Some(promoted[0]) }) }
28           _4 = _9;                         // scope 0 at $DIR/slice_len.rs:5:6: 5:19
29           _3 = _4;                         // scope 0 at $DIR/slice_len.rs:5:6: 5:19
30           _2 = move _3 as &[u32] (Pointer(Unsize)); // scope 0 at $DIR/slice_len.rs:5:6: 5:19

thread '[mir-opt] mir-opt/const_prop/slice_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/slice_len.main.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3516:25
---- [mir-opt] mir-opt/inline/inline-retag.rs stdout ----
---- [mir-opt] mir-opt/inline/inline-retag.rs stdout ----
35         _10 = const bar::promoted[1];    // scope 1 at $DIR/inline-retag.rs:12:7: 12:9
36                                          // ty::Const
37                                          // + ty: &i32
-                                          // + val: Unevaluated(WithOptConstParam { did: DefId(0:4 ~ inline_retag[317d]::bar), const_param_did: None }, [], Some(promoted[1]))
+                                          // + val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:4 ~ inline_retag[317d]::bar), const_param_did: None }, substs: [], promoted: Some(promoted[1]) })
39                                          // mir::Constant
40                                          // + span: $DIR/inline-retag.rs:12:7: 12:9
-                                          // + literal: Const { ty: &i32, val: Unevaluated(WithOptConstParam { did: DefId(0:4 ~ inline_retag[317d]::bar), const_param_did: None }, [], Some(promoted[1])) }
+                                          // + literal: Const { ty: &i32, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:4 ~ inline_retag[317d]::bar), const_param_did: None }, substs: [], promoted: Some(promoted[1]) }) }
42         Retag(_10);                      // scope 1 at $DIR/inline-retag.rs:12:7: 12:9
43         _4 = &(*_10);                    // scope 1 at $DIR/inline-retag.rs:12:7: 12:9
44         Retag(_4);                       // scope 1 at $DIR/inline-retag.rs:12:7: 12:9

49         _9 = const bar::promoted[0];     // scope 1 at $DIR/inline-retag.rs:12:11: 12:14
50                                          // ty::Const
51                                          // + ty: &i32
-                                          // + val: Unevaluated(WithOptConstParam { did: DefId(0:4 ~ inline_retag[317d]::bar), const_param_did: None }, [], Some(promoted[0]))
+                                          // + val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:4 ~ inline_retag[317d]::bar), const_param_did: None }, substs: [], promoted: Some(promoted[0]) })
53                                          // mir::Constant
54                                          // + span: $DIR/inline-retag.rs:12:11: 12:14
-                                          // + literal: Const { ty: &i32, val: Unevaluated(WithOptConstParam { did: DefId(0:4 ~ inline_retag[317d]::bar), const_param_did: None }, [], Some(promoted[0])) }
+                                          // + literal: Const { ty: &i32, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:4 ~ inline_retag[317d]::bar), const_param_did: None }, substs: [], promoted: Some(promoted[0]) }) }
56         Retag(_9);                       // scope 1 at $DIR/inline-retag.rs:12:11: 12:14
57         _7 = &(*_9);                     // scope 1 at $DIR/inline-retag.rs:12:11: 12:14
58         Retag(_7);                       // scope 1 at $DIR/inline-retag.rs:12:11: 12:14

thread '[mir-opt] mir-opt/inline/inline-retag.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_retag.bar.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3516:25
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
84           _28 = const main::promoted[0];   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
85                                            // ty::Const
86                                            // + ty: &i32
-                                            // + val: Unevaluated(WithOptConstParam { did: DefId(0:3 ~ issue_73223[317d]::main), const_param_did: None }, [], Some(promoted[0]))
+                                            // + val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ issue_73223[317d]::main), const_param_did: None }, substs: [], promoted: Some(promoted[0]) })
88                                            // mir::Constant
89                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                                            // + literal: Const { ty: &i32, val: Unevaluated(WithOptConstParam { did: DefId(0:3 ~ issue_73223[317d]::main), const_param_did: None }, [], Some(promoted[0])) }
+                                            // + literal: Const { ty: &i32, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ issue_73223[317d]::main), const_param_did: None }, substs: [], promoted: Some(promoted[0]) }) }
91           _11 = _28;                       // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
92           (_9.0: &i32) = move _10;         // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
93           (_9.1: &i32) = move _11;         // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

thread '[mir-opt] mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.64bit.diff', src/tools/compiletest/src/runtest.rs:3516:25
---- [mir-opt] mir-opt/lower_intrinsics.rs stdout ----
---- [mir-opt] mir-opt/lower_intrinsics.rs stdout ----
47           _19 = const discriminant::<T>::promoted[2]; // scope 0 at $DIR/lower_intrinsics.rs:70:42: 70:44
48                                            // ty::Const
49                                            // + ty: &i32
-                                            // + val: Unevaluated(WithOptConstParam { did: DefId(0:27 ~ lower_intrinsics[8787]::discriminant), const_param_did: None }, [T], Some(promoted[2]))
+                                            // + val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:27 ~ lower_intrinsics[8787]::discriminant), const_param_did: None }, substs: [T], promoted: Some(promoted[2]) })
51                                            // mir::Constant
52                                            // + span: $DIR/lower_intrinsics.rs:70:42: 70:44
-                                            // + literal: Const { ty: &i32, val: Unevaluated(WithOptConstParam { did: DefId(0:27 ~ lower_intrinsics[8787]::discriminant), const_param_did: None }, [T], Some(promoted[2])) }
+                                            // + literal: Const { ty: &i32, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:27 ~ lower_intrinsics[8787]::discriminant), const_param_did: None }, substs: [T], promoted: Some(promoted[2]) }) }
54           _7 = &(*_19);                    // scope 0 at $DIR/lower_intrinsics.rs:70:42: 70:44
55           _6 = &(*_7);                     // scope 0 at $DIR/lower_intrinsics.rs:70:42: 70:44
56 -         _5 = discriminant_value::<i32>(move _6) -> bb2; // scope 0 at $DIR/lower_intrinsics.rs:70:5: 70:45

71           _18 = const discriminant::<T>::promoted[1]; // scope 0 at $DIR/lower_intrinsics.rs:71:42: 71:45
72                                            // ty::Const
73                                            // + ty: &()
-                                            // + val: Unevaluated(WithOptConstParam { did: DefId(0:27 ~ lower_intrinsics[8787]::discriminant), const_param_did: None }, [T], Some(promoted[1]))
+                                            // + val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:27 ~ lower_intrinsics[8787]::discriminant), const_param_did: None }, substs: [T], promoted: Some(promoted[1]) })
75                                            // mir::Constant
76                                            // + span: $DIR/lower_intrinsics.rs:71:42: 71:45
-                                            // + literal: Const { ty: &(), val: Unevaluated(WithOptConstParam { did: DefId(0:27 ~ lower_intrinsics[8787]::discriminant), const_param_did: None }, [T], Some(promoted[1])) }
+                                            // + literal: Const { ty: &(), val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:27 ~ lower_intrinsics[8787]::discriminant), const_param_did: None }, substs: [T], promoted: Some(promoted[1]) }) }
78           _11 = &(*_18);                   // scope 0 at $DIR/lower_intrinsics.rs:71:42: 71:45
79           _10 = &(*_11);                   // scope 0 at $DIR/lower_intrinsics.rs:71:42: 71:45
80 -         _9 = discriminant_value::<()>(move _10) -> bb3; // scope 0 at $DIR/lower_intrinsics.rs:71:5: 71:46

95           _17 = const discriminant::<T>::promoted[0]; // scope 0 at $DIR/lower_intrinsics.rs:72:42: 72:47
96                                            // ty::Const
97                                            // + ty: &E
-                                            // + val: Unevaluated(WithOptConstParam { did: DefId(0:27 ~ lower_intrinsics[8787]::discriminant), const_param_did: None }, [T], Some(promoted[0]))
+                                            // + val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:27 ~ lower_intrinsics[8787]::discriminant), const_param_did: None }, substs: [T], promoted: Some(promoted[0]) })
99                                            // mir::Constant
100                                            // + span: $DIR/lower_intrinsics.rs:72:42: 72:47
-                                            // + literal: Const { ty: &E, val: Unevaluated(WithOptConstParam { did: DefId(0:27 ~ lower_intrinsics[8787]::discriminant), const_param_did: None }, [T], Some(promoted[0])) }
+                                            // + literal: Const { ty: &E, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:27 ~ lower_intrinsics[8787]::discriminant), const_param_did: None }, substs: [T], promoted: Some(promoted[0]) }) }
102           _15 = &(*_17);                   // scope 0 at $DIR/lower_intrinsics.rs:72:42: 72:47
103           _14 = &(*_15);                   // scope 0 at $DIR/lower_intrinsics.rs:72:42: 72:47
104 -         _13 = discriminant_value::<E>(move _14) -> bb4; // scope 0 at $DIR/lower_intrinsics.rs:72:5: 72:48

thread '[mir-opt] mir-opt/lower_intrinsics.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/lower_intrinsics.discriminant.LowerIntrinsics.diff', src/tools/compiletest/src/runtest.rs:3516:25
---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
54         _11 = const full_tested_match::promoted[0]; // scope 0 at $DIR/match_false_edges.rs:16:14: 16:15
55                                          // ty::Const
56                                          // + ty: &std::option::Option<i32>
-                                          // + val: Unevaluated(WithOptConstParam { did: DefId(0:5 ~ match_false_edges[317d]::full_tested_match), const_param_did: None }, [], Some(promoted[0]))
+                                          // + val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:5 ~ match_false_edges[317d]::full_tested_match), const_param_did: None }, substs: [], promoted: Some(promoted[0]) })
58                                          // mir::Constant
59                                          // + span: $DIR/match_false_edges.rs:16:14: 16:15
-                                          // + literal: Const { ty: &std::option::Option<i32>, val: Unevaluated(WithOptConstParam { did: DefId(0:5 ~ match_false_edges[317d]::full_tested_match), const_param_did: None }, [], Some(promoted[0])) }
+                                          // + literal: Const { ty: &std::option::Option<i32>, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:5 ~ match_false_edges[317d]::full_tested_match), const_param_did: None }, substs: [], promoted: Some(promoted[0]) }) }
61         _6 = &(((*_11) as Some).0: i32); // scope 0 at $DIR/match_false_edges.rs:16:14: 16:15
62         _4 = &shallow _2;                // scope 0 at $DIR/match_false_edges.rs:15:19: 15:27
63         StorageLive(_7);                 // scope 0 at $DIR/match_false_edges.rs:16:20: 16:27

thread '[mir-opt] mir-opt/match_false_edges.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/match_false_edges.full_tested_match.PromoteTemps.after.mir', src/tools/compiletest/src/runtest.rs:3516:25
---- [mir-opt] mir-opt/retag.rs stdout ----
---- [mir-opt] mir-opt/retag.rs stdout ----
149         _27 = const main::promoted[0];   // scope 7 at $DIR/retag.rs:47:21: 47:23
150                                          // ty::Const
151                                          // + ty: &i32
-                                          // + val: Unevaluated(WithOptConstParam { did: DefId(0:13 ~ retag[317d]::main), const_param_did: None }, [], Some(promoted[0]))
+                                          // + val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:13 ~ retag[317d]::main), const_param_did: None }, substs: [], promoted: Some(promoted[0]) })
153                                          // mir::Constant
154                                          // + span: $DIR/retag.rs:47:21: 47:23
-                                          // + literal: Const { ty: &i32, val: Unevaluated(WithOptConstParam { did: DefId(0:13 ~ retag[317d]::main), const_param_did: None }, [], Some(promoted[0])) }
+                                          // + literal: Const { ty: &i32, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:13 ~ retag[317d]::main), const_param_did: None }, substs: [], promoted: Some(promoted[0]) }) }
156         Retag(_27);                      // scope 7 at $DIR/retag.rs:47:21: 47:23
157         _23 = &(*_27);                   // scope 7 at $DIR/retag.rs:47:21: 47:23
158         Retag(_23);                      // scope 7 at $DIR/retag.rs:47:21: 47:23

thread '[mir-opt] mir-opt/retag.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/retag.main.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3516:25

failures:
    [mir-opt] mir-opt/const-promotion-extern-static.rs
    [mir-opt] mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs
---
test result: FAILED. 145 passed; 11 failed; 3 ignored; 0 measured; 0 filtered out; finished in 2.60s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:53
