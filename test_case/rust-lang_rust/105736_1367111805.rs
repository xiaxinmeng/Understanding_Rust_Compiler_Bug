plain
 finished in 0.627 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 215 tests
......................................................i.....................F........... 88/215
...........................................i..F................F.....F...............ii. 176/215
......i..............F.................
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] src/test/mir-opt/dataflow-const-prop/inherit_overflow.rs stdout ----
---- [mir-opt] src/test/mir-opt/dataflow-const-prop/inherit_overflow.rs stdout ----
11       scope 2 (inlined <u8 as Add>::add) { // at $DIR/inherit_overflow.rs:7:13: 7:47
12           debug self => _2;                // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
13           debug other => _3;               // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
-           let mut _4: u8;                  // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
-           let mut _5: u8;                  // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
-           let mut _6: (u8, bool);          // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           let mut _4: (u8, bool);          // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
18   
19       bb0: {


22           _2 = const u8::MAX;              // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
23           StorageLive(_3);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
24           _3 = const 1_u8;                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
-           StorageLive(_4);                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
-           _4 = const u8::MAX;              // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
-           StorageLive(_5);                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
-           _5 = const 1_u8;                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
-           _6 = CheckedAdd(const u8::MAX, const 1_u8); // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
-           assert(!move (_6.1: bool), "attempt to compute `{} + {}`, which would overflow", const u8::MAX, const 1_u8) -> bb1; // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           _4 = CheckedAdd(const u8::MAX, const 1_u8); // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           assert(!move (_4.1: bool), "attempt to compute `{} + {}`, which would overflow", const u8::MAX, const 1_u8) -> bb1; // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
32   
33       bb1: {


- -         _1 = move (_6.0: u8);            // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ -         _1 = move (_4.0: u8);            // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
35 +         _1 = const 0_u8;                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
-           StorageDead(_5);                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
-           StorageDead(_4);                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
38           StorageDead(_3);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
39           StorageDead(_2);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
40           StorageDead(_1);                 // scope 0 at $DIR/inherit_overflow.rs:+3:47: +3:48

thread '[mir-opt] src/test/mir-opt/dataflow-const-prop/inherit_overflow.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/dataflow-const-prop/inherit_overflow.main.DataflowConstProp.diff', src/tools/compiletest/src/runtest.rs:3464:21

---- [mir-opt] src/test/mir-opt/inline/inline_generator.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/inline_generator.rs stdout ----
7       let mut _2: std::pin::Pin<&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]>; // in scope 0 at $DIR/inline_generator.rs:+1:14: +1:32
8       let mut _3: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 0 at $DIR/inline_generator.rs:+1:23: +1:31
9       let mut _4: [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 0 at $DIR/inline_generator.rs:+1:28: +1:31
- +     let mut _7: bool;                    // in scope 0 at $DIR/inline_generator.rs:+1:14: +1:46
+ +     let mut _5: bool;                    // in scope 0 at $DIR/inline_generator.rs:+1:14: +1:46
11       scope 1 {
12           debug _r => _1;                  // in scope 1 at $DIR/inline_generator.rs:+1:9: +1:11

15 +     }
15 +     }
16 +     scope 3 (inlined Pin::<&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]>::new) { // at $DIR/inline_generator.rs:9:14: 9:32
17 +         debug pointer => _3;             // in scope 3 at $SRC_DIR/core/src/pin.rs:LL:COL
- +         let mut _5: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 3 at $SRC_DIR/core/src/pin.rs:LL:COL
19 +         scope 4 {
20 +             scope 5 (inlined Pin::<&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]>::new_unchecked) { // at $SRC_DIR/core/src/pin.rs:LL:COL
- +                 debug pointer => _5;     // in scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
- +                 let mut _6: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +                 debug pointer => _3;     // in scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
24 +         }
25 +     }


26 +     scope 6 (inlined g::{closure#0}) {   // at $DIR/inline_generator.rs:9:14: 9:46
- +         debug a => _7;                   // in scope 6 at $DIR/inline_generator.rs:15:6: 15:7
- +         let mut _8: i32;                 // in scope 6 at $DIR/inline_generator.rs:15:17: 15:39
- +         let mut _9: u32;                 // in scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         debug a => _5;                   // in scope 6 at $DIR/inline_generator.rs:15:6: 15:7
+ +         let mut _6: i32;                 // in scope 6 at $DIR/inline_generator.rs:15:17: 15:39
+ +         let mut _7: u32;                 // in scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         let mut _8: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         let mut _9: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 6 at $DIR/inline_generator.rs:15:5: 15:41
30 +         let mut _10: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 6 at $DIR/inline_generator.rs:15:5: 15:41
- +         let mut _11: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 6 at $DIR/inline_generator.rs:15:5: 15:41
- +         let mut _12: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 6 at $DIR/inline_generator.rs:15:5: 15:41
34   
35       bb0: {

55 -     }
55 -     }
56 - 
57 -     bb2: {
- +         StorageLive(_5);                 // scope 4 at $SRC_DIR/core/src/pin.rs:LL:COL
- +         _5 = move _3;                    // scope 4 at $SRC_DIR/core/src/pin.rs:LL:COL
- +         StorageLive(_6);                 // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
- +         _6 = move _5;                    // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
62 +         Deinit(_2);                      // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
- +         (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]) = move _6; // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
- +         StorageDead(_6);                 // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
- +         StorageDead(_5);                 // scope 4 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +         (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]) = move _3; // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
66           StorageDead(_3);                 // scope 0 at $DIR/inline_generator.rs:+1:31: +1:32
67 -         _1 = <[generator@$DIR/inline_generator.rs:15:5: 15:8] as Generator<bool>>::resume(move _2, const false) -> [return: bb3, unwind: bb4]; // scope 0 at $DIR/inline_generator.rs:+1:14: +1:46
68 -                                          // mir::Constant

69 -                                          // + span: $DIR/inline_generator.rs:9:33: 9:39
70 -                                          // + literal: Const { ty: for<'a> fn(Pin<&'a mut [generator@$DIR/inline_generator.rs:15:5: 15:8]>, bool) -> GeneratorState<<[generator@$DIR/inline_generator.rs:15:5: 15:8] as Generator<bool>>::Yield, <[generator@$DIR/inline_generator.rs:15:5: 15:8] as Generator<bool>>::Return> {<[generator@$DIR/inline_generator.rs:15:5: 15:8] as Generator<bool>>::resume}, val: Value(<ZST>) }
- +         StorageLive(_7);                 // scope 0 at $DIR/inline_generator.rs:+1:14: +1:46
- +         _7 = const false;                // scope 0 at $DIR/inline_generator.rs:+1:14: +1:46
- +         _10 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
- +         _9 = discriminant((*_10));       // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
- +         switchInt(move _9) -> [0: bb3, 1: bb8, 3: bb7, otherwise: bb9]; // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         StorageLive(_5);                 // scope 0 at $DIR/inline_generator.rs:+1:14: +1:46
+ +         _5 = const false;                // scope 0 at $DIR/inline_generator.rs:+1:14: +1:46
+ +         _8 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         _7 = discriminant((*_8));        // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         switchInt(move _7) -> [0: bb3, 1: bb8, 3: bb7, otherwise: bb9]; // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
77   
78 -     bb3: {

79 +     bb1: {
79 +     bb1: {
- +         StorageDead(_7);                 // scope 0 at $DIR/inline_generator.rs:+1:14: +1:46
+ +         StorageDead(_5);                 // scope 0 at $DIR/inline_generator.rs:+1:14: +1:46
81           StorageDead(_2);                 // scope 0 at $DIR/inline_generator.rs:+1:45: +1:46
82           StorageDead(_4);                 // scope 0 at $DIR/inline_generator.rs:+1:46: +1:47
83           _0 = const ();                   // scope 0 at $DIR/inline_generator.rs:+0:11: +2:2
91 +     }
92 + 
93 +     bb3: {
93 +     bb3: {
- +         StorageLive(_8);                 // scope 6 at $DIR/inline_generator.rs:15:17: 15:39
- +         switchInt(move _7) -> [0: bb5, otherwise: bb4]; // scope 6 at $DIR/inline_generator.rs:15:20: 15:21
+ +         StorageLive(_6);                 // scope 6 at $DIR/inline_generator.rs:15:17: 15:39
+ +         switchInt(move _5) -> [0: bb5, otherwise: bb4]; // scope 6 at $DIR/inline_generator.rs:15:20: 15:21
97 + 
98 +     bb4: {


- +         _8 = const 7_i32;                // scope 6 at $DIR/inline_generator.rs:15:24: 15:25
+ +         _6 = const 7_i32;                // scope 6 at $DIR/inline_generator.rs:15:24: 15:25
100 +         goto -> bb6;                     // scope 6 at $DIR/inline_generator.rs:15:17: 15:39
102 + 

103 +     bb5: {
103 +     bb5: {
- +         _8 = const 13_i32;               // scope 6 at $DIR/inline_generator.rs:15:35: 15:37
+ +         _6 = const 13_i32;               // scope 6 at $DIR/inline_generator.rs:15:35: 15:37
105 +         goto -> bb6;                     // scope 6 at $DIR/inline_generator.rs:15:17: 15:39
107 + 

108 +     bb6: {
108 +     bb6: {
109 +         Deinit(_1);                      // scope 6 at $DIR/inline_generator.rs:15:11: 15:39
- +         ((_1 as Yielded).0: i32) = move _8; // scope 6 at $DIR/inline_generator.rs:15:11: 15:39
+ +         ((_1 as Yielded).0: i32) = move _6; // scope 6 at $DIR/inline_generator.rs:15:11: 15:39
111 +         discriminant(_1) = 0;            // scope 6 at $DIR/inline_generator.rs:15:11: 15:39
- +         _11 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:11: 15:39
- +         discriminant((*_11)) = 3;        // scope 6 at $DIR/inline_generator.rs:15:11: 15:39
+ +         _9 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:11: 15:39
+ +         discriminant((*_9)) = 3;         // scope 6 at $DIR/inline_generator.rs:15:11: 15:39
114 +         goto -> bb1;                     // scope 0 at $DIR/inline_generator.rs:15:11: 15:39
116 + 

117 +     bb7: {
117 +     bb7: {
- +         StorageLive(_8);                 // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
- +         StorageDead(_8);                 // scope 6 at $DIR/inline_generator.rs:15:38: 15:39
+ +         StorageLive(_6);                 // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         StorageDead(_6);                 // scope 6 at $DIR/inline_generator.rs:15:38: 15:39
120 +         Deinit(_1);                      // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
- +         ((_1 as Complete).0: bool) = move _7; // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
+ +         ((_1 as Complete).0: bool) = move _5; // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
122 +         discriminant(_1) = 1;            // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
- +         _12 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
- +         discriminant((*_12)) = 1;        // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
+ +         _10 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
+ +         discriminant((*_10)) = 1;        // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
125 +         goto -> bb1;                     // scope 0 at $DIR/inline_generator.rs:15:41: 15:41
127 + 


thread '[mir-opt] src/test/mir-opt/inline/inline_generator.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_generator.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3464:21
---- [mir-opt] src/test/mir-opt/issue_101973.rs stdout ----
---- [mir-opt] src/test/mir-opt/issue_101973.rs stdout ----
26       scope 3 (inlined core::num::<impl u32>::rotate_right) { // at $DIR/issue_101973.rs:14:5: 14:58
27           debug self => _4;                // in scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
28           debug n => _6;                   // in scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-           let mut _15: u32;                // in scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-           let mut _16: u32;                // in scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
32   
33       bb0: {

54       bb2: {
54       bb2: {
55           _6 = move (_11.0: u32);          // scope 0 at $DIR/issue_101973.rs:+1:31: +1:57
56           StorageDead(_7);                 // scope 0 at $DIR/issue_101973.rs:+1:56: +1:57
-           StorageLive(_15);                // scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-           _15 = _4;                        // scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-           StorageLive(_16);                // scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-           _16 = _6;                        // scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-           _3 = rotate_right::<u32>(move _15, move _16) -> bb4; // scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+           _3 = rotate_right::<u32>(move _4, move _6) -> bb4; // scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
62                                            // mir::Constant
63                                            // + span: $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
64                                            // + literal: Const { ty: extern "rust-intrinsic" fn(u32, u32) -> u32 {rotate_right::<u32>}, val: Value(<ZST>) }
81       }
82   
83       bb4: {
83       bb4: {
-           StorageDead(_16);                // scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-           StorageDead(_15);                // scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
86           StorageDead(_6);                 // scope 0 at $DIR/issue_101973.rs:+1:57: +1:58
87           StorageDead(_4);                 // scope 0 at $DIR/issue_101973.rs:+1:57: +1:58
88           _2 = move _3 as i32 (IntToInt);  // scope 0 at $DIR/issue_101973.rs:+1:5: +1:65

thread '[mir-opt] src/test/mir-opt/issue_101973.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_101973.inner.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3464:21
---- [mir-opt] src/test/mir-opt/issues/issue_59352.rs stdout ----
---- [mir-opt] src/test/mir-opt/issues/issue_59352.rs stdout ----
3 fn num_to_digit(_1: char) -> u32 {
4     debug num => _1;                     // in scope 0 at $DIR/issue_59352.rs:+0:21: +0:24
5     let mut _0: u32;                     // return place in scope 0 at $DIR/issue_59352.rs:+0:35: +0:38
-     let mut _2: std::option::Option<u32>; // in scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
-     let mut _3: u32;                     // in scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
-     let mut _9: isize;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+     let mut _2: char;                    // in scope 0 at $DIR/issue_59352.rs:+2:8: +2:11
+     let mut _3: std::option::Option<u32>; // in scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
+     let mut _4: u32;                     // in scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
9     scope 1 (inlined char::methods::<impl char>::is_digit) { // at $DIR/issue_59352.rs:14:8: 14:23
-         debug self => _1;                // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         debug radix => _3;               // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         let mut _4: &std::option::Option<u32>; // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         let _5: std::option::Option<u32>; // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         let mut _6: char;                // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         debug self => _2;                // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         debug radix => _4;               // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         let mut _5: &std::option::Option<u32>; // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         let _6: std::option::Option<u32>; // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
15         scope 2 (inlined Option::<u32>::is_some) { // at $SRC_DIR/core/src/char/methods.rs:LL:COL
-             debug self => _4;            // in scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
+             debug self => _5;            // in scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
+             let mut _7: isize;           // in scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
18     }
18     }
19     scope 3 (inlined #[track_caller] Option::<u32>::unwrap) { // at $DIR/issue_59352.rs:14:26: 14:50

-         debug self => _2;                // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
-         let mut _7: isize;               // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
-         let mut _8: !;                   // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         debug self => _3;                // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         let mut _8: isize;               // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         let mut _9: !;                   // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
23         scope 4 {
24             debug val => _0;             // in scope 4 at $SRC_DIR/core/src/option.rs:LL:COL

26     }
27 
28     bb0: {
28     bb0: {
-         StorageLive(_3);                 // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
-         StorageLive(_4);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         StorageLive(_2);                 // scope 0 at $DIR/issue_59352.rs:+2:8: +2:11
+         _2 = _1;                         // scope 0 at $DIR/issue_59352.rs:+2:8: +2:11
+         StorageLive(_4);                 // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
31         StorageLive(_5);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
32         StorageLive(_6);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         _6 = _1;                         // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         _5 = char::methods::<impl char>::to_digit(move _6, const 8_u32) -> bb5; // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         _6 = char::methods::<impl char>::to_digit(move _2, const 8_u32) -> bb5; // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
35                                          // mir::Constant
36                                          // + span: $SRC_DIR/core/src/char/methods.rs:LL:COL
37                                          // + literal: Const { ty: fn(char, u32) -> Option<u32> {char::methods::<impl char>::to_digit}, val: Value(<ZST>) }
38     }
39 
40     bb1: {
40     bb1: {
-         StorageLive(_2);                 // scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
-         _2 = char::methods::<impl char>::to_digit(move _1, const 8_u32) -> bb2; // scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
+         StorageDead(_7);                 // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
+         StorageLive(_3);                 // scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
+         _3 = char::methods::<impl char>::to_digit(move _1, const 8_u32) -> bb2; // scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
43                                          // mir::Constant
44                                          // + span: $DIR/issue_59352.rs:14:30: 14:38
45                                          // + literal: Const { ty: fn(char, u32) -> Option<u32> {char::methods::<impl char>::to_digit}, val: Value(<ZST>) }
46     }
47 
48     bb2: {
48     bb2: {
-         _7 = discriminant(_2);           // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
-         switchInt(move _7) -> [0: bb6, 1: bb8, otherwise: bb7]; // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         _8 = discriminant(_3);           // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         switchInt(move _8) -> [0: bb6, 1: bb8, otherwise: bb7]; // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
52 
53     bb3: {


+         StorageDead(_7);                 // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
54         _0 = const 0_u32;                // scope 0 at $DIR/issue_59352.rs:+2:60: +2:61
55         goto -> bb4;                     // scope 0 at $DIR/issue_59352.rs:+2:5: +2:63

60     }
61 
62     bb5: {
62     bb5: {
-         _4 = &_5;                        // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         StorageDead(_6);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         _9 = discriminant((*_4));        // scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
-         StorageDead(_4);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         _5 = &_6;                        // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         StorageLive(_7);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         _7 = discriminant((*_5));        // scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
67         StorageDead(_5);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         StorageDead(_3);                 // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
-         switchInt(move _9) -> [1: bb1, otherwise: bb3]; // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
+         StorageDead(_6);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         StorageDead(_4);                 // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
+         StorageDead(_2);                 // scope 0 at $DIR/issue_59352.rs:+2:22: +2:23
+         switchInt(move _7) -> [1: bb1, otherwise: bb3]; // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
71 
72     bb6: {


-         StorageLive(_8);                 // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
-         _8 = core::panicking::panic(const "called `Option::unwrap()` on a `None` value"); // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         StorageLive(_9);                 // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         _9 = core::panicking::panic(const "called `Option::unwrap()` on a `None` value"); // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
75                                          // mir::Constant
76                                          // + span: $SRC_DIR/core/src/option.rs:LL:COL
77                                          // + literal: Const { ty: fn(&'static str) -> ! {core::panicking::panic}, val: Value(<ZST>) }
85     }
86 
87     bb8: {
87     bb8: {
-         _0 = move ((_2 as Some).0: u32); // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
-         StorageDead(_2);                 // scope 0 at $DIR/issue_59352.rs:+2:49: +2:50
+         _0 = move ((_3 as Some).0: u32); // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         StorageDead(_3);                 // scope 0 at $DIR/issue_59352.rs:+2:49: +2:50
90         goto -> bb4;                     // scope 0 at $DIR/issue_59352.rs:+2:5: +2:63
92 }


thread '[mir-opt] src/test/mir-opt/issues/issue_59352.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issues/issue_59352.num_to_digit.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3464:21
---- [mir-opt] src/test/mir-opt/separate_const_switch.rs stdout ----
17           scope 2 {
17           scope 2 {
18               scope 8 (inlined #[track_caller] <Result<i32, i32> as FromResidual<Result<Infallible, i32>>>::from_residual) { // at $DIR/separate_const_switch.rs:25:8: 25:10
19                   debug residual => _8;    // in scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
-                   let _16: i32;            // in scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
-                   let mut _17: i32;        // in scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
-                   let mut _18: i32;        // in scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
+                   let mut _14: i32;        // in scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
+                   let mut _15: i32;        // in scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
23                   scope 9 {
-                       debug e => _16;      // in scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
+                       debug e => _15;      // in scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
26               }
27           }


34       scope 5 (inlined <Result<i32, i32> as Try>::branch) { // at $DIR/separate_const_switch.rs:25:8: 25:10
35           debug self => _4;                // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
36           let mut _10: isize;              // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
-           let _11: i32;                    // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
-           let mut _12: i32;                // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
-           let _13: i32;                    // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
-           let mut _14: std::result::Result<std::convert::Infallible, i32>; // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
-           let mut _15: i32;                // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
+           let mut _11: i32;                // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
+           let mut _12: std::result::Result<std::convert::Infallible, i32>; // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
+           let mut _13: i32;                // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
42           scope 6 {
43               debug v => _11;              // in scope 6 at $SRC_DIR/core/src/result.rs:LL:COL


52           StorageLive(_3);                 // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10
53           StorageLive(_4);                 // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:9
54           _4 = _1;                         // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:9
+           StorageLive(_11);                // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10
+           StorageLive(_13);                // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10
55           _10 = discriminant(_4);          // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
56 -         switchInt(move _10) -> [0: bb7, 1: bb5, otherwise: bb6]; // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
57 +         switchInt(move _10) -> [0: bb6, 1: bb4, otherwise: bb5]; // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
58       }
59   
60       bb1: {
60       bb1: {
+ -         StorageDead(_13);                // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10
+ -         StorageDead(_11);                // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10
61 -         StorageDead(_4);                 // scope 0 at $DIR/separate_const_switch.rs:+1:9: +1:10
62 -         _5 = discriminant(_3);           // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10
63 -         switchInt(move _5) -> [0: bb2, 1: bb4, otherwise: bb3]; // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10

87           _6 = ((_3 as Break).0: std::result::Result<std::convert::Infallible, i32>); // scope 0 at $DIR/separate_const_switch.rs:+1:9: +1:10
88           StorageLive(_8);                 // scope 2 at $DIR/separate_const_switch.rs:+1:9: +1:10
89           _8 = _6;                         // scope 2 at $DIR/separate_const_switch.rs:+1:9: +1:10
-           StorageLive(_16);                // scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
-           _16 = move ((_8 as Err).0: i32); // scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageLive(_17);                // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageLive(_18);                // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
-           _18 = move _16;                  // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
- -         _17 = <i32 as From<i32>>::from(move _18) -> bb8; // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
- +         _17 = <i32 as From<i32>>::from(move _18) -> bb7; // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
+           StorageLive(_15);                // scope 2 at $DIR/separate_const_switch.rs:+1:8: +1:10
+           _15 = move ((_8 as Err).0: i32); // scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
+           StorageLive(_14);                // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
+ -         _14 = <i32 as From<i32>>::from(move _15) -> bb8; // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
+ +         _14 = <i32 as From<i32>>::from(move _15) -> bb7; // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
97                                            // mir::Constant
98                                            // + span: $SRC_DIR/core/src/result.rs:LL:COL
99                                            // + literal: Const { ty: fn(i32) -> i32 {<i32 as From<i32>>::from}, val: Value(<ZST>) }
101   
102 -     bb5: {
103 +     bb4: {
103 +     bb4: {
-           StorageLive(_13);                // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
105           _13 = move ((_4 as Err).0: i32); // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageLive(_14);                // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageLive(_15);                // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
-           _15 = move _13;                  // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
-           Deinit(_14);                     // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
-           ((_14 as Err).0: i32) = move _15; // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
-           discriminant(_14) = 1;           // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageDead(_15);                // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
+           StorageLive(_12);                // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
+           Deinit(_12);                     // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
+           ((_12 as Err).0: i32) = move _13; // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
+           discriminant(_12) = 1;           // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
113           Deinit(_3);                      // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
-           ((_3 as Break).0: std::result::Result<std::convert::Infallible, i32>) = move _14; // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
+           ((_3 as Break).0: std::result::Result<std::convert::Infallible, i32>) = move _12; // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
115           discriminant(_3) = 1;            // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageDead(_14);                // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageDead(_13);                // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
+           StorageDead(_12);                // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
118 -         goto -> bb1;                     // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
+ +         StorageDead(_13);                // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10
+ +         StorageDead(_11);                // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10
119 +         StorageDead(_4);                 // scope 0 at $DIR/separate_const_switch.rs:+1:9: +1:10
120 +         _5 = discriminant(_3);           // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10
121 +         switchInt(move _5) -> [0: bb1, 1: bb3, otherwise: bb2]; // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10
128   
129 -     bb7: {
130 +     bb6: {
130 +     bb6: {
-           StorageLive(_11);                // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
132           _11 = move ((_4 as Ok).0: i32);  // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageLive(_12);                // scope 6 at $SRC_DIR/core/src/result.rs:LL:COL
-           _12 = move _11;                  // scope 6 at $SRC_DIR/core/src/result.rs:LL:COL
135           Deinit(_3);                      // scope 6 at $SRC_DIR/core/src/result.rs:LL:COL
-           ((_3 as Continue).0: i32) = move _12; // scope 6 at $SRC_DIR/core/src/result.rs:LL:COL
+           ((_3 as Continue).0: i32) = move _11; // scope 6 at $SRC_DIR/core/src/result.rs:LL:COL
137           discriminant(_3) = 0;            // scope 6 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageDead(_12);                // scope 6 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageDead(_11);                // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
140 -         goto -> bb1;                     // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
+ +         StorageDead(_13);                // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10
+ +         StorageDead(_11);                // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10
141 +         StorageDead(_4);                 // scope 0 at $DIR/separate_const_switch.rs:+1:9: +1:10
142 +         _5 = discriminant(_3);           // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10
143 +         switchInt(move _5) -> [0: bb1, 1: bb3, otherwise: bb2]; // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10
145   
146 -     bb8: {
147 +     bb7: {
147 +     bb7: {
-           StorageDead(_18);                // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
149           Deinit(_0);                      // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
-           ((_0 as Err).0: i32) = move _17; // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
+           ((_0 as Err).0: i32) = move _14; // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
151           discriminant(_0) = 1;            // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageDead(_17);                // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageDead(_16);                // scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
+           StorageDead(_14);                // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
+           StorageDead(_15);                // scope 2 at $DIR/separate_const_switch.rs:+1:8: +1:10
154           StorageDead(_8);                 // scope 2 at $DIR/separate_const_switch.rs:+1:9: +1:10
155           StorageDead(_6);                 // scope 0 at $DIR/separate_const_switch.rs:+1:9: +1:10
156           StorageDead(_2);                 // scope 0 at $DIR/separate_const_switch.rs:+1:10: +1:11

thread '[mir-opt] src/test/mir-opt/separate_const_switch.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/separate_const_switch.identity.SeparateConstSwitch.diff', src/tools/compiletest/src/runtest.rs:3464:21

failures:
    [mir-opt] src/test/mir-opt/dataflow-const-prop/inherit_overflow.rs
    [mir-opt] src/test/mir-opt/inline/inline_generator.rs
