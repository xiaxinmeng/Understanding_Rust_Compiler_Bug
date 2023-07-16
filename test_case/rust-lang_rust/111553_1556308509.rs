plain
test [mir-opt] tests/mir-opt/unusual_item_types.rs ... ok

failures:

---- [mir-opt] tests/mir-opt/pre-codegen/checked_ops.rs stdout ----
3 fn ilog2(_1: u32) -> u32 {
4     debug x => _1;                       // in scope 0 at $DIR/checked_ops.rs:+0:14: +0:15
5     let mut _0: u32;                     // return place in scope 0 at $DIR/checked_ops.rs:+0:25: +0:28
-     scope 1 (inlined #[track_caller] core::num::<impl u32>::ilog2) { // at $DIR/checked_ops.rs:21:7: 21:14
-         debug self => _1;                // in scope 1 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-         let mut _2: std::option::Option<u32>; // in scope 1 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-         let mut _3: isize;               // in scope 1 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-         let mut _4: !;                   // in scope 1 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-         scope 2 {
-             debug log => _0;             // in scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-     }
15 
16     bb0: {
16     bb0: {
-         StorageLive(_2);                 // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-         _2 = core::num::<impl u32>::checked_ilog2(_1) -> bb1; // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+         _0 = core::num::<impl u32>::ilog2(_1) -> bb1; // scope 0 at $DIR/checked_ops.rs:+1:5: +1:14
19                                          // mir::Constant
-                                          // + span: $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-                                          // + literal: Const { ty: fn(u32) -> Option<u32> {core::num::<impl u32>::checked_ilog2}, val: Value(<ZST>) }
+                                          // + span: $DIR/checked_ops.rs:21:7: 21:12
+                                          // + literal: Const { ty: fn(u32) -> u32 {core::num::<impl u32>::ilog2}, val: Value(<ZST>) }
23 
24     bb1: {


-         _3 = discriminant(_2);           // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-         switchInt(move _3) -> [1: bb2, otherwise: bb3]; // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
- 
-     bb2: {
-     bb2: {
-         _0 = ((_2 as Some).0: u32);      // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-         StorageDead(_2);                 // scope 1 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
32         return;                          // scope 0 at $DIR/checked_ops.rs:+2:2: +2:2
- 
-     bb3: {
-     bb3: {
-         _4 = core::num::int_log10::panic_for_nonpositive_argument(); // scope 1 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-                                          // mir::Constant
-                                          // + span: $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-                                          // + literal: Const { ty: fn() -> ! {core::num::int_log10::panic_for_nonpositive_argument}, val: Value(<ZST>) }
41 }
42 


thread '[mir-opt] tests/mir-opt/pre-codegen/checked_ops.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/pre-codegen/checked_ops.ilog2.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3639:21


failures:
    [mir-opt] tests/mir-opt/pre-codegen/checked_ops.rs
