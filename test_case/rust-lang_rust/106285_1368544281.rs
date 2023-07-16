plain
.......i.................................
failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu

---- [mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs stdout ----
33           _2 = &raw const (*_3);           // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:+1:25: +1:35
34           _1 = move _2 as *const [i32] (Pointer(Unsize)); // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:+1:25: +1:35
35           StorageDead(_2);                 // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:+1:34: +1:35
-           StorageDead(_3);                 // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:+1:35: +1:36
+           nop;                             // scope 0 at $DIR/bad_op_unsafe_oob_for_slices.rs:+1:35: +1:36
37           StorageLive(_5);                 // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:13: +3:15
38           StorageLive(_6);                 // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:23: +3:24
39           _6 = const 3_usize;              // scope 2 at $DIR/bad_op_unsafe_oob_for_slices.rs:+3:23: +3:24

thread '[mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3465:21


failures:
    [mir-opt] src/test/mir-opt/const_prop/bad_op_unsafe_oob_for_slices.rs
