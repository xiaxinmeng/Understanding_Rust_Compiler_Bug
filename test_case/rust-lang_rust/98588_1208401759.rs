plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
.....
failures:

---- [mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs stdout ----
25           StorageLive(_1);                 // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:+1:9: +1:10
26           StorageLive(_2);                 // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:+1:25: +1:35
27           StorageLive(_3);                 // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:+1:25: +1:35
-           _9 = _;                          // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:+1:25: +1:35
+           _9 = const _;                    // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:+1:25: +1:35
29                                            // mir::Constant
30                                            // + span: $DIR/bad_op_unsafe_oob_for_slices.rs:5:25: 5:35
31                                            // + literal: Const { ty: &[i32; 3], val: Unevaluated(main, [], Some(promoted[0])) }

thread '[mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3513:25

---- [mir-opt] src/test/mir-opt/issue-73223.rs stdout ----
---- [mir-opt] src/test/mir-opt/issue-73223.rs stdout ----
83           StorageLive(_10);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
84           _10 = &_1;                       // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
85           StorageLive(_11);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-        _28 = const _;                   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _28 = const _;                   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
87                                            // mir::Constant
88                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
89                                            // + literal: Const { ty: &i32, val: Unevaluated(main, [], Some(promoted[0])) }

thread '[mir-opt] src/test/mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.32bit.diff', src/tools/compiletest/src/runtest.rs:3513:25

failures:
    [mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs
    [mir-opt] src/test/mir-opt/issue-73223.rs
