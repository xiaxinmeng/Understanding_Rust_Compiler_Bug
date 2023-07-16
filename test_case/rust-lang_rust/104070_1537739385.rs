plain
test [mir-opt] tests/mir-opt/inline/polymorphic_recursion.rs ... ok

failures:

---- [mir-opt] tests/mir-opt/issue_104451.rs stdout ----
11         StorageLive(_1);                 // scope 1 at $DIR/issue_104451.rs:+2:9: +2:62
12         StorageLive(_2);                 // scope 1 at $DIR/issue_104451.rs:+2:45: +2:47
13         _2 = ();                         // scope 1 at $DIR/issue_104451.rs:+2:45: +2:47
-         _1 = const_eval_select::<(), fn() -> ! {ow_ct}, fn() -> ! {ow_ct}, !>(move _2, ow_ct, ow_ct); // scope 1 at $DIR/issue_104451.rs:+2:9: +2:62
+         _1 = const_eval_select::<(), fn() -> ! {ow_ct}, fn() -> ! {ow_ct}, !>(move _2, ow_ct, ow_ct) -> unwind unreachable; // scope 1 at $DIR/issue_104451.rs:+2:9: +2:62
15                                          // mir::Constant
16                                          // + span: $DIR/issue_104451.rs:6:9: 6:44
17                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn((), fn() -> ! {ow_ct}, fn() -> ! {ow_ct}) -> ! {const_eval_select::<(), fn() -> ! {ow_ct}, fn() -> ! {ow_ct}, !>}, val: Value(<ZST>) }

thread '[mir-opt] tests/mir-opt/issue_104451.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/issue_104451.main.AbortUnwindingCalls.after.mir', src/tools/compiletest/src/runtest.rs:3634:21


failures:
    [mir-opt] tests/mir-opt/issue_104451.rs
