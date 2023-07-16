plain
.............ii.iiii.........ii.......i.................................

failures:

---- [mir-opt] tests/mir-opt/const_prop/transmute.rs stdout ----
19 +         _2 = const {0x1 as &Never};      // scope 2 at $DIR/transmute.rs:+1:30: +1:48
20 +                                          // mir::Constant
21 +                                          // + span: no-location
- +                                          // + literal: Const { ty: &Never, val: Value(Scalar(0x0000000000000001)) }
+ +                                          // + literal: Const { ty: &Never, val: Value(Scalar(0x00000001)) }
23           StorageLive(_3);                 // scope 1 at $DIR/transmute.rs:+2:5: +2:16
24           unreachable;                     // scope 1 at $DIR/transmute.rs:+2:11: +2:13


thread '[mir-opt] tests/mir-opt/const_prop/transmute.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/transmute.unreachable_ref.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3553:21


failures:
    [mir-opt] tests/mir-opt/const_prop/transmute.rs
