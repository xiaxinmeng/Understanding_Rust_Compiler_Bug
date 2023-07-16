plain
failures:

---- [mir-opt] tests/mir-opt/const_prop/offset_of.rs stdout ----
22   
23       bb0: {
24           StorageLive(_1);                 // scope 0 at $DIR/offset_of.rs:+1:9: +1:10
- -         _1 = OffsetOf(Alpha, [0]);       // scope 0 at $DIR/offset_of.rs:+1:13: +1:33
- +         _1 = const 4_usize;              // scope 0 at $DIR/offset_of.rs:+1:13: +1:33
+ -         _1 = OffsetOf(Alpha, [0]);       // scope 0 at $SRC_DIR/core/src/mem/offset_of.rs:LL:COL
+ +         _1 = const 4_usize;              // scope 0 at $SRC_DIR/core/src/mem/offset_of.rs:LL:COL
27           StorageLive(_2);                 // scope 1 at $DIR/offset_of.rs:+2:9: +2:10
- -         _2 = OffsetOf(Alpha, [1]);       // scope 1 at $DIR/offset_of.rs:+2:13: +2:33
- +         _2 = const 0_usize;              // scope 1 at $DIR/offset_of.rs:+2:13: +2:33
+ -         _2 = OffsetOf(Alpha, [1]);       // scope 1 at $SRC_DIR/core/src/mem/offset_of.rs:LL:COL
+ +         _2 = const 0_usize;              // scope 1 at $SRC_DIR/core/src/mem/offset_of.rs:LL:COL
30           StorageLive(_3);                 // scope 2 at $DIR/offset_of.rs:+3:9: +3:11
- -         _3 = OffsetOf(Alpha, [2, 0]);    // scope 2 at $DIR/offset_of.rs:+3:14: +3:36
- +         _3 = const 2_usize;              // scope 2 at $DIR/offset_of.rs:+3:14: +3:36
+ -         _3 = OffsetOf(Alpha, [2, 0]);    // scope 2 at $SRC_DIR/core/src/mem/offset_of.rs:LL:COL
+ +         _3 = const 2_usize;              // scope 2 at $SRC_DIR/core/src/mem/offset_of.rs:LL:COL
33           StorageLive(_4);                 // scope 3 at $DIR/offset_of.rs:+4:9: +4:11
- -         _4 = OffsetOf(Alpha, [2, 1]);    // scope 3 at $DIR/offset_of.rs:+4:14: +4:36
- +         _4 = const 3_usize;              // scope 3 at $DIR/offset_of.rs:+4:14: +4:36
+ -         _4 = OffsetOf(Alpha, [2, 1]);    // scope 3 at $SRC_DIR/core/src/mem/offset_of.rs:LL:COL
+ +         _4 = const 3_usize;              // scope 3 at $SRC_DIR/core/src/mem/offset_of.rs:LL:COL
36           StorageDead(_4);                 // scope 3 at $DIR/offset_of.rs:+5:1: +5:2
37           StorageDead(_3);                 // scope 2 at $DIR/offset_of.rs:+5:1: +5:2
38           StorageDead(_2);                 // scope 1 at $DIR/offset_of.rs:+5:1: +5:2

thread '[mir-opt] tests/mir-opt/const_prop/offset_of.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/offset_of.concrete.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3553:21


failures:
    [mir-opt] tests/mir-opt/const_prop/offset_of.rs
