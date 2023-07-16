plain
    Finished release [optimized] target(s) in 0.19s
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 179 tests
i....................F.................i.............F.........................i........ 88/179
.......i........F...............F.........................i............................. 176/179
failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu

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

thread '[mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3466:25

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

thread '[mir-opt] src/test/mir-opt/const_prop/slice_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/slice_len.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3466:25
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

thread '[mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_into_box_place.main.Inline.32bit.diff', src/tools/compiletest/src/runtest.rs:3466:25
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

thread '[mir-opt] src/test/mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.32bit.diff', src/tools/compiletest/src/runtest.rs:3466:25

failures:
    [mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs
    [mir-opt] src/test/mir-opt/const_prop/slice_len.rs
