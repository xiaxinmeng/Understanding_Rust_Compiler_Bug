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
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...........ii........i................................
failures:

---- [mir-opt] tests/mir-opt/funky_arms.rs stdout ----
30               scope 3 {
31                   debug precision => _10;  // in scope 3 at $DIR/funky_arms.rs:+13:17: +13:26
32                   let _10: usize;          // in scope 3 at $DIR/funky_arms.rs:+13:17: +13:26
+                   scope 5 (inlined Formatter::<'_>::precision) { // at $DIR/funky_arms.rs:24:34: 24:45
+                       debug self => _8;    // in scope 5 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
33               }
34           }
35       }


+       scope 4 (inlined Formatter::<'_>::sign_plus) { // at $DIR/funky_arms.rs:15:26: 15:37
+           debug self => _5;                // in scope 4 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+           let mut _22: u32;                // in scope 4 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+           let mut _23: u32;                // in scope 4 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+           let mut _24: u32;                // in scope 4 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
36   
37       bb0: {
37       bb0: {
38           StorageLive(_4);                 // scope 0 at $DIR/funky_arms.rs:+4:9: +4:19

39           StorageLive(_5);                 // scope 0 at $DIR/funky_arms.rs:+4:22: +4:37
40           _5 = &(*_1);                     // scope 0 at $DIR/funky_arms.rs:+4:22: +4:37
-           _4 = Formatter::<'_>::sign_plus(move _5) -> bb1; // scope 0 at $DIR/funky_arms.rs:+4:22: +4:37
-                                            // mir::Constant
-                                            // + span: $DIR/funky_arms.rs:15:26: 15:35
-                                            // + literal: Const { ty: for<'a> fn(&'a Formatter<'_>) -> bool {Formatter::<'_>::sign_plus}, val: Value(<ZST>) }
-   
-       bb1: {
-       bb1: {
+           StorageLive(_22);                // scope 4 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+           StorageLive(_23);                // scope 4 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+           _23 = ((*_5).0: u32);            // scope 4 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+           StorageLive(_24);                // scope 4 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+           _24 = const 1_u32;               // scope 4 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ -         _22 = BitAnd(move _23, move _24); // scope 4 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+ +         _22 = BitAnd(move _23, const 1_u32); // scope 4 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+           StorageDead(_24);                // scope 4 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+           StorageDead(_23);                // scope 4 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+           _4 = Ne(move _22, const 0_u32);  // scope 4 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
+           StorageDead(_22);                // scope 4 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
48           StorageDead(_5);                 // scope 0 at $DIR/funky_arms.rs:+4:36: +4:37
49           StorageLive(_6);                 // scope 1 at $DIR/funky_arms.rs:+8:9: +8:13
-           switchInt(_4) -> [0: bb3, otherwise: bb2]; // scope 1 at $DIR/funky_arms.rs:+8:16: +8:32
+           switchInt(_4) -> [0: bb2, otherwise: bb1]; // scope 1 at $DIR/funky_arms.rs:+8:16: +8:32
52   
-       bb2: {
+       bb1: {
+       bb1: {
54           Deinit(_6);                      // scope 1 at $DIR/funky_arms.rs:+10:17: +10:41
55           discriminant(_6) = 1;            // scope 1 at $DIR/funky_arms.rs:+10:17: +10:41
-           goto -> bb4;                     // scope 1 at $DIR/funky_arms.rs:+10:17: +10:41
+           goto -> bb3;                     // scope 1 at $DIR/funky_arms.rs:+10:17: +10:41
58   
-       bb3: {
+       bb2: {
+       bb2: {
60           Deinit(_6);                      // scope 1 at $DIR/funky_arms.rs:+9:18: +9:38
61           discriminant(_6) = 0;            // scope 1 at $DIR/funky_arms.rs:+9:18: +9:38
-           goto -> bb4;                     // scope 1 at $DIR/funky_arms.rs:+9:18: +9:38
+           goto -> bb3;                     // scope 1 at $DIR/funky_arms.rs:+9:18: +9:38
64   
-       bb4: {
+       bb3: {
+       bb3: {
66           StorageLive(_7);                 // scope 3 at $DIR/funky_arms.rs:+13:30: +13:45
67           StorageLive(_8);                 // scope 3 at $DIR/funky_arms.rs:+13:30: +13:45
68           _8 = &(*_1);                     // scope 3 at $DIR/funky_arms.rs:+13:30: +13:45

-           _7 = Formatter::<'_>::precision(move _8) -> bb5; // scope 3 at $DIR/funky_arms.rs:+13:30: +13:45
-                                            // mir::Constant
-                                            // + span: $DIR/funky_arms.rs:24:34: 24:43
-                                            // + literal: Const { ty: for<'a> fn(&'a Formatter<'_>) -> Option<usize> {Formatter::<'_>::precision}, val: Value(<ZST>) }
-   
-       bb5: {
-       bb5: {
+           _7 = ((*_8).4: std::option::Option<usize>); // scope 5 at $SRC_DIR/core/src/fmt/mod.rs:LL:COL
76           StorageDead(_8);                 // scope 3 at $DIR/funky_arms.rs:+13:44: +13:45
77           _9 = discriminant(_7);           // scope 3 at $DIR/funky_arms.rs:+13:12: +13:27
-           switchInt(move _9) -> [1: bb6, otherwise: bb8]; // scope 3 at $DIR/funky_arms.rs:+13:12: +13:27
+           switchInt(move _9) -> [1: bb4, otherwise: bb6]; // scope 3 at $DIR/funky_arms.rs:+13:12: +13:27
80   
-       bb6: {
+       bb4: {
+       bb4: {
82           StorageLive(_10);                // scope 3 at $DIR/funky_arms.rs:+13:17: +13:26
83           _10 = ((_7 as Some).0: usize);   // scope 3 at $DIR/funky_arms.rs:+13:17: +13:26
84           StorageLive(_11);                // scope 3 at $DIR/funky_arms.rs:+15:43: +15:46

90           _15 = _10 as u32 (IntToInt);     // scope 3 at $DIR/funky_arms.rs:+15:59: +15:75
91           _14 = Add(move _15, const 1_u32); // scope 3 at $DIR/funky_arms.rs:+15:59: +15:79
92           StorageDead(_15);                // scope 3 at $DIR/funky_arms.rs:+15:78: +15:79
-           _0 = float_to_exponential_common_exact::<T>(move _11, _2, move _13, move _14, _3) -> bb7; // scope 3 at $DIR/funky_arms.rs:+15:9: +15:87
+           _0 = float_to_exponential_common_exact::<T>(move _11, _2, move _13, move _14, _3) -> bb5; // scope 3 at $DIR/funky_arms.rs:+15:9: +15:87
94                                            // mir::Constant
95                                            // + span: $DIR/funky_arms.rs:26:9: 26:42
96                                            // + literal: Const { ty: for<'a, 'b, 'c> fn(&'a mut Formatter<'b>, &'c T, Sign, u32, bool) -> Result<(), std::fmt::Error> {float_to_exponential_common_exact::<T>}, val: Value(<ZST>) }
97       }
98   
-       bb7: {
+       bb5: {
+       bb5: {
100           StorageDead(_14);                // scope 3 at $DIR/funky_arms.rs:+15:86: +15:87
101           StorageDead(_13);                // scope 3 at $DIR/funky_arms.rs:+15:86: +15:87
102           StorageDead(_11);                // scope 3 at $DIR/funky_arms.rs:+15:86: +15:87

-           goto -> bb10;                    // scope 2 at $DIR/funky_arms.rs:+13:5: +18:6
+           goto -> bb8;                     // scope 2 at $DIR/funky_arms.rs:+13:5: +18:6
105   
-       bb8: {
+       bb6: {
+       bb6: {
107           StorageLive(_18);                // scope 2 at $DIR/funky_arms.rs:+17:46: +17:49
108           _18 = &mut (*_1);                // scope 2 at $DIR/funky_arms.rs:+17:46: +17:49
109           StorageLive(_20);                // scope 2 at $DIR/funky_arms.rs:+17:56: +17:60

110           _20 = _6;                        // scope 2 at $DIR/funky_arms.rs:+17:56: +17:60
-           _0 = float_to_exponential_common_shortest::<T>(move _18, _2, move _20, _3) -> bb9; // scope 2 at $DIR/funky_arms.rs:+17:9: +17:68
+           _0 = float_to_exponential_common_shortest::<T>(move _18, _2, move _20, _3) -> bb7; // scope 2 at $DIR/funky_arms.rs:+17:9: +17:68
112                                            // mir::Constant
113                                            // + span: $DIR/funky_arms.rs:28:9: 28:45
114                                            // + literal: Const { ty: for<'a, 'b, 'c> fn(&'a mut Formatter<'b>, &'c T, Sign, bool) -> Result<(), std::fmt::Error> {float_to_exponential_common_shortest::<T>}, val: Value(<ZST>) }
115       }
116   
-       bb9: {
+       bb7: {
+       bb7: {
118           StorageDead(_20);                // scope 2 at $DIR/funky_arms.rs:+17:67: +17:68
119           StorageDead(_18);                // scope 2 at $DIR/funky_arms.rs:+17:67: +17:68
-           goto -> bb10;                    // scope 2 at $DIR/funky_arms.rs:+13:5: +18:6
+           goto -> bb8;                     // scope 2 at $DIR/funky_arms.rs:+13:5: +18:6
122   
-       bb10: {
+       bb8: {
+       bb8: {
124           StorageDead(_6);                 // scope 1 at $DIR/funky_arms.rs:+19:1: +19:2
125           StorageDead(_4);                 // scope 0 at $DIR/funky_arms.rs:+19:1: +19:2
126           StorageDead(_7);                 // scope 0 at $DIR/funky_arms.rs:+19:1: +19:2

thread '[mir-opt] tests/mir-opt/funky_arms.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/funky_arms.float_to_exponential_common.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3483:21


failures:
    [mir-opt] tests/mir-opt/funky_arms.rs
