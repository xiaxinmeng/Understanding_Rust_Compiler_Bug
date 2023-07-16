plain
....................iii.iiiii.........ii........i................................

failures:

---- [mir-opt] tests/mir-opt/const_prop/discriminant.rs stdout ----
19           StorageLive(_3);                 // scope 2 at $DIR/discriminant.rs:+1:34: +1:44
20 -         _3 = Option::<bool>::Some(const true); // scope 2 at $DIR/discriminant.rs:+1:34: +1:44
21 -         _4 = discriminant(_3);           // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
- -         switchInt(move _4) -> [1: bb1, otherwise: bb3]; // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
+ -         switchInt(move _4) -> [1: bb2, otherwise: bb3]; // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
23 +         _3 = const Option::<bool>::Some(true); // scope 2 at $DIR/discriminant.rs:+1:34: +1:44
24 +                                          // mir::Constant
25 +                                          // + span: no-location


26 +                                          // + literal: Const { ty: Option<bool>, val: Value(Scalar(0x01)) }
27 +         _4 = const 1_isize;              // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
- +         switchInt(const 1_isize) -> [1: bb1, otherwise: bb3]; // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
+ +         switchInt(const 1_isize) -> [1: bb2, otherwise: bb3]; // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
30   
31       bb1: {


- -         switchInt(((_3 as Some).0: bool)) -> [0: bb3, otherwise: bb2]; // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
- +         switchInt(const true) -> [0: bb3, otherwise: bb2]; // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
+           _2 = const 42_i32;               // scope 2 at $DIR/discriminant.rs:+1:47: +1:49
+           goto -> bb4;                     // scope 0 at $DIR/discriminant.rs:+1:13: +1:64
35   
36       bb2: {


-           _2 = const 42_i32;               // scope 2 at $DIR/discriminant.rs:+1:47: +1:49
-           goto -> bb4;                     // scope 0 at $DIR/discriminant.rs:+1:13: +1:64
+ -         switchInt(((_3 as Some).0: bool)) -> [0: bb3, otherwise: bb1]; // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
+ +         switchInt(const true) -> [0: bb3, otherwise: bb1]; // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
40   
41       bb3: {


thread '[mir-opt] tests/mir-opt/const_prop/discriminant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/discriminant.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3639:21


failures:
    [mir-opt] tests/mir-opt/const_prop/discriminant.rs
