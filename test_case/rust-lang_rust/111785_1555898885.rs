plain
 finished in 0.609 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 255 tests
.....F................F......F...F.....F....................................F...i.......  88/255
.................................F.........F.F.......F...i...........iii.......F.F...F.. 176/255
...F...F.F........iii.iiiii........Fii.....F..i.................F..............
failures:

---- [mir-opt] tests/mir-opt/bool_compare.rs stdout ----
---- [mir-opt] tests/mir-opt/bool_compare.rs stdout ----
thread '[mir-opt] tests/mir-opt/bool_compare.rs' panicked at 'the mir dump file for bool_compare.opt1.InstSimplify.before.mir does not exist (requested in /checkout/tests/mir-opt/bool_compare.rs)', src/tools/compiletest/src/runtest.rs:3652:17

---- [mir-opt] tests/mir-opt/casts.rs stdout ----
---- [mir-opt] tests/mir-opt/casts.rs stdout ----
thread '[mir-opt] tests/mir-opt/casts.rs' panicked at 'the mir dump file for casts.redundant.InstSimplify.before.mir does not exist (requested in /checkout/tests/mir-opt/casts.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/combine_array_len.rs stdout ----
---- [mir-opt] tests/mir-opt/combine_array_len.rs stdout ----
thread '[mir-opt] tests/mir-opt/combine_array_len.rs' panicked at 'the mir dump file for combine_array_len.norm2.InstSimplify.before.mir does not exist (requested in /checkout/tests/mir-opt/combine_array_len.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/combine_transmutes.rs stdout ----
---- [mir-opt] tests/mir-opt/combine_transmutes.rs stdout ----
thread '[mir-opt] tests/mir-opt/combine_transmutes.rs' panicked at 'the mir dump file for combine_transmutes.identity_transmutes.InstSimplify.before.mir does not exist (requested in /checkout/tests/mir-opt/combine_transmutes.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/combine_clone_of_primitives.rs stdout ----
---- [mir-opt] tests/mir-opt/combine_clone_of_primitives.rs stdout ----
thread '[mir-opt] tests/mir-opt/combine_clone_of_primitives.rs' panicked at 'the mir dump file for combine_clone_of_primitives.{impl#0}-clone.InstSimplify.before.mir does not exist (requested in /checkout/tests/mir-opt/combine_clone_of_primitives.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/const_prop/slice_len.rs stdout ----
22                                            // mir::Constant
22                                            // mir::Constant
23                                            // + span: $DIR/slice_len.rs:8:6: 8:19
24                                            // + literal: Const { ty: &[u32; 3], val: Unevaluated(main, [], Some(promoted[0])) }
-           _4 = _9;                         // scope 0 at $DIR/slice_len.rs:+1:6: +1:19
-           _3 = _4;                         // scope 0 at $DIR/slice_len.rs:+1:6: +1:19
Build completed unsuccessfully in 0:12:36
+           _4 = &(*_9);                     // scope 0 at $DIR/slice_len.rs:+1:6: +1:19
+           _3 = &(*_4);                     // scope 0 at $DIR/slice_len.rs:+1:6: +1:19
27           _2 = move _3 as &[u32] (Pointer(Unsize)); // scope 0 at $DIR/slice_len.rs:+1:6: +1:19
28           StorageDead(_3);                 // scope 0 at $DIR/slice_len.rs:+1:18: +1:19
29           StorageLive(_6);                 // scope 0 at $DIR/slice_len.rs:+1:31: +1:32

30           _6 = const 1_usize;              // scope 0 at $DIR/slice_len.rs:+1:31: +1:32
- -         _7 = Len((*_2));                 // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
- -         _8 = Lt(_6, _7);                 // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
- -         assert(move _8, "index out of bounds: the length is {} but the index is {}", move _7, _6) -> bb1; // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
- +         _7 = const 3_usize;              // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
- +         _8 = const true;                 // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
- +         assert(const true, "index out of bounds: the length is {} but the index is {}", move _7, _6) -> bb1; // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
+           _7 = Len((*_2));                 // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
+           _8 = Lt(_6, _7);                 // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
+           assert(move _8, "index out of bounds: the length is {} but the index is {}", move _7, _6) -> bb1; // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
38   
39       bb1: {


40 -         _1 = (*_2)[_6];                  // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
- +         _1 = const 2_u32;                // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
+ +         _1 = (*_2)[1 of 2];              // scope 0 at $DIR/slice_len.rs:+1:5: +1:33
42           StorageDead(_6);                 // scope 0 at $DIR/slice_len.rs:+1:33: +1:34
43           StorageDead(_4);                 // scope 0 at $DIR/slice_len.rs:+1:33: +1:34
44           StorageDead(_2);                 // scope 0 at $DIR/slice_len.rs:+1:33: +1:34

thread '[mir-opt] tests/mir-opt/const_prop/slice_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/slice_len.main.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/dont_yeet_assert.rs stdout ----
---- [mir-opt] tests/mir-opt/dont_yeet_assert.rs stdout ----
thread '[mir-opt] tests/mir-opt/dont_yeet_assert.rs' panicked at 'the mir dump file for dont_yeet_assert.generic.InstSimplify.before.mir does not exist (requested in /checkout/tests/mir-opt/dont_yeet_assert.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/funky_arms.rs stdout ----
86   
87       bb6: {
87       bb6: {
88           _10 = ((_7 as Some).0: usize);   // scope 3 at $DIR/funky_arms.rs:+13:17: +13:26
+           StorageLive(_11);                // scope 3 at $DIR/funky_arms.rs:+15:43: +15:46
+           _11 = &mut (*_1);                // scope 3 at $DIR/funky_arms.rs:+15:43: +15:46
+           StorageLive(_12);                // scope 3 at $DIR/funky_arms.rs:+15:48: +15:51
+           _12 = &(*_2);                    // scope 3 at $DIR/funky_arms.rs:+15:48: +15:51
89           StorageLive(_13);                // scope 3 at $DIR/funky_arms.rs:+15:53: +15:57
90           _13 = _6;                        // scope 3 at $DIR/funky_arms.rs:+15:53: +15:57
91           StorageLive(_14);                // scope 3 at $DIR/funky_arms.rs:+15:59: +15:79

93           _15 = _10 as u32 (IntToInt);     // scope 3 at $DIR/funky_arms.rs:+15:59: +15:75
94           _14 = Add(move _15, const 1_u32); // scope 3 at $DIR/funky_arms.rs:+15:59: +15:79
95           StorageDead(_15);                // scope 3 at $DIR/funky_arms.rs:+15:78: +15:79
-           _0 = float_to_exponential_common_exact::<T>(_1, _2, move _13, move _14, _3) -> bb7; // scope 3 at $DIR/funky_arms.rs:+15:9: +15:87
+           _0 = float_to_exponential_common_exact::<T>(move _11, move _12, move _13, move _14, _3) -> bb7; // scope 3 at $DIR/funky_arms.rs:+15:9: +15:87
97                                            // mir::Constant
98                                            // + span: $DIR/funky_arms.rs:27:9: 27:42
99                                            // + literal: Const { ty: for<'a, 'b, 'c> fn(&'a mut Formatter<'b>, &'c T, Sign, u32, bool) -> Result<(), std::fmt::Error> {float_to_exponential_common_exact::<T>}, val: Value(<ZST>) }
102       bb7: {
102       bb7: {
103           StorageDead(_14);                // scope 3 at $DIR/funky_arms.rs:+15:86: +15:87
104           StorageDead(_13);                // scope 3 at $DIR/funky_arms.rs:+15:86: +15:87
+           StorageDead(_12);                // scope 3 at $DIR/funky_arms.rs:+15:86: +15:87
+           StorageDead(_11);                // scope 3 at $DIR/funky_arms.rs:+15:86: +15:87
105           goto -> bb10;                    // scope 2 at $DIR/funky_arms.rs:+13:5: +18:6
107   

108       bb8: {
108       bb8: {
+           StorageLive(_18);                // scope 2 at $DIR/funky_arms.rs:+17:46: +17:49
+           _18 = &mut (*_1);                // scope 2 at $DIR/funky_arms.rs:+17:46: +17:49
+           StorageLive(_19);                // scope 2 at $DIR/funky_arms.rs:+17:51: +17:54
+           _19 = &(*_2);                    // scope 2 at $DIR/funky_arms.rs:+17:51: +17:54
109           StorageLive(_20);                // scope 2 at $DIR/funky_arms.rs:+17:56: +17:60
110           _20 = _6;                        // scope 2 at $DIR/funky_arms.rs:+17:56: +17:60
-           _0 = float_to_exponential_common_shortest::<T>(_1, _2, move _20, _3) -> bb9; // scope 2 at $DIR/funky_arms.rs:+17:9: +17:68
+           _0 = float_to_exponential_common_shortest::<T>(move _18, move _19, move _20, _3) -> bb9; // scope 2 at $DIR/funky_arms.rs:+17:9: +17:68
112                                            // mir::Constant
113                                            // + span: $DIR/funky_arms.rs:29:9: 29:45
114                                            // + literal: Const { ty: for<'a, 'b, 'c> fn(&'a mut Formatter<'b>, &'c T, Sign, bool) -> Result<(), std::fmt::Error> {float_to_exponential_common_shortest::<T>}, val: Value(<ZST>) }
116   
117       bb9: {
117       bb9: {
118           StorageDead(_20);                // scope 2 at $DIR/funky_arms.rs:+17:67: +17:68
+           StorageDead(_19);                // scope 2 at $DIR/funky_arms.rs:+17:67: +17:68
+           StorageDead(_18);                // scope 2 at $DIR/funky_arms.rs:+17:67: +17:68
119           goto -> bb10;                    // scope 2 at $DIR/funky_arms.rs:+13:5: +18:6
121   


thread '[mir-opt] tests/mir-opt/funky_arms.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/funky_arms.float_to_exponential_common.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/equal_true.rs stdout ----
---- [mir-opt] tests/mir-opt/equal_true.rs stdout ----
thread '[mir-opt] tests/mir-opt/equal_true.rs' panicked at 'the mir dump file for equal_true.opt.InstSimplify.before.mir does not exist (requested in /checkout/tests/mir-opt/equal_true.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/inline/dyn_trait.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/dyn_trait.rs stdout ----
8       let mut _3: &C;                      // in scope 0 at $DIR/dyn_trait.rs:+1:14: +1:15
9 +     scope 1 (inlined mk_cycle::<<C as Cache>::V>) { // at $DIR/dyn_trait.rs:28:5: 28:16
10 +         debug c => _2;                   // in scope 1 at $DIR/dyn_trait.rs:21:27: 21:28
+ +         let mut _4: &dyn Cache<V = <C as Cache>::V>; // in scope 1 at $DIR/dyn_trait.rs:22:5: 22:22
12   
13       bb0: {


17           _2 = move _3 as &dyn Cache<V = <C as Cache>::V> (Pointer(Unsize)); // scope 0 at $DIR/dyn_trait.rs:+1:14: +1:15
18           StorageDead(_3);                 // scope 0 at $DIR/dyn_trait.rs:+1:14: +1:15
19 -         _0 = mk_cycle::<<C as Cache>::V>(move _2) -> bb1; // scope 0 at $DIR/dyn_trait.rs:+1:5: +1:16
- +         _0 = <dyn Cache<V = <C as Cache>::V> as Cache>::store_nocache(_2) -> bb1; // scope 1 at $DIR/dyn_trait.rs:22:5: 22:22
+ +         StorageLive(_4);                 // scope 1 at $DIR/dyn_trait.rs:22:5: 22:22
+ +         _4 = &(*_2);                     // scope 1 at $DIR/dyn_trait.rs:22:5: 22:22
+ +         _0 = <dyn Cache<V = <C as Cache>::V> as Cache>::store_nocache(move _4) -> bb1; // scope 1 at $DIR/dyn_trait.rs:22:5: 22:22
21                                            // mir::Constant
22 -                                          // + span: $DIR/dyn_trait.rs:28:5: 28:13
23 -                                          // + literal: Const { ty: for<'a> fn(&'a (dyn Cache<V = <C as Cache>::V> + 'a)) {mk_cycle::<<C as Cache>::V>}, val: Value(<ZST>) }
26       }
27   
28       bb1: {
28       bb1: {
+ +         StorageDead(_4);                 // scope 1 at $DIR/dyn_trait.rs:22:21: 22:22
29           StorageDead(_2);                 // scope 0 at $DIR/dyn_trait.rs:+1:15: +1:16
30           return;                          // scope 0 at $DIR/dyn_trait.rs:+2:2: +2:2


thread '[mir-opt] tests/mir-opt/inline/dyn_trait.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/dyn_trait.try_execute_query.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/instsimplify_duplicate_switch_targets.rs stdout ----
---- [mir-opt] tests/mir-opt/instsimplify_duplicate_switch_targets.rs stdout ----
thread '[mir-opt] tests/mir-opt/instsimplify_duplicate_switch_targets.rs' panicked at 'the mir dump file for instsimplify_duplicate_switch_targets.assert_zero.InstSimplify.before.mir does not exist (requested in /checkout/tests/mir-opt/instsimplify_duplicate_switch_targets.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/intrinsic_asserts.rs stdout ----
---- [mir-opt] tests/mir-opt/intrinsic_asserts.rs stdout ----
thread '[mir-opt] tests/mir-opt/intrinsic_asserts.rs' panicked at 'the mir dump file for intrinsic_asserts.removable.InstSimplify.before.mir does not exist (requested in /checkout/tests/mir-opt/intrinsic_asserts.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/inline/inline_trait_method_2.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_trait_method_2.rs stdout ----
7     let mut _3: &dyn X;                  // in scope 0 at $DIR/inline_trait_method_2.rs:+1:10: +1:11
8     scope 1 (inlined test) {             // at $DIR/inline_trait_method_2.rs:6:5: 6:12
9         debug x => _2;                   // in scope 1 at $DIR/inline_trait_method_2.rs:10:9: 10:10
+         let mut _4: &dyn X;              // in scope 1 at $DIR/inline_trait_method_2.rs:11:5: 11:10
11 
12     bb0: {


15         _3 = &(*_1);                     // scope 0 at $DIR/inline_trait_method_2.rs:+1:10: +1:11
16         _2 = move _3 as &dyn X (Pointer(Unsize)); // scope 0 at $DIR/inline_trait_method_2.rs:+1:10: +1:11
17         StorageDead(_3);                 // scope 0 at $DIR/inline_trait_method_2.rs:+1:10: +1:11
-         _0 = <dyn X as X>::y(_2) -> bb1; // scope 1 at $DIR/inline_trait_method_2.rs:11:5: 11:10
+         StorageLive(_4);                 // scope 1 at $DIR/inline_trait_method_2.rs:11:5: 11:10
+         _4 = &(*_2);                     // scope 1 at $DIR/inline_trait_method_2.rs:11:5: 11:10
+         _0 = <dyn X as X>::y(move _4) -> bb1; // scope 1 at $DIR/inline_trait_method_2.rs:11:5: 11:10
19                                          // mir::Constant
20                                          // + span: $DIR/inline_trait_method_2.rs:11:7: 11:8
21                                          // + literal: Const { ty: for<'a> fn(&'a dyn X) -> bool {<dyn X as X>::y}, val: Value(<ZST>) }
22     }
23 
24     bb1: {
24     bb1: {
+         StorageDead(_4);                 // scope 1 at $DIR/inline_trait_method_2.rs:11:9: 11:10
25         StorageDead(_2);                 // scope 0 at $DIR/inline_trait_method_2.rs:+1:11: +1:12
26         return;                          // scope 0 at $DIR/inline_trait_method_2.rs:+2:2: +2:2


thread '[mir-opt] tests/mir-opt/inline/inline_trait_method_2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_trait_method_2.test2.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/issue_58867_inline_as_ref_as_mut.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/issue_58867_inline_as_ref_as_mut.rs stdout ----
8     let mut _4: &mut [T];                // in scope 0 at $DIR/issue_58867_inline_as_ref_as_mut.rs:+1:5: +1:15
9     scope 1 (inlined <[T] as AsMut<[T]>>::as_mut) { // at $DIR/issue_58867_inline_as_ref_as_mut.rs:3:7: 3:15
10         debug self => _4;                // in scope 1 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
+         let mut _5: &mut [T];            // in scope 1 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
12 
13     bb0: {


15         StorageLive(_3);                 // scope 0 at $DIR/issue_58867_inline_as_ref_as_mut.rs:+1:5: +1:15
16         StorageLive(_4);                 // scope 0 at $DIR/issue_58867_inline_as_ref_as_mut.rs:+1:5: +1:15
17         _4 = &mut (*_1);                 // scope 0 at $DIR/issue_58867_inline_as_ref_as_mut.rs:+1:5: +1:15
-         _3 = _4;                         // scope 1 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
+         StorageLive(_5);                 // scope 1 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
+         _5 = &mut (*_4);                 // scope 1 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
+         _3 = &mut (*_5);                 // scope 1 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
+         StorageDead(_5);                 // scope 1 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
19         _2 = &mut (*_3);                 // scope 0 at $DIR/issue_58867_inline_as_ref_as_mut.rs:+1:5: +1:15
20         StorageDead(_4);                 // scope 0 at $DIR/issue_58867_inline_as_ref_as_mut.rs:+1:14: +1:15
21         _0 = &mut (*_2);                 // scope 0 at $DIR/issue_58867_inline_as_ref_as_mut.rs:+1:5: +1:15

thread '[mir-opt] tests/mir-opt/inline/issue_58867_inline_as_ref_as_mut.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/issue_58867_inline_as_ref_as_mut.a.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/issue_78192.rs stdout ----
---- [mir-opt] tests/mir-opt/issue_78192.rs stdout ----
thread '[mir-opt] tests/mir-opt/issue_78192.rs' panicked at 'the mir dump file for issue_78192.f.InstSimplify.before.mir does not exist (requested in /checkout/tests/mir-opt/issue_78192.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/issue_76432.rs stdout ----
29   
30       bb0: {
30       bb0: {
31           StorageLive(_2);                 // scope 0 at $DIR/issue_76432.rs:+1:9: +1:10
+           StorageLive(_3);                 // scope 0 at $DIR/issue_76432.rs:+1:19: +1:29
32           StorageLive(_5);                 // scope 0 at $DIR/issue_76432.rs:+1:20: +1:29
33           _5 = [_1, _1, _1];               // scope 0 at $DIR/issue_76432.rs:+1:20: +1:29
-           _4 = &_5;                        // scope 0 at $DIR/issue_76432.rs:+1:19: +1:29
-           _2 = _4 as &[T] (Pointer(Unsize)); // scope 0 at $DIR/issue_76432.rs:+1:19: +1:29
+           _3 = &_5;                        // scope 0 at $DIR/issue_76432.rs:+1:19: +1:29
+           _2 = move _3 as &[T] (Pointer(Unsize)); // scope 0 at $DIR/issue_76432.rs:+1:19: +1:29
+           StorageDead(_3);                 // scope 0 at $DIR/issue_76432.rs:+1:28: +1:29
36           _9 = Len((*_2));                 // scope 1 at $DIR/issue_76432.rs:+3:9: +3:33
37           _10 = const 3_usize;             // scope 1 at $DIR/issue_76432.rs:+3:9: +3:33
38 -         _11 = Eq(move _9, const 3_usize); // scope 1 at $DIR/issue_76432.rs:+3:9: +3:33

thread '[mir-opt] tests/mir-opt/issue_76432.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/issue_76432.test.SimplifyComparisonIntegral.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/not_equal_false.rs stdout ----
---- [mir-opt] tests/mir-opt/not_equal_false.rs stdout ----
thread '[mir-opt] tests/mir-opt/not_equal_false.rs' panicked at 'the mir dump file for not_equal_false.opt.InstSimplify.before.mir does not exist (requested in /checkout/tests/mir-opt/not_equal_false.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/pre-codegen/optimizes_into_variable.rs stdout ----
---- [mir-opt] tests/mir-opt/pre-codegen/optimizes_into_variable.rs stdout ----
38           _4 = [const 0_i32, const 1_i32, const 2_i32, const 3_i32, const 4_i32, const 5_i32]; // scope 1 at $DIR/optimizes_into_variable.rs:+2:13: +2:31
39           StorageLive(_5);                 // scope 1 at $DIR/optimizes_into_variable.rs:+2:32: +2:33
40           _5 = const 3_usize;              // scope 1 at $DIR/optimizes_into_variable.rs:+2:32: +2:33
-           _6 = const 6_usize;              // scope 1 at $DIR/optimizes_into_variable.rs:+2:13: +2:34
+ -         _6 = Len(_4);                    // scope 1 at $DIR/optimizes_into_variable.rs:+2:13: +2:34
42 -         _7 = Lt(_5, _6);                 // scope 1 at $DIR/optimizes_into_variable.rs:+2:13: +2:34
43 -         assert(move _7, "index out of bounds: the length is {} but the index is {}", move _6, _5) -> bb2; // scope 1 at $DIR/optimizes_into_variable.rs:+2:13: +2:34
+ +         _6 = const 6_usize;              // scope 1 at $DIR/optimizes_into_variable.rs:+2:13: +2:34
44 +         _7 = const true;                 // scope 1 at $DIR/optimizes_into_variable.rs:+2:13: +2:34
45 +         assert(const true, "index out of bounds: the length is {} but the index is {}", const 6_usize, const 3_usize) -> bb2; // scope 1 at $DIR/optimizes_into_variable.rs:+2:13: +2:34


thread '[mir-opt] tests/mir-opt/pre-codegen/optimizes_into_variable.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/pre-codegen/optimizes_into_variable.main.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/slice_filter.rs stdout ----
---- [mir-opt] tests/mir-opt/slice_filter.rs stdout ----
29       let mut _26: &(usize, usize, usize, usize); // in scope 0 at $DIR/slice_filter.rs:+0:26: +0:38
30       let mut _27: &(usize, usize, usize, usize); // in scope 0 at $DIR/slice_filter.rs:+0:26: +0:38
31       let mut _28: &(usize, usize, usize, usize); // in scope 0 at $DIR/slice_filter.rs:+0:26: +0:38
-       let mut _31: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
-       let mut _32: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
-       let mut _37: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
-       let mut _38: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
-       let mut _43: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
-       let mut _44: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
-       let mut _49: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
-       let mut _50: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
40       scope 1 {
41 -         debug a => _3;                   // in scope 1 at $DIR/slice_filter.rs:+0:27: +0:28
42 -         debug b => _4;                   // in scope 1 at $DIR/slice_filter.rs:+0:30: +0:31

53 +             debug other => &&((*_27).2: usize); // in scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
54               let mut _29: &usize;         // in scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
55               let mut _30: &usize;         // in scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               let mut _31: &usize;         // in scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               let mut _32: &usize;         // in scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
56               scope 3 (inlined cmp::impls::<impl PartialOrd for usize>::le) { // at $SRC_DIR/core/src/cmp.rs:LL:COL
57 -                 debug self => _29;       // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
58 -                 debug other => _30;      // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL

69 +             debug other => &&((*_25).0: usize); // in scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
70               let mut _35: &usize;         // in scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
71               let mut _36: &usize;         // in scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               let mut _37: &usize;         // in scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               let mut _38: &usize;         // in scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
72               scope 5 (inlined cmp::impls::<impl PartialOrd for usize>::le) { // at $SRC_DIR/core/src/cmp.rs:LL:COL
73 -                 debug self => _35;       // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
74 -                 debug other => _36;      // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL

85 +             debug other => &&((*_26).1: usize); // in scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
86               let mut _41: &usize;         // in scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
87               let mut _42: &usize;         // in scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               let mut _43: &usize;         // in scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               let mut _44: &usize;         // in scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
88               scope 7 (inlined cmp::impls::<impl PartialOrd for usize>::le) { // at $SRC_DIR/core/src/cmp.rs:LL:COL
89 -                 debug self => _41;       // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
90 -                 debug other => _42;      // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL

101 +             debug other => &&((*_28).3: usize); // in scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
102               let mut _47: &usize;         // in scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
103               let mut _48: &usize;         // in scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               let mut _49: &usize;         // in scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
+               let mut _50: &usize;         // in scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
104               scope 9 (inlined cmp::impls::<impl PartialOrd for usize>::le) { // at $SRC_DIR/core/src/cmp.rs:LL:COL
105 -                 debug self => _47;       // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
106 -                 debug other => _48;      // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL

131 -         _9 = &_3;                        // scope 1 at $DIR/slice_filter.rs:+0:40: +0:41
132 -         StorageLive(_10);                // scope 1 at $DIR/slice_filter.rs:+0:45: +0:46
133 -         StorageLive(_11);                // scope 1 at $DIR/slice_filter.rs:+0:45: +0:46
- -         _11 = _5;                        // scope 1 at $DIR/slice_filter.rs:+0:45: +0:46
+ -         _11 = &(*_5);                    // scope 1 at $DIR/slice_filter.rs:+0:45: +0:46
135 -         _10 = &_11;                      // scope 1 at $DIR/slice_filter.rs:+0:45: +0:46
- -         _29 = deref_copy (*_9);          // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
- -         _30 = deref_copy (*_10);         // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         StorageLive(_29);                // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _31 = deref_copy (*_9);          // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _29 = &(*_31);                   // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         StorageLive(_30);                // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _32 = deref_copy (*_10);         // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _30 = &(*_32);                   // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
138           StorageLive(_33);                // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
139 -         _33 = (*_29);                    // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
140 +         _33 = ((*_25).0: usize);         // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL

144           _8 = Le(move _33, move _34);     // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
145           StorageDead(_34);                // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
146           StorageDead(_33);                // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         StorageDead(_30);                // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         StorageDead(_29);                // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
147 -         StorageDead(_11);                // scope 1 at $DIR/slice_filter.rs:+0:45: +0:46
148 -         StorageDead(_10);                // scope 1 at $DIR/slice_filter.rs:+0:45: +0:46
149 -         StorageDead(_9);                 // scope 1 at $DIR/slice_filter.rs:+0:45: +0:46

162 -         _18 = &_5;                       // scope 1 at $DIR/slice_filter.rs:+0:60: +0:61
163 -         StorageLive(_19);                // scope 1 at $DIR/slice_filter.rs:+0:65: +0:66
164 -         StorageLive(_20);                // scope 1 at $DIR/slice_filter.rs:+0:65: +0:66
- -         _20 = _3;                        // scope 1 at $DIR/slice_filter.rs:+0:65: +0:66
+ -         _20 = &(*_3);                    // scope 1 at $DIR/slice_filter.rs:+0:65: +0:66
166 -         _19 = &_20;                      // scope 1 at $DIR/slice_filter.rs:+0:65: +0:66
- -         _35 = deref_copy (*_18);         // scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
- -         _36 = deref_copy (*_19);         // scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         StorageLive(_35);                // scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _37 = deref_copy (*_18);         // scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _35 = &(*_37);                   // scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         StorageLive(_36);                // scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _38 = deref_copy (*_19);         // scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _36 = &(*_38);                   // scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
169           StorageLive(_39);                // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
170 -         _39 = (*_35);                    // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
171 +         _39 = ((*_27).2: usize);         // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL

175           _17 = Le(move _39, move _40);    // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
176           StorageDead(_40);                // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
177           StorageDead(_39);                // scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         StorageDead(_36);                // scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         StorageDead(_35);                // scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
178 -         StorageDead(_20);                // scope 1 at $DIR/slice_filter.rs:+0:65: +0:66
179 -         StorageDead(_19);                // scope 1 at $DIR/slice_filter.rs:+0:65: +0:66
180 -         StorageDead(_18);                // scope 1 at $DIR/slice_filter.rs:+0:65: +0:66

204 -         _13 = &_6;                       // scope 1 at $DIR/slice_filter.rs:+0:50: +0:51
205 -         StorageLive(_14);                // scope 1 at $DIR/slice_filter.rs:+0:55: +0:56
206 -         StorageLive(_15);                // scope 1 at $DIR/slice_filter.rs:+0:55: +0:56
- -         _15 = _4;                        // scope 1 at $DIR/slice_filter.rs:+0:55: +0:56
+ -         _15 = &(*_4);                    // scope 1 at $DIR/slice_filter.rs:+0:55: +0:56
208 -         _14 = &_15;                      // scope 1 at $DIR/slice_filter.rs:+0:55: +0:56
- -         _41 = deref_copy (*_13);         // scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
- -         _42 = deref_copy (*_14);         // scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         StorageLive(_41);                // scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _43 = deref_copy (*_13);         // scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _41 = &(*_43);                   // scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         StorageLive(_42);                // scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _44 = deref_copy (*_14);         // scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _42 = &(*_44);                   // scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
211           StorageLive(_45);                // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
212 -         _45 = (*_41);                    // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
213 +         _45 = ((*_28).3: usize);         // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL

217           _12 = Le(move _45, move _46);    // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
218           StorageDead(_46);                // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
219           StorageDead(_45);                // scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         StorageDead(_42);                // scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         StorageDead(_41);                // scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
220 -         StorageDead(_15);                // scope 1 at $DIR/slice_filter.rs:+0:55: +0:56
221 -         StorageDead(_14);                // scope 1 at $DIR/slice_filter.rs:+0:55: +0:56
222 -         StorageDead(_13);                // scope 1 at $DIR/slice_filter.rs:+0:55: +0:56

237 -         _22 = &_4;                       // scope 1 at $DIR/slice_filter.rs:+0:70: +0:71
238 -         StorageLive(_23);                // scope 1 at $DIR/slice_filter.rs:+0:75: +0:76
239 -         StorageLive(_24);                // scope 1 at $DIR/slice_filter.rs:+0:75: +0:76
- -         _24 = _6;                        // scope 1 at $DIR/slice_filter.rs:+0:75: +0:76
+ -         _24 = &(*_6);                    // scope 1 at $DIR/slice_filter.rs:+0:75: +0:76
241 -         _23 = &_24;                      // scope 1 at $DIR/slice_filter.rs:+0:75: +0:76
- -         _47 = deref_copy (*_22);         // scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
- -         _48 = deref_copy (*_23);         // scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         StorageLive(_47);                // scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _49 = deref_copy (*_22);         // scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _47 = &(*_49);                   // scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         StorageLive(_48);                // scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _50 = deref_copy (*_23);         // scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _48 = &(*_50);                   // scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
244           StorageLive(_51);                // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
245 -         _51 = (*_47);                    // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
246 +         _51 = ((*_26).1: usize);         // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL

250           _21 = Le(move _51, move _52);    // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
251           StorageDead(_52);                // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
252           StorageDead(_51);                // scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         StorageDead(_48);                // scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         StorageDead(_47);                // scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
253 -         StorageDead(_24);                // scope 1 at $DIR/slice_filter.rs:+0:75: +0:76
254 -         StorageDead(_23);                // scope 1 at $DIR/slice_filter.rs:+0:75: +0:76
255 -         StorageDead(_22);                // scope 1 at $DIR/slice_filter.rs:+0:75: +0:76

thread '[mir-opt] tests/mir-opt/slice_filter.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/slice_filter.variant_a-{closure#0}.ReferencePropagation.diff', src/tools/compiletest/src/runtest.rs:3639:21

failures:
    [mir-opt] tests/mir-opt/bool_compare.rs
    [mir-opt] tests/mir-opt/casts.rs
