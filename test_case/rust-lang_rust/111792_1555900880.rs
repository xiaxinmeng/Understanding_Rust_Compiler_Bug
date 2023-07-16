plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 255 tests
................................................................................i.......  88/255
....................................................F....i...........iii................ 176/255
.......F....F.....iii.iiiii.........ii........i................................
failures:

---- [mir-opt] tests/mir-opt/if_condition_int.rs stdout ----
---- [mir-opt] tests/mir-opt/if_condition_int.rs stdout ----
thread '[mir-opt] tests/mir-opt/if_condition_int.rs' panicked at 'the mir dump file for if_condition_int.opt_u32.SimplifyComparisonIntegral.before.mir does not exist (requested in /checkout/tests/mir-opt/if_condition_int.rs)', src/tools/compiletest/src/runtest.rs:3652:17

---- [mir-opt] tests/mir-opt/issue_76432.rs stdout ----
---- [mir-opt] tests/mir-opt/issue_76432.rs stdout ----
thread '[mir-opt] tests/mir-opt/issue_76432.rs' panicked at 'the mir dump file for issue_76432.test.SimplifyComparisonIntegral.before.mir does not exist (requested in /checkout/tests/mir-opt/issue_76432.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/issues/issue_59352.rs stdout ----
---- [mir-opt] tests/mir-opt/issues/issue_59352.rs stdout ----
3 fn num_to_digit(_1: char) -> u32 {
4     debug num => _1;                     // in scope 0 at $DIR/issue_59352.rs:+0:21: +0:24
5     let mut _0: u32;                     // return place in scope 0 at $DIR/issue_59352.rs:+0:35: +0:38
-     let mut _2: std::option::Option<u32>; // in scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
+     let mut _2: bool;                    // in scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
+     let mut _3: std::option::Option<u32>; // in scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
7     scope 1 (inlined char::methods::<impl char>::is_digit) { // at $DIR/issue_59352.rs:15:12: 15:23
8         debug self => _1;                // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
9         debug radix => const 8_u32;      // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL


-         let mut _3: &std::option::Option<u32>; // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         let _4: std::option::Option<u32>; // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         let mut _4: &std::option::Option<u32>; // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         let _5: std::option::Option<u32>; // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
12         scope 2 (inlined Option::<u32>::is_some) { // at $SRC_DIR/core/src/char/methods.rs:LL:COL
-             debug self => _3;            // in scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
-             let mut _5: isize;           // in scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
+             debug self => _4;            // in scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
+             let mut _6: isize;           // in scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
16     }
16     }
17     scope 3 (inlined #[track_caller] Option::<u32>::unwrap) { // at $DIR/issue_59352.rs:15:42: 15:50

-         debug self => _2;                // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
-         let mut _6: isize;               // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
-         let mut _7: !;                   // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         debug self => _3;                // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         let mut _7: isize;               // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         let mut _8: !;                   // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
21         scope 4 {
22             debug val => _0;             // in scope 4 at $SRC_DIR/core/src/option.rs:LL:COL

24     }
25 
26     bb0: {
26     bb0: {
-         StorageLive(_3);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         StorageLive(_2);                 // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
28         StorageLive(_4);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         _4 = char::methods::<impl char>::to_digit(_1, const 8_u32) -> bb5; // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         StorageLive(_5);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         _5 = char::methods::<impl char>::to_digit(_1, const 8_u32) -> bb5; // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
30                                          // mir::Constant
31                                          // + span: $SRC_DIR/core/src/char/methods.rs:LL:COL
32                                          // + literal: Const { ty: fn(char, u32) -> Option<u32> {char::methods::<impl char>::to_digit}, val: Value(<ZST>) }
33     }
34 
35     bb1: {
35     bb1: {
-         StorageLive(_2);                 // scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
-         _2 = char::methods::<impl char>::to_digit(_1, const 8_u32) -> bb2; // scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
+         StorageLive(_3);                 // scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
+         _3 = char::methods::<impl char>::to_digit(_1, const 8_u32) -> bb2; // scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
38                                          // mir::Constant
39                                          // + span: $DIR/issue_59352.rs:15:30: 15:38
40                                          // + literal: Const { ty: fn(char, u32) -> Option<u32> {char::methods::<impl char>::to_digit}, val: Value(<ZST>) }
41     }
42 
43     bb2: {
43     bb2: {
-         _6 = discriminant(_2);           // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
-         switchInt(move _6) -> [0: bb6, 1: bb8, otherwise: bb7]; // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         _7 = discriminant(_3);           // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         switchInt(move _7) -> [0: bb6, 1: bb8, otherwise: bb7]; // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
47 
48     bb3: {

51     }
51     }
52 
53     bb4: {
+         StorageDead(_2);                 // scope 0 at $DIR/issue_59352.rs:+2:62: +2:63
54         return;                          // scope 0 at $DIR/issue_59352.rs:+3:2: +3:2
56 

57     bb5: {
57     bb5: {
-         _3 = &_4;                        // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         _5 = discriminant((*_3));        // scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
-         StorageDead(_3);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         _4 = &_5;                        // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         _6 = discriminant((*_4));        // scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
+         _2 = Eq(_6, const 1_isize);      // scope 2 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
61         StorageDead(_4);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         switchInt(move _5) -> [1: bb1, otherwise: bb3]; // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
+         StorageDead(_5);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         switchInt(move _2) -> [0: bb3, otherwise: bb1]; // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
64 
65     bb6: {


-         _7 = core::panicking::panic(const "called `Option::unwrap()` on a `None` value"); // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         _8 = core::panicking::panic(const "called `Option::unwrap()` on a `None` value"); // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
67                                          // mir::Constant
68                                          // + span: $SRC_DIR/core/src/option.rs:LL:COL
69                                          // + literal: Const { ty: fn(&'static str) -> ! {core::panicking::panic}, val: Value(<ZST>) }
77     }
78 
79     bb8: {
79     bb8: {
-         _0 = move ((_2 as Some).0: u32); // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
-         StorageDead(_2);                 // scope 0 at $DIR/issue_59352.rs:+2:49: +2:50
+         _0 = move ((_3 as Some).0: u32); // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         StorageDead(_3);                 // scope 0 at $DIR/issue_59352.rs:+2:49: +2:50
82         goto -> bb4;                     // scope 0 at $DIR/issue_59352.rs:+2:5: +2:63
84 }


thread '[mir-opt] tests/mir-opt/issues/issue_59352.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/issues/issue_59352.num_to_digit.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3639:21

failures:
    [mir-opt] tests/mir-opt/if_condition_int.rs
    [mir-opt] tests/mir-opt/issue_76432.rs
