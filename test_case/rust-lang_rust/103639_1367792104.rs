plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 215 tests
......................................................i................................. 88/215
...........................................i...F....................F................ii. 176/215
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] src/test/mir-opt/inline/inline_generator.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/inline_generator.rs stdout ----
7       let mut _2: std::pin::Pin<&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]>; // in scope 0 at $DIR/inline_generator.rs:+1:14: +1:32
8       let mut _3: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 0 at $DIR/inline_generator.rs:+1:23: +1:31
9       let mut _4: [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 0 at $DIR/inline_generator.rs:+1:28: +1:31
- +     let mut _6: bool;                    // in scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
+ +     let mut _7: bool;                    // in scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
11       scope 1 {
12           debug _r => _1;                  // in scope 1 at $DIR/inline_generator.rs:+1:9: +1:11


17 +         debug pointer => _3;             // in scope 3 at $SRC_DIR/core/src/pin.rs:LL:COL
18 +         let mut _5: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 3 at $SRC_DIR/core/src/pin.rs:LL:COL
19 +         scope 4 {
+ +             scope 5 (inlined Pin::<&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]>::new_unchecked) { // at $SRC_DIR/core/src/pin.rs:LL:COL
+ +                 debug pointer => _5;     // in scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +                 let mut _6: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +             }
21 +     }
21 +     }
- +     scope 5 (inlined g::{closure#0}) {   // at $DIR/inline_generator.rs:9:33: 9:46
- +         debug a => _6;                   // in scope 5 at $DIR/inline_generator.rs:15:6: 15:7
- +         let mut _7: i32;                 // in scope 5 at $DIR/inline_generator.rs:15:17: 15:39
- +         let mut _8: u32;                 // in scope 5 at $DIR/inline_generator.rs:15:5: 15:41
- +         let mut _9: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 5 at $DIR/inline_generator.rs:15:5: 15:41
- +         let mut _10: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 5 at $DIR/inline_generator.rs:15:5: 15:41
- +         let mut _11: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 5 at $DIR/inline_generator.rs:15:5: 15:41
+ +     scope 6 (inlined g::{closure#0}) {   // at $DIR/inline_generator.rs:9:33: 9:46
+ +         debug a => _7;                   // in scope 6 at $DIR/inline_generator.rs:15:6: 15:7
+ +         let mut _8: i32;                 // in scope 6 at $DIR/inline_generator.rs:15:17: 15:39
+ +         let mut _9: u32;                 // in scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         let mut _10: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         let mut _11: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         let mut _12: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 6 at $DIR/inline_generator.rs:15:5: 15:41
30   
31       bb0: {


44 +         discriminant(_4) = 0;            // scope 2 at $DIR/inline_generator.rs:15:5: 15:41
45           _3 = &mut _4;                    // scope 0 at $DIR/inline_generator.rs:+1:23: +1:31
46 -         _2 = Pin::<&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]>::new(move _3) -> [return: bb2, unwind: bb4]; // scope 0 at $DIR/inline_generator.rs:+1:14: +1:32
- +         StorageLive(_5);                 // scope 4 at $SRC_DIR/core/src/pin.rs:LL:COL
- +         _5 = move _3;                    // scope 4 at $SRC_DIR/core/src/pin.rs:LL:COL
- +         _2 = Pin::<&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]>::new_unchecked(move _5) -> [return: bb3, unwind: bb2]; // scope 4 at $SRC_DIR/core/src/pin.rs:LL:COL
-                                            // mir::Constant
+ -                                          // mir::Constant
51 -                                          // + span: $DIR/inline_generator.rs:9:14: 9:22
- +                                          // + span: $SRC_DIR/core/src/pin.rs:LL:COL
-                                            // + user_ty: UserType(0)
+ -                                          // + user_ty: UserType(0)
54 -                                          // + literal: Const { ty: fn(&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]) -> Pin<&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]> {Pin::<&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]>::new}, val: Value(<ZST>) }
- +                                          // + literal: Const { ty: unsafe fn(&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]) -> Pin<&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]> {Pin::<&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]>::new_unchecked}, val: Value(<ZST>) }
-   
+ -     }
+ - 
58 -     bb2: {
58 -     bb2: {
- -         StorageDead(_3);                 // scope 0 at $DIR/inline_generator.rs:+1:31: +1:32
+ +         StorageLive(_5);                 // scope 4 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +         _5 = move _3;                    // scope 4 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +         StorageLive(_6);                 // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +         _6 = move _5;                    // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +         Deinit(_2);                      // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +         (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]) = move _6; // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +         StorageDead(_6);                 // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +         StorageDead(_5);                 // scope 4 at $SRC_DIR/core/src/pin.rs:LL:COL
+           StorageDead(_3);                 // scope 0 at $DIR/inline_generator.rs:+1:31: +1:32
60 -         _1 = <[generator@$DIR/inline_generator.rs:15:5: 15:8] as Generator<bool>>::resume(move _2, const false) -> [return: bb3, unwind: bb4]; // scope 0 at $DIR/inline_generator.rs:+1:14: +1:46
61 -                                          // mir::Constant
62 -                                          // + span: $DIR/inline_generator.rs:9:33: 9:39

63 -                                          // + literal: Const { ty: for<'a> fn(Pin<&'a mut [generator@$DIR/inline_generator.rs:15:5: 15:8]>, bool) -> GeneratorState<<[generator@$DIR/inline_generator.rs:15:5: 15:8] as Generator<bool>>::Yield, <[generator@$DIR/inline_generator.rs:15:5: 15:8] as Generator<bool>>::Return> {<[generator@$DIR/inline_generator.rs:15:5: 15:8] as Generator<bool>>::resume}, val: Value(<ZST>) }
- -     }
- - 
+ +         StorageLive(_7);                 // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
+ +         _7 = const false;                // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
+ +         _10 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         _9 = discriminant((*_10));       // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         switchInt(move _9) -> [0: bb3, 1: bb8, 3: bb7, otherwise: bb9]; // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+   
66 -     bb3: {
67 +     bb1: {
67 +     bb1: {
- +         StorageDead(_6);                 // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
+ +         StorageDead(_7);                 // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
69           StorageDead(_2);                 // scope 0 at $DIR/inline_generator.rs:+1:45: +1:46
70           StorageDead(_4);                 // scope 0 at $DIR/inline_generator.rs:+1:46: +1:47
71           _0 = const ();                   // scope 0 at $DIR/inline_generator.rs:+0:11: +2:2
79 +     }
80 + 
81 +     bb3: {
81 +     bb3: {
- +         StorageDead(_5);                 // scope 4 at $SRC_DIR/core/src/pin.rs:LL:COL
- +         StorageDead(_3);                 // scope 0 at $DIR/inline_generator.rs:+1:31: +1:32
- +         StorageLive(_6);                 // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
- +         _6 = const false;                // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
- +         _9 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 5 at $DIR/inline_generator.rs:15:5: 15:41
- +         _8 = discriminant((*_9));        // scope 5 at $DIR/inline_generator.rs:15:5: 15:41
- +         switchInt(move _8) -> [0: bb4, 1: bb9, 3: bb8, otherwise: bb10]; // scope 5 at $DIR/inline_generator.rs:15:5: 15:41
+ +         StorageLive(_8);                 // scope 6 at $DIR/inline_generator.rs:15:17: 15:39
+ +         switchInt(move _7) -> [0: bb5, otherwise: bb4]; // scope 6 at $DIR/inline_generator.rs:15:20: 15:21
90 + 
91 +     bb4: {


- +         StorageLive(_7);                 // scope 5 at $DIR/inline_generator.rs:15:17: 15:39
- +         switchInt(move _6) -> [0: bb6, otherwise: bb5]; // scope 5 at $DIR/inline_generator.rs:15:20: 15:21
+ +         _8 = const 7_i32;                // scope 6 at $DIR/inline_generator.rs:15:24: 15:25
+ +         goto -> bb6;                     // scope 6 at $DIR/inline_generator.rs:15:17: 15:39
95 + 
96 +     bb5: {


- +         _7 = const 7_i32;                // scope 5 at $DIR/inline_generator.rs:15:24: 15:25
- +         goto -> bb7;                     // scope 5 at $DIR/inline_generator.rs:15:17: 15:39
+ +         _8 = const 13_i32;               // scope 6 at $DIR/inline_generator.rs:15:35: 15:37
+ +         goto -> bb6;                     // scope 6 at $DIR/inline_generator.rs:15:17: 15:39
100 + 
101 +     bb6: {


- +         _7 = const 13_i32;               // scope 5 at $DIR/inline_generator.rs:15:35: 15:37
- +         goto -> bb7;                     // scope 5 at $DIR/inline_generator.rs:15:17: 15:39
+ +         Deinit(_1);                      // scope 6 at $DIR/inline_generator.rs:15:11: 15:39
+ +         ((_1 as Yielded).0: i32) = move _8; // scope 6 at $DIR/inline_generator.rs:15:11: 15:39
+ +         discriminant(_1) = 0;            // scope 6 at $DIR/inline_generator.rs:15:11: 15:39
+ +         _11 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:11: 15:39
+ +         discriminant((*_11)) = 3;        // scope 6 at $DIR/inline_generator.rs:15:11: 15:39
+ +         goto -> bb1;                     // scope 0 at $DIR/inline_generator.rs:15:11: 15:39
105 + 
106 +     bb7: {


- +         Deinit(_1);                      // scope 5 at $DIR/inline_generator.rs:15:11: 15:39
- +         ((_1 as Yielded).0: i32) = move _7; // scope 5 at $DIR/inline_generator.rs:15:11: 15:39
- +         discriminant(_1) = 0;            // scope 5 at $DIR/inline_generator.rs:15:11: 15:39
- +         _10 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 5 at $DIR/inline_generator.rs:15:11: 15:39
- +         discriminant((*_10)) = 3;        // scope 5 at $DIR/inline_generator.rs:15:11: 15:39
- +         goto -> bb1;                     // scope 0 at $DIR/inline_generator.rs:15:11: 15:39
+ +         StorageLive(_8);                 // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         StorageDead(_8);                 // scope 6 at $DIR/inline_generator.rs:15:38: 15:39
+ +         Deinit(_1);                      // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
+ +         ((_1 as Complete).0: bool) = move _7; // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
+ +         discriminant(_1) = 1;            // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
+ +         _12 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
+ +         discriminant((*_12)) = 1;        // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
+ +         goto -> bb1;                     // scope 0 at $DIR/inline_generator.rs:15:41: 15:41
114 + 
115 +     bb8: {


- +         StorageLive(_7);                 // scope 5 at $DIR/inline_generator.rs:15:5: 15:41
- +         StorageDead(_7);                 // scope 5 at $DIR/inline_generator.rs:15:38: 15:39
- +         Deinit(_1);                      // scope 5 at $DIR/inline_generator.rs:15:41: 15:41
- +         ((_1 as Complete).0: bool) = move _6; // scope 5 at $DIR/inline_generator.rs:15:41: 15:41
- +         discriminant(_1) = 1;            // scope 5 at $DIR/inline_generator.rs:15:41: 15:41
- +         _11 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 5 at $DIR/inline_generator.rs:15:41: 15:41
- +         discriminant((*_11)) = 1;        // scope 5 at $DIR/inline_generator.rs:15:41: 15:41
- +         goto -> bb1;                     // scope 0 at $DIR/inline_generator.rs:15:41: 15:41
+ +         assert(const false, "generator resumed after completion") -> [success: bb8, unwind: bb2]; // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
125 + 
126 +     bb9: {


- +         assert(const false, "generator resumed after completion") -> [success: bb9, unwind: bb2]; // scope 5 at $DIR/inline_generator.rs:15:5: 15:41
- +     }
- +     bb10: {
- +     bb10: {
- +         unreachable;                     // scope 5 at $DIR/inline_generator.rs:15:5: 15:41
+ +         unreachable;                     // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
133   }
134   


thread '[mir-opt] src/test/mir-opt/inline/inline_generator.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_generator.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3465:21

---- [mir-opt] src/test/mir-opt/issues/issue_59352.rs stdout ----
---- [mir-opt] src/test/mir-opt/issues/issue_59352.rs stdout ----
3 fn num_to_digit(_1: char) -> u32 {
4     debug num => _1;                     // in scope 0 at $DIR/issue_59352.rs:+0:21: +0:24
5     let mut _0: u32;                     // return place in scope 0 at $DIR/issue_59352.rs:+0:35: +0:38
-     let mut _2: bool;                    // in scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
-     let mut _3: std::option::Option<u32>; // in scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
-     let mut _4: u32;                     // in scope 0 at $DIR/issue_59352.rs:+2:12: +2:23
+     let mut _2: std::option::Option<u32>; // in scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
+     let mut _3: u32;                     // in scope 0 at $DIR/issue_59352.rs:+2:12: +2:23
+     let mut _9: isize;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
9     scope 1 (inlined char::methods::<impl char>::is_digit) { // at $DIR/issue_59352.rs:14:12: 14:23
-         debug self => _7;                // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         debug radix => _4;               // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         let mut _5: &std::option::Option<u32>; // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         let _6: std::option::Option<u32>; // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         let mut _7: char;                // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         debug self => _1;                // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         debug radix => _3;               // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         let mut _4: &std::option::Option<u32>; // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         let _5: std::option::Option<u32>; // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         let mut _6: char;                // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         scope 2 (inlined Option::<u32>::is_some) { // at $SRC_DIR/core/src/char/methods.rs:LL:COL
+             debug self => _4;            // in scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
15     }
15     }
-     scope 2 (inlined #[track_caller] Option::<u32>::unwrap) { // at $DIR/issue_59352.rs:14:42: 14:50
-         debug self => _3;                // in scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
-         let mut _8: isize;               // in scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
-         let mut _9: !;                   // in scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
-         scope 3 {
-             debug val => _0;             // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+     scope 3 (inlined #[track_caller] Option::<u32>::unwrap) { // at $DIR/issue_59352.rs:14:42: 14:50
+         debug self => _2;                // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         let mut _7: isize;               // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         let mut _8: !;                   // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         scope 4 {
+             debug val => _0;             // in scope 4 at $SRC_DIR/core/src/option.rs:LL:COL
23     }
24 

25     bb0: {
25     bb0: {
-         StorageLive(_2);                 // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
-         _7 = _1;                         // scope 0 at $DIR/issue_59352.rs:+2:8: +2:11
-         StorageLive(_4);                 // scope 0 at $DIR/issue_59352.rs:+2:12: +2:23
+         StorageLive(_3);                 // scope 0 at $DIR/issue_59352.rs:+2:12: +2:23
+         StorageLive(_4);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
29         StorageLive(_5);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
30         StorageLive(_6);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         _6 = char::methods::<impl char>::to_digit(move _7, const 8_u32) -> bb5; // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         _6 = _1;                         // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         _5 = char::methods::<impl char>::to_digit(move _6, const 8_u32) -> bb5; // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
32                                          // mir::Constant
33                                          // + span: $SRC_DIR/core/src/char/methods.rs:LL:COL
34                                          // + literal: Const { ty: fn(char, u32) -> Option<u32> {char::methods::<impl char>::to_digit}, val: Value(<ZST>) }
35     }
36 
37     bb1: {
37     bb1: {
-         StorageLive(_3);                 // scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
-         _3 = char::methods::<impl char>::to_digit(move _1, const 8_u32) -> bb2; // scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
+         StorageLive(_2);                 // scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
+         _2 = char::methods::<impl char>::to_digit(move _1, const 8_u32) -> bb2; // scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
40                                          // mir::Constant
41                                          // + span: $DIR/issue_59352.rs:14:30: 14:38
42                                          // + literal: Const { ty: fn(char, u32) -> Option<u32> {char::methods::<impl char>::to_digit}, val: Value(<ZST>) }
43     }
44 
45     bb2: {
45     bb2: {
-         _8 = discriminant(_3);           // scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
-         switchInt(move _8) -> [0: bb7, 1: bb9, otherwise: bb8]; // scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
+         _7 = discriminant(_2);           // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         switchInt(move _7) -> [0: bb6, 1: bb8, otherwise: bb7]; // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
49 
50     bb3: {

53     }
53     }
54 
55     bb4: {
-         StorageDead(_2);                 // scope 0 at $DIR/issue_59352.rs:+2:62: +2:63
57         return;                          // scope 0 at $DIR/issue_59352.rs:+3:2: +3:2
59 

60     bb5: {
60     bb5: {
-         _5 = &_6;                        // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         _2 = Option::<u32>::is_some(move _5) -> bb6; // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-                                          // mir::Constant
-                                          // + span: $SRC_DIR/core/src/char/methods.rs:LL:COL
-                                          // + literal: Const { ty: for<'a> fn(&'a Option<u32>) -> bool {Option::<u32>::is_some}, val: Value(<ZST>) }
- 
-     bb6: {
-     bb6: {
-         StorageDead(_5);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         _4 = &_5;                        // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
70         StorageDead(_6);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         StorageDead(_4);                 // scope 0 at $DIR/issue_59352.rs:+2:12: +2:23
-         switchInt(move _2) -> [0: bb3, otherwise: bb1]; // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
+         _9 = discriminant((*_4));        // scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
+         StorageDead(_4);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         StorageDead(_5);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         StorageDead(_3);                 // scope 0 at $DIR/issue_59352.rs:+2:12: +2:23
+         switchInt(move _9) -> [1: bb1, otherwise: bb3]; // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
74 
-     bb7: {
-     bb7: {
-         StorageLive(_9);                 // scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
-         _9 = core::panicking::panic(const "called `Option::unwrap()` on a `None` value"); // scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
+     bb6: {
+         StorageLive(_8);                 // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         _8 = core::panicking::panic(const "called `Option::unwrap()` on a `None` value"); // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
78                                          // mir::Constant
79                                          // + span: $SRC_DIR/core/src/option.rs:LL:COL
80                                          // + literal: Const { ty: fn(&'static str) -> ! {core::panicking::panic}, val: Value(<ZST>) }

83                                          // + literal: Const { ty: &str, val: Value(Slice(..)) }
85 
-     bb8: {
-     bb8: {
-         unreachable;                     // scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
+     bb7: {
+         unreachable;                     // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
89 
-     bb9: {
-     bb9: {
-         _0 = move ((_3 as Some).0: u32); // scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
-         StorageDead(_3);                 // scope 0 at $DIR/issue_59352.rs:+2:49: +2:50
+     bb8: {
+         _0 = move ((_2 as Some).0: u32); // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         StorageDead(_2);                 // scope 0 at $DIR/issue_59352.rs:+2:49: +2:50
93         goto -> bb4;                     // scope 0 at $DIR/issue_59352.rs:+2:5: +2:63
95 }


thread '[mir-opt] src/test/mir-opt/issues/issue_59352.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issues/issue_59352.num_to_digit.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3465:21

failures:
    [mir-opt] src/test/mir-opt/inline/inline_generator.rs
    [mir-opt] src/test/mir-opt/issues/issue_59352.rs
