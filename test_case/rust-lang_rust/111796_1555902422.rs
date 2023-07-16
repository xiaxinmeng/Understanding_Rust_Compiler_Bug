plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 255 tests
................................................................................i.......  88/255
.....................................................F...i.F......F.Fiii................ 176/255
...........F......iii.iiiii.........ii........i...F.F..F.......................
failures:

---- [mir-opt] tests/mir-opt/inline/asm_unwind.rs stdout ----
31 +     }
31 +     }
32 + 
33 +     bb2: {
- +         drop(_2) -> bb1;                 // scope 1 at $DIR/asm_unwind.rs:17:1: 17:2
+ +         drop(_2) -> [return: bb1, unwind: bb4]; // scope 1 at $DIR/asm_unwind.rs:17:1: 17:2
36 + 
36 + 
37 +     bb3 (cleanup): {

thread '[mir-opt] tests/mir-opt/inline/asm_unwind.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/asm_unwind.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
Build completed unsuccessfully in 0:13:33

---- [mir-opt] tests/mir-opt/inline/cycle.rs stdout ----
52 +     bb4: {
52 +     bb4: {
53 +         StorageDead(_5);                 // scope 1 at $DIR/cycle.rs:6:5: 6:8
54 +         StorageDead(_4);                 // scope 1 at $DIR/cycle.rs:6:7: 6:8
- +         drop(_2) -> bb1;                 // scope 1 at $DIR/cycle.rs:7:1: 7:2
+ +         drop(_2) -> [return: bb1, unwind: bb3]; // scope 1 at $DIR/cycle.rs:7:1: 7:2
57   }
58   


thread '[mir-opt] tests/mir-opt/inline/cycle.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/cycle.g.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/inline_diverging.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_diverging.rs stdout ----
50 +         _1 = (move _6, move _7);         // scope 3 at $DIR/inline_diverging.rs:29:5: 29:11
51 +         StorageDead(_6);                 // scope 3 at $DIR/inline_diverging.rs:29:10: 29:11
52 +         StorageDead(_3);                 // scope 1 at $DIR/inline_diverging.rs:30:1: 30:2
- +         drop(_2) -> bb2;                 // scope 1 at $DIR/inline_diverging.rs:30:1: 30:2
+ +         drop(_2) -> [return: bb2, unwind: bb5]; // scope 1 at $DIR/inline_diverging.rs:30:1: 30:2
55 + 
56 +     bb2: {


thread '[mir-opt] tests/mir-opt/inline/inline_diverging.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_diverging.h.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/inline_cycle.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_cycle.rs stdout ----
27 +         StorageLive(_3);                 // scope 0 at $DIR/inline_cycle.rs:+1:5: +1:12
28 +         StorageLive(_4);                 // scope 1 at $DIR/inline_cycle.rs:55:5: 55:8
29 +         _4 = const ();                   // scope 1 at $DIR/inline_cycle.rs:55:5: 55:8
- +         _3 = move _2() -> bb1;           // scope 2 at $SRC_DIR/core/src/ops/function.rs:LL:COL
+ +         _3 = move _2() -> [return: bb2, unwind: bb1]; // scope 2 at $SRC_DIR/core/src/ops/function.rs:LL:COL
32   
-       bb1: {
+ -     bb1: {
+ -     bb1: {
+ +     bb1 (cleanup): {
+ +         resume;                          // scope 1 at $DIR/inline_cycle.rs:54:1: 56:2
+ +     }
+ +     bb2: {
+ +     bb2: {
34 +         StorageDead(_4);                 // scope 1 at $DIR/inline_cycle.rs:55:5: 55:8
35 +         StorageDead(_3);                 // scope 0 at $DIR/inline_cycle.rs:+1:5: +1:12
36 +         StorageDead(_2);                 // scope 0 at $DIR/inline_cycle.rs:+1:5: +1:12

thread '[mir-opt] tests/mir-opt/inline/inline_cycle.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_cycle.two.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/issues/issue_59352.rs stdout ----
---- [mir-opt] tests/mir-opt/issues/issue_59352.rs stdout ----
14             let mut _5: isize;           // in scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
16     }
16     }
-     scope 3 (inlined #[track_caller] Option::<u32>::unwrap) { // at $DIR/issue_59352.rs:15:42: 15:50
-         debug self => _2;                // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
-         let mut _6: isize;               // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
-         let mut _7: !;                   // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
-         scope 4 {
-             debug val => _0;             // in scope 4 at $SRC_DIR/core/src/option.rs:LL:COL
-     }
25 
26     bb0: {
26     bb0: {
27         StorageLive(_3);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL

28         StorageLive(_4);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         _4 = char::methods::<impl char>::to_digit(_1, const 8_u32) -> bb5; // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         _4 = char::methods::<impl char>::to_digit(_1, const 8_u32) -> bb6; // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
30                                          // mir::Constant
31                                          // + span: $SRC_DIR/core/src/char/methods.rs:LL:COL
32                                          // + literal: Const { ty: fn(char, u32) -> Option<u32> {char::methods::<impl char>::to_digit}, val: Value(<ZST>) }
41     }
42 
43     bb2: {
43     bb2: {
-         _6 = discriminant(_2);           // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
-         switchInt(move _6) -> [0: bb6, 1: bb8, otherwise: bb7]; // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         _0 = Option::<u32>::unwrap(move _2) -> bb3; // scope 0 at $DIR/issue_59352.rs:+2:26: +2:50
+                                          // mir::Constant
+                                          // + span: $DIR/issue_59352.rs:15:42: 15:48
+                                          // + literal: Const { ty: fn(Option<u32>) -> u32 {Option::<u32>::unwrap}, val: Value(<ZST>) }
47 
48     bb3: {


-         _0 = const 0_u32;                // scope 0 at $DIR/issue_59352.rs:+2:60: +2:61
-         goto -> bb4;                     // scope 0 at $DIR/issue_59352.rs:+2:5: +2:63
+         StorageDead(_2);                 // scope 0 at $DIR/issue_59352.rs:+2:49: +2:50
+         goto -> bb5;                     // scope 0 at $DIR/issue_59352.rs:+2:5: +2:63
52 
53     bb4: {


-         return;                          // scope 0 at $DIR/issue_59352.rs:+3:2: +3:2
+         _0 = const 0_u32;                // scope 0 at $DIR/issue_59352.rs:+2:60: +2:61
+         goto -> bb5;                     // scope 0 at $DIR/issue_59352.rs:+2:5: +2:63
56 
57     bb5: {


+         return;                          // scope 0 at $DIR/issue_59352.rs:+3:2: +3:2
+ 
+     bb6: {
+     bb6: {
58         _3 = &_4;                        // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
59         _5 = discriminant((*_3));        // scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
60         StorageDead(_3);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL

61         StorageDead(_4);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         switchInt(move _5) -> [1: bb1, otherwise: bb3]; // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
- 
-     bb6: {
-     bb6: {
-         _7 = core::panicking::panic(const "called `Option::unwrap()` on a `None` value"); // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
-                                          // mir::Constant
-                                          // + span: $SRC_DIR/core/src/option.rs:LL:COL
-                                          // + literal: Const { ty: fn(&'static str) -> ! {core::panicking::panic}, val: Value(<ZST>) }
-                                          // mir::Constant
-                                          // + span: $SRC_DIR/core/src/option.rs:LL:COL
-                                          // + literal: Const { ty: &str, val: Value(Slice(..)) }
- 
-     bb7: {
-     bb7: {
-         unreachable;                     // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
- 
-     bb8: {
-     bb8: {
-         _0 = move ((_2 as Some).0: u32); // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
-         StorageDead(_2);                 // scope 0 at $DIR/issue_59352.rs:+2:49: +2:50
-         goto -> bb4;                     // scope 0 at $DIR/issue_59352.rs:+2:5: +2:63
+         switchInt(move _5) -> [1: bb1, otherwise: bb4]; // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
84 }
85 


thread '[mir-opt] tests/mir-opt/issues/issue_59352.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/issues/issue_59352.num_to_digit.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/remove_unneeded_drops.rs stdout ----
---- [mir-opt] tests/mir-opt/remove_unneeded_drops.rs stdout ----
14 -         nop;                             // scope 0 at $DIR/remove_unneeded_drops.rs:+1:5: +1:12
15           StorageLive(_3);                 // scope 0 at $DIR/remove_unneeded_drops.rs:+1:10: +1:11
16           _3 = _1;                         // scope 0 at $DIR/remove_unneeded_drops.rs:+1:10: +1:11
- -         drop(_3) -> bb1;                 // scope 1 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
+ -         drop(_3) -> [return: bb1, unwind: bb2]; // scope 1 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
19 - 
20 -     bb1: {


22 -         nop;                             // scope 0 at $DIR/remove_unneeded_drops.rs:+1:12: +1:13
23 -         nop;                             // scope 0 at $DIR/remove_unneeded_drops.rs:+0:17: +2:2
24           return;                          // scope 0 at $DIR/remove_unneeded_drops.rs:+2:2: +2:2
+ -     }
+ - 
+ -     bb2 (cleanup): {
+ -         resume;                          // scope 1 at $SRC_DIR/core/src/mem/mod.rs:LL:COL
26   }
27   


thread '[mir-opt] tests/mir-opt/remove_unneeded_drops.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/remove_unneeded_drops.opt.RemoveUnneededDrops.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/separate_const_switch.rs stdout ----
15       scope 1 {
15       scope 1 {
16           debug residual => _6;            // in scope 1 at $DIR/separate_const_switch.rs:+1:9: +1:10
17           scope 2 {
-               scope 8 (inlined #[track_caller] <Result<i32, i32> as FromResidual<Result<Infallible, i32>>>::from_residual) { // at $DIR/separate_const_switch.rs:25:8: 25:10
-                   debug residual => _8;    // in scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
-                   let _14: i32;            // in scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
-                   let mut _15: i32;        // in scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
-                   scope 9 {
-                       debug e => _14;      // in scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
-                       scope 10 (inlined <i32 as From<i32>>::from) { // at $SRC_DIR/core/src/result.rs:LL:COL
-                           debug t => _14;  // in scope 10 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
-                   }
-               }
29           }
30       }
30       }
31       scope 3 {

55           StorageLive(_11);                // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10
56           StorageLive(_12);                // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10
57           _10 = discriminant(_4);          // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
-           switchInt(move _10) -> [0: bb7, 1: bb5, otherwise: bb6]; // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
+           switchInt(move _10) -> [0: bb8, 1: bb6, otherwise: bb7]; // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
60   
61       bb1: {


86           _6 = ((_3 as Break).0: std::result::Result<std::convert::Infallible, i32>); // scope 0 at $DIR/separate_const_switch.rs:+1:9: +1:10
87           StorageLive(_8);                 // scope 2 at $DIR/separate_const_switch.rs:+1:9: +1:10
88           _8 = _6;                         // scope 2 at $DIR/separate_const_switch.rs:+1:9: +1:10
-           StorageLive(_14);                // scope 2 at $DIR/separate_const_switch.rs:+1:8: +1:10
-           _14 = move ((_8 as Err).0: i32); // scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageLive(_15);                // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
-           _15 = move _14;                  // scope 10 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
-           _0 = Result::<i32, i32>::Err(move _15); // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageDead(_15);                // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageDead(_14);                // scope 2 at $DIR/separate_const_switch.rs:+1:8: +1:10
+           _0 = <Result<i32, i32> as FromResidual<Result<Infallible, i32>>>::from_residual(move _8) -> bb5; // scope 2 at $DIR/separate_const_switch.rs:+1:8: +1:10
+                                            // mir::Constant
+                                            // + span: $DIR/separate_const_switch.rs:25:9: 25:10
+                                            // + literal: Const { ty: fn(Result<Infallible, i32>) -> Result<i32, i32> {<Result<i32, i32> as FromResidual<Result<Infallible, i32>>>::from_residual}, val: Value(<ZST>) }
+   
+       bb5: {
+       bb5: {
96           StorageDead(_8);                 // scope 2 at $DIR/separate_const_switch.rs:+1:9: +1:10
97           StorageDead(_6);                 // scope 0 at $DIR/separate_const_switch.rs:+1:9: +1:10
98           StorageDead(_2);                 // scope 0 at $DIR/separate_const_switch.rs:+1:10: +1:11

100           return;                          // scope 0 at $DIR/separate_const_switch.rs:+2:2: +2:2
102   
-       bb5: {
+       bb6: {
+       bb6: {
104           _12 = move ((_4 as Err).0: i32); // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
105           StorageLive(_13);                // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
106           _13 = Result::<Infallible, i32>::Err(move _12); // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL

109           goto -> bb1;                     // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
111   
-       bb6: {
+       bb7: {
+       bb7: {
113           unreachable;                     // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
115   

-       bb7: {
+       bb8: {
+       bb8: {
117           _11 = move ((_4 as Ok).0: i32);  // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
118           _3 = ControlFlow::<Result<Infallible, i32>, i32>::Continue(move _11); // scope 6 at $SRC_DIR/core/src/result.rs:LL:COL
119           goto -> bb1;                     // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL

thread '[mir-opt] tests/mir-opt/separate_const_switch.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/separate_const_switch.identity.SeparateConstSwitch.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/simplify_locals_fixedpoint.rs stdout ----
42       }
43   
44       bb3: {
44       bb3: {
-           drop(_1) -> bb4;                 // scope 0 at $DIR/simplify_locals_fixedpoint.rs:+6:1: +6:2
+           drop(_1) -> [return: bb4, unwind: bb5]; // scope 0 at $DIR/simplify_locals_fixedpoint.rs:+6:1: +6:2
47   
48       bb4: {


49           StorageDead(_1);                 // scope 0 at $DIR/simplify_locals_fixedpoint.rs:+6:1: +6:2
50           return;                          // scope 0 at $DIR/simplify_locals_fixedpoint.rs:+6:2: +6:2
+   
+   
+       bb5 (cleanup): {
+           resume;                          // scope 0 at $DIR/simplify_locals_fixedpoint.rs:+0:1: +6:2
52   }
53   


thread '[mir-opt] tests/mir-opt/simplify_locals_fixedpoint.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/simplify_locals_fixedpoint.foo.SimplifyLocals-final.diff', src/tools/compiletest/src/runtest.rs:3639:21

failures:
    [mir-opt] tests/mir-opt/inline/asm_unwind.rs
    [mir-opt] tests/mir-opt/inline/cycle.rs
