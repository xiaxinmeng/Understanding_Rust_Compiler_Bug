plain
 finished in 0.505 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 255 tests
.......................................................F......................F.i.......  88/255
...............................F.........................i...........iii................ 176/255
...F..............iii.iiiii.........ii...F....i..F.............................
failures:

---- [mir-opt] tests/mir-opt/const_prop/control_flow_simplification.rs stdout ----
2 
2 
3 fn hello() -> () {
4     let mut _0: ();                      // return place in scope 0 at $DIR/control_flow_simplification.rs:+0:14: +0:14
+     let mut _1: !;                       // in scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
6     bb0: {
6     bb0: {
+         switchInt(const false) -> [0: bb2, otherwise: bb1]; // scope 0 at $DIR/control_flow_simplification.rs:+1:8: +1:21
+ 
+     bb1: {
+     bb1: {
+         _1 = begin_panic::<&str>(const "explicit panic"); // scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
+                                          // mir::Constant
+                                          // + span: $SRC_DIR/std/src/panic.rs:LL:COL
+                                          // + literal: Const { ty: fn(&str) -> ! {begin_panic::<&str>}, val: Value(<ZST>) }
+                                          // mir::Constant
+                                          // + span: $SRC_DIR/std/src/panic.rs:LL:COL
+                                          // + literal: Const { ty: &str, val: Value(Slice(..)) }
+ 
+     bb2: {
+     bb2: {
7         return;                          // scope 0 at $DIR/control_flow_simplification.rs:+4:2: +4:2
9 }


thread '[mir-opt] tests/mir-opt/const_prop/control_flow_simplification.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/control_flow_simplification.hello.PreCodegen.before.mir', src/tools/compiletest/src/runtest.rs:3639:21

---- [mir-opt] tests/mir-opt/const_prop/switch_int.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/switch_int.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_prop/switch_int.rs' panicked at 'the mir dump file for switch_int.main.SimplifyConstCondition-after-const-prop.before.mir does not exist (requested in /checkout/tests/mir-opt/const_prop/switch_int.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/dest-prop/unreachable.rs stdout ----
---- [mir-opt] tests/mir-opt/dest-prop/unreachable.rs stdout ----
14       let mut _9: T;                       // in scope 0 at $DIR/unreachable.rs:+5:14: +5:15
15       scope 1 {
16 -         debug b => _2;                   // in scope 1 at $DIR/unreachable.rs:+1:9: +1:10
- +         debug b => _1;                   // in scope 1 at $DIR/unreachable.rs:+1:9: +1:10
+ +         debug b => _8;                   // in scope 1 at $DIR/unreachable.rs:+1:9: +1:10
18       }
19   
20       bb0: {


21 -         StorageLive(_2);                 // scope 0 at $DIR/unreachable.rs:+1:9: +1:10
22 -         _2 = _1;                         // scope 0 at $DIR/unreachable.rs:+1:13: +1:14
23 +         nop;                             // scope 0 at $DIR/unreachable.rs:+1:9: +1:10
- +         nop;                             // scope 0 at $DIR/unreachable.rs:+1:13: +1:14
+ +         _8 = _1;                         // scope 0 at $DIR/unreachable.rs:+1:13: +1:14
25           StorageLive(_3);                 // scope 1 at $DIR/unreachable.rs:+2:8: +2:13
26           _3 = const false;                // scope 1 at $DIR/unreachable.rs:+2:8: +2:13
- -         goto -> bb3;                     // scope 1 at $DIR/unreachable.rs:+2:8: +2:13
- +         goto -> bb1;                     // scope 1 at $DIR/unreachable.rs:+2:8: +2:13
+           switchInt(const false) -> [0: bb3, otherwise: bb1]; // scope 1 at $DIR/unreachable.rs:+2:8: +2:13
30   
31       bb1: {


- -         StorageLive(_4);                 // scope 1 at $DIR/unreachable.rs:+3:9: +3:16
+           StorageLive(_4);                 // scope 1 at $DIR/unreachable.rs:+3:9: +3:16
33 -         StorageLive(_5);                 // scope 1 at $DIR/unreachable.rs:+3:11: +3:12
34 -         _5 = _1;                         // scope 1 at $DIR/unreachable.rs:+3:11: +3:12
35 -         StorageLive(_6);                 // scope 1 at $DIR/unreachable.rs:+3:14: +3:15

36 -         _6 = _2;                         // scope 1 at $DIR/unreachable.rs:+3:14: +3:15
37 -         _4 = g::<T>(move _5, move _6) -> bb2; // scope 1 at $DIR/unreachable.rs:+3:9: +3:16
- -                                          // mir::Constant
- -                                          // + span: $DIR/unreachable.rs:12:9: 12:10
- -                                          // + literal: Const { ty: fn(T, T) {g::<T>}, val: Value(<ZST>) }
- -     }
- -     bb2: {
- -     bb2: {
+ +         nop;                             // scope 1 at $DIR/unreachable.rs:+3:11: +3:12
+ +         nop;                             // scope 1 at $DIR/unreachable.rs:+3:11: +3:12
+ +         nop;                             // scope 1 at $DIR/unreachable.rs:+3:14: +3:15
+ +         nop;                             // scope 1 at $DIR/unreachable.rs:+3:14: +3:15
+ +         _4 = g::<T>(move _1, move _8) -> bb2; // scope 1 at $DIR/unreachable.rs:+3:9: +3:16
+                                            // mir::Constant
+                                            // + span: $DIR/unreachable.rs:12:9: 12:10
+                                            // + literal: Const { ty: fn(T, T) {g::<T>}, val: Value(<ZST>) }
+   
+       bb2: {
+       bb2: {
44 -         StorageDead(_6);                 // scope 1 at $DIR/unreachable.rs:+3:15: +3:16
45 -         StorageDead(_5);                 // scope 1 at $DIR/unreachable.rs:+3:15: +3:16
- -         StorageDead(_4);                 // scope 1 at $DIR/unreachable.rs:+3:16: +3:17
- -         _0 = const ();                   // scope 1 at $DIR/unreachable.rs:+2:14: +4:6
- -         goto -> bb5;                     // scope 1 at $DIR/unreachable.rs:+2:5: +6:6
- -     }
- -     bb3: {
- -     bb3: {
+ +         nop;                             // scope 1 at $DIR/unreachable.rs:+3:15: +3:16
+ +         nop;                             // scope 1 at $DIR/unreachable.rs:+3:15: +3:16
+           StorageDead(_4);                 // scope 1 at $DIR/unreachable.rs:+3:16: +3:17
+           _0 = const ();                   // scope 1 at $DIR/unreachable.rs:+2:14: +4:6
+           goto -> bb5;                     // scope 1 at $DIR/unreachable.rs:+2:5: +6:6
+   
+       bb3: {
+       bb3: {
52           StorageLive(_7);                 // scope 1 at $DIR/unreachable.rs:+5:9: +5:16
53 -         StorageLive(_8);                 // scope 1 at $DIR/unreachable.rs:+5:11: +5:12
54 -         _8 = _2;                         // scope 1 at $DIR/unreachable.rs:+5:11: +5:12

56 +         nop;                             // scope 1 at $DIR/unreachable.rs:+5:11: +5:12
57           StorageLive(_9);                 // scope 1 at $DIR/unreachable.rs:+5:14: +5:15
58 -         _9 = _2;                         // scope 1 at $DIR/unreachable.rs:+5:14: +5:15
- -         _7 = g::<T>(move _8, move _9) -> bb4; // scope 1 at $DIR/unreachable.rs:+5:9: +5:16
- +         _9 = _1;                         // scope 1 at $DIR/unreachable.rs:+5:14: +5:15
- +         _7 = g::<T>(move _1, move _9) -> bb2; // scope 1 at $DIR/unreachable.rs:+5:9: +5:16
+ +         _9 = _8;                         // scope 1 at $DIR/unreachable.rs:+5:14: +5:15
+           _7 = g::<T>(move _8, move _9) -> bb4; // scope 1 at $DIR/unreachable.rs:+5:9: +5:16
62                                            // mir::Constant
63                                            // + span: $DIR/unreachable.rs:14:9: 14:10
64                                            // + literal: Const { ty: fn(T, T) {g::<T>}, val: Value(<ZST>) }
65       }
66   
- -     bb4: {
- +     bb2: {
- +     bb2: {
+       bb4: {
69           StorageDead(_9);                 // scope 1 at $DIR/unreachable.rs:+5:15: +5:16
70 -         StorageDead(_8);                 // scope 1 at $DIR/unreachable.rs:+5:15: +5:16
71 +         nop;                             // scope 1 at $DIR/unreachable.rs:+5:15: +5:16

72           StorageDead(_7);                 // scope 1 at $DIR/unreachable.rs:+5:16: +5:17
73           _0 = const ();                   // scope 1 at $DIR/unreachable.rs:+4:12: +6:6
- -         goto -> bb5;                     // scope 1 at $DIR/unreachable.rs:+2:5: +6:6
- +         goto -> bb3;                     // scope 1 at $DIR/unreachable.rs:+2:5: +6:6
+           goto -> bb5;                     // scope 1 at $DIR/unreachable.rs:+2:5: +6:6
77   
- -     bb5: {
- +     bb3: {
+       bb5: {
+       bb5: {
80           StorageDead(_3);                 // scope 1 at $DIR/unreachable.rs:+6:5: +6:6
81 -         StorageDead(_2);                 // scope 0 at $DIR/unreachable.rs:+7:1: +7:2
82 +         nop;                             // scope 0 at $DIR/unreachable.rs:+7:1: +7:2

thread '[mir-opt] tests/mir-opt/dest-prop/unreachable.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/dest-prop/unreachable.f.DestinationPropagation.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/issue_101973.rs stdout ----
---- [mir-opt] tests/mir-opt/issue_101973.rs stdout ----
35           StorageLive(_4);                 // scope 0 at $DIR/issue_101973.rs:+1:5: +1:17
36           StorageLive(_14);                // scope 2 at $DIR/issue_101973.rs:8:12: 8:27
37           StorageLive(_15);                // scope 2 at $DIR/issue_101973.rs:8:12: 8:20
-           _15 = Shr(_1, const 0_i32);      // scope 2 at $DIR/issue_101973.rs:8:12: 8:20
-           _14 = BitAnd(move _15, const 255_u32); // scope 2 at $DIR/issue_101973.rs:8:12: 8:27
-           StorageDead(_15);                // scope 2 at $DIR/issue_101973.rs:8:26: 8:27
-           _4 = BitOr(const 0_u32, move _14); // scope 2 at $DIR/issue_101973.rs:8:5: 8:27
-           StorageDead(_14);                // scope 2 at $DIR/issue_101973.rs:8:26: 8:27
-           StorageLive(_6);                 // scope 0 at $DIR/issue_101973.rs:+1:31: +1:57
-           StorageLive(_7);                 // scope 0 at $DIR/issue_101973.rs:+1:31: +1:52
-           StorageLive(_8);                 // scope 0 at $DIR/issue_101973.rs:+1:32: +1:45
- -         _10 = const 8_i32 as u32 (IntToInt); // scope 0 at $DIR/issue_101973.rs:+1:32: +1:45
- -         _11 = Lt(move _10, const 32_u32); // scope 0 at $DIR/issue_101973.rs:+1:32: +1:45
- -         assert(move _11, "attempt to shift right by `{}`, which would overflow", const 8_i32) -> bb1; // scope 0 at $DIR/issue_101973.rs:+1:32: +1:45
- +         _10 = const 8_u32;               // scope 0 at $DIR/issue_101973.rs:+1:32: +1:45
- +         _11 = const true;                // scope 0 at $DIR/issue_101973.rs:+1:32: +1:45
- +         assert(const true, "attempt to shift right by `{}`, which would overflow", const 8_i32) -> bb1; // scope 0 at $DIR/issue_101973.rs:+1:32: +1:45
+           assert(const true, "attempt to shift right by `{}`, which would overflow", const 0_i32) -> bb3; // scope 2 at $DIR/issue_101973.rs:8:12: 8:20
53   
54       bb1: {

66       bb2: {
66       bb2: {
67           _6 = Shl(move _7, const 1_i32);  // scope 0 at $DIR/issue_101973.rs:+1:31: +1:57
68           StorageDead(_7);                 // scope 0 at $DIR/issue_101973.rs:+1:56: +1:57
-           _3 = rotate_right::<u32>(_4, _6) -> [return: bb3, unwind unreachable]; // scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+           _3 = rotate_right::<u32>(_4, _6) -> [return: bb4, unwind unreachable]; // scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
70                                            // mir::Constant
71                                            // + span: $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
72                                            // + literal: Const { ty: extern "rust-intrinsic" fn(u32, u32) -> u32 {rotate_right::<u32>}, val: Value(<ZST>) }
73       }
74   
75       bb3: {
75       bb3: {
+           _15 = Shr(_1, const 0_i32);      // scope 2 at $DIR/issue_101973.rs:8:12: 8:20
+           _14 = BitAnd(move _15, const 255_u32); // scope 2 at $DIR/issue_101973.rs:8:12: 8:27
+           StorageDead(_15);                // scope 2 at $DIR/issue_101973.rs:8:26: 8:27
+           _4 = BitOr(const 0_u32, move _14); // scope 2 at $DIR/issue_101973.rs:8:5: 8:27
+           StorageDead(_14);                // scope 2 at $DIR/issue_101973.rs:8:26: 8:27
+           StorageLive(_6);                 // scope 0 at $DIR/issue_101973.rs:+1:31: +1:57
+           StorageLive(_7);                 // scope 0 at $DIR/issue_101973.rs:+1:31: +1:52
+           StorageLive(_8);                 // scope 0 at $DIR/issue_101973.rs:+1:32: +1:45
+ -         _10 = const 8_i32 as u32 (IntToInt); // scope 0 at $DIR/issue_101973.rs:+1:32: +1:45
+ -         _11 = Lt(move _10, const 32_u32); // scope 0 at $DIR/issue_101973.rs:+1:32: +1:45
+ -         assert(move _11, "attempt to shift right by `{}`, which would overflow", const 8_i32) -> bb1; // scope 0 at $DIR/issue_101973.rs:+1:32: +1:45
+ +         _10 = const 8_u32;               // scope 0 at $DIR/issue_101973.rs:+1:32: +1:45
+ +         _11 = const true;                // scope 0 at $DIR/issue_101973.rs:+1:32: +1:45
+ +         assert(const true, "attempt to shift right by `{}`, which would overflow", const 8_i32) -> bb1; // scope 0 at $DIR/issue_101973.rs:+1:32: +1:45
+   
+       bb4: {
+       bb4: {
76           StorageDead(_6);                 // scope 0 at $DIR/issue_101973.rs:+1:57: +1:58
77           StorageDead(_4);                 // scope 0 at $DIR/issue_101973.rs:+1:57: +1:58
78           _2 = move _3 as i32 (IntToInt);  // scope 0 at $DIR/issue_101973.rs:+1:5: +1:65

thread '[mir-opt] tests/mir-opt/issue_101973.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/issue_101973.inner.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/pre-codegen/optimizes_into_variable.rs stdout ----
13     }
14 
15     bb0: {
15     bb0: {
+         assert(!const false, "attempt to compute `{} + {}`, which would overflow", const 2_i32, const 2_i32) -> bb1; // scope 0 at $DIR/optimizes_into_variable.rs:+1:13: +1:18
+ 
+     bb1: {
+     bb1: {
+         assert(const true, "index out of bounds: the length is {} but the index is {}", const 6_usize, const 3_usize) -> bb2; // scope 1 at $DIR/optimizes_into_variable.rs:+2:13: +2:34
+ 
+     bb2: {
+     bb2: {
16         return;                          // scope 0 at $DIR/optimizes_into_variable.rs:+4:2: +4:2
18 }


thread '[mir-opt] tests/mir-opt/pre-codegen/optimizes_into_variable.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/pre-codegen/optimizes_into_variable.main.SimplifyLocals-final.after.64bit.mir', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/simplify_if.rs stdout ----
---- [mir-opt] tests/mir-opt/simplify_if.rs stdout ----
thread '[mir-opt] tests/mir-opt/simplify_if.rs' panicked at 'the mir dump file for simplify_if.main.SimplifyConstCondition-after-const-prop.before.mir does not exist (requested in /checkout/tests/mir-opt/simplify_if.rs)', src/tools/compiletest/src/runtest.rs:3652:17

failures:
    [mir-opt] tests/mir-opt/const_prop/control_flow_simplification.rs
    [mir-opt] tests/mir-opt/const_prop/switch_int.rs
