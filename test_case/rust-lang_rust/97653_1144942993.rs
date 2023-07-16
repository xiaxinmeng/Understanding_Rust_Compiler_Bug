plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..
failures:

---- [mir-opt] src/test/mir-opt/const_prop/reify_fn_ptr.rs stdout ----
19                                            // + literal: Const { ty: fn() {main}, val: Value(Scalar(<ZST>)) }
20           _2 = move _3 as usize (PointerExposeAddress); // scope 0 at $DIR/reify_fn_ptr.rs:4:13: 4:26
21           StorageDead(_3);                 // scope 0 at $DIR/reify_fn_ptr.rs:4:25: 4:26
-           _1 = move _2 as *const fn() (Misc); // scope 0 at $DIR/reify_fn_ptr.rs:4:13: 4:41
+           _1 = move _2 as *const fn() (PointerFromExposedAddress); // scope 0 at $DIR/reify_fn_ptr.rs:4:13: 4:41
23           StorageDead(_2);                 // scope 0 at $DIR/reify_fn_ptr.rs:4:40: 4:41
24           StorageDead(_1);                 // scope 0 at $DIR/reify_fn_ptr.rs:4:41: 4:42
25           nop;                             // scope 0 at $DIR/reify_fn_ptr.rs:3:11: 5:2

thread '[mir-opt] src/test/mir-opt/const_prop/reify_fn_ptr.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/reify_fn_ptr.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3420:25


failures:
    [mir-opt] src/test/mir-opt/const_prop/reify_fn_ptr.rs
