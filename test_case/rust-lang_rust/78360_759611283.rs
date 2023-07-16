plain

---- [mir-opt] mir-opt/const_prop/array_index.rs stdout ----
13       }
14   
15       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/array_index.rs:5:9: 5:10
-           StorageLive(_2);                 // scope 0 at $DIR/array_index.rs:5:18: 5:30
18           _2 = [const 0_u32, const 1_u32, const 2_u32, const 3_u32]; // scope 0 at $DIR/array_index.rs:5:18: 5:30
-           StorageLive(_3);                 // scope 0 at $DIR/array_index.rs:5:31: 5:32
20           _3 = const 2_usize;              // scope 0 at $DIR/array_index.rs:5:31: 5:32
21           _4 = const 4_usize;              // scope 0 at $DIR/array_index.rs:5:18: 5:33
22 -         _5 = Lt(_3, _4);                 // scope 0 at $DIR/array_index.rs:5:18: 5:33
28       bb1: {
28       bb1: {
29 -         _1 = _2[_3];                     // scope 0 at $DIR/array_index.rs:5:18: 5:33
30 +         _1 = const 2_u32;                // scope 0 at $DIR/array_index.rs:5:18: 5:33
-           StorageDead(_3);                 // scope 0 at $DIR/array_index.rs:5:33: 5:34
-           StorageDead(_2);                 // scope 0 at $DIR/array_index.rs:5:33: 5:34
33           _0 = const ();                   // scope 0 at $DIR/array_index.rs:4:11: 6:2
-           StorageDead(_1);                 // scope 0 at $DIR/array_index.rs:6:1: 6:2
35           return;                          // scope 0 at $DIR/array_index.rs:6:2: 6:2
37   }


thread '[mir-opt] mir-opt/const_prop/array_index.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/array_index.main.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3452:25

---- [mir-opt] mir-opt/const_prop/bad_op_div_by_zero.rs stdout ----
18       }
19   
19   
20       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/bad_op_div_by_zero.rs:4:9: 4:10
22           _1 = const 0_i32;                // scope 0 at $DIR/bad_op_div_by_zero.rs:4:13: 4:14
-           StorageLive(_2);                 // scope 1 at $DIR/bad_op_div_by_zero.rs:5:9: 5:11
-           StorageLive(_3);                 // scope 1 at $DIR/bad_op_div_by_zero.rs:5:18: 5:19
25 -         _3 = _1;                         // scope 1 at $DIR/bad_op_div_by_zero.rs:5:18: 5:19
26 -         _4 = Eq(_3, const 0_i32);        // scope 1 at $DIR/bad_op_div_by_zero.rs:5:14: 5:19
27 -         assert(!move _4, "attempt to divide `{}` by zero", const 1_i32) -> bb1; // scope 1 at $DIR/bad_op_div_by_zero.rs:5:14: 5:19
44       bb2: {
44       bb2: {
45 -         _2 = Div(const 1_i32, move _3);  // scope 1 at $DIR/bad_op_div_by_zero.rs:5:14: 5:19
46 +         _2 = Div(const 1_i32, const 0_i32); // scope 1 at $DIR/bad_op_div_by_zero.rs:5:14: 5:19
-           StorageDead(_3);                 // scope 1 at $DIR/bad_op_div_by_zero.rs:5:18: 5:19
48           _0 = const ();                   // scope 0 at $DIR/bad_op_div_by_zero.rs:3:11: 6:2
-           StorageDead(_2);                 // scope 1 at $DIR/bad_op_div_by_zero.rs:6:1: 6:2
-           StorageDead(_1);                 // scope 0 at $DIR/bad_op_div_by_zero.rs:6:1: 6:2
51           return;                          // scope 0 at $DIR/bad_op_div_by_zero.rs:6:2: 6:2
53   }


thread '[mir-opt] mir-opt/const_prop/bad_op_div_by_zero.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/bad_op_div_by_zero.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3452:25
---- [mir-opt] mir-opt/const_prop/bad_op_mod_by_zero.rs stdout ----
18       }
19   
20       bb0: {
20       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/bad_op_mod_by_zero.rs:4:9: 4:10
22           _1 = const 0_i32;                // scope 0 at $DIR/bad_op_mod_by_zero.rs:4:13: 4:14
-           StorageLive(_2);                 // scope 1 at $DIR/bad_op_mod_by_zero.rs:5:9: 5:11
-           StorageLive(_3);                 // scope 1 at $DIR/bad_op_mod_by_zero.rs:5:18: 5:19
25 -         _3 = _1;                         // scope 1 at $DIR/bad_op_mod_by_zero.rs:5:18: 5:19
26 -         _4 = Eq(_3, const 0_i32);        // scope 1 at $DIR/bad_op_mod_by_zero.rs:5:14: 5:19
27 -         assert(!move _4, "attempt to calculate the remainder of `{}` with a divisor of zero", const 1_i32) -> bb1; // scope 1 at $DIR/bad_op_mod_by_zero.rs:5:14: 5:19
44       bb2: {
44       bb2: {
45 -         _2 = Rem(const 1_i32, move _3);  // scope 1 at $DIR/bad_op_mod_by_zero.rs:5:14: 5:19
46 +         _2 = Rem(const 1_i32, const 0_i32); // scope 1 at $DIR/bad_op_mod_by_zero.rs:5:14: 5:19
-           StorageDead(_3);                 // scope 1 at $DIR/bad_op_mod_by_zero.rs:5:18: 5:19
48           _0 = const ();                   // scope 0 at $DIR/bad_op_mod_by_zero.rs:3:11: 6:2
-           StorageDead(_2);                 // scope 1 at $DIR/bad_op_mod_by_zero.rs:6:1: 6:2
-           StorageDead(_1);                 // scope 0 at $DIR/bad_op_mod_by_zero.rs:6:1: 6:2
51           return;                          // scope 0 at $DIR/bad_op_mod_by_zero.rs:6:2: 6:2
53   }


thread '[mir-opt] mir-opt/const_prop/bad_op_mod_by_zero.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/bad_op_mod_by_zero.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3452:25
---- [mir-opt] mir-opt/const_debuginfo.rs stdout ----
53       }
54   
55       bb0: {
55       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/const_debuginfo.rs:9:9: 9:10
57           _1 = const 1_u8;                 // scope 0 at $DIR/const_debuginfo.rs:9:13: 9:16
-           StorageLive(_2);                 // scope 1 at $DIR/const_debuginfo.rs:10:9: 10:10
59           _2 = const 2_u8;                 // scope 1 at $DIR/const_debuginfo.rs:10:13: 10:16
-           StorageLive(_3);                 // scope 2 at $DIR/const_debuginfo.rs:11:9: 11:10
61           _3 = const 3_u8;                 // scope 2 at $DIR/const_debuginfo.rs:11:13: 11:16
-           StorageLive(_4);                 // scope 3 at $DIR/const_debuginfo.rs:12:9: 12:12
-           StorageLive(_5);                 // scope 3 at $DIR/const_debuginfo.rs:12:15: 12:20
-           StorageLive(_6);                 // scope 3 at $DIR/const_debuginfo.rs:12:15: 12:16
65           _6 = const 1_u8;                 // scope 3 at $DIR/const_debuginfo.rs:12:15: 12:16
-           StorageLive(_7);                 // scope 3 at $DIR/const_debuginfo.rs:12:19: 12:20
67           _7 = const 2_u8;                 // scope 3 at $DIR/const_debuginfo.rs:12:19: 12:20
68           _5 = const 3_u8;                 // scope 3 at $DIR/const_debuginfo.rs:12:15: 12:20
-           StorageDead(_7);                 // scope 3 at $DIR/const_debuginfo.rs:12:19: 12:20
-           StorageDead(_6);                 // scope 3 at $DIR/const_debuginfo.rs:12:19: 12:20
-           StorageLive(_8);                 // scope 3 at $DIR/const_debuginfo.rs:12:23: 12:24
72           _8 = const 3_u8;                 // scope 3 at $DIR/const_debuginfo.rs:12:23: 12:24
73           _4 = const 6_u8;                 // scope 3 at $DIR/const_debuginfo.rs:12:15: 12:24
-           StorageDead(_8);                 // scope 3 at $DIR/const_debuginfo.rs:12:23: 12:24
-           StorageDead(_5);                 // scope 3 at $DIR/const_debuginfo.rs:12:23: 12:24
-           StorageLive(_9);                 // scope 4 at $DIR/const_debuginfo.rs:14:9: 14:10
77           _9 = const "hello, world!";      // scope 4 at $DIR/const_debuginfo.rs:14:13: 14:28
78                                            // ty::Const
79                                            // + ty: &str
81                                            // mir::Constant
81                                            // mir::Constant
82                                            // + span: $DIR/const_debuginfo.rs:14:13: 14:28
83                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [8191], len: Size { raw: 13 } }, size: Size { raw: 13 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 13 }) }
-           StorageLive(_10);                // scope 5 at $DIR/const_debuginfo.rs:16:9: 16:10
85           (_10.0: bool) = const true;      // scope 5 at $DIR/const_debuginfo.rs:16:13: 16:34
86           (_10.1: bool) = const false;     // scope 5 at $DIR/const_debuginfo.rs:16:13: 16:34
87           (_10.2: u32) = const 123_u32;    // scope 5 at $DIR/const_debuginfo.rs:16:13: 16:34

-           StorageLive(_11);                // scope 6 at $DIR/const_debuginfo.rs:18:9: 18:10
89           ((_11 as Some).0: u16) = const 99_u16; // scope 6 at $DIR/const_debuginfo.rs:18:13: 18:24
90           discriminant(_11) = 1;           // scope 6 at $DIR/const_debuginfo.rs:18:13: 18:24
-           StorageLive(_12);                // scope 7 at $DIR/const_debuginfo.rs:20:9: 20:10
92           (_12.0: u32) = const 32_u32;     // scope 7 at $DIR/const_debuginfo.rs:20:13: 20:35
93           (_12.1: u32) = const 32_u32;     // scope 7 at $DIR/const_debuginfo.rs:20:13: 20:35
-           StorageLive(_13);                // scope 8 at $DIR/const_debuginfo.rs:21:9: 21:10
-           StorageLive(_14);                // scope 8 at $DIR/const_debuginfo.rs:21:13: 21:16
96           _14 = const 32_u32;              // scope 8 at $DIR/const_debuginfo.rs:21:13: 21:16
-           StorageLive(_15);                // scope 8 at $DIR/const_debuginfo.rs:21:19: 21:22
98           _15 = const 32_u32;              // scope 8 at $DIR/const_debuginfo.rs:21:19: 21:22
99           _13 = const 64_u32;              // scope 8 at $DIR/const_debuginfo.rs:21:13: 21:22
-           StorageDead(_15);                // scope 8 at $DIR/const_debuginfo.rs:21:21: 21:22
-           StorageDead(_14);                // scope 8 at $DIR/const_debuginfo.rs:21:21: 21:22
102           _0 = const ();                   // scope 0 at $DIR/const_debuginfo.rs:8:11: 22:2
-           StorageDead(_13);                // scope 8 at $DIR/const_debuginfo.rs:22:1: 22:2
-           StorageDead(_12);                // scope 7 at $DIR/const_debuginfo.rs:22:1: 22:2
-           StorageDead(_11);                // scope 6 at $DIR/const_debuginfo.rs:22:1: 22:2
-           StorageDead(_10);                // scope 5 at $DIR/const_debuginfo.rs:22:1: 22:2
-           StorageDead(_9);                 // scope 4 at $DIR/const_debuginfo.rs:22:1: 22:2
-           StorageDead(_4);                 // scope 3 at $DIR/const_debuginfo.rs:22:1: 22:2
-           StorageDead(_3);                 // scope 2 at $DIR/const_debuginfo.rs:22:1: 22:2
-           StorageDead(_2);                 // scope 1 at $DIR/const_debuginfo.rs:22:1: 22:2
-           StorageDead(_1);                 // scope 0 at $DIR/const_debuginfo.rs:22:1: 22:2
112           return;                          // scope 0 at $DIR/const_debuginfo.rs:22:2: 22:2
114   }


thread '[mir-opt] mir-opt/const_debuginfo.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_debuginfo.main.ConstDebugInfo.diff', src/tools/compiletest/src/runtest.rs:3452:25
---- [mir-opt] mir-opt/const_allocation.rs stdout ----
---- [mir-opt] mir-opt/const_allocation.rs stdout ----
6     let mut _2: &&[(std::option::Option<i32>, &[&str])]; // in scope 0 at $DIR/const_allocation.rs:8:5: 8:8
8     bb0: {
8     bb0: {
-         StorageLive(_1);                 // scope 0 at $DIR/const_allocation.rs:8:5: 8:8
-         StorageLive(_2);                 // scope 0 at $DIR/const_allocation.rs:8:5: 8:8
11         _2 = const {alloc0: &&[(Option<i32>, &[&str])]}; // scope 0 at $DIR/const_allocation.rs:8:5: 8:8
12                                          // ty::Const
13                                          // + ty: &&[(std::option::Option<i32>, &[&str])]

16                                          // + span: $DIR/const_allocation.rs:8:5: 8:8
17                                          // + literal: Const { ty: &&[(std::option::Option<i32>, &[&str])], val: Value(Scalar(alloc0)) }
18         _1 = (*_2);                      // scope 0 at $DIR/const_allocation.rs:8:5: 8:8
-         StorageDead(_2);                 // scope 0 at $DIR/const_allocation.rs:8:8: 8:9
-         StorageDead(_1);                 // scope 0 at $DIR/const_allocation.rs:8:8: 8:9
21         _0 = const ();                   // scope 0 at $DIR/const_allocation.rs:7:11: 9:2
22         return;                          // scope 0 at $DIR/const_allocation.rs:9:2: 9:2


thread '[mir-opt] mir-opt/const_allocation.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation.main.ConstProp.after.64bit.mir', src/tools/compiletest/src/runtest.rs:3452:25
---- [mir-opt] mir-opt/const_prop/cast.rs stdout ----
13       }
14   
15       bb0: {
15       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/cast.rs:4:9: 4:10
17 -         _1 = const 42_u8 as u32 (Misc);  // scope 0 at $DIR/cast.rs:4:13: 4:24
- +         _1 = const 42_u32;               // scope 0 at $DIR/cast.rs:4:13: 4:24
-           StorageLive(_2);                 // scope 1 at $DIR/cast.rs:6:9: 6:10
20 -         _2 = const 42_u32 as u8 (Misc);  // scope 1 at $DIR/cast.rs:6:13: 6:24
+ +         _1 = const 42_u32;               // scope 0 at $DIR/cast.rs:4:13: 4:24
21 +         _2 = const 42_u8;                // scope 1 at $DIR/cast.rs:6:13: 6:24
22           _0 = const ();                   // scope 0 at $DIR/cast.rs:3:11: 7:2
-           StorageDead(_2);                 // scope 1 at $DIR/cast.rs:7:1: 7:2
-           StorageDead(_1);                 // scope 0 at $DIR/cast.rs:7:1: 7:2
25           return;                          // scope 0 at $DIR/cast.rs:7:2: 7:2
27   }


thread '[mir-opt] mir-opt/const_prop/cast.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/cast.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3452:25
---- [mir-opt] mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs stdout ----
22       }
23   
24       bb0: {
24       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:9: 5:10
-           StorageLive(_2);                 // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35
-           StorageLive(_3);                 // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35
28           _9 = const main::promoted[0];    // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35
29                                            // ty::Const
30                                            // + ty: &[i32; 3]

35           _3 = _9;                         // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35
36           _2 = &raw const (*_3);           // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35
37           _1 = move _2 as *const [i32] (Pointer(Unsize)); // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35
-           StorageDead(_2);                 // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:34: 5:35
-           StorageDead(_3);                 // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:5:35: 5:36
-           StorageLive(_5);                 // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:7:13: 7:15
-           StorageLive(_6);                 // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:7:23: 7:24
42           _6 = const 3_usize;              // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:7:23: 7:24
43           _7 = Len((*_1));                 // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:7:18: 7:25
44 -         _8 = Lt(_6, _7);                 // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:7:18: 7:25
49   
50       bb1: {
50       bb1: {
51           _5 = (*_1)[_6];                  // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:7:18: 7:25
-           StorageDead(_6);                 // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:7:25: 7:26
53           _0 = const ();                   // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:6:5: 8:6
-           StorageDead(_5);                 // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:8:5: 8:6
-           StorageDead(_1);                 // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:9:1: 9:2
56           return;                          // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:9:2: 9:2
58   }


thread '[mir-opt] mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.main.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3452:25
---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
6     let mut _2: &&[(std::option::Option<i32>, &[&u8])]; // in scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
8     bb0: {
8     bb0: {
-         StorageLive(_1);                 // scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
-         StorageLive(_2);                 // scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
11         _2 = const {alloc0: &&[(Option<i32>, &[&u8])]}; // scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
12                                          // ty::Const
13                                          // + ty: &&[(std::option::Option<i32>, &[&u8])]

16                                          // + span: $DIR/const_allocation2.rs:5:5: 5:8
17                                          // + literal: Const { ty: &&[(std::option::Option<i32>, &[&u8])], val: Value(Scalar(alloc0)) }
18         _1 = (*_2);                      // scope 0 at $DIR/const_allocation2.rs:5:5: 5:8
-         StorageDead(_2);                 // scope 0 at $DIR/const_allocation2.rs:5:8: 5:9
-         StorageDead(_1);                 // scope 0 at $DIR/const_allocation2.rs:5:8: 5:9
21         _0 = const ();                   // scope 0 at $DIR/const_allocation2.rs:4:11: 6:2
22         return;                          // scope 0 at $DIR/const_allocation2.rs:6:2: 6:2


thread '[mir-opt] mir-opt/const_allocation2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation2.main.ConstProp.after.64bit.mir', src/tools/compiletest/src/runtest.rs:3452:25
---- [mir-opt] mir-opt/const_prop/checked_add.rs stdout ----
10       }
11   
12       bb0: {
12       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/checked_add.rs:5:9: 5:10
14 -         _2 = CheckedAdd(const 1_u32, const 1_u32); // scope 0 at $DIR/checked_add.rs:5:18: 5:23
15 -         assert(!move (_2.1: bool), "attempt to compute `{} + {}`, which would overflow", const 1_u32, const 1_u32) -> bb1; // scope 0 at $DIR/checked_add.rs:5:18: 5:23
16 +         _2 = const (2_u32, false);       // scope 0 at $DIR/checked_add.rs:5:18: 5:23

24 -         _1 = move (_2.0: u32);           // scope 0 at $DIR/checked_add.rs:5:18: 5:23
25 +         _1 = const 2_u32;                // scope 0 at $DIR/checked_add.rs:5:18: 5:23
26           _0 = const ();                   // scope 0 at $DIR/checked_add.rs:4:11: 6:2
-           StorageDead(_1);                 // scope 0 at $DIR/checked_add.rs:6:1: 6:2
28           return;                          // scope 0 at $DIR/checked_add.rs:6:2: 6:2
30   }


thread '[mir-opt] mir-opt/const_prop/checked_add.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/checked_add.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3452:25
---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
6     let mut _2: &&Packed;                // in scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
8     bb0: {
8     bb0: {
-         StorageLive(_1);                 // scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
-         StorageLive(_2);                 // scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
11         _2 = const {alloc0: &&Packed};   // scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
12                                          // ty::Const
13                                          // + ty: &&Packed

16                                          // + span: $DIR/const_allocation3.rs:5:5: 5:8
17                                          // + literal: Const { ty: &&Packed, val: Value(Scalar(alloc0)) }
18         _1 = (*_2);                      // scope 0 at $DIR/const_allocation3.rs:5:5: 5:8
-         StorageDead(_2);                 // scope 0 at $DIR/const_allocation3.rs:5:8: 5:9
-         StorageDead(_1);                 // scope 0 at $DIR/const_allocation3.rs:5:8: 5:9
21         _0 = const ();                   // scope 0 at $DIR/const_allocation3.rs:4:11: 6:2
22         return;                          // scope 0 at $DIR/const_allocation3.rs:6:2: 6:2


thread '[mir-opt] mir-opt/const_allocation3.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_allocation3.main.ConstProp.after.64bit.mir', src/tools/compiletest/src/runtest.rs:3452:25
---- [mir-opt] mir-opt/const_prop/const_prop_fails_gracefully.rs stdout ----
13       }
14   
15       bb0: {
15       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/const_prop_fails_gracefully.rs:7:9: 7:10
-           StorageLive(_2);                 // scope 0 at $DIR/const_prop_fails_gracefully.rs:7:13: 7:30
-           StorageLive(_3);                 // scope 0 at $DIR/const_prop_fails_gracefully.rs:7:13: 7:16
19           _3 = const FOO;                  // scope 0 at $DIR/const_prop_fails_gracefully.rs:7:13: 7:16
20                                            // ty::Const
21                                            // + ty: &i32

25                                            // + literal: Const { ty: &i32, val: Unevaluated(WithOptConstParam { did: DefId(0:5 ~ const_prop_fails_gracefully[317d]::main::FOO), const_param_did: None }, [], None) }
26           _2 = &raw const (*_3);           // scope 0 at $DIR/const_prop_fails_gracefully.rs:7:13: 7:16
27           _1 = move _2 as usize (Misc);    // scope 0 at $DIR/const_prop_fails_gracefully.rs:7:13: 7:39
-           StorageDead(_2);                 // scope 0 at $DIR/const_prop_fails_gracefully.rs:7:38: 7:39
-           StorageDead(_3);                 // scope 0 at $DIR/const_prop_fails_gracefully.rs:7:39: 7:40
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-           StorageLive(_4);                 // scope 1 at $DIR/const_prop_fails_gracefully.rs:8:5: 8:12
-           StorageLive(_5);                 // scope 1 at $DIR/const_prop_fails_gracefully.rs:8:10: 8:11
32           _5 = _1;                         // scope 1 at $DIR/const_prop_fails_gracefully.rs:8:10: 8:11
33           _4 = read(move _5) -> bb1;       // scope 1 at $DIR/const_prop_fails_gracefully.rs:8:5: 8:12
34                                            // mir::Constant
37       }
38   
39       bb1: {
39       bb1: {
-           StorageDead(_5);                 // scope 1 at $DIR/const_prop_fails_gracefully.rs:8:11: 8:12
-           StorageDead(_4);                 // scope 1 at $DIR/const_prop_fails_gracefully.rs:8:12: 8:13
42           _0 = const ();                   // scope 0 at $DIR/const_prop_fails_gracefully.rs:5:11: 9:2
-           StorageDead(_1);                 // scope 0 at $DIR/const_prop_fails_gracefully.rs:9:1: 9:2
44           return;                          // scope 0 at $DIR/const_prop_fails_gracefully.rs:9:2: 9:2
46   }


thread '[mir-opt] mir-opt/const_prop/const_prop_fails_gracefully.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/const_prop_fails_gracefully.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3452:25
---- [mir-opt] mir-opt/const_prop/indirect.rs stdout ----
11       }
12   
13       bb0: {
13       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/indirect.rs:5:9: 5:10
-           StorageLive(_2);                 // scope 0 at $DIR/indirect.rs:5:13: 5:25
16 -         _2 = const 2_u32 as u8 (Misc);   // scope 0 at $DIR/indirect.rs:5:13: 5:25
17 -         _3 = CheckedAdd(_2, const 1_u8); // scope 0 at $DIR/indirect.rs:5:13: 5:29
18 -         assert(!move (_3.1: bool), "attempt to compute `{} + {}`, which would overflow", move _2, const 1_u8) -> bb1; // scope 0 at $DIR/indirect.rs:5:13: 5:29
27       bb1: {
27       bb1: {
28 -         _1 = move (_3.0: u8);            // scope 0 at $DIR/indirect.rs:5:13: 5:29
29 +         _1 = const 3_u8;                 // scope 0 at $DIR/indirect.rs:5:13: 5:29
-           StorageDead(_2);                 // scope 0 at $DIR/indirect.rs:5:28: 5:29
31           _0 = const ();                   // scope 0 at $DIR/indirect.rs:4:11: 6:2
-           StorageDead(_1);                 // scope 0 at $DIR/indirect.rs:6:1: 6:2
33           return;                          // scope 0 at $DIR/indirect.rs:6:2: 6:2
35   }


thread '[mir-opt] mir-opt/const_prop/indirect.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/indirect.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3452:25
---- [mir-opt] mir-opt/const_prop/control-flow-simplification.rs stdout ----
---- [mir-opt] mir-opt/const_prop/control-flow-simplification.rs stdout ----
7       let mut _2: !;                       // in scope 0 at $SRC_DIR/std/src/macros.rs:LL:COL
9       bb0: {
9       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/control-flow-simplification.rs:12:8: 12:21
11 -         _1 = const <bool as NeedsDrop>::NEEDS; // scope 0 at $DIR/control-flow-simplification.rs:12:8: 12:21
12 -         switchInt(_1) -> [false: bb1, otherwise: bb2]; // scope 0 at $DIR/control-flow-simplification.rs:12:5: 14:6
13 +         _1 = const false;                // scope 0 at $DIR/control-flow-simplification.rs:12:8: 12:21
16   
17       bb1: {
17       bb1: {
18           _0 = const ();                   // scope 0 at $DIR/control-flow-simplification.rs:14:6: 14:6
-           StorageDead(_1);                 // scope 0 at $DIR/control-flow-simplification.rs:15:1: 15:2
20           return;                          // scope 0 at $DIR/control-flow-simplification.rs:15:2: 15:2
22   

23       bb2: {
23       bb2: {
-           StorageLive(_2);                 // scope 0 at $SRC_DIR/std/src/macros.rs:LL:COL
25           begin_panic::<&str>(const "explicit panic"); // scope 0 at $SRC_DIR/std/src/macros.rs:LL:COL
26                                            // mir::Constant
27                                            // + span: $SRC_DIR/std/src/macros.rs:LL:COL

thread '[mir-opt] mir-opt/const_prop/control-flow-simplification.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/control_flow_simplification.hello.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3452:25
---- [mir-opt] mir-opt/const_prop/issue-66971.rs stdout ----
---- [mir-opt] mir-opt/const_prop/issue-66971.rs stdout ----
8       let mut _3: ();                      // in scope 0 at $DIR/issue-66971.rs:16:13: 16:15
10       bb0: {
10       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/issue-66971.rs:16:5: 16:23
-           StorageLive(_2);                 // scope 0 at $DIR/issue-66971.rs:16:12: 16:22
-           StorageLive(_3);                 // scope 0 at $DIR/issue-66971.rs:16:13: 16:15
14 -         (_2.0: ()) = move _3;            // scope 0 at $DIR/issue-66971.rs:16:12: 16:22
15 +         (_2.0: ()) = const ();           // scope 0 at $DIR/issue-66971.rs:16:12: 16:22
16           (_2.1: u8) = const 0_u8;         // scope 0 at $DIR/issue-66971.rs:16:12: 16:22

17           (_2.2: u8) = const 0_u8;         // scope 0 at $DIR/issue-66971.rs:16:12: 16:22
-           StorageDead(_3);                 // scope 0 at $DIR/issue-66971.rs:16:21: 16:22
19           _1 = encode(move _2) -> bb1;     // scope 0 at $DIR/issue-66971.rs:16:5: 16:23
20                                            // mir::Constant
21                                            // + span: $DIR/issue-66971.rs:16:5: 16:11
23       }
24   
25       bb1: {
25       bb1: {
-           StorageDead(_2);                 // scope 0 at $DIR/issue-66971.rs:16:22: 16:23
-           StorageDead(_1);                 // scope 0 at $DIR/issue-66971.rs:16:23: 16:24
28           _0 = const ();                   // scope 0 at $DIR/issue-66971.rs:15:11: 17:2
29           return;                          // scope 0 at $DIR/issue-66971.rs:17:2: 17:2


thread '[mir-opt] mir-opt/const_prop/issue-66971.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/issue_66971.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3452:25
---- [mir-opt] mir-opt/combine_array_len.rs stdout ----
26       }
27   
28       bb0: {
28       bb0: {
-           StorageLive(_2);                 // scope 0 at $DIR/combine_array_len.rs:5:9: 5:10
-           StorageLive(_3);                 // scope 0 at $DIR/combine_array_len.rs:5:15: 5:16
31           _3 = const 0_usize;              // scope 0 at $DIR/combine_array_len.rs:5:15: 5:16
32 -         _4 = Len(_1);                    // scope 0 at $DIR/combine_array_len.rs:5:13: 5:17
33 +         _4 = const 2_usize;              // scope 0 at $DIR/combine_array_len.rs:5:13: 5:17
37   
38       bb1: {
38       bb1: {
39           _2 = _1[_3];                     // scope 0 at $DIR/combine_array_len.rs:5:13: 5:17
-           StorageDead(_3);                 // scope 0 at $DIR/combine_array_len.rs:5:17: 5:18
-           StorageLive(_6);                 // scope 1 at $DIR/combine_array_len.rs:6:9: 6:10
-           StorageLive(_7);                 // scope 1 at $DIR/combine_array_len.rs:6:15: 6:16
43           _7 = const 1_usize;              // scope 1 at $DIR/combine_array_len.rs:6:15: 6:16
44 -         _8 = Len(_1);                    // scope 1 at $DIR/combine_array_len.rs:6:13: 6:17
45 +         _8 = const 2_usize;              // scope 1 at $DIR/combine_array_len.rs:6:13: 6:17
49   
50       bb2: {
50       bb2: {
51           _6 = _1[_7];                     // scope 1 at $DIR/combine_array_len.rs:6:13: 6:17
-           StorageDead(_7);                 // scope 1 at $DIR/combine_array_len.rs:6:17: 6:18
-           StorageLive(_10);                // scope 2 at $DIR/combine_array_len.rs:7:5: 7:8
-           StorageLive(_11);                // scope 2 at $DIR/combine_array_len.rs:7:5: 7:6
55           _11 = _2;                        // scope 2 at $DIR/combine_array_len.rs:7:5: 7:6
-           StorageLive(_12);                // scope 2 at $DIR/combine_array_len.rs:7:7: 7:8
57           _12 = _2;                        // scope 2 at $DIR/combine_array_len.rs:7:7: 7:8
58           _10 = Mul(move _11, move _12);   // scope 2 at $DIR/combine_array_len.rs:7:5: 7:8
-           StorageDead(_12);                // scope 2 at $DIR/combine_array_len.rs:7:7: 7:8
-           StorageDead(_11);                // scope 2 at $DIR/combine_array_len.rs:7:7: 7:8
-           StorageLive(_13);                // scope 2 at $DIR/combine_array_len.rs:7:11: 7:14
-           StorageLive(_14);                // scope 2 at $DIR/combine_array_len.rs:7:11: 7:12
63           _14 = _6;                        // scope 2 at $DIR/combine_array_len.rs:7:11: 7:12
-           StorageLive(_15);                // scope 2 at $DIR/combine_array_len.rs:7:13: 7:14
65           _15 = _6;                        // scope 2 at $DIR/combine_array_len.rs:7:13: 7:14
66           _13 = Mul(move _14, move _15);   // scope 2 at $DIR/combine_array_len.rs:7:11: 7:14
-           StorageDead(_15);                // scope 2 at $DIR/combine_array_len.rs:7:13: 7:14
-           StorageDead(_14);                // scope 2 at $DIR/combine_array_len.rs:7:13: 7:14
69           _0 = Add(move _10, move _13);    // scope 2 at $DIR/combine_array_len.rs:7:5: 7:14
-           StorageDead(_13);                // scope 2 at $DIR/combine_array_len.rs:7:13: 7:14
-           StorageDead(_10);                // scope 2 at $DIR/combine_array_len.rs:7:13: 7:14
-           StorageDead(_6);                 // scope 1 at $DIR/combine_array_len.rs:8:1: 8:2
-           StorageDead(_2);                 // scope 0 at $DIR/combine_array_len.rs:8:1: 8:2
74           return;                          // scope 0 at $DIR/combine_array_len.rs:8:2: 8:2
76   }


thread '[mir-opt] mir-opt/combine_array_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/combine_array_len.norm2.InstCombine.64bit.diff', src/tools/compiletest/src/runtest.rs:3452:25
---- [mir-opt] mir-opt/const_prop/large_array_index.rs stdout ----
13       }
14   
15       bb0: {
15       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/large_array_index.rs:6:9: 6:10
-           StorageLive(_2);                 // scope 0 at $DIR/large_array_index.rs:6:17: 6:29
18           _2 = [const 0_u8; 5000];         // scope 0 at $DIR/large_array_index.rs:6:17: 6:29
-           StorageLive(_3);                 // scope 0 at $DIR/large_array_index.rs:6:30: 6:31
20           _3 = const 2_usize;              // scope 0 at $DIR/large_array_index.rs:6:30: 6:31
21           _4 = const 5000_usize;           // scope 0 at $DIR/large_array_index.rs:6:17: 6:32
22 -         _5 = Lt(_3, _4);                 // scope 0 at $DIR/large_array_index.rs:6:17: 6:32
27   
28       bb1: {
28       bb1: {
29           _1 = _2[_3];                     // scope 0 at $DIR/large_array_index.rs:6:17: 6:32
-           StorageDead(_3);                 // scope 0 at $DIR/large_array_index.rs:6:32: 6:33
-           StorageDead(_2);                 // scope 0 at $DIR/large_array_index.rs:6:32: 6:33
32           _0 = const ();                   // scope 0 at $DIR/large_array_index.rs:4:11: 7:2
-           StorageDead(_1);                 // scope 0 at $DIR/large_array_index.rs:7:1: 7:2
34           return;                          // scope 0 at $DIR/large_array_index.rs:7:2: 7:2
36   }


thread '[mir-opt] mir-opt/const_prop/large_array_index.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/large_array_index.main.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3452:25
---- [mir-opt] mir-opt/const_prop/issue-67019.rs stdout ----
---- [mir-opt] mir-opt/const_prop/issue-67019.rs stdout ----
8       let mut _3: (u8, u8);                // in scope 0 at $DIR/issue-67019.rs:11:11: 11:17
10       bb0: {
10       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/issue-67019.rs:11:5: 11:20
-           StorageLive(_2);                 // scope 0 at $DIR/issue-67019.rs:11:10: 11:19
-           StorageLive(_3);                 // scope 0 at $DIR/issue-67019.rs:11:11: 11:17
14           (_3.0: u8) = const 1_u8;         // scope 0 at $DIR/issue-67019.rs:11:11: 11:17
15           (_3.1: u8) = const 2_u8;         // scope 0 at $DIR/issue-67019.rs:11:11: 11:17
16 -         (_2.0: (u8, u8)) = move _3;      // scope 0 at $DIR/issue-67019.rs:11:10: 11:19
18 +                                          // mir::Constant
18 +                                          // mir::Constant
19 +                                          // + span: $DIR/issue-67019.rs:11:10: 11:19
20 +                                          // + literal: Const { ty: (u8, u8), val: Value(ByRef { alloc: Allocation { bytes: [1, 2], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [3], len: Size { raw: 2 } }, size: Size { raw: 2 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
-           StorageDead(_3);                 // scope 0 at $DIR/issue-67019.rs:11:18: 11:19
22           _1 = test(move _2) -> bb1;       // scope 0 at $DIR/issue-67019.rs:11:5: 11:20
23                                            // mir::Constant
24                                            // + span: $DIR/issue-67019.rs:11:5: 11:9
26       }
27   
28       bb1: {
28       bb1: {
-           StorageDead(_2);                 // scope 0 at $DIR/issue-67019.rs:11:19: 11:20
-           StorageDead(_1);                 // scope 0 at $DIR/issue-67019.rs:11:20: 11:21
31           _0 = const ();                   // scope 0 at $DIR/issue-67019.rs:10:11: 12:2
32           return;                          // scope 0 at $DIR/issue-67019.rs:12:2: 12:2


thread '[mir-opt] mir-opt/const_prop/issue-67019.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/issue_67019.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3452:25
---- [mir-opt] mir-opt/const_prop/ref_deref.rs stdout ----
---- [mir-opt] mir-opt/const_prop/ref_deref.rs stdout ----
9       let mut _4: &i32;                    // in scope 0 at $DIR/ref_deref.rs:5:6: 5:10
---
test result: FAILED. 80 passed; 69 failed; 3 ignored; 0 measured; 0 filtered out; finished in 3.47s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "11.0.1-rust-1.51.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:23:45
