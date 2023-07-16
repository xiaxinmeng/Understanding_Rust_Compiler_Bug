plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 215 tests
......................................................i................................. 88/215
...........................................i......F..................F...............ii. 176/215
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] src/test/mir-opt/inline/inline_generator.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/inline_generator.rs stdout ----
7       let mut _2: std::pin::Pin<&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]>; // in scope 0 at $DIR/inline_generator.rs:+1:14: +1:32
8       let mut _3: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 0 at $DIR/inline_generator.rs:+1:23: +1:31
9       let mut _4: [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 0 at $DIR/inline_generator.rs:+1:28: +1:31
- +     let mut _7: bool;                    // in scope 0 at $DIR/inline_generator.rs:+1:14: +1:46
+ +     let mut _7: bool;                    // in scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
11       scope 1 {
12           debug _r => _1;                  // in scope 1 at $DIR/inline_generator.rs:+1:9: +1:11

23 +             }
24 +         }
25 +     }
25 +     }
- +     scope 6 (inlined g::{closure#0}) {   // at $DIR/inline_generator.rs:9:14: 9:46
+ +     scope 6 (inlined g::{closure#0}) {   // at $DIR/inline_generator.rs:9:33: 9:46
27 +         debug a => _7;                   // in scope 6 at $DIR/inline_generator.rs:15:6: 15:7
28 +         let mut _8: i32;                 // in scope 6 at $DIR/inline_generator.rs:15:17: 15:39
29 +         let mut _9: u32;                 // in scope 6 at $DIR/inline_generator.rs:15:5: 15:41
68 -                                          // mir::Constant
68 -                                          // mir::Constant
69 -                                          // + span: $DIR/inline_generator.rs:9:33: 9:39
70 -                                          // + literal: Const { ty: for<'a> fn(Pin<&'a mut [generator@$DIR/inline_generator.rs:15:5: 15:8]>, bool) -> GeneratorState<<[generator@$DIR/inline_generator.rs:15:5: 15:8] as Generator<bool>>::Yield, <[generator@$DIR/inline_generator.rs:15:5: 15:8] as Generator<bool>>::Return> {<[generator@$DIR/inline_generator.rs:15:5: 15:8] as Generator<bool>>::resume}, val: Value(<ZST>) }
- +         StorageLive(_7);                 // scope 0 at $DIR/inline_generator.rs:+1:14: +1:46
- +         _7 = const false;                // scope 0 at $DIR/inline_generator.rs:+1:14: +1:46
+ +         StorageLive(_7);                 // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
+ +         _7 = const false;                // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
73 +         _10 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
74 +         _9 = discriminant((*_10));       // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
75 +         switchInt(move _9) -> [0: bb3, 1: bb8, 3: bb7, otherwise: bb9]; // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
77   
78 -     bb3: {
79 +     bb1: {
79 +     bb1: {
- +         StorageDead(_7);                 // scope 0 at $DIR/inline_generator.rs:+1:14: +1:46
+ +         StorageDead(_7);                 // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
81           StorageDead(_2);                 // scope 0 at $DIR/inline_generator.rs:+1:45: +1:46
82           StorageDead(_4);                 // scope 0 at $DIR/inline_generator.rs:+1:46: +1:47
83           _0 = const ();                   // scope 0 at $DIR/inline_generator.rs:+0:11: +2:2

thread '[mir-opt] src/test/mir-opt/inline/inline_generator.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_generator.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3465:21

---- [mir-opt] src/test/mir-opt/issues/issue_59352.rs stdout ----
---- [mir-opt] src/test/mir-opt/issues/issue_59352.rs stdout ----
4     debug num => _1;                     // in scope 0 at $DIR/issue_59352.rs:+0:21: +0:24
5     let mut _0: u32;                     // return place in scope 0 at $DIR/issue_59352.rs:+0:35: +0:38
6     let mut _2: std::option::Option<u32>; // in scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
-     let mut _3: u32;                     // in scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
+     let mut _3: u32;                     // in scope 0 at $DIR/issue_59352.rs:+2:12: +2:23
8     let mut _9: isize;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-     scope 1 (inlined char::methods::<impl char>::is_digit) { // at $DIR/issue_59352.rs:14:8: 14:23
+     scope 1 (inlined char::methods::<impl char>::is_digit) { // at $DIR/issue_59352.rs:14:12: 14:23
10         debug self => _1;                // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
11         debug radix => _3;               // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
12         let mut _4: &std::option::Option<u32>; // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL

16             debug self => _4;            // in scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
18     }
18     }
-     scope 3 (inlined #[track_caller] Option::<u32>::unwrap) { // at $DIR/issue_59352.rs:14:26: 14:50
+     scope 3 (inlined #[track_caller] Option::<u32>::unwrap) { // at $DIR/issue_59352.rs:14:42: 14:50
20         debug self => _2;                // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
21         let mut _7: isize;               // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
22         let mut _8: !;                   // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
26     }
27 
28     bb0: {
28     bb0: {
-         StorageLive(_3);                 // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
+         StorageLive(_3);                 // scope 0 at $DIR/issue_59352.rs:+2:12: +2:23
30         StorageLive(_4);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
31         StorageLive(_5);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
32         StorageLive(_6);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL

65         _9 = discriminant((*_4));        // scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
66         StorageDead(_4);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
67         StorageDead(_5);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         StorageDead(_3);                 // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
+         StorageDead(_3);                 // scope 0 at $DIR/issue_59352.rs:+2:12: +2:23
69         switchInt(move _9) -> [1: bb1, otherwise: bb3]; // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
71 


thread '[mir-opt] src/test/mir-opt/issues/issue_59352.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issues/issue_59352.num_to_digit.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3465:21

failures:
    [mir-opt] src/test/mir-opt/inline/inline_generator.rs
    [mir-opt] src/test/mir-opt/issues/issue_59352.rs
