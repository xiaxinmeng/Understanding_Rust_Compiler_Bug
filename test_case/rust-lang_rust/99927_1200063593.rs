plain
.....
failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu

---- [mir-opt] src/test/mir-opt/issue-73223.rs stdout ----
98           _14 = core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _15, move _17, move _19); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
99                                            // mir::Constant
100                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                                            // + literal: Const { ty: for<'r, 's, 't0> fn(core::panicking::AssertKind, &'r i32, &'s i32, Option<Arguments<'t0>>) -> ! {core::panicking::assert_failed::<i32, i32>}, val: Value(<ZST>) }
+                                            // + literal: Const { ty: [fn item {core::panicking::assert_failed::<i32, i32>}: for<'r, 's, 't0> fn(core::panicking::AssertKind, &'r i32, &'s i32, Option<Arguments<'t0>>) -> !], val: Value(<ZST>) }
102                                            // mir::Constant
103                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
104                                            // + literal: Const { ty: core::panicking::AssertKind, val: Value(Scalar(0x00)) }

thread '[mir-opt] src/test/mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.PreCodegen.32bit.diff', src/tools/compiletest/src/runtest.rs:3513:25


failures:
    [mir-opt] src/test/mir-opt/issue-73223.rs
