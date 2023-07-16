plain
 finished in 0.596 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 173 tests
.....F.......................F..........i.............................................F. 88/173
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.i..........F..........F..F.........F..F............i......................FF.......F

---- [mir-opt] src/test/mir-opt/const_debuginfo.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_debuginfo.rs stdout ----
77           _9 = const "hello, world!";      // scope 4 at $DIR/const_debuginfo.rs:14:13: 14:28
78                                            // mir::Constant
79                                            // + span: $DIR/const_debuginfo.rs:14:13: 14:28
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [8191], len: Size { raw: 13 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 13 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [8191], len: Size::from_bytes(13) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 13 }) }
81           StorageLive(_10);                // scope 5 at $DIR/const_debuginfo.rs:16:9: 16:10
82           Deinit(_10);                     // scope 5 at $DIR/const_debuginfo.rs:16:13: 16:34
83           (_10.0: bool) = const true;      // scope 5 at $DIR/const_debuginfo.rs:16:13: 16:34

thread '[mir-opt] src/test/mir-opt/const_debuginfo.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_debuginfo.main.ConstDebugInfo.diff', src/tools/compiletest/src/runtest.rs:3410:25

---- [mir-opt] src/test/mir-opt/const_prop/control-flow-simplification.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/control-flow-simplification.rs stdout ----
22                                            // + literal: Const { ty: fn(&str) -> ! {begin_panic::<&str>}, val: Value(Scalar(<ZST>)) }
23                                            // mir::Constant
24                                            // + span: $SRC_DIR/std/src/panic.rs:LL:COL
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size::from_bytes(14) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 14 }) }
27   
28       bb2: {


thread '[mir-opt] src/test/mir-opt/const_prop/control-flow-simplification.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/control_flow_simplification.hello.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3410:25
---- [mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs stdout ----
45 -     bb2: {
45 -     bb2: {
46 +                                          // + span: $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
47 +                                          // + user_ty: UserType(0)
- +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [65535], len: Size { raw: 16 } }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
+ +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [65535], len: Size::from_bytes(16) }, align: Align::from_bytes(8), mutability: Not, extra: () }, offset: Size::from_bytes(0) }) }
49 +         ((*_7).1: usize) = const 0_usize; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
50 +         StorageDead(_7);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
51           _1 = move _5;                    // scope 0 at $DIR/inline-into-box-place.rs:8:29: 8:43

thread '[mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_into_box_place.main.Inline.64bit.diff', src/tools/compiletest/src/runtest.rs:3410:25
---- [mir-opt] src/test/mir-opt/inline/inline-diverging.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/inline-diverging.rs stdout ----
43 +                                          // + literal: Const { ty: fn(&str) -> ! {begin_panic::<&str>}, val: Value(Scalar(<ZST>)) }
44 +                                          // mir::Constant
45 +                                          // + span: $SRC_DIR/std/src/panic.rs:LL:COL
- +                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
+ +                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size::from_bytes(14) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 14 }) }
48   }
49   


thread '[mir-opt] src/test/mir-opt/inline/inline-diverging.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_diverging.g.Inline.diff', src/tools/compiletest/src/runtest.rs:3410:25
---- [mir-opt] src/test/mir-opt/issues/issue-59352.rs stdout ----
---- [mir-opt] src/test/mir-opt/issues/issue-59352.rs stdout ----
92                                          // + literal: Const { ty: fn(&'static str) -> ! {core::panicking::panic}, val: Value(Scalar(<ZST>)) }
93                                          // mir::Constant
94                                          // + span: $SRC_DIR/core/src/option.rs:LL:COL
-                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [99, 97, 108, 108, 101, 100, 32, 96, 79, 112, 116, 105, 111, 110, 58, 58, 117, 110, 119, 114, 97, 112, 40, 41, 96, 32, 111, 110, 32, 97, 32, 96, 78, 111, 110, 101, 96, 32, 118, 97, 108, 117, 101], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [8796093022207], len: Size { raw: 43 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 43 }) }
+                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [99, 97, 108, 108, 101, 100, 32, 96, 79, 112, 116, 105, 111, 110, 58, 58, 117, 110, 119, 114, 97, 112, 40, 41, 96, 32, 111, 110, 32, 97, 32, 96, 78, 111, 110, 101, 96, 32, 118, 97, 108, 117, 101], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [8796093022207], len: Size::from_bytes(43) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 43 }) }
97 
98     bb7: {


thread '[mir-opt] src/test/mir-opt/issues/issue-59352.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issues/issue_59352.num_to_digit.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3410:25
---- [mir-opt] src/test/mir-opt/issue_76432.rs stdout ----
---- [mir-opt] src/test/mir-opt/issue_76432.rs stdout ----
73                                            // + literal: Const { ty: fn(&'static str) -> ! {core::panicking::panic}, val: Value(Scalar(<ZST>)) }
74                                            // mir::Constant
75                                            // + span: $SRC_DIR/core/src/panic.rs:LL:COL
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [105, 110, 116, 101, 114, 110, 97, 108, 32, 101, 114, 114, 111, 114, 58, 32, 101, 110, 116, 101, 114, 101, 100, 32, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 32, 99, 111, 100, 101], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1099511627775], len: Size { raw: 40 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 40 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [105, 110, 116, 101, 114, 110, 97, 108, 32, 101, 114, 114, 111, 114, 58, 32, 101, 110, 116, 101, 114, 101, 100, 32, 117, 110, 114, 101, 97, 99, 104, 97, 98, 108, 101, 32, 99, 111, 100, 101], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1099511627775], len: Size::from_bytes(40) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 40 }) }
78   
79       bb2: {


thread '[mir-opt] src/test/mir-opt/issue_76432.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_76432.test.SimplifyComparisonIntegral.diff', src/tools/compiletest/src/runtest.rs:3410:25
---- [mir-opt] src/test/mir-opt/no-drop-for-inactive-variant.rs stdout ----
---- [mir-opt] src/test/mir-opt/no-drop-for-inactive-variant.rs stdout ----
26                                          // + literal: Const { ty: fn(&str) -> ! {begin_panic::<&str>}, val: Value(Scalar(<ZST>)) }
27                                          // mir::Constant
28                                          // + span: $SRC_DIR/std/src/panic.rs:LL:COL
-                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size { raw: 14 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
+                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [16383], len: Size::from_bytes(14) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 14 }) }
31 
32     bb2: {


thread '[mir-opt] src/test/mir-opt/no-drop-for-inactive-variant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/no_drop_for_inactive_variant.unwrap.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3410:25
---- [mir-opt] src/test/mir-opt/no-spurious-drop-after-call.rs stdout ----
---- [mir-opt] src/test/mir-opt/no-spurious-drop-after-call.rs stdout ----
15         _4 = const "";                   // scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:22
16                                          // mir::Constant
17                                          // + span: $DIR/no-spurious-drop-after-call.rs:9:20: 9:22
-                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [], len: Size { raw: 0 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 0 }) }
+                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [], len: Size::from_bytes(0) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 0 }) }
19         _3 = &(*_4);                     // scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:34
20         _2 = <str as ToString>::to_string(move _3) -> bb1; // scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:34
21                                          // mir::Constant

thread '[mir-opt] src/test/mir-opt/no-spurious-drop-after-call.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/no_spurious_drop_after_call.main.ElaborateDrops.before.mir', src/tools/compiletest/src/runtest.rs:3410:25
---- [mir-opt] src/test/mir-opt/uninhabited_enum_branching.rs stdout ----
---- [mir-opt] src/test/mir-opt/uninhabited_enum_branching.rs stdout ----
28           _5 = const "C";                  // scope 0 at $DIR/uninhabited_enum_branching.rs:23:21: 23:24
29                                            // mir::Constant
30                                            // + span: $DIR/uninhabited_enum_branching.rs:23:21: 23:24
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [67], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [67], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size::from_bytes(1) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 1 }) }
32           _1 = &(*_5);                     // scope 0 at $DIR/uninhabited_enum_branching.rs:23:21: 23:24
33           StorageDead(_5);                 // scope 0 at $DIR/uninhabited_enum_branching.rs:23:23: 23:24
34           goto -> bb4;                     // scope 0 at $DIR/uninhabited_enum_branching.rs:23:23: 23:24

38           _1 = const "A(Empty)";           // scope 0 at $DIR/uninhabited_enum_branching.rs:21:24: 21:34
39                                            // mir::Constant
40                                            // + span: $DIR/uninhabited_enum_branching.rs:21:24: 21:34
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [65, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [65, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size::from_bytes(8) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 8 }) }
42           goto -> bb4;                     // scope 0 at $DIR/uninhabited_enum_branching.rs:21:24: 21:34
44   


47           _4 = const "B(Empty)";           // scope 0 at $DIR/uninhabited_enum_branching.rs:22:24: 22:34
48                                            // mir::Constant
49                                            // + span: $DIR/uninhabited_enum_branching.rs:22:24: 22:34
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [66, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [66, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size::from_bytes(8) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 8 }) }
51           _1 = &(*_4);                     // scope 0 at $DIR/uninhabited_enum_branching.rs:22:24: 22:34
52           StorageDead(_4);                 // scope 0 at $DIR/uninhabited_enum_branching.rs:22:33: 22:34
53           goto -> bb4;                     // scope 0 at $DIR/uninhabited_enum_branching.rs:22:33: 22:34

69           _9 = const "E";                  // scope 0 at $DIR/uninhabited_enum_branching.rs:28:21: 28:24
70                                            // mir::Constant
71                                            // + span: $DIR/uninhabited_enum_branching.rs:28:21: 28:24
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [69], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [69], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size::from_bytes(1) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 1 }) }
73           _6 = &(*_9);                     // scope 0 at $DIR/uninhabited_enum_branching.rs:28:21: 28:24
74           StorageDead(_9);                 // scope 0 at $DIR/uninhabited_enum_branching.rs:28:23: 28:24
75           goto -> bb7;                     // scope 0 at $DIR/uninhabited_enum_branching.rs:28:23: 28:24

79           _6 = const "D";                  // scope 0 at $DIR/uninhabited_enum_branching.rs:27:21: 27:24
80                                            // mir::Constant
81                                            // + span: $DIR/uninhabited_enum_branching.rs:27:21: 27:24
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [68], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [68], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size::from_bytes(1) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 1 }) }
83           goto -> bb7;                     // scope 0 at $DIR/uninhabited_enum_branching.rs:27:21: 27:24
85   


thread '[mir-opt] src/test/mir-opt/uninhabited_enum_branching.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/uninhabited_enum_branching.main.UninhabitedEnumBranching.diff', src/tools/compiletest/src/runtest.rs:3410:25
---- [mir-opt] src/test/mir-opt/uninhabited_enum_branching2.rs stdout ----
---- [mir-opt] src/test/mir-opt/uninhabited_enum_branching2.rs stdout ----
42           _8 = const "D";                  // scope 1 at $DIR/uninhabited_enum_branching2.rs:25:21: 25:24
43                                            // mir::Constant
44                                            // + span: $DIR/uninhabited_enum_branching2.rs:25:21: 25:24
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [68], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [68], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size::from_bytes(1) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 1 }) }
46           _3 = &(*_8);                     // scope 1 at $DIR/uninhabited_enum_branching2.rs:25:21: 25:24
47           StorageDead(_8);                 // scope 1 at $DIR/uninhabited_enum_branching2.rs:25:23: 25:24
48           goto -> bb5;                     // scope 1 at $DIR/uninhabited_enum_branching2.rs:25:23: 25:24

52           _3 = const "A(Empty)";           // scope 1 at $DIR/uninhabited_enum_branching2.rs:22:24: 22:34
53                                            // mir::Constant
54                                            // + span: $DIR/uninhabited_enum_branching2.rs:22:24: 22:34
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [65, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [65, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size::from_bytes(8) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 8 }) }
56           goto -> bb5;                     // scope 1 at $DIR/uninhabited_enum_branching2.rs:22:24: 22:34
58   


61           _6 = const "B(Empty)";           // scope 1 at $DIR/uninhabited_enum_branching2.rs:23:24: 23:34
62                                            // mir::Constant
63                                            // + span: $DIR/uninhabited_enum_branching2.rs:23:24: 23:34
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [66, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [66, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size::from_bytes(8) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 8 }) }
65           _3 = &(*_6);                     // scope 1 at $DIR/uninhabited_enum_branching2.rs:23:24: 23:34
66           StorageDead(_6);                 // scope 1 at $DIR/uninhabited_enum_branching2.rs:23:33: 23:34
67           goto -> bb5;                     // scope 1 at $DIR/uninhabited_enum_branching2.rs:23:33: 23:34

72           _7 = const "C";                  // scope 1 at $DIR/uninhabited_enum_branching2.rs:24:21: 24:24
73                                            // mir::Constant
74                                            // + span: $DIR/uninhabited_enum_branching2.rs:24:21: 24:24
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [67], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [67], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size::from_bytes(1) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 1 }) }
76           _3 = &(*_7);                     // scope 1 at $DIR/uninhabited_enum_branching2.rs:24:21: 24:24
77           StorageDead(_7);                 // scope 1 at $DIR/uninhabited_enum_branching2.rs:24:23: 24:24
78           goto -> bb5;                     // scope 1 at $DIR/uninhabited_enum_branching2.rs:24:23: 24:24

92           _13 = const "D";                 // scope 1 at $DIR/uninhabited_enum_branching2.rs:32:21: 32:24
93                                            // mir::Constant
94                                            // + span: $DIR/uninhabited_enum_branching2.rs:32:21: 32:24
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [68], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [68], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size::from_bytes(1) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 1 }) }
96           _9 = &(*_13);                    // scope 1 at $DIR/uninhabited_enum_branching2.rs:32:21: 32:24
97           StorageDead(_13);                // scope 1 at $DIR/uninhabited_enum_branching2.rs:32:23: 32:24
98           goto -> bb10;                    // scope 1 at $DIR/uninhabited_enum_branching2.rs:32:23: 32:24

102           _9 = const "A(Empty)";           // scope 1 at $DIR/uninhabited_enum_branching2.rs:29:24: 29:34
103                                            // mir::Constant
104                                            // + span: $DIR/uninhabited_enum_branching2.rs:29:24: 29:34
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [65, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [65, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size::from_bytes(8) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 8 }) }
106           goto -> bb10;                    // scope 1 at $DIR/uninhabited_enum_branching2.rs:29:24: 29:34
108   


111           _11 = const "B(Empty)";          // scope 1 at $DIR/uninhabited_enum_branching2.rs:30:24: 30:34
112                                            // mir::Constant
113                                            // + span: $DIR/uninhabited_enum_branching2.rs:30:24: 30:34
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [66, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [66, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size::from_bytes(8) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 8 }) }
115           _9 = &(*_11);                    // scope 1 at $DIR/uninhabited_enum_branching2.rs:30:24: 30:34
116           StorageDead(_11);                // scope 1 at $DIR/uninhabited_enum_branching2.rs:30:33: 30:34
117           goto -> bb10;                    // scope 1 at $DIR/uninhabited_enum_branching2.rs:30:33: 30:34

122           _12 = const "C";                 // scope 1 at $DIR/uninhabited_enum_branching2.rs:31:21: 31:24
123                                            // mir::Constant
124                                            // + span: $DIR/uninhabited_enum_branching2.rs:31:21: 31:24
-                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [67], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size { raw: 1 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
+                                            // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [67], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [1], len: Size::from_bytes(1) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 1 }) }
126           _9 = &(*_12);                    // scope 1 at $DIR/uninhabited_enum_branching2.rs:31:21: 31:24
127           StorageDead(_12);                // scope 1 at $DIR/uninhabited_enum_branching2.rs:31:23: 31:24
128           goto -> bb10;                    // scope 1 at $DIR/uninhabited_enum_branching2.rs:31:23: 31:24

thread '[mir-opt] src/test/mir-opt/uninhabited_enum_branching2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/uninhabited_enum_branching2.main.UninhabitedEnumBranching.diff', src/tools/compiletest/src/runtest.rs:3410:25
---- [mir-opt] src/test/mir-opt/storage_live_dead_in_statics.rs stdout ----
---- [mir-opt] src/test/mir-opt/storage_live_dead_in_statics.rs stdout ----
192         _2 = Foo { tup: const "hi", data: move _3 }; // scope 0 at $DIR/storage_live_dead_in_statics.rs:5:29: 23:2
193                                          // mir::Constant
194                                          // + span: $DIR/storage_live_dead_in_statics.rs:6:10: 6:14
-                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [104, 105], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [3], len: Size { raw: 2 } }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 2 }) }
+                                          // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [104, 105], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [3], len: Size::from_bytes(2) }, align: Align::from_bytes(1), mutability: Not, extra: () }, start: 0, end: 2 }) }
196         StorageDead(_3);                 // scope 0 at $DIR/storage_live_dead_in_statics.rs:23:1: 23:2
197         _1 = &_2;                        // scope 0 at $DIR/storage_live_dead_in_statics.rs:5:28: 23:2
198         _0 = &(*_1);                     // scope 0 at $DIR/storage_live_dead_in_statics.rs:5:28: 23:2

thread '[mir-opt] src/test/mir-opt/storage_live_dead_in_statics.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/storage_live_dead_in_statics.XXX.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3410:25

failures:
    [mir-opt] src/test/mir-opt/const_debuginfo.rs
    [mir-opt] src/test/mir-opt/const_prop/control-flow-simplification.rs
