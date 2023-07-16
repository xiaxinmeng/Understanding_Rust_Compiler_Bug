plain
 finished in 0.645 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 210 tests
.FF......F..F.F.FF...F.F..............F...F........i..........F....FFF..........F...FF.F 88/210
...F.FF.....FF.FFFF.FFF.....F..F......i.FF.F....F.....FF..F..F.FF.F.F.FFF.F.F.FFiiFF.F.F 176/210
.i...FF..FFF.FFFF......FFFFF.FFFF.

---- [mir-opt] src/test/mir-opt/building/issue_101867.rs stdout ----
---- [mir-opt] src/test/mir-opt/building/issue_101867.rs stdout ----
27         StorageLive(_5);                 // scope 1 at $DIR/issue_101867.rs:+2:14: +2:15
28         FakeRead(ForMatchedPlace(None), _1); // scope 1 at $DIR/issue_101867.rs:+2:19: +2:20
29         _6 = discriminant(_1);           // scope 1 at $DIR/issue_101867.rs:+2:19: +2:20
-         switchInt(move _6) -> [1_isize: bb4, otherwise: bb3]; // scope 1 at $DIR/issue_101867.rs:+2:9: +2:16
+         switchInt(move _6) -> [1: bb4, otherwise: bb3]; // scope 1 at $DIR/issue_101867.rs:+2:9: +2:16
32 
33     bb1: {


thread '[mir-opt] src/test/mir-opt/building/issue_101867.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/building/issue_101867.main.built.after.mir', src/tools/compiletest/src/runtest.rs:3451:21

---- [mir-opt] src/test/mir-opt/bool_compare.rs stdout ----
---- [mir-opt] src/test/mir-opt/bool_compare.rs stdout ----
14 -         _2 = Ne(move _3, const true);    // scope 0 at $DIR/bool_compare.rs:+1:8: +1:17
15 +         _2 = Not(move _3);               // scope 0 at $DIR/bool_compare.rs:+1:8: +1:17
16           StorageDead(_3);                 // scope 0 at $DIR/bool_compare.rs:+1:16: +1:17
-           switchInt(move _2) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/bool_compare.rs:+1:8: +1:17
+           switchInt(move _2) -> [0: bb2, otherwise: bb1]; // scope 0 at $DIR/bool_compare.rs:+1:8: +1:17
19   
20       bb1: {


thread '[mir-opt] src/test/mir-opt/bool_compare.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/bool_compare.opt1.InstCombine.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/building/issue_49232.rs stdout ----
---- [mir-opt] src/test/mir-opt/building/issue_49232.rs stdout ----
25         StorageLive(_3);                 // scope 0 at $DIR/issue_49232.rs:+3:19: +3:23
26         _3 = const true;                 // scope 0 at $DIR/issue_49232.rs:+3:19: +3:23
27         FakeRead(ForMatchedPlace(None), _3); // scope 0 at $DIR/issue_49232.rs:+3:19: +3:23
-         switchInt(_3) -> [false: bb3, otherwise: bb4]; // scope 0 at $DIR/issue_49232.rs:+3:13: +3:23
+         switchInt(_3) -> [0: bb3, otherwise: bb4]; // scope 0 at $DIR/issue_49232.rs:+3:13: +3:23
30 
31     bb3: {


thread '[mir-opt] src/test/mir-opt/building/issue_49232.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/building/issue_49232.main.built.after.mir', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/76803_regression.rs stdout ----
8   
9       bb0: {
9       bb0: {
10           _2 = discriminant(_1);           // scope 0 at $DIR/76803_regression.rs:+1:11: +1:12
-           switchInt(move _2) -> [0_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/76803_regression.rs:+1:5: +1:12
+           switchInt(move _2) -> [0: bb2, otherwise: bb1]; // scope 0 at $DIR/76803_regression.rs:+1:5: +1:12
13   
14       bb1: {


thread '[mir-opt] src/test/mir-opt/76803_regression.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/76803_regression.encode.SimplifyBranchSame.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/building/match_false_edges.rs stdout ----
---- [mir-opt] src/test/mir-opt/building/match_false_edges.rs stdout ----
28         _2 = Option::<i32>::Some(const 42_i32); // scope 0 at $DIR/match_false_edges.rs:+1:19: +1:27
29         FakeRead(ForMatchedPlace(None), _2); // scope 0 at $DIR/match_false_edges.rs:+1:19: +1:27
30         _3 = discriminant(_2);           // scope 0 at $DIR/match_false_edges.rs:+1:19: +1:27
-         switchInt(move _3) -> [0_isize: bb1, 1_isize: bb2, otherwise: bb4]; // scope 0 at $DIR/match_false_edges.rs:+1:13: +1:27
+         switchInt(move _3) -> [0: bb1, 1: bb2, otherwise: bb4]; // scope 0 at $DIR/match_false_edges.rs:+1:13: +1:27
33 
34     bb1: {

60     }
60     }
61 
62     bb6: {
-         switchInt(move _7) -> [false: bb8, otherwise: bb7]; // scope 0 at $DIR/match_false_edges.rs:+2:20: +2:27
+         switchInt(move _7) -> [0: bb8, otherwise: bb7]; // scope 0 at $DIR/match_false_edges.rs:+2:20: +2:27
65 
66     bb7: {


thread '[mir-opt] src/test/mir-opt/building/match_false_edges.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/building/match_false_edges.full_tested_match.built.after.mir', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/const_goto_const_eval_fail.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_goto_const_eval_fail.rs stdout ----
10           StorageLive(_1);                 // scope 0 at $DIR/const_goto_const_eval_fail.rs:+1:11: +6:6
11           StorageLive(_2);                 // scope 0 at $DIR/const_goto_const_eval_fail.rs:+2:15: +2:16
12           _2 = const A;                    // scope 0 at $DIR/const_goto_const_eval_fail.rs:+2:15: +2:16
-           switchInt(_2) -> [1_i32: bb2, 2_i32: bb2, 3_i32: bb2, otherwise: bb1]; // scope 0 at $DIR/const_goto_const_eval_fail.rs:+2:9: +2:16
+           switchInt(_2) -> [1: bb2, 2: bb2, 3: bb2, otherwise: bb1]; // scope 0 at $DIR/const_goto_const_eval_fail.rs:+2:9: +2:16
15   
16       bb1: {

21       bb2: {
21       bb2: {
22           _1 = const B;                    // scope 0 at $DIR/const_goto_const_eval_fail.rs:+3:26: +3:27
23 -         goto -> bb3;                     // scope 0 at $DIR/const_goto_const_eval_fail.rs:+3:26: +3:27
- +         switchInt(_1) -> [false: bb4, otherwise: bb3]; // scope 0 at $DIR/const_goto_const_eval_fail.rs:+1:5: +6:6
+ +         switchInt(_1) -> [0: bb4, otherwise: bb3]; // scope 0 at $DIR/const_goto_const_eval_fail.rs:+1:5: +6:6
26   
27       bb3: {


- -         switchInt(_1) -> [false: bb5, otherwise: bb4]; // scope 0 at $DIR/const_goto_const_eval_fail.rs:+1:5: +6:6
+ -         switchInt(_1) -> [0: bb5, otherwise: bb4]; // scope 0 at $DIR/const_goto_const_eval_fail.rs:+1:5: +6:6
30 - 
31 -     bb4: {


thread '[mir-opt] src/test/mir-opt/const_goto_const_eval_fail.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_goto_const_eval_fail.f.ConstGoto.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/building/simple_match.rs stdout ----
6 
7     bb0: {
7     bb0: {
8         FakeRead(ForMatchedPlace(None), _1); // scope 0 at $DIR/simple_match.rs:+1:11: +1:12
-         switchInt(_1) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/simple_match.rs:+1:5: +1:12
+         switchInt(_1) -> [0: bb2, otherwise: bb1]; // scope 0 at $DIR/simple_match.rs:+1:5: +1:12
11 
12     bb1: {


thread '[mir-opt] src/test/mir-opt/building/simple_match.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/building/simple_match.match_bool.built.after.mir', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/const_goto.rs stdout ----
11       bb0: {
11       bb0: {
12 -         StorageLive(_2);                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
13 -         _3 = discriminant(_1);           // scope 0 at $DIR/const_goto.rs:+1:17: +1:20
- -         switchInt(move _3) -> [1_isize: bb2, 2_isize: bb2, otherwise: bb1]; // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+ -         switchInt(move _3) -> [1: bb2, 2: bb2, otherwise: bb1]; // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
15 +         _2 = discriminant(_1);           // scope 0 at $DIR/const_goto.rs:+1:17: +1:20
- +         switchInt(move _2) -> [1_isize: bb2, 2_isize: bb2, otherwise: bb1]; // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+ +         switchInt(move _2) -> [1: bb2, 2: bb2, otherwise: bb1]; // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
18   
19       bb1: {

29 -     }
29 -     }
30 - 
31 -     bb3: {
- -         switchInt(move _2) -> [false: bb5, otherwise: bb4]; // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+ -         switchInt(move _2) -> [0: bb5, otherwise: bb4]; // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
34 - 
35 -     bb4: {


thread '[mir-opt] src/test/mir-opt/const_goto.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_goto.issue_77355_opt.ConstGoto.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/const_goto_storage.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_goto_storage.rs stdout ----
23 -         StorageLive(_5);                 // scope 0 at $DIR/const_goto_storage.rs:+2:21: +2:52
24 -         StorageLive(_6);                 // scope 0 at $DIR/const_goto_storage.rs:+2:24: +2:28
25 -         _6 = const true;                 // scope 0 at $DIR/const_goto_storage.rs:+2:24: +2:28
- -         switchInt(move _6) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/const_goto_storage.rs:+2:24: +2:28
+ -         switchInt(move _6) -> [0: bb2, otherwise: bb1]; // scope 0 at $DIR/const_goto_storage.rs:+2:24: +2:28
27 +         StorageLive(_2);                 // scope 0 at $DIR/const_goto_storage.rs:+2:24: +2:28
28 +         _2 = const true;                 // scope 0 at $DIR/const_goto_storage.rs:+2:24: +2:28
- +         switchInt(move _2) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/const_goto_storage.rs:+2:24: +2:28
+ +         switchInt(move _2) -> [0: bb2, otherwise: bb1]; // scope 0 at $DIR/const_goto_storage.rs:+2:24: +2:28
31   
32       bb1: {

41 - 
41 - 
42 -     bb3: {
43 -         StorageDead(_6);                 // scope 0 at $DIR/const_goto_storage.rs:+2:51: +2:52
- -         switchInt(move _5) -> [false: bb5, otherwise: bb4]; // scope 0 at $DIR/const_goto_storage.rs:+2:21: +2:52
+ -         switchInt(move _5) -> [0: bb5, otherwise: bb4]; // scope 0 at $DIR/const_goto_storage.rs:+2:21: +2:52
46 - 
47 -     bb4: {

56 - 
56 - 
57 -     bb6: {
58 -         StorageDead(_5);                 // scope 0 at $DIR/const_goto_storage.rs:+2:75: +2:76
- -         switchInt(move _4) -> [false: bb8, otherwise: bb7]; // scope 0 at $DIR/const_goto_storage.rs:+2:18: +2:76
+ -         switchInt(move _4) -> [0: bb8, otherwise: bb7]; // scope 0 at $DIR/const_goto_storage.rs:+2:18: +2:76
61 - 
62 -     bb7: {

70 -     }
70 -     }
71 - 
72 -     bb9: {
- -         switchInt(move _3) -> [false: bb11, otherwise: bb10]; // scope 0 at $DIR/const_goto_storage.rs:+2:15: +6:10
+ -         switchInt(move _3) -> [0: bb11, otherwise: bb10]; // scope 0 at $DIR/const_goto_storage.rs:+2:15: +6:10
75 - 
76 -     bb10: {


thread '[mir-opt] src/test/mir-opt/const_goto_storage.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_goto_storage.match_nested_if.ConstGoto.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/const_prop/control_flow_simplification.rs stdout ----
9       bb0: {
9       bb0: {
10           StorageLive(_1);                 // scope 0 at $DIR/control_flow_simplification.rs:+1:8: +1:21
11           _1 = const _;                    // scope 0 at $DIR/control_flow_simplification.rs:+1:8: +1:21
- -         switchInt(move _1) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/control_flow_simplification.rs:+1:8: +1:21
- +         switchInt(const false) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/control_flow_simplification.rs:+1:8: +1:21
+ -         switchInt(move _1) -> [0: bb2, otherwise: bb1]; // scope 0 at $DIR/control_flow_simplification.rs:+1:8: +1:21
+ +         switchInt(const false) -> [0: bb2, otherwise: bb1]; // scope 0 at $DIR/control_flow_simplification.rs:+1:8: +1:21
15   
16       bb1: {


thread '[mir-opt] src/test/mir-opt/const_prop/control_flow_simplification.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/control_flow_simplification.hello.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/const_prop/discriminant.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/discriminant.rs stdout ----
21           ((_3 as Some).0: bool) = const true; // scope 2 at $DIR/discriminant.rs:+1:34: +1:44
22           discriminant(_3) = 1;            // scope 2 at $DIR/discriminant.rs:+1:34: +1:44
23 -         _4 = discriminant(_3);           // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
- -         switchInt(move _4) -> [1_isize: bb1, otherwise: bb3]; // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
+ -         switchInt(move _4) -> [1: bb1, otherwise: bb3]; // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
25 +         _4 = const 1_isize;              // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
- +         switchInt(const 1_isize) -> [1_isize: bb1, otherwise: bb3]; // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
+ +         switchInt(const 1_isize) -> [1: bb1, otherwise: bb3]; // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
28   
29       bb1: {


-           switchInt(((_3 as Some).0: bool)) -> [false: bb3, otherwise: bb2]; // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
+           switchInt(((_3 as Some).0: bool)) -> [0: bb3, otherwise: bb2]; // scope 2 at $DIR/discriminant.rs:+1:21: +1:31
32   
33       bb2: {


thread '[mir-opt] src/test/mir-opt/const_prop/discriminant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/discriminant.main.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/const_prop/switch_int.rs stdout ----
8       bb0: {
8       bb0: {
9           StorageLive(_1);                 // scope 0 at $DIR/switch_int.rs:+1:11: +1:12
10           _1 = const 1_i32;                // scope 0 at $DIR/switch_int.rs:+1:11: +1:12
- -         switchInt(_1) -> [1_i32: bb2, otherwise: bb1]; // scope 0 at $DIR/switch_int.rs:+1:5: +1:12
- +         switchInt(const 1_i32) -> [1_i32: bb2, otherwise: bb1]; // scope 0 at $DIR/switch_int.rs:+1:5: +1:12
+ -         switchInt(_1) -> [1: bb2, otherwise: bb1]; // scope 0 at $DIR/switch_int.rs:+1:5: +1:12
+ +         switchInt(const 1_i32) -> [1: bb2, otherwise: bb1]; // scope 0 at $DIR/switch_int.rs:+1:5: +1:12
14   
15       bb1: {


thread '[mir-opt] src/test/mir-opt/const_prop/switch_int.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/switch_int.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/dataflow-const-prop/enum.rs stdout ----
---- [mir-opt] src/test/mir-opt/dataflow-const-prop/enum.rs stdout ----
28           discriminant(_1) = 0;            // scope 0 at $DIR/enum.rs:+1:13: +1:21
29           StorageLive(_2);                 // scope 1 at $DIR/enum.rs:+2:9: +2:10
30           _3 = discriminant(_1);           // scope 1 at $DIR/enum.rs:+2:19: +2:20
-           switchInt(move _3) -> [0_isize: bb3, 1_isize: bb1, otherwise: bb2]; // scope 1 at $DIR/enum.rs:+2:13: +2:20
+           switchInt(move _3) -> [0: bb3, 1: bb1, otherwise: bb2]; // scope 1 at $DIR/enum.rs:+2:13: +2:20
33   
34       bb1: {


thread '[mir-opt] src/test/mir-opt/dataflow-const-prop/enum.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/dataflow-const-prop/enum.main.DataflowConstProp.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/dataflow-const-prop/issue_81605.rs stdout ----
---- [mir-opt] src/test/mir-opt/dataflow-const-prop/issue_81605.rs stdout ----
10           StorageLive(_1);                 // scope 0 at $DIR/issue_81605.rs:+1:9: +1:33
11           StorageLive(_2);                 // scope 0 at $DIR/issue_81605.rs:+1:12: +1:16
12           _2 = const true;                 // scope 0 at $DIR/issue_81605.rs:+1:12: +1:16
- -         switchInt(move _2) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/issue_81605.rs:+1:12: +1:16
- +         switchInt(const true) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/issue_81605.rs:+1:12: +1:16
+ -         switchInt(move _2) -> [0: bb2, otherwise: bb1]; // scope 0 at $DIR/issue_81605.rs:+1:12: +1:16
+ +         switchInt(const true) -> [0: bb2, otherwise: bb1]; // scope 0 at $DIR/issue_81605.rs:+1:12: +1:16
16   
17       bb1: {


thread '[mir-opt] src/test/mir-opt/dataflow-const-prop/issue_81605.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/dataflow-const-prop/issue_81605.f.DataflowConstProp.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/dataflow-const-prop/if.rs stdout ----
---- [mir-opt] src/test/mir-opt/dataflow-const-prop/if.rs stdout ----
42 +         _4 = const 1_i32;                // scope 1 at $DIR/if.rs:+2:16: +2:17
43 +         _3 = const true;                 // scope 1 at $DIR/if.rs:+2:16: +2:22
44           StorageDead(_4);                 // scope 1 at $DIR/if.rs:+2:21: +2:22
- -         switchInt(move _3) -> [false: bb2, otherwise: bb1]; // scope 1 at $DIR/if.rs:+2:16: +2:22
- +         switchInt(const true) -> [false: bb2, otherwise: bb1]; // scope 1 at $DIR/if.rs:+2:16: +2:22
+ -         switchInt(move _3) -> [0: bb2, otherwise: bb1]; // scope 1 at $DIR/if.rs:+2:16: +2:22
+ +         switchInt(const true) -> [0: bb2, otherwise: bb1]; // scope 1 at $DIR/if.rs:+2:16: +2:22
48   
49       bb1: {


73 +         _9 = const 1_i32;                // scope 3 at $DIR/if.rs:+5:16: +5:17
74 +         _8 = const true;                 // scope 3 at $DIR/if.rs:+5:16: +5:22
75           StorageDead(_9);                 // scope 3 at $DIR/if.rs:+5:21: +5:22
- -         switchInt(move _8) -> [false: bb5, otherwise: bb4]; // scope 3 at $DIR/if.rs:+5:16: +5:22
- +         switchInt(const true) -> [false: bb5, otherwise: bb4]; // scope 3 at $DIR/if.rs:+5:16: +5:22
+ -         switchInt(move _8) -> [0: bb5, otherwise: bb4]; // scope 3 at $DIR/if.rs:+5:16: +5:22
+ +         switchInt(const true) -> [0: bb5, otherwise: bb4]; // scope 3 at $DIR/if.rs:+5:16: +5:22
79   
80       bb4: {


thread '[mir-opt] src/test/mir-opt/dataflow-const-prop/if.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/dataflow-const-prop/if.main.DataflowConstProp.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/dead-store-elimination/cycle.rs stdout ----
37       }
38   
39       bb2: {
39       bb2: {
- -         switchInt(move _5) -> [false: bb4, otherwise: bb3]; // scope 0 at $DIR/cycle.rs:+3:11: +3:17
- +         switchInt(move _4) -> [false: bb4, otherwise: bb3]; // scope 0 at $DIR/cycle.rs:+3:11: +3:17
+ -         switchInt(move _5) -> [0: bb4, otherwise: bb3]; // scope 0 at $DIR/cycle.rs:+3:11: +3:17
+ +         switchInt(move _4) -> [0: bb4, otherwise: bb3]; // scope 0 at $DIR/cycle.rs:+3:11: +3:17
43   
44       bb3: {


thread '[mir-opt] src/test/mir-opt/dead-store-elimination/cycle.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/dead-store-elimination/cycle.cycle.DeadStoreElimination.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/deaggregator_test_enum_2.rs stdout ----
12       bb0: {
12       bb0: {
13           StorageLive(_3);                 // scope 0 at $DIR/deaggregator_test_enum_2.rs:+1:8: +1:9
14           _3 = _1;                         // scope 0 at $DIR/deaggregator_test_enum_2.rs:+1:8: +1:9
-           switchInt(move _3) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/deaggregator_test_enum_2.rs:+1:8: +1:9
+           switchInt(move _3) -> [0: bb2, otherwise: bb1]; // scope 0 at $DIR/deaggregator_test_enum_2.rs:+1:8: +1:9
17   
18       bb1: {


thread '[mir-opt] src/test/mir-opt/deaggregator_test_enum_2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/deaggregator_test_enum_2.test1.Deaggregator.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/deref-patterns/string.rs stdout ----
---- [mir-opt] src/test/mir-opt/deref-patterns/string.rs stdout ----
17         _7 = const false;                // scope 0 at $DIR/string.rs:+1:11: +1:12
18         _7 = const true;                 // scope 0 at $DIR/string.rs:+1:11: +1:12
19         _5 = discriminant(_1);           // scope 0 at $DIR/string.rs:+1:11: +1:12
-         switchInt(move _5) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/string.rs:+1:5: +1:12
+         switchInt(move _5) -> [1: bb2, otherwise: bb1]; // scope 0 at $DIR/string.rs:+1:5: +1:12
22 
23     bb1: {

47     }
47     }
48 
49     bb4: {
-         switchInt(move _4) -> [false: bb1, otherwise: bb5]; // scope 0 at $DIR/string.rs:+2:14: +2:17
+         switchInt(move _4) -> [0: bb1, otherwise: bb5]; // scope 0 at $DIR/string.rs:+2:14: +2:17
52 
53     bb5: {

69     }
69     }
70 
71     bb9: {
-         switchInt(_7) -> [false: bb7, otherwise: bb8]; // scope 0 at $DIR/string.rs:+5:1: +5:2
+         switchInt(_7) -> [0: bb7, otherwise: bb8]; // scope 0 at $DIR/string.rs:+5:1: +5:2
74 }
75 


thread '[mir-opt] src/test/mir-opt/deref-patterns/string.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/deref-patterns/string.foo.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/deduplicate_blocks.rs stdout ----
---- [mir-opt] src/test/mir-opt/deduplicate_blocks.rs stdout ----
28           _7 = Len((*_2));                 // scope 0 at $DIR/deduplicate_blocks.rs:+2:9: +2:37
29           _8 = const 4_usize;              // scope 0 at $DIR/deduplicate_blocks.rs:+2:9: +2:37
30           _9 = Ge(move _7, move _8);       // scope 0 at $DIR/deduplicate_blocks.rs:+2:9: +2:37
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-           switchInt(move _9) -> [false: bb6, otherwise: bb2]; // scope 0 at $DIR/deduplicate_blocks.rs:+2:9: +2:37
+           switchInt(move _9) -> [0: bb6, otherwise: bb2]; // scope 0 at $DIR/deduplicate_blocks.rs:+2:9: +2:37
33   
34       bb2: {


-           switchInt((*_2)[0 of 4]) -> [47_u8: bb3, otherwise: bb6]; // scope 0 at $DIR/deduplicate_blocks.rs:+1:5: +1:23
+           switchInt((*_2)[0 of 4]) -> [47: bb3, otherwise: bb6]; // scope 0 at $DIR/deduplicate_blocks.rs:+1:5: +1:23
37   
38       bb3: {


-           switchInt((*_2)[1 of 4]) -> [47_u8: bb4, otherwise: bb6]; // scope 0 at $DIR/deduplicate_blocks.rs:+1:5: +1:23
+           switchInt((*_2)[1 of 4]) -> [47: bb4, otherwise: bb6]; // scope 0 at $DIR/deduplicate_blocks.rs:+1:5: +1:23
41   
42       bb4: {


-           switchInt((*_2)[2 of 4]) -> [47_u8: bb5, otherwise: bb6]; // scope 0 at $DIR/deduplicate_blocks.rs:+1:5: +1:23
+           switchInt((*_2)[2 of 4]) -> [47: bb5, otherwise: bb6]; // scope 0 at $DIR/deduplicate_blocks.rs:+1:5: +1:23
45   
46       bb5: {


- -         switchInt((*_2)[3 of 4]) -> [47_u8: bb11, otherwise: bb6]; // scope 0 at $DIR/deduplicate_blocks.rs:+1:5: +1:23
- +         switchInt((*_2)[3 of 4]) -> [47_u8: bb10, otherwise: bb6]; // scope 0 at $DIR/deduplicate_blocks.rs:+1:5: +1:23
+ -         switchInt((*_2)[3 of 4]) -> [47: bb11, otherwise: bb6]; // scope 0 at $DIR/deduplicate_blocks.rs:+1:5: +1:23
+ +         switchInt((*_2)[3 of 4]) -> [47: bb10, otherwise: bb6]; // scope 0 at $DIR/deduplicate_blocks.rs:+1:5: +1:23
50   
51       bb6: {


52           _4 = Len((*_2));                 // scope 0 at $DIR/deduplicate_blocks.rs:+3:9: +3:31
53           _5 = const 3_usize;              // scope 0 at $DIR/deduplicate_blocks.rs:+3:9: +3:31
54           _6 = Ge(move _4, move _5);       // scope 0 at $DIR/deduplicate_blocks.rs:+3:9: +3:31
-           switchInt(move _6) -> [false: bb10, otherwise: bb7]; // scope 0 at $DIR/deduplicate_blocks.rs:+3:9: +3:31
+           switchInt(move _6) -> [0: bb10, otherwise: bb7]; // scope 0 at $DIR/deduplicate_blocks.rs:+3:9: +3:31
57   
58       bb7: {


-           switchInt((*_2)[0 of 3]) -> [47_u8: bb8, otherwise: bb10]; // scope 0 at $DIR/deduplicate_blocks.rs:+1:5: +1:23
+           switchInt((*_2)[0 of 3]) -> [47: bb8, otherwise: bb10]; // scope 0 at $DIR/deduplicate_blocks.rs:+1:5: +1:23
61   
62       bb8: {


-           switchInt((*_2)[1 of 3]) -> [47_u8: bb9, otherwise: bb10]; // scope 0 at $DIR/deduplicate_blocks.rs:+1:5: +1:23
+           switchInt((*_2)[1 of 3]) -> [47: bb9, otherwise: bb10]; // scope 0 at $DIR/deduplicate_blocks.rs:+1:5: +1:23
65   
66       bb9: {


- -         switchInt((*_2)[2 of 3]) -> [47_u8: bb12, 33_u8: bb13, otherwise: bb10]; // scope 0 at $DIR/deduplicate_blocks.rs:+1:5: +1:23
- +         switchInt((*_2)[2 of 3]) -> [47_u8: bb11, 33_u8: bb11, otherwise: bb10]; // scope 0 at $DIR/deduplicate_blocks.rs:+1:5: +1:23
+ -         switchInt((*_2)[2 of 3]) -> [47: bb12, 33: bb13, otherwise: bb10]; // scope 0 at $DIR/deduplicate_blocks.rs:+1:5: +1:23
+ +         switchInt((*_2)[2 of 3]) -> [47: bb11, 33: bb11, otherwise: bb10]; // scope 0 at $DIR/deduplicate_blocks.rs:+1:5: +1:23
70   
71       bb10: {


thread '[mir-opt] src/test/mir-opt/deduplicate_blocks.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/deduplicate_blocks.is_line_doc_comment_2.DeduplicateBlocks.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/derefer_terminator_test.rs stdout ----
---- [mir-opt] src/test/mir-opt/derefer_terminator_test.rs stdout ----
54           _6 = &_7;                        // scope 2 at $DIR/derefer_terminator_test.rs:+3:18: +3:21
55           _5 = &_6;                        // scope 2 at $DIR/derefer_terminator_test.rs:+3:17: +3:21
56           _4 = &_5;                        // scope 2 at $DIR/derefer_terminator_test.rs:+3:15: +3:22
- -         switchInt((*(*(*(*_4))))) -> [false: bb3, otherwise: bb4]; // scope 2 at $DIR/derefer_terminator_test.rs:+3:5: +3:22
+ -         switchInt((*(*(*(*_4))))) -> [0: bb3, otherwise: bb4]; // scope 2 at $DIR/derefer_terminator_test.rs:+3:5: +3:22
58 +         _10 = deref_copy (*_4);          // scope 2 at $DIR/derefer_terminator_test.rs:+3:5: +3:22
59 +         _11 = deref_copy (*_10);         // scope 2 at $DIR/derefer_terminator_test.rs:+3:5: +3:22
60 +         _12 = deref_copy (*_11);         // scope 2 at $DIR/derefer_terminator_test.rs:+3:5: +3:22

- +         switchInt((*_12)) -> [false: bb3, otherwise: bb4]; // scope 2 at $DIR/derefer_terminator_test.rs:+3:5: +3:22
+ +         switchInt((*_12)) -> [0: bb3, otherwise: bb4]; // scope 2 at $DIR/derefer_terminator_test.rs:+3:5: +3:22
63   
64       bb3: {


thread '[mir-opt] src/test/mir-opt/derefer_terminator_test.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/derefer_terminator_test.main.Derefer.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/derefer_complex_case.rs stdout ----
62       bb3: {
62       bb3: {
63           StorageDead(_8);                 // scope 1 at $DIR/derefer_complex_case.rs:+1:25: +1:26
64           _10 = discriminant(_7);          // scope 1 at $DIR/derefer_complex_case.rs:+1:17: +1:26
-           switchInt(move _10) -> [0_isize: bb6, 1_isize: bb4, otherwise: bb5]; // scope 1 at $DIR/derefer_complex_case.rs:+1:17: +1:26
+           switchInt(move _10) -> [0: bb6, 1: bb4, otherwise: bb5]; // scope 1 at $DIR/derefer_complex_case.rs:+1:17: +1:26
67   
68       bb4: {


thread '[mir-opt] src/test/mir-opt/derefer_complex_case.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/derefer_complex_case.main.Derefer.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/dest-prop/branch.rs stdout ----
37       }
38   
39       bb2: {
39       bb2: {
-           switchInt(move _3) -> [false: bb4, otherwise: bb3]; // scope 1 at $DIR/branch.rs:+3:16: +3:22
+           switchInt(move _3) -> [0: bb4, otherwise: bb3]; // scope 1 at $DIR/branch.rs:+3:16: +3:22
42   
43       bb3: {


thread '[mir-opt] src/test/mir-opt/dest-prop/branch.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/dest-prop/branch.foo.DestinationPropagation.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/early_otherwise_branch.rs stdout ----
---- [mir-opt] src/test/mir-opt/early_otherwise_branch.rs stdout ----
31           StorageDead(_5);                 // scope 0 at $DIR/early_otherwise_branch.rs:+1:16: +1:17
32           StorageDead(_4);                 // scope 0 at $DIR/early_otherwise_branch.rs:+1:16: +1:17
33           _7 = discriminant((_3.0: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch.rs:+1:11: +1:17
- -         switchInt(move _7) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch.rs:+1:5: +1:17
+ -         switchInt(move _7) -> [1: bb2, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch.rs:+1:5: +1:17
35 +         StorageLive(_10);                // scope 0 at $DIR/early_otherwise_branch.rs:+1:5: +1:17
36 +         _10 = discriminant((_3.1: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch.rs:+1:5: +1:17
37 +         StorageLive(_11);                // scope 0 at $DIR/early_otherwise_branch.rs:+1:5: +1:17

38 +         _11 = Ne(_7, move _10);          // scope 0 at $DIR/early_otherwise_branch.rs:+1:5: +1:17
39 +         StorageDead(_10);                // scope 0 at $DIR/early_otherwise_branch.rs:+1:5: +1:17
- +         switchInt(move _11) -> [false: bb4, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch.rs:+1:5: +1:17
+ +         switchInt(move _11) -> [0: bb4, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch.rs:+1:5: +1:17
42   
43       bb1: {

49   
49   
50       bb2: {
51 -         _6 = discriminant((_3.1: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch.rs:+1:11: +1:17
- -         switchInt(move _6) -> [1_isize: bb3, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch.rs:+1:5: +1:17
+ -         switchInt(move _6) -> [1: bb3, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch.rs:+1:5: +1:17
54 - 
55 -     bb3: {

72 + 
72 + 
73 +     bb4: {
74 +         StorageDead(_11);                // scope 0 at $DIR/early_otherwise_branch.rs:+1:5: +1:17
- +         switchInt(_7) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch.rs:+1:5: +1:17
+ +         switchInt(_7) -> [1: bb2, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch.rs:+1:5: +1:17
77   }
78   


thread '[mir-opt] src/test/mir-opt/early_otherwise_branch.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/early_otherwise_branch.opt1.EarlyOtherwiseBranch.diff', src/tools/compiletest/src/runtest.rs:3451:21
---- [mir-opt] src/test/mir-opt/early_otherwise_branch_3_element_tuple.rs stdout ----
---- [mir-opt] src/test/mir-opt/early_otherwise_branch_3_element_tuple.rs stdout ----
42           StorageDead(_6);                 // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:+1:19: +1:20
43           StorageDead(_5);                 // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:+1:19: +1:20
44           _10 = discriminant((_4.0: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:+1:11: +1:20
- -         switchInt(move _10) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:+1:5: +1:20
+ -         switchInt(move _10) -> [1: bb2, otherwise: bb1]; // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:+1:5: +1:20
46 +         StorageLive(_14);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:+1:5: +1:20
47 +         _14 = discriminant((_4.1: std::option::Option<u32>)); // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:+1:5: +1:20
48 +         StorageLive(_15);                // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:+1:5: +1:20

49 +         _15 = Ne(_10, move _14);         // scope 0 at $DIR/early_otherwise_branch_3_element_tuple.rs:+1:5: +1:20
