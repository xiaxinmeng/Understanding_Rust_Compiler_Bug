plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:ece894d15649f0f9d7884f915dc821f00bd0418b)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 230 tests
..................................................................i..................... 88/230
.F......................................................i..F....................F..F.... 176/230
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...........ii.......i......F..F.....................F.

---- [mir-opt] tests/mir-opt/dataflow-const-prop/inherit_overflow.rs stdout ----
---- [mir-opt] tests/mir-opt/dataflow-const-prop/inherit_overflow.rs stdout ----
10       scope 2 (inlined <u8 as Add>::add) { // at $DIR/inherit_overflow.rs:7:13: 7:47
11           debug self => _1;                // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
12           debug other => _2;               // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
-           let mut _3: u8;                  // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
-           let mut _4: u8;                  // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
-           let mut _5: (u8, bool);          // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           let mut _3: (u8, bool);          // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
17   
18       bb0: {


20           _1 = const u8::MAX;              // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
21           StorageLive(_2);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
22           _2 = const 1_u8;                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
-           _5 = CheckedAdd(const u8::MAX, const 1_u8); // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
-           assert(!move (_5.1: bool), "attempt to compute `{} + {}`, which would overflow", const u8::MAX, const 1_u8) -> bb1; // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           _3 = CheckedAdd(const u8::MAX, const 1_u8); // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           assert(!move (_3.1: bool), "attempt to compute `{} + {}`, which would overflow", const u8::MAX, const 1_u8) -> bb1; // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
26   
27       bb1: {


+           StorageDead(_2);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
+           StorageDead(_1);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
28           return;                          // scope 0 at $DIR/inherit_overflow.rs:+4:2: +4:2
30   }


thread '[mir-opt] tests/mir-opt/dataflow-const-prop/inherit_overflow.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/dataflow-const-prop/inherit_overflow.main.DataflowConstProp.diff', src/tools/compiletest/src/runtest.rs:3483:21

---- [mir-opt] tests/mir-opt/inline/inline_generator.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_generator.rs stdout ----
7       let mut _2: std::pin::Pin<&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]>; // in scope 0 at $DIR/inline_generator.rs:+1:14: +1:32
8       let mut _3: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 0 at $DIR/inline_generator.rs:+1:23: +1:31
9       let mut _4: [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 0 at $DIR/inline_generator.rs:+1:28: +1:31
- +     let mut _7: bool;                    // in scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
+ +     let mut _5: bool;                    // in scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
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


26 +     scope 6 (inlined g::{closure#0}) {   // at $DIR/inline_generator.rs:9:33: 9:46
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
- +         StorageLive(_7);                 // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
- +         _7 = const false;                // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
- +         _10 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
- +         _9 = discriminant((*_10));       // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
- +         switchInt(move _9) -> [0: bb3, 1: bb8, 3: bb7, otherwise: bb9]; // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         StorageLive(_5);                 // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
+ +         _5 = const false;                // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
+ +         _8 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         _7 = discriminant((*_8));        // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         switchInt(move _7) -> [0: bb3, 1: bb8, 3: bb7, otherwise: bb9]; // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
77   
78 -     bb3: {

79 +     bb1: {
79 +     bb1: {
- +         StorageDead(_7);                 // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
+ +         StorageDead(_5);                 // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
81           StorageDead(_2);                 // scope 0 at $DIR/inline_generator.rs:+1:45: +1:46
82           StorageDead(_4);                 // scope 0 at $DIR/inline_generator.rs:+1:46: +1:47
83           _0 = const ();                   // scope 0 at $DIR/inline_generator.rs:+0:11: +2:2
91 +     }
92 + 
93 +     bb3: {
93 +     bb3: {
- +         StorageLive(_8);                 // scope 6 at $DIR/inline_generator.rs:15:17: 15:39
- +         switchInt(_7) -> [0: bb5, otherwise: bb4]; // scope 6 at $DIR/inline_generator.rs:15:20: 15:21
+ +         StorageLive(_6);                 // scope 6 at $DIR/inline_generator.rs:15:17: 15:39
+ +         switchInt(_5) -> [0: bb5, otherwise: bb4]; // scope 6 at $DIR/inline_generator.rs:15:20: 15:21
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
- +         ((_1 as Complete).0: bool) = _7; // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
+ +         ((_1 as Complete).0: bool) = _5; // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
122 +         discriminant(_1) = 1;            // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
- +         _12 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
- +         discriminant((*_12)) = 1;        // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
+ +         _10 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
+ +         discriminant((*_10)) = 1;        // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
125 +         goto -> bb1;                     // scope 0 at $DIR/inline_generator.rs:15:41: 15:41
127 + 


thread '[mir-opt] tests/mir-opt/inline/inline_generator.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_generator.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3483:21
---- [mir-opt] tests/mir-opt/issues/issue_59352.rs stdout ----
---- [mir-opt] tests/mir-opt/issues/issue_59352.rs stdout ----
5     let mut _0: u32;                     // return place in scope 0 at $DIR/issue_59352.rs:+0:35: +0:38
6     let mut _2: std::option::Option<u32>; // in scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
7     let mut _3: u32;                     // in scope 0 at $DIR/issue_59352.rs:+2:12: +2:23
-     let mut _9: isize;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
9     scope 1 (inlined char::methods::<impl char>::is_digit) { // at $DIR/issue_59352.rs:14:12: 14:23
10         debug self => _1;                // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
11         debug radix => _3;               // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL

12         let mut _4: &std::option::Option<u32>; // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
13         let _5: std::option::Option<u32>; // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         let mut _6: char;                // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
15         scope 2 (inlined Option::<u32>::is_some) { // at $SRC_DIR/core/src/char/methods.rs:LL:COL
16             debug self => _4;            // in scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
+             let mut _6: isize;           // in scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
18     }
18     }
19     scope 3 (inlined #[track_caller] Option::<u32>::unwrap) { // at $DIR/issue_59352.rs:14:42: 14:50

29         StorageLive(_3);                 // scope 0 at $DIR/issue_59352.rs:+2:12: +2:23
30         StorageLive(_4);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
31         StorageLive(_5);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         StorageLive(_6);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         _6 = _1;                         // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         _5 = char::methods::<impl char>::to_digit(move _6, const 8_u32) -> bb5; // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         _5 = char::methods::<impl char>::to_digit(_1, const 8_u32) -> bb5; // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
35                                          // mir::Constant
36                                          // + span: $SRC_DIR/core/src/char/methods.rs:LL:COL
37                                          // + literal: Const { ty: fn(char, u32) -> Option<u32> {char::methods::<impl char>::to_digit}, val: Value(<ZST>) }
39 
40     bb1: {
40     bb1: {
41         StorageLive(_2);                 // scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
-         _2 = char::methods::<impl char>::to_digit(move _1, const 8_u32) -> bb2; // scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
+         _2 = char::methods::<impl char>::to_digit(_1, const 8_u32) -> bb2; // scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
43                                          // mir::Constant
44                                          // + span: $DIR/issue_59352.rs:14:30: 14:38
45                                          // + literal: Const { ty: fn(char, u32) -> Option<u32> {char::methods::<impl char>::to_digit}, val: Value(<ZST>) }
61 
62     bb5: {
62     bb5: {
63         _4 = &_5;                        // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         StorageDead(_6);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         _9 = discriminant((*_4));        // scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
+         _6 = discriminant((*_4));        // scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
66         StorageDead(_4);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
67         StorageDead(_5);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
68         StorageDead(_3);                 // scope 0 at $DIR/issue_59352.rs:+2:12: +2:23

-         switchInt(move _9) -> [1: bb1, otherwise: bb3]; // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
+         switchInt(move _6) -> [1: bb1, otherwise: bb3]; // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
71 
72     bb6: {


thread '[mir-opt] tests/mir-opt/issues/issue_59352.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/issues/issue_59352.num_to_digit.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3483:21
---- [mir-opt] tests/mir-opt/issue_101973.rs stdout ----
---- [mir-opt] tests/mir-opt/issue_101973.rs stdout ----
26       scope 3 (inlined core::num::<impl u32>::rotate_right) { // at $DIR/issue_101973.rs:14:18: 14:58
27           debug self => _4;                // in scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
28           debug n => _6;                   // in scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-           let mut _15: u32;                // in scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-           let mut _16: u32;                // in scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
32   
33       bb0: {

71       }
71       }
72   
73       bb4: {
+           StorageDead(_6);                 // scope 0 at $DIR/issue_101973.rs:+1:57: +1:58
+           StorageDead(_4);                 // scope 0 at $DIR/issue_101973.rs:+1:57: +1:58
74           _2 = move _3 as i32 (IntToInt);  // scope 0 at $DIR/issue_101973.rs:+1:5: +1:65
75           StorageDead(_3);                 // scope 0 at $DIR/issue_101973.rs:+1:64: +1:65
76           _0 = move _2 as i64 (IntToInt);  // scope 0 at $DIR/issue_101973.rs:+1:5: +1:72

thread '[mir-opt] tests/mir-opt/issue_101973.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/issue_101973.inner.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3483:21
---- [mir-opt] tests/mir-opt/separate_const_switch.rs stdout ----
---- [mir-opt] tests/mir-opt/separate_const_switch.rs stdout ----
12       let mut _7: !;                       // in scope 0 at $DIR/separate_const_switch.rs:+1:9: +1:10
13       let mut _8: std::result::Result<std::convert::Infallible, i32>; // in scope 0 at $DIR/separate_const_switch.rs:+1:9: +1:10
14       let _9: i32;                         // in scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10
+       let mut _16: i32;                    // in scope 0 at $SRC_DIR/core/src/result.rs:LL:COL
15       scope 1 {
16           debug residual => _6;            // in scope 1 at $DIR/separate_const_switch.rs:+1:9: +1:10
17           scope 2 {

18               scope 8 (inlined #[track_caller] <Result<i32, i32> as FromResidual<Result<Infallible, i32>>>::from_residual) { // at $DIR/separate_const_switch.rs:25:8: 25:10
19                   debug residual => _8;    // in scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
-                   let _16: i32;            // in scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
-                   let mut _17: i32;        // in scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
-                   let mut _18: i32;        // in scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
+                   let _14: i32;            // in scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
+                   let mut _15: i32;        // in scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
23                   scope 9 {
-                       debug e => _16;      // in scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
+                       debug e => _14;      // in scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
25                       scope 10 (inlined <i32 as From<i32>>::from) { // at $SRC_DIR/core/src/result.rs:LL:COL
-                           debug t => _18;  // in scope 10 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
+                           debug t => _16;  // in scope 10 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
28                   }
29               }


38           debug self => _4;                // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
39           let mut _10: isize;              // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
40           let _11: i32;                    // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
-           let mut _12: i32;                // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
-           let _13: i32;                    // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
-           let mut _14: std::result::Result<std::convert::Infallible, i32>; // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
-           let mut _15: i32;                // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
+           let _12: i32;                    // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
+           let mut _13: std::result::Result<std::convert::Infallible, i32>; // in scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
45           scope 6 {
46               debug v => _11;              // in scope 6 at $SRC_DIR/core/src/result.rs:LL:COL

48           scope 7 {
48           scope 7 {
-               debug e => _13;              // in scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
+               debug e => _12;              // in scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
51       }
52   


90           _6 = ((_3 as Break).0: std::result::Result<std::convert::Infallible, i32>); // scope 0 at $DIR/separate_const_switch.rs:+1:9: +1:10
91           StorageLive(_8);                 // scope 2 at $DIR/separate_const_switch.rs:+1:9: +1:10
92           _8 = _6;                         // scope 2 at $DIR/separate_const_switch.rs:+1:9: +1:10
-           StorageLive(_16);                // scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
-           _16 = move ((_8 as Err).0: i32); // scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageLive(_17);                // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageLive(_18);                // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
-           _18 = move _16;                  // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
-           _17 = move _18;                  // scope 10 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
-           StorageDead(_18);                // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
+           StorageLive(_14);                // scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
+           _14 = move ((_8 as Err).0: i32); // scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
+           StorageLive(_15);                // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
+           StorageLive(_16);                // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
+           _16 = move _14;                  // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
+           _15 = move _16;                  // scope 10 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
+           StorageDead(_16);                // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
100           Deinit(_0);                      // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
-           ((_0 as Err).0: i32) = move _17; // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
+           ((_0 as Err).0: i32) = move _15; // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
102           discriminant(_0) = 1;            // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageDead(_17);                // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageDead(_16);                // scope 8 at $SRC_DIR/core/src/result.rs:LL:COL
+           StorageDead(_15);                // scope 9 at $SRC_DIR/core/src/result.rs:LL:COL
105           StorageDead(_8);                 // scope 2 at $DIR/separate_const_switch.rs:+1:9: +1:10
106           StorageDead(_6);                 // scope 0 at $DIR/separate_const_switch.rs:+1:9: +1:10
107           StorageDead(_2);                 // scope 0 at $DIR/separate_const_switch.rs:+1:10: +1:11
111   
112 -     bb5: {
113 +     bb4: {
113 +     bb4: {
-           StorageLive(_13);                // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
-           _13 = move ((_4 as Err).0: i32); // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageLive(_14);                // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageLive(_15);                // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
-           _15 = move _13;                  // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
-           Deinit(_14);                     // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
-           ((_14 as Err).0: i32) = move _15; // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
-           discriminant(_14) = 1;           // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageDead(_15);                // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
+           StorageLive(_12);                // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
+           _12 = move ((_4 as Err).0: i32); // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
+           StorageLive(_13);                // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
+           Deinit(_13);                     // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
+           ((_13 as Err).0: i32) = move _12; // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
+           discriminant(_13) = 1;           // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
123           Deinit(_3);                      // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
-           ((_3 as Break).0: std::result::Result<std::convert::Infallible, i32>) = move _14; // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
+           ((_3 as Break).0: std::result::Result<std::convert::Infallible, i32>) = move _13; // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
125           discriminant(_3) = 1;            // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageDead(_14);                // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageDead(_13);                // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
+           StorageDead(_13);                // scope 7 at $SRC_DIR/core/src/result.rs:LL:COL
128 -         goto -> bb1;                     // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
129 +         StorageDead(_4);                 // scope 0 at $DIR/separate_const_switch.rs:+1:9: +1:10
130 +         _5 = discriminant(_3);           // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10
140 +     bb6: {
140 +     bb6: {
141           StorageLive(_11);                // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
142           _11 = move ((_4 as Ok).0: i32);  // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageLive(_12);                // scope 6 at $SRC_DIR/core/src/result.rs:LL:COL
-           _12 = move _11;                  // scope 6 at $SRC_DIR/core/src/result.rs:LL:COL
145           Deinit(_3);                      // scope 6 at $SRC_DIR/core/src/result.rs:LL:COL
-           ((_3 as Continue).0: i32) = move _12; // scope 6 at $SRC_DIR/core/src/result.rs:LL:COL
+           ((_3 as Continue).0: i32) = move _11; // scope 6 at $SRC_DIR/core/src/result.rs:LL:COL
147           discriminant(_3) = 0;            // scope 6 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageDead(_12);                // scope 6 at $SRC_DIR/core/src/result.rs:LL:COL
-           StorageDead(_11);                // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
150 -         goto -> bb1;                     // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
151 +         StorageDead(_4);                 // scope 0 at $DIR/separate_const_switch.rs:+1:9: +1:10
152 +         _5 = discriminant(_3);           // scope 0 at $DIR/separate_const_switch.rs:+1:8: +1:10

thread '[mir-opt] tests/mir-opt/separate_const_switch.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/separate_const_switch.identity.SeparateConstSwitch.diff', src/tools/compiletest/src/runtest.rs:3483:21
---- [mir-opt] tests/mir-opt/simplify_locals_fixedpoint.rs stdout ----
---- [mir-opt] tests/mir-opt/simplify_locals_fixedpoint.rs stdout ----
41           StorageLive(_6);                 // scope 1 at $DIR/simplify_locals_fixedpoint.rs:+1:18: +1:19
42           _6 = (((_1.0: std::option::Option<u8>) as Some).0: u8); // scope 1 at $DIR/simplify_locals_fixedpoint.rs:+1:18: +1:19
43 -         StorageLive(_7);                 // scope 1 at $DIR/simplify_locals_fixedpoint.rs:+2:12: +2:20
- -         StorageLive(_8);                 // scope 1 at $DIR/simplify_locals_fixedpoint.rs:+2:12: +2:13
- -         _8 = _6;                         // scope 1 at $DIR/simplify_locals_fixedpoint.rs:+2:12: +2:13
- -         _7 = Gt(move _8, const 42_u8);   // scope 1 at $DIR/simplify_locals_fixedpoint.rs:+2:12: +2:20
- -         StorageDead(_8);                 // scope 1 at $DIR/simplify_locals_fixedpoint.rs:+2:19: +2:20
+ -         _7 = Gt(_6, const 42_u8);        // scope 1 at $DIR/simplify_locals_fixedpoint.rs:+2:12: +2:20
48 -         StorageDead(_7);                 // scope 1 at $DIR/simplify_locals_fixedpoint.rs:+4:9: +4:10
-           StorageDead(_6);                 // scope 0 at $DIR/simplify_locals_fixedpoint.rs:+5:5: +5:6
50           goto -> bb3;                     // scope 0 at $DIR/simplify_locals_fixedpoint.rs:+1:5: +5:6
52   


thread '[mir-opt] tests/mir-opt/simplify_locals_fixedpoint.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/simplify_locals_fixedpoint.foo.SimplifyLocals-final.diff', src/tools/compiletest/src/runtest.rs:3483:21
---- [mir-opt] tests/mir-opt/slice_filter.rs stdout ----
---- [mir-opt] tests/mir-opt/slice_filter.rs stdout ----
29       let mut _26: &(usize, usize, usize, usize); // in scope 0 at $DIR/slice_filter.rs:+0:26: +0:38
30       let mut _27: &(usize, usize, usize, usize); // in scope 0 at $DIR/slice_filter.rs:+0:26: +0:38
31       let mut _28: &(usize, usize, usize, usize); // in scope 0 at $DIR/slice_filter.rs:+0:26: +0:38
+       let mut _31: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
+       let mut _32: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
+       let mut _37: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
+       let mut _38: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
+       let mut _43: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
+       let mut _44: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
+       let mut _49: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
+       let mut _50: &usize;                 // in scope 0 at $SRC_DIR/core/src/cmp.rs:LL:COL
32       scope 1 {
33           debug a => _3;                   // in scope 1 at $DIR/slice_filter.rs:+0:27: +0:28
34           debug b => _4;                   // in scope 1 at $DIR/slice_filter.rs:+0:30: +0:31

39               debug other => _10;          // in scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
40               let mut _29: &usize;         // in scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
41               let mut _30: &usize;         // in scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
-               let mut _31: &usize;         // in scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
-               let mut _32: &usize;         // in scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
44               scope 3 (inlined cmp::impls::<impl PartialOrd for usize>::le) { // at $SRC_DIR/core/src/cmp.rs:LL:COL
- -                 debug self => _29;       // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
- -                 debug other => _30;      // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
- +                 debug self => _31;       // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
- +                 debug other => _32;      // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -                 debug self => _31;       // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -                 debug other => _32;      // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ +                 debug self => _29;       // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ +                 debug other => _30;      // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
49                   let mut _33: usize;      // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
50                   let mut _34: usize;      // in scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL


55               debug other => _19;          // in scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
56               let mut _35: &usize;         // in scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
57               let mut _36: &usize;         // in scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
-               let mut _37: &usize;         // in scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
-               let mut _38: &usize;         // in scope 4 at $SRC_DIR/core/src/cmp.rs:LL:COL
60               scope 5 (inlined cmp::impls::<impl PartialOrd for usize>::le) { // at $SRC_DIR/core/src/cmp.rs:LL:COL
- -                 debug self => _35;       // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
- -                 debug other => _36;      // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
- +                 debug self => _37;       // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
- +                 debug other => _38;      // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -                 debug self => _37;       // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -                 debug other => _38;      // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ +                 debug self => _35;       // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ +                 debug other => _36;      // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
65                   let mut _39: usize;      // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL
66                   let mut _40: usize;      // in scope 5 at $SRC_DIR/core/src/cmp.rs:LL:COL


71               debug other => _14;          // in scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
72               let mut _41: &usize;         // in scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
73               let mut _42: &usize;         // in scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
-               let mut _43: &usize;         // in scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
-               let mut _44: &usize;         // in scope 6 at $SRC_DIR/core/src/cmp.rs:LL:COL
76               scope 7 (inlined cmp::impls::<impl PartialOrd for usize>::le) { // at $SRC_DIR/core/src/cmp.rs:LL:COL
- -                 debug self => _41;       // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
- -                 debug other => _42;      // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
- +                 debug self => _43;       // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
- +                 debug other => _44;      // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -                 debug self => _43;       // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -                 debug other => _44;      // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ +                 debug self => _41;       // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ +                 debug other => _42;      // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
81                   let mut _45: usize;      // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL
82                   let mut _46: usize;      // in scope 7 at $SRC_DIR/core/src/cmp.rs:LL:COL


87               debug other => _23;          // in scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
88               let mut _47: &usize;         // in scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
89               let mut _48: &usize;         // in scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
-               let mut _49: &usize;         // in scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
-               let mut _50: &usize;         // in scope 8 at $SRC_DIR/core/src/cmp.rs:LL:COL
92               scope 9 (inlined cmp::impls::<impl PartialOrd for usize>::le) { // at $SRC_DIR/core/src/cmp.rs:LL:COL
- -                 debug self => _47;       // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
- -                 debug other => _48;      // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
- +                 debug self => _49;       // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
- +                 debug other => _50;      // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -                 debug self => _49;       // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -                 debug other => _50;      // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ +                 debug self => _47;       // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ +                 debug other => _48;      // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
97                   let mut _51: usize;      // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL
98                   let mut _52: usize;      // in scope 9 at $SRC_DIR/core/src/cmp.rs:LL:COL


121           StorageLive(_11);                // scope 1 at $DIR/slice_filter.rs:+0:45: +0:46
122           _11 = _5;                        // scope 1 at $DIR/slice_filter.rs:+0:45: +0:46
123           _10 = &_11;                      // scope 1 at $DIR/slice_filter.rs:+0:45: +0:46
- -         StorageLive(_29);                // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           _31 = deref_copy (*_9);          // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
- -         _29 = _31;                       // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
- -         StorageLive(_30);                // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
-           _32 = deref_copy (*_10);         // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
- -         _30 = _32;                       // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           _29 = deref_copy (*_9);          // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
+           _30 = deref_copy (*_10);         // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         StorageLive(_31);                // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _31 = _29;                       // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         StorageLive(_32);                // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _32 = _30;                       // scope 2 at $SRC_DIR/core/src/cmp.rs:LL:COL
130           StorageLive(_33);                // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
- -         _33 = (*_29);                    // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
- +         _33 = (*_31);                    // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ -         _33 = (*_31);                    // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
+ +         _33 = (*_29);                    // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
133           StorageLive(_34);                // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
- -         _34 = (*_30);                    // scope 3 at $SRC_DIR/core/src/cmp.rs:LL:COL
