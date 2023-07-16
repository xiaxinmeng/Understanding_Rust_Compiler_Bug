plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 245 tests
.............................................................................i..........  88/245
.....................................................i...........ii..i.F................ 176/245

failures:


---- [mir-opt] tests/mir-opt/instcombine_duplicate_switch_targets_e2e.rs stdout ----
7     scope 1 (inlined unreachable_unchecked) { // at $DIR/instcombine_duplicate_switch_targets_e2e.rs:13:21: 13:55
8         scope 2 {
9             scope 3 (inlined unreachable_unchecked::runtime) { // at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+                 let _3: !;               // in scope 3 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
11         }
12     }

13 
13 
14     bb0: {
15         _2 = discriminant(_1);           // scope 0 at $DIR/instcombine_duplicate_switch_targets_e2e.rs:+1:11: +1:12
-         switchInt(move _2) -> [0: bb2, otherwise: bb1]; // scope 0 at $DIR/instcombine_duplicate_switch_targets_e2e.rs:+1:5: +1:12
+         switchInt(move _2) -> [0: bb3, 1: bb1, otherwise: bb2]; // scope 0 at $DIR/instcombine_duplicate_switch_targets_e2e.rs:+1:5: +1:12
18 
19     bb1: {


-         unreachable;                     // scope 2 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+         _3 = core::panicking::panic_nounwind(const "unsafe precondition(s) violated: hint::unreachable_unchecked must never be reached") -> unwind unreachable; // scope 3 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+                                          // mir::Constant
+                                          // + span: $SRC_DIR/core/src/intrinsics.rs:LL:COL
+                                          // + literal: Const { ty: fn(&'static str) -> ! {core::panicking::panic_nounwind}, val: Value(<ZST>) }
+                                          // mir::Constant
+                                          // + span: $SRC_DIR/core/src/intrinsics.rs:LL:COL
+                                          // + literal: Const { ty: &str, val: Value(Slice(..)) }
22 
23     bb2: {


+         unreachable;                     // scope 0 at $DIR/instcombine_duplicate_switch_targets_e2e.rs:+1:11: +1:12
+ 
+     bb3: {
+     bb3: {
24         _0 = move _1;                    // scope 0 at $DIR/instcombine_duplicate_switch_targets_e2e.rs:+2:21: +2:22
25         return;                          // scope 0 at $DIR/instcombine_duplicate_switch_targets_e2e.rs:+5:2: +5:2


thread '[mir-opt] tests/mir-opt/instcombine_duplicate_switch_targets_e2e.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/instcombine_duplicate_switch_targets_e2e.ub_if_b.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3527:21


failures:
    [mir-opt] tests/mir-opt/instcombine_duplicate_switch_targets_e2e.rs
