plain
Suite("src/test/mir-opt") not skipped for "bootstrap::test::MirOpt" -- not in ["src/tools/tidy"]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 164 tests
........................F...F......i........................F.....F.F....F......i................... 100/164
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..F............F...........F..iF....F.......F....F.FF....F......

---- [mir-opt] mir-opt/const_prop/discriminant.rs stdout ----
---- [mir-opt] mir-opt/const_prop/discriminant.rs stdout ----
18           ((_3 as Some).0: bool) = const true; // scope 0 at $DIR/discriminant.rs:11:34: 11:44
19           discriminant(_3) = 1;            // scope 0 at $DIR/discriminant.rs:11:34: 11:44
20 -         _4 = discriminant(_3);           // scope 0 at $DIR/discriminant.rs:11:21: 11:31
- -         switchInt(move _4) -> [1_isize: bb1, otherwise: bb3]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
+ -         switchInt(move _4) -> [1_isize: bb1, 0_isize: bb3, otherwise: bb5]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
22 +         _4 = const 1_isize;              // scope 0 at $DIR/discriminant.rs:11:21: 11:31
- +         switchInt(const 1_isize) -> [1_isize: bb1, otherwise: bb3]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
+ +         switchInt(const 1_isize) -> [1_isize: bb1, 0_isize: bb3, otherwise: bb5]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
25   
26       bb1: {


44           nop;                             // scope 0 at $DIR/discriminant.rs:10:11: 12:2
45           StorageDead(_1);                 // scope 0 at $DIR/discriminant.rs:12:1: 12:2
46           return;                          // scope 0 at $DIR/discriminant.rs:12:2: 12:2
+   
+       bb5: {
+       bb5: {
+           unreachable;                     // scope 0 at $DIR/discriminant.rs:10:1: 12:2
48   }
49   


thread '[mir-opt] mir-opt/const_prop/discriminant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/discriminant.main.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3578:25

---- [mir-opt] mir-opt/76803_regression.rs stdout ----
8   
9       bb0: {
9       bb0: {
10           _2 = discriminant(_1);           // scope 0 at $DIR/76803_regression.rs:11:11: 11:12
-           switchInt(move _2) -> [0_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/76803_regression.rs:11:5: 11:12
+           switchInt(move _2) -> [0_isize: bb2, 1_isize: bb1, otherwise: bb4]; // scope 0 at $DIR/76803_regression.rs:11:5: 11:12
13   
14       bb1: {

23   
23   
24       bb3: {
25           return;                          // scope 0 at $DIR/76803_regression.rs:15:2: 15:2
+   
+       bb4: {
+       bb4: {
+           unreachable;                     // scope 0 at $DIR/76803_regression.rs:10:1: 15:2
27   }
28   


thread '[mir-opt] mir-opt/76803_regression.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/76803_regression.encode.SimplifyBranchSame.diff', src/tools/compiletest/src/runtest.rs:3578:25
---- [mir-opt] mir-opt/early_otherwise_branch_3_element_tuple.rs stdout ----
---- [mir-opt] mir-opt/early_otherwise_branch_3_element_tuple.rs stdout ----
16       let _11: u32;                        // in scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:15: 6:16
17       let _12: u32;                        // in scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:24: 6:25
18       let _13: u32;                        // in scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:33: 6:34
- +     let mut _14: isize;                  // in scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:19: 6:26
- +     let mut _15: bool;                   // in scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:19: 6:26
- +     let mut _16: isize;                  // in scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:28: 6:35
- +     let mut _17: bool;                   // in scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:28: 6:35
23       scope 1 {
24           debug a => _11;                  // in scope 1 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:15: 6:16
25           debug b => _12;                  // in scope 1 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:24: 6:25

41           StorageDead(_6);                 // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:19: 5:20
42           StorageDead(_5);                 // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:19: 5:20
43           _10 = discriminant((_4.0: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:11: 5:20
- -         switchInt(move _10) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:5: 5:20
- +         StorageLive(_14);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:5: 5:20
- +         _14 = discriminant((_4.1: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:5: 5:20
- +         StorageLive(_15);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:5: 5:20
- +         _15 = Ne(_14, _10);              // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:5: 5:20
- +         StorageDead(_14);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:5: 5:20
- +         switchInt(move _15) -> [false: bb5, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:5: 5:20
+           switchInt(move _10) -> [1_isize: bb2, 0_isize: bb1, otherwise: bb6]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:5: 5:20
52   
53       bb1: {


- +         StorageDead(_17);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:7:14: 7:15
- +         StorageDead(_15);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:7:14: 7:15
56           _0 = const 1_u32;                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:7:14: 7:15
- -         goto -> bb5;                     // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:7:14: 7:15
- +         goto -> bb4;                     // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:7:14: 7:15
+           goto -> bb5;                     // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:7:14: 7:15
60   
61       bb2: {


- -         _9 = discriminant((_4.1: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:11: 5:20
- -         switchInt(move _9) -> [1_isize: bb3, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:5: 5:20
- -     }
- -     bb3: {
- -     bb3: {
+           _9 = discriminant((_4.1: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:11: 5:20
+           switchInt(move _9) -> [1_isize: bb3, 0_isize: bb1, otherwise: bb7]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:5: 5:20
+   
+       bb3: {
+       bb3: {
67           _8 = discriminant((_4.2: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:11: 5:20
- -         switchInt(move _8) -> [1_isize: bb4, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:5: 5:20
- +         switchInt(move _8) -> [1_isize: bb3, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:5: 5:20
+           switchInt(move _8) -> [1_isize: bb4, 0_isize: bb1, otherwise: bb8]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:5:5: 5:20
71   
- -     bb4: {
- +     bb3: {
+       bb4: {
+       bb4: {
74           StorageLive(_11);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:15: 6:16
75           _11 = (((_4.0: std::option::Option<u32>) as Some).0: u32); // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:15: 6:16
76           StorageLive(_12);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:24: 6:25

81           StorageDead(_13);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:40: 6:41
82           StorageDead(_12);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:40: 6:41
83           StorageDead(_11);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:40: 6:41
- -         goto -> bb5;                     // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:40: 6:41
- +         goto -> bb4;                     // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:40: 6:41
+           goto -> bb5;                     // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:40: 6:41
87   
- -     bb5: {
- +     bb4: {
+       bb5: {
+       bb5: {
90           StorageDead(_4);                 // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:9:1: 9:2
91           return;                          // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:9:2: 9:2
- +     }
- +     bb5: {
- +     bb5: {
- +         StorageDead(_15);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:19: 6:26
- +         switchInt(_10) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:6:19: 6:26
+   
+       bb6: {
+       bb6: {
+           unreachable;                     // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:4:1: 9:2
+   
+       bb7: {
+       bb7: {
+           unreachable;                     // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:4:1: 9:2
+   
+       bb8: {
+       bb8: {
+           unreachable;                     // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:4:1: 9:2
98   }
99   


thread '[mir-opt] mir-opt/early_otherwise_branch_3_element_tuple.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/early_otherwise_branch_3_element_tuple.opt1.EarlyOtherwiseBranch.diff', src/tools/compiletest/src/runtest.rs:3578:25
---- [mir-opt] mir-opt/early_otherwise_branch.rs stdout ----
---- [mir-opt] mir-opt/early_otherwise_branch.rs stdout ----
12       let mut _7: isize;                   // in scope 0 at $DIR/early_otherwise_branch.rs:5:10: 5:17
13       let _8: u32;                         // in scope 0 at $DIR/early_otherwise_branch.rs:5:15: 5:16
14       let _9: u32;                         // in scope 0 at $DIR/early_otherwise_branch.rs:5:24: 5:25
- +     let mut _10: isize;                  // in scope 0 at $DIR/early_otherwise_branch.rs:5:19: 5:26
- +     let mut _11: bool;                   // in scope 0 at $DIR/early_otherwise_branch.rs:5:19: 5:26
17       scope 1 {
18           debug a => _8;                   // in scope 1 at $DIR/early_otherwise_branch.rs:5:15: 5:16
19           debug b => _9;                   // in scope 1 at $DIR/early_otherwise_branch.rs:5:24: 5:25

30           StorageDead(_5);                 // scope 0 at $DIR/early_otherwise_branch.rs:4:16: 4:17
31           StorageDead(_4);                 // scope 0 at $DIR/early_otherwise_branch.rs:4:16: 4:17
32           _7 = discriminant((_3.0: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch.rs:4:11: 4:17
- -         switchInt(move _7) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch.rs:4:5: 4:17
- +         StorageLive(_10);                // scope 0 at $DIR/early_otherwise_branch.rs:4:5: 4:17
- +         _10 = discriminant((_3.1: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch.rs:4:5: 4:17
- +         StorageLive(_11);                // scope 0 at $DIR/early_otherwise_branch.rs:4:5: 4:17
- +         _11 = Ne(_10, _7);               // scope 0 at $DIR/early_otherwise_branch.rs:4:5: 4:17
- +         StorageDead(_10);                // scope 0 at $DIR/early_otherwise_branch.rs:4:5: 4:17
- +         switchInt(move _11) -> [false: bb4, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch.rs:4:5: 4:17
+           switchInt(move _7) -> [1_isize: bb2, 0_isize: bb1, otherwise: bb5]; // scope 0 at $DIR/early_otherwise_branch.rs:4:5: 4:17
41   
42       bb1: {


- +         StorageDead(_11);                // scope 0 at $DIR/early_otherwise_branch.rs:6:14: 6:15
44           _0 = const 1_u32;                // scope 0 at $DIR/early_otherwise_branch.rs:6:14: 6:15
- -         goto -> bb4;                     // scope 0 at $DIR/early_otherwise_branch.rs:6:14: 6:15
- +         goto -> bb3;                     // scope 0 at $DIR/early_otherwise_branch.rs:6:14: 6:15
+           goto -> bb4;                     // scope 0 at $DIR/early_otherwise_branch.rs:6:14: 6:15
48   
49       bb2: {


- -         _6 = discriminant((_3.1: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch.rs:4:11: 4:17
- -         switchInt(move _6) -> [1_isize: bb3, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch.rs:4:5: 4:17
- -     }
- -     bb3: {
- -     bb3: {
+           _6 = discriminant((_3.1: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch.rs:4:11: 4:17
+           switchInt(move _6) -> [1_isize: bb3, 0_isize: bb1, otherwise: bb6]; // scope 0 at $DIR/early_otherwise_branch.rs:4:5: 4:17
+   
+       bb3: {
+       bb3: {
55           StorageLive(_8);                 // scope 0 at $DIR/early_otherwise_branch.rs:5:15: 5:16
56           _8 = (((_3.0: std::option::Option<u32>) as Some).0: u32); // scope 0 at $DIR/early_otherwise_branch.rs:5:15: 5:16
57           StorageLive(_9);                 // scope 0 at $DIR/early_otherwise_branch.rs:5:24: 5:25

59           _0 = const 0_u32;                // scope 1 at $DIR/early_otherwise_branch.rs:5:31: 5:32
60           StorageDead(_9);                 // scope 0 at $DIR/early_otherwise_branch.rs:5:31: 5:32
61           StorageDead(_8);                 // scope 0 at $DIR/early_otherwise_branch.rs:5:31: 5:32
- -         goto -> bb4;                     // scope 0 at $DIR/early_otherwise_branch.rs:5:31: 5:32
- +         goto -> bb3;                     // scope 0 at $DIR/early_otherwise_branch.rs:5:31: 5:32
+           goto -> bb4;                     // scope 0 at $DIR/early_otherwise_branch.rs:5:31: 5:32
65   
- -     bb4: {
- +     bb3: {
+       bb4: {
+       bb4: {
68           StorageDead(_3);                 // scope 0 at $DIR/early_otherwise_branch.rs:8:1: 8:2
69           return;                          // scope 0 at $DIR/early_otherwise_branch.rs:8:2: 8:2
- +     }
- +     bb4: {
- +     bb4: {
- +         StorageDead(_11);                // scope 0 at $DIR/early_otherwise_branch.rs:5:19: 5:26
- +         switchInt(_7) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch.rs:5:19: 5:26
+   
+       bb5: {
+       bb5: {
+           unreachable;                     // scope 0 at $DIR/early_otherwise_branch.rs:3:1: 8:2
+   
+       bb6: {
+       bb6: {
+           unreachable;                     // scope 0 at $DIR/early_otherwise_branch.rs:3:1: 8:2
76   }
77   


thread '[mir-opt] mir-opt/early_otherwise_branch.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/early_otherwise_branch.opt1.EarlyOtherwiseBranch.diff', src/tools/compiletest/src/runtest.rs:3578:25
---- [mir-opt] mir-opt/early_otherwise_branch_noopt.rs stdout ----
---- [mir-opt] mir-opt/early_otherwise_branch_noopt.rs stdout ----
37           StorageDead(_5);                 // scope 0 at $DIR/early_otherwise_branch_noopt.rs:8:16: 8:17
38           StorageDead(_4);                 // scope 0 at $DIR/early_otherwise_branch_noopt.rs:8:16: 8:17
39           _8 = discriminant((_3.0: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch_noopt.rs:8:11: 8:17
-           switchInt(move _8) -> [0_isize: bb1, otherwise: bb3]; // scope 0 at $DIR/early_otherwise_branch_noopt.rs:8:5: 8:17
+           switchInt(move _8) -> [0_isize: bb1, 1_isize: bb3, otherwise: bb8]; // scope 0 at $DIR/early_otherwise_branch_noopt.rs:8:5: 8:17
42   
43       bb1: {


44           _6 = discriminant((_3.1: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch_noopt.rs:8:11: 8:17
-           switchInt(move _6) -> [0_isize: bb2, otherwise: bb6]; // scope 0 at $DIR/early_otherwise_branch_noopt.rs:8:5: 8:17
+           switchInt(move _6) -> [0_isize: bb2, 1_isize: bb6, otherwise: bb9]; // scope 0 at $DIR/early_otherwise_branch_noopt.rs:8:5: 8:17
47   
48       bb2: {

52   
52   
53       bb3: {
54           _7 = discriminant((_3.1: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch_noopt.rs:8:11: 8:17
-           switchInt(move _7) -> [0_isize: bb5, otherwise: bb4]; // scope 0 at $DIR/early_otherwise_branch_noopt.rs:8:5: 8:17
+           switchInt(move _7) -> [0_isize: bb5, 1_isize: bb4, otherwise: bb10]; // scope 0 at $DIR/early_otherwise_branch_noopt.rs:8:5: 8:17
57   
58       bb4: {

85       bb7: {
85       bb7: {
86           StorageDead(_3);                 // scope 0 at $DIR/early_otherwise_branch_noopt.rs:14:1: 14:2
87           return;                          // scope 0 at $DIR/early_otherwise_branch_noopt.rs:14:2: 14:2
+   
+       bb8: {
+       bb8: {
+           unreachable;                     // scope 0 at $DIR/early_otherwise_branch_noopt.rs:7:1: 14:2
+   
+       bb9: {
+       bb9: {
+           unreachable;                     // scope 0 at $DIR/early_otherwise_branch_noopt.rs:7:1: 14:2
+   
+       bb10: {
+       bb10: {
+           unreachable;                     // scope 0 at $DIR/early_otherwise_branch_noopt.rs:7:1: 14:2
89   }
90   


thread '[mir-opt] mir-opt/early_otherwise_branch_noopt.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/early_otherwise_branch_noopt.noopt1.EarlyOtherwiseBranch.diff', src/tools/compiletest/src/runtest.rs:3578:25
---- [mir-opt] mir-opt/funky_arms.rs stdout ----
73       bb5: {
73       bb5: {
74           StorageDead(_8);                 // scope 2 at $DIR/funky_arms.rs:24:44: 24:45
75           _9 = discriminant(_7);           // scope 2 at $DIR/funky_arms.rs:24:12: 24:27
-           switchInt(move _9) -> [1_isize: bb6, otherwise: bb8]; // scope 2 at $DIR/funky_arms.rs:24:12: 24:27
+           switchInt(move _9) -> [1_isize: bb6, 0_isize: bb8, otherwise: bb11]; // scope 2 at $DIR/funky_arms.rs:24:12: 24:27
78   
79       bb6: {


139           StorageDead(_4);                 // scope 0 at $DIR/funky_arms.rs:30:1: 30:2
140           StorageDead(_7);                 // scope 0 at $DIR/funky_arms.rs:30:1: 30:2
141           return;                          // scope 0 at $DIR/funky_arms.rs:30:2: 30:2
+   
+       bb11: {
+       bb11: {
+           unreachable;                     // scope 0 at $DIR/funky_arms.rs:11:1: 30:2
143   }
144   


thread '[mir-opt] mir-opt/funky_arms.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/funky_arms.float_to_exponential_common.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3578:25
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
157           StorageDead(_1);                 // scope 0 at $DIR/issue-73223.rs:9:1: 9:2
158           return;                          // scope 0 at $DIR/issue-73223.rs:9:2: 9:2
+   
+       bb5: {
+       bb5: {
+           unreachable;                     // scope 0 at $DIR/issue-73223.rs:1:1: 9:2
160   }
161   
162 


thread '[mir-opt] mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.64bit.diff', src/tools/compiletest/src/runtest.rs:3578:25
---- [mir-opt] mir-opt/matches_reduce_branches.rs stdout ----
---- [mir-opt] mir-opt/matches_reduce_branches.rs stdout ----
5       debug bar => _1;                     // in scope 0 at $DIR/matches_reduce_branches.rs:7:8: 7:11
6       let mut _0: ();                      // return place in scope 0 at $DIR/matches_reduce_branches.rs:7:25: 7:25
7       let mut _2: isize;                   // in scope 0 at $DIR/matches_reduce_branches.rs:8:22: 8:26
- +     let mut _3: isize;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
10       bb0: {
10       bb0: {
11           _2 = discriminant(_1);           // scope 0 at $DIR/matches_reduce_branches.rs:8:17: 8:20

- -         switchInt(move _2) -> [0_isize: bb2, otherwise: bb1]; // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
- -     }
- -     bb1: {
- -     bb1: {
- -         goto -> bb3;                     // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
- -     }
- -     bb2: {
- -     bb2: {
- -         goto -> bb3;                     // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
- -     }
- -     bb3: {
- -     bb3: {
- +         StorageLive(_3);                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
- +         _3 = move _2;                    // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
- +         StorageDead(_3);                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           switchInt(move _2) -> [0_isize: bb2, 1_isize: bb1, otherwise: bb4]; // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+   
+       bb1: {
+       bb1: {
+           goto -> bb3;                     // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+   
+       bb2: {
+       bb2: {
+           goto -> bb3;                     // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+   
+       bb3: {
+       bb3: {
27           return;                          // scope 0 at $DIR/matches_reduce_branches.rs:11:2: 11:2
+   
+       bb4: {
+       bb4: {
+           unreachable;                     // scope 0 at $DIR/matches_reduce_branches.rs:7:1: 11:2
29   }
30   


thread '[mir-opt] mir-opt/matches_reduce_branches.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/matches_reduce_branches.foo.MatchBranchSimplification.64bit.diff', src/tools/compiletest/src/runtest.rs:3578:25
---- [mir-opt] mir-opt/matches_u8.rs stdout ----
8   
9       bb0: {
9       bb0: {
10           _2 = discriminant(_1);           // scope 0 at $DIR/matches_u8.rs:12:11: 12:12
-           switchInt(move _2) -> [0_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/matches_u8.rs:12:5: 12:12
+           switchInt(move _2) -> [0_isize: bb2, 1_isize: bb1, otherwise: bb4]; // scope 0 at $DIR/matches_u8.rs:12:5: 12:12
13   
14       bb1: {

23   
23   
24       bb3: {
25           return;                          // scope 0 at $DIR/matches_u8.rs:16:2: 16:2
+   
+       bb4: {
+       bb4: {
+           unreachable;                     // scope 0 at $DIR/matches_u8.rs:11:1: 16:2
27   }
28   


thread '[mir-opt] mir-opt/matches_u8.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/matches_u8.exhaustive_match.MatchBranchSimplification.64bit.diff', src/tools/compiletest/src/runtest.rs:3578:25
---- [mir-opt] mir-opt/remove_storage_markers.rs stdout ----
---- [mir-opt] mir-opt/remove_storage_markers.rs stdout ----
67           _9 = &mut (*_10);                // scope 3 at $DIR/remove_storage_markers.rs:8:14: 8:19
68 -         StorageLive(_18);                // scope 7 at $DIR/remove_storage_markers.rs:8:14: 8:19
69           _18 = &mut (*_9);                // scope 7 at $DIR/remove_storage_markers.rs:8:14: 8:19
-           _8 = <std::ops::Range<i32> as iter::range::RangeIteratorImpl>::spec_next(move _18) -> bb4; // scope 7 at $DIR/remove_storage_markers.rs:8:14: 8:19
+           _8 = <std::ops::Range<i32> as iter::range::RangeIteratorImpl>::spec_next(move _18) -> bb5; // scope 7 at $DIR/remove_storage_markers.rs:8:14: 8:19
71                                            // mir::Constant
72                                            // + span: $DIR/remove_storage_markers.rs:8:14: 8:19
73                                            // + literal: Const { ty: for<'r> fn(&'r mut std::ops::Range<i32>) -> std::option::Option<<std::ops::Range<i32> as std::iter::range::RangeIteratorImpl>::Item> {<std::ops::Range<i32> as std::iter::range::RangeIteratorImpl>::spec_next}, val: Value(Scalar(<ZST>)) }
113       }
114   
115       bb4: {
115       bb4: {
+           unreachable;                     // scope 0 at $DIR/remove_storage_markers.rs:6:1: 11:2
+   
+       bb5: {
+       bb5: {
116 -         StorageDead(_18);                // scope 7 at $DIR/remove_storage_markers.rs:8:14: 8:19
117 -         StorageDead(_9);                 // scope 3 at $DIR/remove_storage_markers.rs:8:18: 8:19
118           _11 = discriminant(_8);          // scope 3 at $DIR/remove_storage_markers.rs:8:14: 8:19

-           switchInt(move _11) -> [0_isize: bb2, otherwise: bb3]; // scope 3 at $DIR/remove_storage_markers.rs:8:14: 8:19
+           switchInt(move _11) -> [0_isize: bb2, 1_isize: bb3, otherwise: bb4]; // scope 3 at $DIR/remove_storage_markers.rs:8:14: 8:19
121   }
122   


thread '[mir-opt] mir-opt/remove_storage_markers.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/remove_storage_markers.main.RemoveStorageMarkers.diff', src/tools/compiletest/src/runtest.rs:3578:25
---- [mir-opt] mir-opt/simplify-locals-fixedpoint.rs stdout ----
---- [mir-opt] mir-opt/simplify-locals-fixedpoint.rs stdout ----
26           StorageDead(_3);                 // scope 0 at $DIR/simplify-locals-fixedpoint.rs:4:68: 4:69
27           StorageDead(_2);                 // scope 0 at $DIR/simplify-locals-fixedpoint.rs:4:68: 4:69
28           _5 = discriminant((_1.0: std::option::Option<u8>)); // scope 0 at $DIR/simplify-locals-fixedpoint.rs:4:12: 4:27
-           switchInt(move _5) -> [1_isize: bb1, otherwise: bb3]; // scope 0 at $DIR/simplify-locals-fixedpoint.rs:4:12: 4:27
+           switchInt(move _5) -> [1_isize: bb1, 0_isize: bb3, otherwise: bb5]; // scope 0 at $DIR/simplify-locals-fixedpoint.rs:4:12: 4:27
31   
32       bb1: {

54       bb4: {
54       bb4: {
55           StorageDead(_1);                 // scope 0 at $DIR/simplify-locals-fixedpoint.rs:9:1: 9:2
56           return;                          // scope 0 at $DIR/simplify-locals-fixedpoint.rs:9:2: 9:2
+   
+       bb5: {
+       bb5: {
+           unreachable;                     // scope 0 at $DIR/simplify-locals-fixedpoint.rs:3:1: 9:2
58   }
59   


thread '[mir-opt] mir-opt/simplify-locals-fixedpoint.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_locals_fixedpoint.foo.SimplifyLocals.diff', src/tools/compiletest/src/runtest.rs:3578:25
---- [mir-opt] mir-opt/separate_const_switch.rs stdout ----
30       bb0: {
30       bb0: {
31           StorageLive(_2);                 // scope 0 at $DIR/separate_const_switch.rs:14:11: 19:6
32           _3 = discriminant(_1);           // scope 0 at $DIR/separate_const_switch.rs:15:15: 15:16
-           switchInt(move _3) -> [0_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/separate_const_switch.rs:15:9: 15:16
+ -         switchInt(move _3) -> [0_isize: bb2, 1_isize: bb1, otherwise: bb7]; // scope 0 at $DIR/separate_const_switch.rs:15:9: 15:16
+ +         switchInt(move _3) -> [0_isize: bb2, 1_isize: bb1, otherwise: bb6]; // scope 0 at $DIR/separate_const_switch.rs:15:9: 15:16
35   
36       bb1: {


44           StorageDead(_6);                 // scope 0 at $DIR/separate_const_switch.rs:17:43: 17:44
45 -         goto -> bb3;                     // scope 0 at $DIR/separate_const_switch.rs:17:43: 17:44
46 +         _8 = discriminant(_2);           // scope 0 at $DIR/separate_const_switch.rs:14:11: 19:6
- +         switchInt(move _8) -> [0_isize: bb4, otherwise: bb3]; // scope 0 at $DIR/separate_const_switch.rs:14:5: 19:6
+ +         switchInt(move _8) -> [0_isize: bb4, 1_isize: bb3, otherwise: bb7]; // scope 0 at $DIR/separate_const_switch.rs:14:5: 19:6
49   
50       bb2: {

61 - 
61 - 
62 -     bb3: {
63           _8 = discriminant(_2);           // scope 0 at $DIR/separate_const_switch.rs:14:11: 19:6
- -         switchInt(move _8) -> [0_isize: bb5, otherwise: bb4]; // scope 0 at $DIR/separate_const_switch.rs:14:5: 19:6
- +         switchInt(move _8) -> [0_isize: bb4, otherwise: bb3]; // scope 0 at $DIR/separate_const_switch.rs:14:5: 19:6
+ -         switchInt(move _8) -> [0_isize: bb5, 1_isize: bb4, otherwise: bb8]; // scope 0 at $DIR/separate_const_switch.rs:14:5: 19:6
+ +         switchInt(move _8) -> [0_isize: bb4, 1_isize: bb3, otherwise: bb7]; // scope 0 at $DIR/separate_const_switch.rs:14:5: 19:6
67   
68 -     bb4: {

93 +     bb5: {
93 +     bb5: {
94           StorageDead(_2);                 // scope 0 at $DIR/separate_const_switch.rs:23:1: 23:2
95           return;                          // scope 0 at $DIR/separate_const_switch.rs:23:2: 23:2
+   
+ -     bb7: {
+ +     bb6: {
+ +     bb6: {
+           unreachable;                     // scope 0 at $DIR/separate_const_switch.rs:9:1: 23:2
+   
+ -     bb8: {
+ +     bb7: {
+ +     bb7: {
+           unreachable;                     // scope 0 at $DIR/separate_const_switch.rs:9:1: 23:2
97   }
98   


thread '[mir-opt] mir-opt/separate_const_switch.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/separate_const_switch.too_complex.SeparateConstSwitch.diff', src/tools/compiletest/src/runtest.rs:3578:25
---- [mir-opt] mir-opt/simplify_try.rs stdout ----
---- [mir-opt] mir-opt/simplify_try.rs stdout ----
46           _3 = move _4;                    // scope 4 at $DIR/simplify_try.rs:21:19: 21:33
47           StorageDead(_4);                 // scope 0 at $DIR/simplify_try.rs:21:32: 21:33
48           _5 = discriminant(_3);           // scope 0 at $DIR/simplify_try.rs:21:19: 21:33
-           switchInt(move _5) -> [0_isize: bb1, otherwise: bb2]; // scope 0 at $DIR/simplify_try.rs:21:13: 21:33
+           switchInt(move _5) -> [0_isize: bb1, 1_isize: bb2, otherwise: bb3]; // scope 0 at $DIR/simplify_try.rs:21:13: 21:33
51   
52       bb1: {

---
test result: FAILED. 145 passed; 16 failed; 3 ignored; 0 measured; 0 filtered out; finished in 6.93s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:21
