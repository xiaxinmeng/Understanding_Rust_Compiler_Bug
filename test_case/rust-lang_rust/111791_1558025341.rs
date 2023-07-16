plain
 finished in 0.618 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 256 tests
.....................................F..........................................i.......  88/256
.........................................................i...........i.ii..........F.... 176/256
...........F.......iii.iiiii.........ii.....F..i................................
failures:

---- [mir-opt] tests/mir-opt/const_debuginfo.rs stdout ----
---- [mir-opt] tests/mir-opt/const_debuginfo.rs stdout ----
thread '[mir-opt] tests/mir-opt/const_debuginfo.rs' panicked at 'the mir dump file for const_debuginfo.main.ConstDebugInfo.before.mir does not exist (requested in /checkout/tests/mir-opt/const_debuginfo.rs)', src/tools/compiletest/src/runtest.rs:3652:17

---- [mir-opt] tests/mir-opt/inline/issue_106141.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/issue_106141.rs stdout ----
8 +         let mut _2: bool;                // in scope 1 at $DIR/issue_106141.rs:14:8: 14:21
9 +         let mut _3: &[bool; 1];          // in scope 1 at $DIR/issue_106141.rs:12:18: 12:25
10 +         scope 2 {
- +             debug buffer => const _;     // in scope 2 at $DIR/issue_106141.rs:12:9: 12:15
+ +             debug buffer => _3;          // in scope 2 at $DIR/issue_106141.rs:12:9: 12:15
12 +             scope 3 {
13 +                 debug index => _0;       // in scope 3 at $DIR/issue_106141.rs:13:9: 13:14


thread '[mir-opt] tests/mir-opt/inline/issue_106141.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/issue_106141.outer.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
Build completed unsuccessfully in 0:14:03
---- [mir-opt] tests/mir-opt/issues/issue_59352.rs stdout ----
---- [mir-opt] tests/mir-opt/issues/issue_59352.rs stdout ----
4     debug num => _1;                     // in scope 0 at $DIR/issue_59352.rs:+0:21: +0:24
5     let mut _0: u32;                     // return place in scope 0 at $DIR/issue_59352.rs:+0:35: +0:38
6     let mut _2: std::option::Option<u32>; // in scope 0 at $DIR/issue_59352.rs:+2:26: +2:41
+     let mut _3: u32;                     // in scope 0 at $DIR/issue_59352.rs:+2:12: +2:23
7     scope 1 (inlined char::methods::<impl char>::is_digit) { // at $DIR/issue_59352.rs:15:12: 15:23
8         debug self => _1;                // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         debug radix => const 8_u32;      // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         let mut _3: &std::option::Option<u32>; // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         let _4: std::option::Option<u32>; // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         debug radix => _3;               // in scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
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

18         debug self => _2;                // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
-         let mut _6: isize;               // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
-         let mut _7: !;                   // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         let mut _7: isize;               // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         let mut _8: !;                   // in scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
21         scope 4 {
22             debug val => _0;             // in scope 4 at $SRC_DIR/core/src/option.rs:LL:COL

24     }
25 
26     bb0: {
26     bb0: {
-         StorageLive(_3);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         StorageLive(_3);                 // scope 0 at $DIR/issue_59352.rs:+2:12: +2:23
28         StorageLive(_4);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         _4 = char::methods::<impl char>::to_digit(_1, const 8_u32) -> bb5; // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         StorageLive(_5);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         _5 = char::methods::<impl char>::to_digit(_1, const 8_u32) -> bb5; // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
30                                          // mir::Constant
31                                          // + span: $SRC_DIR/core/src/char/methods.rs:LL:COL
32                                          // + literal: Const { ty: fn(char, u32) -> Option<u32> {char::methods::<impl char>::to_digit}, val: Value(<ZST>) }
41     }
42 
43     bb2: {
43     bb2: {
-         _6 = discriminant(_2);           // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
-         switchInt(move _6) -> [0: bb6, 1: bb8, otherwise: bb7]; // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         _7 = discriminant(_2);           // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         switchInt(move _7) -> [0: bb6, 1: bb8, otherwise: bb7]; // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
47 
48     bb3: {

55     }
55     }
56 
57     bb5: {
-         _3 = &_4;                        // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         _5 = discriminant((*_3));        // scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
-         StorageDead(_3);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         _4 = &_5;                        // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         _6 = discriminant((*_4));        // scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
61         StorageDead(_4);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
-         switchInt(move _5) -> [1: bb1, otherwise: bb3]; // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
+         StorageDead(_5);                 // scope 1 at $SRC_DIR/core/src/char/methods.rs:LL:COL
+         StorageDead(_3);                 // scope 0 at $DIR/issue_59352.rs:+2:12: +2:23
+         switchInt(move _6) -> [1: bb1, otherwise: bb3]; // scope 0 at $DIR/issue_59352.rs:+2:8: +2:23
64 
65     bb6: {


-         _7 = core::panicking::panic(const "called `Option::unwrap()` on a `None` value"); // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
+         _8 = core::panicking::panic(const "called `Option::unwrap()` on a `None` value"); // scope 3 at $SRC_DIR/core/src/option.rs:LL:COL
67                                          // mir::Constant
68                                          // + span: $SRC_DIR/core/src/option.rs:LL:COL
69                                          // + literal: Const { ty: fn(&'static str) -> ! {core::panicking::panic}, val: Value(<ZST>) }

thread '[mir-opt] tests/mir-opt/issues/issue_59352.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/issues/issue_59352.num_to_digit.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/pre-codegen/optimizes_into_variable.rs stdout ----
2 
3 fn main() -> () {
3 fn main() -> () {
4     let mut _0: ();                      // return place in scope 0 at $DIR/optimizes_into_variable.rs:+0:11: +0:11
+     let _1: i32;                         // in scope 0 at $DIR/optimizes_into_variable.rs:+1:9: +1:10
+     let mut _3: u32;                     // in scope 0 at $DIR/optimizes_into_variable.rs:+3:13: +3:36
5     scope 1 {
-         debug x => const 4_i32;          // in scope 1 at $DIR/optimizes_into_variable.rs:+1:9: +1:10
+         debug x => _1;                   // in scope 1 at $DIR/optimizes_into_variable.rs:+1:9: +1:10
+         let _2: i32;                     // in scope 1 at $DIR/optimizes_into_variable.rs:+2:9: +2:10
7         scope 2 {
-             debug y => const 3_i32;      // in scope 2 at $DIR/optimizes_into_variable.rs:+2:9: +2:10
+             debug y => _2;               // in scope 2 at $DIR/optimizes_into_variable.rs:+2:9: +2:10
9             scope 3 {
-                 debug z => const 42_u32; // in scope 3 at $DIR/optimizes_into_variable.rs:+3:9: +3:10
+                 debug z => _3;           // in scope 3 at $DIR/optimizes_into_variable.rs:+3:9: +3:10
12         }
13     }

14 
14 
15     bb0: {
+         StorageLive(_1);                 // scope 0 at $DIR/optimizes_into_variable.rs:+1:9: +1:10
+         StorageLive(_2);                 // scope 1 at $DIR/optimizes_into_variable.rs:+2:9: +2:10
+         StorageDead(_2);                 // scope 1 at $DIR/optimizes_into_variable.rs:+4:1: +4:2
+         StorageDead(_1);                 // scope 0 at $DIR/optimizes_into_variable.rs:+4:1: +4:2
16         return;                          // scope 0 at $DIR/optimizes_into_variable.rs:+4:2: +4:2
18 }


thread '[mir-opt] tests/mir-opt/pre-codegen/optimizes_into_variable.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/pre-codegen/optimizes_into_variable.main.SimplifyLocals-final.after.64bit.mir', src/tools/compiletest/src/runtest.rs:3639:21

failures:
    [mir-opt] tests/mir-opt/const_debuginfo.rs
    [mir-opt] tests/mir-opt/inline/issue_106141.rs
