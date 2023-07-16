plain
##[endgroup]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 255 tests
i..............................................F...........................F....i.......  88/255
..................iii.iiiii.........ii........i................................

failures:


---- [mir-opt] tests/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs stdout ----
28           _9 = const _;                    // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:+1:25: +1:35
29                                            // mir::Constant
30                                            // + span: $DIR/bad_op_unsafe_oob_for_slices.rs:9:25: 9:35
-                                            // + literal: Const { ty: &[i32; 3], val: Unevaluated(main, [], Some(promoted[0])) }
+                                            // + literal: Const { ty: &[i32; 3], val: Unevaluated(main::{promoted#0}, [&[i32; 3]]) }
32           _3 = &(*_9);                     // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:+1:25: +1:35
33           _2 = &raw const (*_3);           // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:+1:25: +1:35
34           _1 = move _2 as *const [i32] (Pointer(Unsize)); // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:+1:25: +1:35

thread '[mir-opt] tests/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3639:21

---- [mir-opt] tests/mir-opt/const_prop/slice_len.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/slice_len.rs stdout ----
21           _9 = const _;                    // scope 0 at $DIR/slice_len.rs:+1:6: +1:19
22                                            // mir::Constant
23                                            // + span: $DIR/slice_len.rs:8:6: 8:19
-                                            // + literal: Const { ty: &[u32; 3], val: Unevaluated(main, [], Some(promoted[0])) }
+                                            // + literal: Const { ty: &[u32; 3], val: Unevaluated(main::{promoted#0}, [&[u32; 3]]) }
Build completed unsuccessfully in 0:01:32
25           _4 = _9;                         // scope 0 at $DIR/slice_len.rs:+1:6: +1:19
26           _3 = _4;                         // scope 0 at $DIR/slice_len.rs:+1:6: +1:19
27           _2 = move _3 as &[u32] (Pointer(Unsize)); // scope 0 at $DIR/slice_len.rs:+1:6: +1:19

thread '[mir-opt] tests/mir-opt/const_prop/slice_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/slice_len.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3639:21

failures:
    [mir-opt] tests/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs
    [mir-opt] tests/mir-opt/const_prop/slice_len.rs
