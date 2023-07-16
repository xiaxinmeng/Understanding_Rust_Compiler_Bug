
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 90 tests
................F............................F................F.F.................FF......
failures:

---- [mir-opt] mir-opt/const_prop/control-flow-simplification.rs stdout ----
51                                                 // + literal: Const { ty: fn(&str) -> ! {std::rt::begin_panic::<&str>}, val: Value(Scalar(<ZST>)) }
52                                                 // ty::Const
53                                                 // + ty: &str
-                                                  // + val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 })
+                                                  // + val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 })
55                                                 // mir::Constant
56                                                 // + span: $SRC_DIR/libstd/macros.rs:LL:COL
-                                                  // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
+                                                  // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
58            }
59        }
60        

thread '[mir-opt] mir-opt/const_prop/control-flow-simplification.rs' panicked at 'Actual MIR output differs from expected MIR output /home/hbinadesk/git/rust/src/test/mir-opt/const_prop/control-flow-simplification/rustc.hello.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3165:25
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
24      -                                          // + ty: fn() -> std::vec::Vec<u32> {std::vec::Vec::<u32>::new}
25      -                                          // + val: Value(Scalar(<ZST>))
26      +                                          // + ty: alloc::raw_vec::RawVec<u32>
-       +                                          // + val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [65535], len: Size { raw: 16 } }, size: Size { raw: 16 }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } })
+       +                                          // + val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [65535], len: Size { raw: 16 } }, size: Size { raw: 16 }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } })
28                                                 // mir::Constant
29      -                                          // + span: $DIR/inline-into-box-place.rs:8:33: 8:41
30      -                                          // + user_ty: UserType(1)

41      -         _0 = const ();                   // bb2[2]: scope 0 at $DIR/inline-into-box-place.rs:7:11: 9:2
42      +                                          // + span: $SRC_DIR/liballoc/vec.rs:LL:COL
43      +                                          // + user_ty: UserType(0)
-       +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [65535], len: Size { raw: 16 } }, size: Size { raw: 16 }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
+       +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [65535], len: Size { raw: 16 } }, size: Size { raw: 16 }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
45      +         ((*_4).1: usize) = const 0usize; // bb0[5]: scope 2 at $SRC_DIR/liballoc/vec.rs:LL:COL
46                                                 // ty::Const
47      +                                          // + ty: usize

thread '[mir-opt] mir-opt/inline/inline-into-box-place.rs' panicked at 'Actual MIR output differs from expected MIR output /home/hbinadesk/git/rust/src/test/mir-opt/inline/inline-into-box-place/64bit/rustc.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3165:25

---- [mir-opt] mir-opt/no-drop-for-inactive-variant.rs stdout ----
32                                               // + literal: Const { ty: fn(&str) -> ! {std::rt::begin_panic::<&str>}, val: Value(Scalar(<ZST>)) }
33                                               // ty::Const
34                                               // + ty: &str
-                                                // + val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 })
+                                                // + val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 })
36                                               // mir::Constant
37                                               // + span: $SRC_DIR/libstd/macros.rs:LL:COL
-                                                // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
+                                                // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 97, 110, 105, 99], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [16383], len: Size { raw: 14 } }, size: Size { raw: 14 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 14 }) }
39          }
40
41          bb3: {

thread '[mir-opt] mir-opt/no-drop-for-inactive-variant.rs' panicked at 'Actual MIR output differs from expected MIR output /home/hbinadesk/git/rust/src/test/mir-opt/no-drop-for-inactive-variant/rustc.unwrap.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3165:25

---- [mir-opt] mir-opt/no-spurious-drop-after-call.rs stdout ----
15              _4 = const "";                   // bb0[4]: scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:22
16                                               // ty::Const
17                                               // + ty: &str
-                                                // + val: Value(Slice { data: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [], len: Size { raw: 0 } }, size: Size { raw: 0 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 0 })
+                                                // + val: Value(Slice { data: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [], len: Size { raw: 0 } }, size: Size { raw: 0 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 0 })
19                                               // mir::Constant
20                                               // + span: $DIR/no-spurious-drop-after-call.rs:9:20: 9:22
-                                                // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [], len: Size { raw: 0 } }, size: Size { raw: 0 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 0 }) }
+                                                // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [], len: Size { raw: 0 } }, size: Size { raw: 0 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 0 }) }
22              _3 = &(*_4);                     // bb0[5]: scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:22
23              _2 = const <str as std::string::ToString>::to_string(move _3) -> bb2; // bb0[6]: scope 0 at $DIR/no-spurious-drop-after-call.rs:9:20: 9:34
24                                               // ty::Const

thread '[mir-opt] mir-opt/no-spurious-drop-after-call.rs' panicked at 'Actual MIR output differs from expected MIR output /home/hbinadesk/git/rust/src/test/mir-opt/no-spurious-drop-after-call/rustc.main.ElaborateDrops.before.mir', src/tools/compiletest/src/runtest.rs:3165:25

---- [mir-opt] mir-opt/uninhabited_enum_branching.rs stdout ----
27                _5 = const "C";                  // bb1[1]: scope 0 at $DIR/uninhabited_enum_branching.rs:23:21: 23:24
28                                                 // ty::Const
29                                                 // + ty: &str
-                                                  // + val: Value(Slice { data: Allocation { bytes: [67], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 })
+                                                  // + val: Value(Slice { data: Allocation { bytes: [67], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 })
31                                                 // mir::Constant
32                                                 // + span: $DIR/uninhabited_enum_branching.rs:23:21: 23:24
-                                                  // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [67], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
+                                                  // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [67], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
34                _1 = &(*_5);                     // bb1[2]: scope 0 at $DIR/uninhabited_enum_branching.rs:23:21: 23:24
35                StorageDead(_5);                 // bb1[3]: scope 0 at $DIR/uninhabited_enum_branching.rs:23:23: 23:24
36                goto -> bb4;                     // bb1[4]: scope 0 at $DIR/uninhabited_enum_branching.rs:20:5: 24:6

40                _1 = const "A(Empty)";           // bb2[0]: scope 0 at $DIR/uninhabited_enum_branching.rs:21:24: 21:34
41                                                 // ty::Const
42                                                 // + ty: &str
-                                                  // + val: Value(Slice { data: Allocation { bytes: [65, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 })
+                                                  // + val: Value(Slice { data: Allocation { bytes: [65, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 })
44                                                 // mir::Constant
45                                                 // + span: $DIR/uninhabited_enum_branching.rs:21:24: 21:34
-                                                  // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [65, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 }) }
+                                                  // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [65, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 }) }
47                goto -> bb4;                     // bb2[1]: scope 0 at $DIR/uninhabited_enum_branching.rs:20:5: 24:6
48            }
49        

52                _4 = const "B(Empty)";           // bb3[1]: scope 0 at $DIR/uninhabited_enum_branching.rs:22:24: 22:34
53                                                 // ty::Const
54                                                 // + ty: &str
-                                                  // + val: Value(Slice { data: Allocation { bytes: [66, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 })
+                                                  // + val: Value(Slice { data: Allocation { bytes: [66, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 })
56                                                 // mir::Constant
57                                                 // + span: $DIR/uninhabited_enum_branching.rs:22:24: 22:34
-                                                  // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [66, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 }) }
+                                                  // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [66, 40, 69, 109, 112, 116, 121, 41], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [255], len: Size { raw: 8 } }, size: Size { raw: 8 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 8 }) }
59                _1 = &(*_4);                     // bb3[2]: scope 0 at $DIR/uninhabited_enum_branching.rs:22:24: 22:34
60                StorageDead(_4);                 // bb3[3]: scope 0 at $DIR/uninhabited_enum_branching.rs:22:33: 22:34
61                goto -> bb4;                     // bb3[4]: scope 0 at $DIR/uninhabited_enum_branching.rs:20:5: 24:6

76                _9 = const "E";                  // bb5[1]: scope 0 at $DIR/uninhabited_enum_branching.rs:28:21: 28:24
77                                                 // ty::Const
78                                                 // + ty: &str
-                                                  // + val: Value(Slice { data: Allocation { bytes: [69], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 })
+                                                  // + val: Value(Slice { data: Allocation { bytes: [69], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 })
80                                                 // mir::Constant
81                                                 // + span: $DIR/uninhabited_enum_branching.rs:28:21: 28:24
-                                                  // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [69], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
+                                                  // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [69], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
83                _6 = &(*_9);                     // bb5[2]: scope 0 at $DIR/uninhabited_enum_branching.rs:28:21: 28:24
84                StorageDead(_9);                 // bb5[3]: scope 0 at $DIR/uninhabited_enum_branching.rs:28:23: 28:24
85                goto -> bb7;                     // bb5[4]: scope 0 at $DIR/uninhabited_enum_branching.rs:26:5: 29:6

89                _6 = const "D";                  // bb6[0]: scope 0 at $DIR/uninhabited_enum_branching.rs:27:21: 27:24
90                                                 // ty::Const
91                                                 // + ty: &str
-                                                  // + val: Value(Slice { data: Allocation { bytes: [68], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 })
+                                                  // + val: Value(Slice { data: Allocation { bytes: [68], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 })
93                                                 // mir::Constant
94                                                 // + span: $DIR/uninhabited_enum_branching.rs:27:21: 27:24
-                                                  // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [68], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
+                                                  // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [68], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [1], len: Size { raw: 1 } }, size: Size { raw: 1 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 1 }) }
96                goto -> bb7;                     // bb6[1]: scope 0 at $DIR/uninhabited_enum_branching.rs:26:5: 29:6
97            }
98        

thread '[mir-opt] mir-opt/uninhabited_enum_branching.rs' panicked at 'Actual MIR output differs from expected MIR output /home/hbinadesk/git/rust/src/test/mir-opt/uninhabited_enum_branching/rustc.main.UninhabitedEnumBranching.diff', src/tools/compiletest/src/runtest.rs:3165:25

---- [mir-opt] mir-opt/storage_live_dead_in_statics.rs stdout ----
653             _2 = Foo { tup: const "hi", data: move _3 }; // bb0[94]: scope 0 at $DIR/storage_live_dead_in_statics.rs:5:29: 23:2
654                                              // ty::Const
655                                              // + ty: &str
-                                                // + val: Value(Slice { data: Allocation { bytes: [104, 105], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [3], len: Size { raw: 2 } }, size: Size { raw: 2 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 2 })
+                                                // + val: Value(Slice { data: Allocation { bytes: [104, 105], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [3], len: Size { raw: 2 } }, size: Size { raw: 2 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 2 })
657                                              // mir::Constant
658                                              // + span: $DIR/storage_live_dead_in_statics.rs:6:10: 6:14
-                                                // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [104, 105], relocations: Relocations(SortedMap { data: [] }), undef_mask: UndefMask { blocks: [3], len: Size { raw: 2 } }, size: Size { raw: 2 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 2 }) }
+                                                // + literal: Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [104, 105], relocations: Relocations(SortedMap { data: [] }), undef_mask: UninitMask { blocks: [3], len: Size { raw: 2 } }, size: Size { raw: 2 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 2 }) }
660             _1 = &_2;                        // bb0[95]: scope 0 at $DIR/storage_live_dead_in_statics.rs:5:28: 23:2
661             _0 = &(*_1);                     // bb0[96]: scope 0 at $DIR/storage_live_dead_in_statics.rs:5:28: 23:2
662             StorageDead(_5);                 // bb0[97]: scope 0 at $DIR/storage_live_dead_in_statics.rs:23:1: 23:2

thread '[mir-opt] mir-opt/storage_live_dead_in_statics.rs' panicked at 'Actual MIR output differs from expected MIR output /home/hbinadesk/git/rust/src/test/mir-opt/storage_live_dead_in_statics/rustc.XXX.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3165:25


failures:
    [mir-opt] mir-opt/const_prop/control-flow-simplification.rs
    [mir-opt] mir-opt/inline/inline-into-box-place.rs
    [mir-opt] mir-opt/no-drop-for-inactive-variant.rs
    [mir-opt] mir-opt/no-spurious-drop-after-call.rs
    [mir-opt] mir-opt/storage_live_dead_in_statics.rs
    [mir-opt] mir-opt/uninhabited_enum_branching.rs

test result: FAILED. 84 passed; 6 failed; 0 ignored; 0 measured; 0 filtered out

thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22


command did not execute successfully: "/home/hbinadesk/git/rust/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/home/hbinadesk/git/rust/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/home/hbinadesk/git/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/home/hbinadesk/git/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/home/hbinadesk/git/rust/src/test/mir-opt" "--build-base" "/home/hbinadesk/git/rust/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/home/hbinadesk/git/rust/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/home/hbinadesk/git/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/home/hbinadesk/git/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python" "--lldb-python" "/usr/bin/python" "--gdb" "/usr/bin/gdb" "--lldb-version" "lldb version 6.0.0\n" "--lldb-python-dir" "/usr/lib/x86_64-linux-gnu/python2.7/site-packages" "--quiet" "--llvm-version" "9.0.1-rust-dev" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" ""
expected success, got: exit code: 101


failed to run: /home/hbinadesk/git/rust/build/bootstrap/debug/bootstrap test
Build completed unsuccessfully in 0:50:34
