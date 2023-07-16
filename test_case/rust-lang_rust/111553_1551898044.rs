plain
test [mir-opt] tests/mir-opt/inline/polymorphic_recursion.rs ... ok

failures:

---- [mir-opt] tests/mir-opt/pre-codegen/checked_ops.rs stdout ----
6     let mut _0: u32;                     // return place in scope 0 at $DIR/checked_ops.rs:+0:42: +0:45
8     bb0: {
8     bb0: {
-         _0 = <u32 as Step>::forward(_1, _2) -> bb1; // scope 0 at $DIR/checked_ops.rs:+1:5: +1:35
+         _0 = <u32 as Step>::forward(_1, _2) -> [return: bb1, unwind unreachable]; // scope 0 at $DIR/checked_ops.rs:+1:5: +1:35
10                                          // mir::Constant
11                                          // + span: $DIR/checked_ops.rs:9:5: 9:29
12                                          // + literal: Const { ty: fn(u32, usize) -> u32 {<u32 as Step>::forward}, val: Value(<ZST>) }

thread '[mir-opt] tests/mir-opt/pre-codegen/checked_ops.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/pre-codegen/checked_ops.step_forward.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3634:21

---- [mir-opt] tests/mir-opt/pre-codegen/loops.rs stdout ----
34     bb1: {
34     bb1: {
35         StorageLive(_6);                 // scope 1 at $DIR/loops.rs:+1:14: +1:24
36         _7 = &mut _4;                    // scope 1 at $DIR/loops.rs:+1:14: +1:24
-         _6 = <std::ops::Range<usize> as iter::range::RangeIteratorImpl>::spec_next(_7) -> bb6; // scope 4 at $SRC_DIR/core/src/iter/range.rs:LL:COL
+         _6 = <std::ops::Range<usize> as iter::range::RangeIteratorImpl>::spec_next(_7) -> [return: bb6, unwind unreachable]; // scope 4 at $SRC_DIR/core/src/iter/range.rs:LL:COL
38                                          // mir::Constant
39                                          // + span: $SRC_DIR/core/src/iter/range.rs:LL:COL
40                                          // + literal: Const { ty: for<'a> fn(&'a mut std::ops::Range<usize>) -> Option<<std::ops::Range<usize> as iter::range::RangeIteratorImpl>::Item> {<std::ops::Range<usize> as iter::range::RangeIteratorImpl>::spec_next}, val: Value(<ZST>) }
42 
43     bb2: {
43     bb2: {
44         _9 = ((_6 as Some).0: usize);    // scope 1 at $DIR/loops.rs:+1:9: +1:10
-         _5 = opaque::<usize>(_9) -> bb5; // scope 2 at $DIR/loops.rs:+2:9: +2:18
+         _5 = opaque::<usize>(_9) -> [return: bb5, unwind unreachable]; // scope 2 at $DIR/loops.rs:+2:9: +2:18
46                                          // mir::Constant
47                                          // + span: $DIR/loops.rs:8:9: 8:15
48                                          // + literal: Const { ty: fn(usize) {opaque::<usize>}, val: Value(<ZST>) }

thread '[mir-opt] tests/mir-opt/pre-codegen/loops.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/pre-codegen/loops.int_range.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3634:21

failures:
    [mir-opt] tests/mir-opt/pre-codegen/checked_ops.rs
    [mir-opt] tests/mir-opt/pre-codegen/loops.rs
