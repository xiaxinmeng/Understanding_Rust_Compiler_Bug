plain
.................................................................................................... 9500/11877
.................................................................................................... 9600/11877
..............................................................................i......i.............. 9700/11877
.................................................................................................... 9800/11877
.......................iiiiiii..iiiiii.i............................................................ 9900/11877
.................................................................................................... 10100/11877
.................................................................................................... 10200/11877
.................................................................................................... 10300/11877
.................................................................................................... 10400/11877
---
Suite("src/test/mir-opt") not skipped for "bootstrap::test::MirOpt" -- not in ["src/tools/tidy"]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 160 tests
..........F..........F.F..F.F...F.i...F.......FF...............................i.....F..F........... 100/160
..F.F........F.......F.....i.....................F.........F
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] mir-opt/const_debuginfo.rs stdout ----
---- [mir-opt] mir-opt/const_debuginfo.rs stdout ----
77           _9 = const "hello, world!";      // scope 4 at $DIR/const_debuginfo.rs:14:13: 14:28
78                                            // ty::Const
79                                            // + ty: &str
-                                            // + val: Value(Slice { data: Allocation { bytes: [104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [8191], len: Size { raw: 13 } }, size: Size { raw: 13 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 13 })
+                                            // + val: Value(Slice { data: Allocation { bytes: [104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [8191], len: Size { raw: 13 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 13 })
81                                            // mir::Constant
82                                            // + span: $DIR/const_debuginfo.rs:14:13: 14:28
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [8191], len: Size { raw: 13 } }, size: Size { raw: 13 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 13 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [8191], len: Size { raw: 13 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 13 }) }
84           StorageLive(_10);                // scope 5 at $DIR/const_debuginfo.rs:16:9: 16:10
85           (_10.0: bool) = const true;      // scope 5 at $DIR/const_debuginfo.rs:16:13: 16:34
86           (_10.1: bool) = const false;     // scope 5 at $DIR/const_debuginfo.rs:16:13: 16:34

thread '[mir-opt] mir-opt/const_debuginfo.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_debuginfo.main.ConstDebugInfo.diff', src/tools/compiletest/src/runtest.rs:3554:25

---- [mir-opt] mir-opt/const_prop/checked_add.rs stdout ----
---- [mir-opt] mir-opt/const_prop/checked_add.rs stdout ----
16 +         _2 = const (2_u32, false);       // scope 0 at $DIR/checked_add.rs:5:18: 5:23
17 +                                          // mir::Constant
18 +                                          // + span: $DIR/checked_add.rs:5:18: 5:23
- +                                          // + literal: Const { ty: (u32, bool), val: Value(ByRef { alloc: Allocation { bytes: [2, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [31], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
+ +                                          // + literal: Const { ty: (u32, bool), val: Value(ByRef { alloc: Allocation { bytes: [2, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [31], len: Size { raw: 8 } }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
20 +         assert(!const false, "attempt to compute `{} + {}`, which would overflow", const 1_u32, const 1_u32) -> bb1; // scope 0 at $DIR/checked_add.rs:5:18: 5:23
22   


thread '[mir-opt] mir-opt/const_prop/checked_add.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/checked_add.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/const_prop/indirect.rs stdout ----
---- [mir-opt] mir-opt/const_prop/indirect.rs stdout ----
20 +         _3 = const (3_u8, false);        // scope 0 at $DIR/indirect.rs:5:13: 5:29
21 +                                          // mir::Constant
22 +                                          // + span: $DIR/indirect.rs:5:13: 5:29
- +                                          // + literal: Const { ty: (u8, bool), val: Value(ByRef { alloc: Allocation { bytes: [3, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [3], len: Size { raw: 2 } }, size: Size { raw: 2 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
+ +                                          // + literal: Const { ty: (u8, bool), val: Value(ByRef { alloc: Allocation { bytes: [3, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [3], len: Size { raw: 2 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
24 +         assert(!const false, "attempt to compute `{} + {}`, which would overflow", const 2_u8, const 1_u8) -> bb1; // scope 0 at $DIR/indirect.rs:5:13: 5:29
26   


thread '[mir-opt] mir-opt/const_prop/indirect.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/indirect.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/const_prop/control-flow-simplification.rs stdout ----
---- [mir-opt] mir-opt/const_prop/control-flow-simplification.rs stdout ----
22                                            // + literal: Const { ty: fn(&str) -> ! {std::rt::begin_panic::<&str>}, val: Value(Scalar(<ZST>)) }
23                                            // ty::Const
24                                            // + ty: &str
-                                            // + val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 })
+                                            // + val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 })
26                                            // mir::Constant
27                                            // + span: $SRC_DIR/std/src/panic.rs:LL:COL
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
30   
31       bb2: {


thread '[mir-opt] mir-opt/const_prop/control-flow-simplification.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/control_flow_simplification.hello.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/const_prop/mutable_variable_aggregate.rs stdout ----
---- [mir-opt] mir-opt/const_prop/mutable_variable_aggregate.rs stdout ----
22 +         _2 = const (42_i32, 99_i32);     // scope 1 at $DIR/mutable_variable_aggregate.rs:7:13: 7:14
23 +                                          // mir::Constant
24 +                                          // + span: $DIR/mutable_variable_aggregate.rs:7:13: 7:14
- +                                          // + literal: Const { ty: (i32, i32), val: Value(ByRef { alloc: Allocation { bytes: [42, 0, 0, 0, 99, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
+ +                                          // + literal: Const { ty: (i32, i32), val: Value(ByRef { alloc: Allocation { bytes: [42, 0, 0, 0, 99, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
26           nop;                             // scope 0 at $DIR/mutable_variable_aggregate.rs:4:11: 8:2
27           StorageDead(_2);                 // scope 1 at $DIR/mutable_variable_aggregate.rs:8:1: 8:2
28           StorageDead(_1);                 // scope 0 at $DIR/mutable_variable_aggregate.rs:8:1: 8:2

thread '[mir-opt] mir-opt/const_prop/mutable_variable_aggregate.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/mutable_variable_aggregate.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/const_prop/issue-67019.rs stdout ----
---- [mir-opt] mir-opt/const_prop/issue-67019.rs stdout ----
17 +         (_2.0: (u8, u8)) = const (1_u8, 2_u8); // scope 0 at $DIR/issue-67019.rs:11:10: 11:19
18 +                                          // mir::Constant
19 +                                          // + span: $DIR/issue-67019.rs:11:10: 11:19
- +                                          // + literal: Const { ty: (u8, u8), val: Value(ByRef { alloc: Allocation { bytes: [1, 2], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [3], len: Size { raw: 2 } }, size: Size { raw: 2 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
+ +                                          // + literal: Const { ty: (u8, u8), val: Value(ByRef { alloc: Allocation { bytes: [1, 2], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [3], len: Size { raw: 2 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
21           StorageDead(_3);                 // scope 0 at $DIR/issue-67019.rs:11:18: 11:19
22           _1 = test(move _2) -> bb1;       // scope 0 at $DIR/issue-67019.rs:11:5: 11:20
23                                            // mir::Constant

thread '[mir-opt] mir-opt/const_prop/issue-67019.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/issue_67019.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs stdout ----
---- [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs stdout ----
29 +         _2 = const (4_i32, false);       // scope 0 at $DIR/optimizes_into_variable.rs:12:13: 12:18
30 +                                          // mir::Constant
31 +                                          // + span: $DIR/optimizes_into_variable.rs:12:13: 12:18
- +                                          // + literal: Const { ty: (i32, bool), val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [31], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
+ +                                          // + literal: Const { ty: (i32, bool), val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [31], len: Size { raw: 8 } }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
33 +         assert(!const false, "attempt to compute `{} + {}`, which would overflow", const 2_i32, const 2_i32) -> bb1; // scope 0 at $DIR/optimizes_into_variable.rs:12:13: 12:18
35   


thread '[mir-opt] mir-opt/const_prop/optimizes_into_variable.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/optimizes_into_variable.main.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/const_prop/return_place.rs stdout ----
---- [mir-opt] mir-opt/const_prop/return_place.rs stdout ----
11 +         _1 = const (4_u32, false);       // scope 0 at $DIR/return_place.rs:6:5: 6:10
12 +                                          // mir::Constant
13 +                                          // + span: $DIR/return_place.rs:6:5: 6:10
- +                                          // + literal: Const { ty: (u32, bool), val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [31], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
+ +                                          // + literal: Const { ty: (u32, bool), val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [31], len: Size { raw: 8 } }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
15 +         assert(!const false, "attempt to compute `{} + {}`, which would overflow", const 2_u32, const 2_u32) -> bb1; // scope 0 at $DIR/return_place.rs:6:5: 6:10
17   


thread '[mir-opt] mir-opt/const_prop/return_place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/return_place.add.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/const_prop/tuple_literal_propagation.rs stdout ----
---- [mir-opt] mir-opt/const_prop/tuple_literal_propagation.rs stdout ----
20 +         _3 = const (1_u32, 2_u32);       // scope 1 at $DIR/tuple_literal_propagation.rs:5:13: 5:14
21 +                                          // mir::Constant
22 +                                          // + span: $DIR/tuple_literal_propagation.rs:5:13: 5:14
- +                                          // + literal: Const { ty: (u32, u32), val: Value(ByRef { alloc: Allocation { bytes: [1, 0, 0, 0, 2, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
+ +                                          // + literal: Const { ty: (u32, u32), val: Value(ByRef { alloc: Allocation { bytes: [1, 0, 0, 0, 2, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
24           _2 = consume(move _3) -> bb1;    // scope 1 at $DIR/tuple_literal_propagation.rs:5:5: 5:15
25                                            // mir::Constant
26                                            // + span: $DIR/tuple_literal_propagation.rs:5:5: 5:12

thread '[mir-opt] mir-opt/const_prop/tuple_literal_propagation.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/tuple_literal_propagation.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/inline/inline-diverging.rs stdout ----
---- [mir-opt] mir-opt/inline/inline-diverging.rs stdout ----
43 +                                          // + literal: Const { ty: fn(&str) -> ! {std::rt::begin_panic::<&str>}, val: Value(Scalar(<ZST>)) }
44 +                                          // ty::Const
45 +                                          // + ty: &str
- +                                          // + val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 })
+ +                                          // + val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 })
47 +                                          // mir::Constant
48 +                                          // + span: $DIR/inline-diverging.rs:16:9: 16:16
- +                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
+ +                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
51   }
52   


thread '[mir-opt] mir-opt/inline/inline-diverging.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_diverging.g.Inline.diff', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
23 +         ((*_4).0: alloc::raw_vec::RawVec<u32>) = const alloc::raw_vec::RawVec::<u32> { ptr: Unique::<u32> { pointer: {0x4 as *const u32}, _marker: PhantomData::<u32> }, cap: 0_usize, alloc: std::alloc::Global }; // scope 2 at $DIR/inline-into-box-place.rs:8:33: 8:43
24 +                                          // ty::Const
25 +                                          // + ty: alloc::raw_vec::RawVec<u32>
- +                                          // + val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [65535], len: Size { raw: 16 } }, size: Size { raw: 16 }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } })
+ +                                          // + val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [65535], len: Size { raw: 16 } }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } })
27                                            // mir::Constant
28 -                                          // + span: $DIR/inline-into-box-place.rs:8:33: 8:41
29 -                                          // + user_ty: UserType(1)
33 -     bb1: {
33 -     bb1: {
34 +                                          // + span: $DIR/inline-into-box-place.rs:8:33: 8:43
35 +                                          // + user_ty: UserType(0)
- +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [65535], len: Size { raw: 16 } }, size: Size { raw: 16 }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
+ +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [65535], len: Size { raw: 16 } }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
37 +         ((*_4).1: usize) = const 0_usize; // scope 2 at $DIR/inline-into-box-place.rs:8:33: 8:43
38 +         StorageDead(_4);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
39           _1 = move _2;                    // scope 0 at $DIR/inline-into-box-place.rs:8:29: 8:43

thread '[mir-opt] mir-opt/inline/inline-into-box-place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_into_box_place.main.Inline.64bit.diff', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/issue_76432.rs stdout ----
---- [mir-opt] mir-opt/issue_76432.rs stdout ----
71                                            // + literal: Const { ty: fn(&'static str) -> ! {core::panicking::panic}, val: Value(Scalar(<ZST>)) }
72                                            // ty::Const
73                                            // + ty: &str
-                                            // + val: Value(Slice { data: Allocation { bytes: [105, 110, 116, 101, 114, 110, 97, 108, 32, 101, 114, 114, 111, 114, 58, 32, 101, 110, 116, 101, 114, 101, 100, 32, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 32, 99, 111, 100, 101], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1099511627775], len: Size { raw: 40 } }, size: Size { raw: 40 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 40 })
+                                            // + val: Value(Slice { data: Allocation { bytes: [105, 110, 116, 101, 114, 110, 97, 108, 32, 101, 114, 114, 111, 114, 58, 32, 101, 110, 116, 101, 114, 101, 100, 32, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 32, 99, 111, 100, 101], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1099511627775], len: Size { raw: 40 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 40 })
75                                            // mir::Constant
76                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [105, 110, 116, 101, 114, 110, 97, 108, 32, 101, 114, 114, 111, 114, 58, 32, 101, 110, 116, 101, 114, 101, 100, 32, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 32, 99, 111, 100, 101], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1099511627775], len: Size { raw: 40 } }, size: Size { raw: 40 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 40 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [105, 110, 116, 101, 114, 110, 97, 108, 32, 101, 114, 114, 111, 114, 58, 32, 101, 110, 116, 101, 114, 101, 100, 32, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 32, 99, 111, 100, 101], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1099511627775], len: Size { raw: 40 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 40 }) }
79   
80       bb2: {


thread '[mir-opt] mir-opt/issue_76432.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_76432.test.SimplifyComparisonIntegral.diff', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/issues/issue-59352.rs stdout ----
---- [mir-opt] mir-opt/issues/issue-59352.rs stdout ----
92                                          // + literal: Const { ty: fn(&'static str) -> ! {core::panicking::panic}, val: Value(Scalar(<ZST>)) }
93                                          // ty::Const
94                                          // + ty: &str
-                                          // + val: Value(Slice { data: Allocation { bytes: [99, 97, 108, 108, 101, 100, 32, 96, 79, 112, 116, 105, 111, 110, 58, 58, 117, 110, 119, 114, 97, 112, 40, 41, 96, 32, 111, 110, 32, 97, 32, 96, 78, 111, 110, 101, 96, 32, 118, 97, 108, 117, 101], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [8796093022207], len: Size { raw: 43 } }, size: Size { raw: 43 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 43 })
+                                          // + val: Value(Slice { data: Allocation { bytes: [99, 97, 108, 108, 101, 100, 32, 96, 79, 112, 116, 105, 111, 110, 58, 58, 117, 110, 119, 114, 97, 112, 40, 41, 96, 32, 111, 110, 32, 97, 32, 96, 78, 111, 110, 101, 96, 32, 118, 97, 108, 117, 101], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [8796093022207], len: Size { raw: 43 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 43 })
96                                          // mir::Constant
97                                          // + span: $DIR/issue-59352.rs:14:26: 14:50
-                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [99, 97, 108, 108, 101, 100, 32, 96, 79, 112, 116, 105, 111, 110, 58, 58, 117, 110, 119, 114, 97, 112, 40, 41, 96, 32, 111, 110, 32, 97, 32, 96, 78, 111, 110, 101, 96, 32, 118, 97, 108, 117, 101], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [8796093022207], len: Size { raw: 43 } }, size: Size { raw: 43 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 43 }) }
+                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [99, 97, 108, 108, 101, 100, 32, 96, 79, 112, 116, 105, 111, 110, 58, 58, 117, 110, 119, 114, 97, 112, 40, 41, 96, 32, 111, 110, 32, 97, 32, 96, 78, 111, 110, 101, 96, 32, 118, 97, 108, 117, 101], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [8796093022207], len: Size { raw: 43 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 43 }) }
100 
101     bb7: {


thread '[mir-opt] mir-opt/issues/issue-59352.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issues/issue_59352.num_to_digit.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/no-drop-for-inactive-variant.rs stdout ----
---- [mir-opt] mir-opt/no-drop-for-inactive-variant.rs stdout ----
26                                          // + literal: Const { ty: fn(&str) -> ! {std::rt::begin_panic::<&str>}, val: Value(Scalar(<ZST>)) }
27                                          // ty::Const
28                                          // + ty: &str
-                                          // + val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 })
+                                          // + val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 })
30                                          // mir::Constant
31                                          // + span: $SRC_DIR/std/src/panic.rs:LL:COL
-                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
+                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
34 
35     bb2: {


thread '[mir-opt] mir-opt/no-drop-for-inactive-variant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/no_drop_for_inactive_variant.unwrap.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/no-spurious-drop-after-call.rs stdout ----
---- [mir-opt] mir-opt/no-spurious-drop-after-call.rs stdout ----
15         _4 = const "";                   // scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:22
16                                          // ty::Const
17                                          // + ty: &str
-                                          // + val: Value(Slice { data: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [], len: Size { raw: 0 } }, size: Size { raw: 0 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 0 })
+                                          // + val: Value(Slice { data: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [], len: Size { raw: 0 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 0 })
19                                          // mir::Constant
20                                          // + span: $DIR/no-spurious-drop-after-call.rs:9:20: 9:22
-                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [], len: Size { raw: 0 } }, size: Size { raw: 0 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 0 }) }
+                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [], len: Size { raw: 0 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 0 }) }
22         _3 = &(*_4);                     // scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:22
23         _2 = <str as ToString>::to_string(move _3) -> bb1; // scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:34
24                                          // mir::Constant

thread '[mir-opt] mir-opt/no-spurious-drop-after-call.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/no_spurious_drop_after_call.main.ElaborateDrops.before.mir', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/uninhabited_enum_branching.rs stdout ----
---- [mir-opt] mir-opt/uninhabited_enum_branching.rs stdout ----
27           _5 = const "C";                  // scope 0 at $DIR/uninhabited_enum_branching.rs:23:21: 23:24
28                                            // ty::Const
29                                            // + ty: &str
-                                            // + val: Value(Slice { data: Allocation { bytes: [67], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 })
+                                            // + val: Value(Slice { data: Allocation { bytes: [67], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 })
31                                            // mir::Constant
32                                            // + span: $DIR/uninhabited_enum_branching.rs:23:21: 23:24
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [67], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [67], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
34           _1 = &(*_5);                     // scope 0 at $DIR/uninhabited_enum_branching.rs:23:21: 23:24
35           StorageDead(_5);                 // scope 0 at $DIR/uninhabited_enum_branching.rs:23:23: 23:24
36           goto -> bb4;                     // scope 0 at $DIR/uninhabited_enum_branching.rs:20:5: 24:6

40           _1 = const "A(Empty)";           // scope 0 at $DIR/uninhabited_enum_branching.rs:21:24: 21:34
41                                            // ty::Const
42                                            // + ty: &str
-                                            // + val: Value(Slice { data: Allocation { bytes: [65, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 })
+                                            // + val: Value(Slice { data: Allocation { bytes: [65, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 })
44                                            // mir::Constant
45                                            // + span: $DIR/uninhabited_enum_branching.rs:21:24: 21:34
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [65, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [65, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 }) }
47           goto -> bb4;                     // scope 0 at $DIR/uninhabited_enum_branching.rs:20:5: 24:6
49   


52           _4 = const "B(Empty)";           // scope 0 at $DIR/uninhabited_enum_branching.rs:22:24: 22:34
53                                            // ty::Const
54                                            // + ty: &str
-                                            // + val: Value(Slice { data: Allocation { bytes: [66, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 })
+                                            // + val: Value(Slice { data: Allocation { bytes: [66, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 })
56                                            // mir::Constant
57                                            // + span: $DIR/uninhabited_enum_branching.rs:22:24: 22:34
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [66, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [66, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 }) }
59           _1 = &(*_4);                     // scope 0 at $DIR/uninhabited_enum_branching.rs:22:24: 22:34
60           StorageDead(_4);                 // scope 0 at $DIR/uninhabited_enum_branching.rs:22:33: 22:34
61           goto -> bb4;                     // scope 0 at $DIR/uninhabited_enum_branching.rs:20:5: 24:6

76           _9 = const "E";                  // scope 0 at $DIR/uninhabited_enum_branching.rs:28:21: 28:24
77                                            // ty::Const
78                                            // + ty: &str
-                                            // + val: Value(Slice { data: Allocation { bytes: [69], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 })
+                                            // + val: Value(Slice { data: Allocation { bytes: [69], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 })
80                                            // mir::Constant
81                                            // + span: $DIR/uninhabited_enum_branching.rs:28:21: 28:24
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [69], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [69], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
83           _6 = &(*_9);                     // scope 0 at $DIR/uninhabited_enum_branching.rs:28:21: 28:24
84           StorageDead(_9);                 // scope 0 at $DIR/uninhabited_enum_branching.rs:28:23: 28:24
85           goto -> bb7;                     // scope 0 at $DIR/uninhabited_enum_branching.rs:26:5: 29:6

89           _6 = const "D";                  // scope 0 at $DIR/uninhabited_enum_branching.rs:27:21: 27:24
90                                            // ty::Const
91                                            // + ty: &str
-                                            // + val: Value(Slice { data: Allocation { bytes: [68], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 })
+                                            // + val: Value(Slice { data: Allocation { bytes: [68], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 })
93                                            // mir::Constant
94                                            // + span: $DIR/uninhabited_enum_branching.rs:27:21: 27:24
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [68], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [68], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
96           goto -> bb7;                     // scope 0 at $DIR/uninhabited_enum_branching.rs:26:5: 29:6
98   


thread '[mir-opt] mir-opt/uninhabited_enum_branching.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/uninhabited_enum_branching.main.UninhabitedEnumBranching.diff', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/storage_live_dead_in_statics.rs stdout ----
---- [mir-opt] mir-opt/storage_live_dead_in_statics.rs stdout ----
192         _2 = Foo { tup: const "hi", data: move _3 }; // scope 0 at $DIR/storage_live_dead_in_statics.rs:5:29: 23:2
193                                          // ty::Const
194                                          // + ty: &str
-                                          // + val: Value(Slice { data: Allocation { bytes: [104, 105], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [3], len: Size { raw: 2 } }, size: Size { raw: 2 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 2 })
+                                          // + val: Value(Slice { data: Allocation { bytes: [104, 105], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [3], len: Size { raw: 2 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 2 })
196                                          // mir::Constant
197                                          // + span: $DIR/storage_live_dead_in_statics.rs:6:10: 6:14
-                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [104, 105], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [3], len: Size { raw: 2 } }, size: Size { raw: 2 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 2 }) }
+                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [104, 105], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [3], len: Size { raw: 2 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 2 }) }
199         StorageDead(_3);                 // scope 0 at $DIR/storage_live_dead_in_statics.rs:23:1: 23:2
200         _1 = &_2;                        // scope 0 at $DIR/storage_live_dead_in_statics.rs:5:28: 23:2
201         _0 = &(*_1);                     // scope 0 at $DIR/storage_live_dead_in_statics.rs:5:28: 23:2

thread '[mir-opt] mir-opt/storage_live_dead_in_statics.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/storage_live_dead_in_statics.XXX.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3554:25

failures:
    [mir-opt] mir-opt/const_debuginfo.rs
    [mir-opt] mir-opt/const_prop/checked_add.rs
---
test result: FAILED. 140 passed; 17 failed; 3 ignored; 0 measured; 0 filtered out; finished in 2.10s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:17
