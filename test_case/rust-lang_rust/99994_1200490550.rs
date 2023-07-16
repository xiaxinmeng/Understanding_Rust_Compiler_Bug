plain
 finished in 0.477 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 181 tests
i...F.........F..F...........F.........i...............F...F........F.........F......... 88/181
...F...Fi.......FF.....F...........FF.....F.FFF...........F.i...F....F.........F.......F 176/181
.F.F.

---- [mir-opt] src/test/mir-opt/const_goto.rs stdout ----
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
44 -     }
44 -     }
45 - 
46 -     bb6: {
- -         StorageDead(_2);                 // scope 0 at $DIR/const_goto.rs:+1:56: +1:57
+ -         StorageDead(_2);                 // scope 0 at $DIR/const_goto.rs:+1:37: +1:38
48 +     bb3: {
49           return;                          // scope 0 at $DIR/const_goto.rs:+2:2: +2:2


thread '[mir-opt] src/test/mir-opt/const_goto.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_goto.issue_77355_opt.ConstGoto.diff', src/tools/compiletest/src/runtest.rs:3513:25

---- [mir-opt] src/test/mir-opt/bool_compare.rs stdout ----
28       }
29   
29   
30       bb3: {
-           StorageDead(_2);                 // scope 0 at $DIR/bool_compare.rs:+1:33: +1:34
+           StorageDead(_2);                 // scope 0 at $DIR/bool_compare.rs:+1:16: +1:17
32           return;                          // scope 0 at $DIR/bool_compare.rs:+2:2: +2:2
34   }


thread '[mir-opt] src/test/mir-opt/bool_compare.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/bool_compare.opt1.InstCombine.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/const_goto_storage.rs stdout ----
40 -     }
41 - 
42 -     bb3: {
42 -     bb3: {
- -         StorageDead(_6);                 // scope 0 at $DIR/const_goto_storage.rs:+2:51: +2:52
+ -         StorageDead(_6);                 // scope 0 at $DIR/const_goto_storage.rs:+2:27: +2:28
44 -         switchInt(move _5) -> [false: bb5, otherwise: bb4]; // scope 0 at $DIR/const_goto_storage.rs:+2:21: +2:52
46 - 

55 -     }
56 - 
56 - 
57 -     bb6: {
- -         StorageDead(_5);                 // scope 0 at $DIR/const_goto_storage.rs:+2:75: +2:76
+ -         StorageDead(_5);                 // scope 0 at $DIR/const_goto_storage.rs:+2:51: +2:52
59 -         switchInt(move _4) -> [false: bb8, otherwise: bb7]; // scope 0 at $DIR/const_goto_storage.rs:+2:18: +2:76
61 - 

74 -     }
75 - 
75 - 
76 -     bb10: {
- -         StorageDead(_4);                 // scope 0 at $DIR/const_goto_storage.rs:+6:9: +6:10
- -         StorageDead(_3);                 // scope 0 at $DIR/const_goto_storage.rs:+6:9: +6:10
- +         StorageDead(_2);                 // scope 0 at $DIR/const_goto_storage.rs:+2:51: +2:52
+ -         StorageDead(_4);                 // scope 0 at $DIR/const_goto_storage.rs:+2:75: +2:76
+ -         StorageDead(_3);                 // scope 0 at $DIR/const_goto_storage.rs:+2:75: +2:76
+ +         StorageDead(_2);                 // scope 0 at $DIR/const_goto_storage.rs:+2:27: +2:28
80           _1 = const true;                 // scope 0 at $DIR/const_goto_storage.rs:+8:17: +8:21
81 -         goto -> bb12;                    // scope 0 at $DIR/const_goto_storage.rs:+8:17: +8:21
82 +         goto -> bb3;                     // scope 0 at $DIR/const_goto_storage.rs:+8:17: +8:21
83       }
84   
85 -     bb11: {
85 -     bb11: {
- -         StorageDead(_4);                 // scope 0 at $DIR/const_goto_storage.rs:+6:9: +6:10
- -         StorageDead(_3);                 // scope 0 at $DIR/const_goto_storage.rs:+6:9: +6:10
+ -         StorageDead(_4);                 // scope 0 at $DIR/const_goto_storage.rs:+2:75: +2:76
+ -         StorageDead(_3);                 // scope 0 at $DIR/const_goto_storage.rs:+2:75: +2:76
88 +     bb2: {
- +         StorageDead(_2);                 // scope 0 at $DIR/const_goto_storage.rs:+2:51: +2:52
+ +         StorageDead(_2);                 // scope 0 at $DIR/const_goto_storage.rs:+2:27: +2:28
90           _1 = const false;                // scope 0 at $DIR/const_goto_storage.rs:+10:14: +10:19
91 -         goto -> bb12;                    // scope 0 at $DIR/const_goto_storage.rs:+10:14: +10:19
92 +         goto -> bb3;                     // scope 0 at $DIR/const_goto_storage.rs:+10:14: +10:19

thread '[mir-opt] src/test/mir-opt/const_goto_storage.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_goto_storage.match_nested_if.ConstGoto.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/const_prop/control-flow-simplification.rs stdout ----
27   
28       bb2: {
28       bb2: {
29           nop;                             // scope 0 at $DIR/control-flow-simplification.rs:+3:6: +3:6
-           StorageDead(_1);                 // scope 0 at $DIR/control-flow-simplification.rs:+3:5: +3:6
+           StorageDead(_1);                 // scope 0 at $DIR/control-flow-simplification.rs:+1:20: +1:21
31           return;                          // scope 0 at $DIR/control-flow-simplification.rs:+4:2: +4:2
33   }


thread '[mir-opt] src/test/mir-opt/const_prop/control-flow-simplification.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/control_flow_simplification.hello.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/dead-store-elimination/cycle.rs stdout ----
---- [mir-opt] src/test/mir-opt/dead-store-elimination/cycle.rs stdout ----
60 -         _4 = const ();                   // scope 0 at $DIR/cycle.rs:+3:18: +8:6
61 +         nop;                             // scope 0 at $DIR/cycle.rs:+3:18: +8:6
62           StorageDead(_6);                 // scope 0 at $DIR/cycle.rs:+8:5: +8:6
-           StorageDead(_5);                 // scope 0 at $DIR/cycle.rs:+8:5: +8:6
+           StorageDead(_5);                 // scope 0 at $DIR/cycle.rs:+3:16: +3:17
64           goto -> bb1;                     // scope 0 at $DIR/cycle.rs:+3:5: +8:6
66   


68           StorageLive(_11);                // scope 0 at $DIR/cycle.rs:+3:5: +8:6
69           _0 = const ();                   // scope 0 at $DIR/cycle.rs:+3:5: +8:6
70           StorageDead(_11);                // scope 0 at $DIR/cycle.rs:+8:5: +8:6
-           StorageDead(_5);                 // scope 0 at $DIR/cycle.rs:+8:5: +8:6
+           StorageDead(_5);                 // scope 0 at $DIR/cycle.rs:+3:16: +3:17
72           return;                          // scope 0 at $DIR/cycle.rs:+9:2: +9:2
74   }


thread '[mir-opt] src/test/mir-opt/dead-store-elimination/cycle.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/dead-store-elimination/cycle.cycle.DeadStoreElimination.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/deaggregator_test_enum_2.rs stdout ----
38       }
39   
40       bb3: {
40       bb3: {
-           StorageDead(_3);                 // scope 0 at $DIR/deaggregator_test_enum_2.rs:+5:5: +5:6
+           StorageDead(_3);                 // scope 0 at $DIR/deaggregator_test_enum_2.rs:+1:8: +1:9
42           return;                          // scope 0 at $DIR/deaggregator_test_enum_2.rs:+6:2: +6:2
44   }


thread '[mir-opt] src/test/mir-opt/deaggregator_test_enum_2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/deaggregator_test_enum_2.test1.Deaggregator.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/dest-prop/branch.rs stdout ----
55       }
56   
57       bb6: {
57       bb6: {
-           StorageDead(_3);                 // scope 1 at $DIR/branch.rs:+8:5: +8:6
+           StorageDead(_3);                 // scope 1 at $DIR/branch.rs:+3:21: +3:22
59           nop;                             // scope 0 at $DIR/branch.rs:+0:11: +9:2
60           StorageDead(_2);                 // scope 1 at $DIR/branch.rs:+9:1: +9:2
61           StorageDead(_1);                 // scope 0 at $DIR/branch.rs:+9:1: +9:2

thread '[mir-opt] src/test/mir-opt/dest-prop/branch.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/dest-prop/branch.main.DestinationPropagation.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/equal_true.rs stdout ----
28       }
29   
30       bb3: {
30       bb3: {
-           StorageDead(_2);                 // scope 0 at $DIR/equal_true.rs:+1:33: +1:34
+           StorageDead(_2);                 // scope 0 at $DIR/equal_true.rs:+1:16: +1:17
32           return;                          // scope 0 at $DIR/equal_true.rs:+2:2: +2:2
34   }


thread '[mir-opt] src/test/mir-opt/equal_true.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/equal_true.opt.InstCombine.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/if-condition-int.rs stdout ----
32       }
33   
34       bb3: {
34       bb3: {
-           StorageDead(_2);                 // scope 0 at $DIR/if-condition-int.rs:+1:31: +1:32
+           StorageDead(_2);                 // scope 0 at $DIR/if-condition-int.rs:+1:14: +1:15
36           return;                          // scope 0 at $DIR/if-condition-int.rs:+2:2: +2:2
38   }


thread '[mir-opt] src/test/mir-opt/if-condition-int.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/if_condition_int.opt_u32.SimplifyComparisonIntegral.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/inline/inline-generator.rs stdout ----
120 +     }
121 + 
122 +     bb6: {
122 +     bb6: {
- +         StorageDead(_9);                 // scope 6 at $DIR/inline-generator.rs:+7:38: +7:39
+ +         StorageDead(_9);                 // scope 6 at $DIR/inline-generator.rs:+7:20: +7:21
124 +         Deinit(_1);                      // scope 6 at $DIR/inline-generator.rs:+7:11: +7:39
125 +         ((_1 as Yielded).0: i32) = move _8; // scope 6 at $DIR/inline-generator.rs:+7:11: +7:39
126 +         discriminant(_1) = 0;            // scope 6 at $DIR/inline-generator.rs:+7:11: +7:39

thread '[mir-opt] src/test/mir-opt/inline/inline-generator.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_generator.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/issue-38669.rs stdout ----
31 
32     bb3: {
32     bb3: {
33         _0 = const ();                   // scope 1 at $DIR/issue-38669.rs:+4:13: +4:18
-         StorageDead(_4);                 // scope 1 at $DIR/issue-38669.rs:+5:9: +5:10
+         StorageDead(_4);                 // scope 1 at $DIR/issue-38669.rs:+3:23: +3:24
35         StorageDead(_3);                 // scope 1 at $DIR/issue-38669.rs:+5:9: +5:10
36         StorageDead(_1);                 // scope 0 at $DIR/issue-38669.rs:+8:1: +8:2
37         return;                          // scope 0 at $DIR/issue-38669.rs:+8:2: +8:2
39 
40     bb4: {
40     bb4: {
41         _3 = const ();                   // scope 1 at $DIR/issue-38669.rs:+5:10: +5:10
-         StorageDead(_4);                 // scope 1 at $DIR/issue-38669.rs:+5:9: +5:10
+         StorageDead(_4);                 // scope 1 at $DIR/issue-38669.rs:+3:23: +3:24
43         StorageDead(_3);                 // scope 1 at $DIR/issue-38669.rs:+5:9: +5:10
44         _1 = const true;                 // scope 1 at $DIR/issue-38669.rs:+6:9: +6:28
45         _2 = const ();                   // scope 1 at $DIR/issue-38669.rs:+2:10: +7:6

thread '[mir-opt] src/test/mir-opt/issue-38669.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_38669.main.SimplifyCfg-initial.after.mir', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/inline/inline-diverging.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/inline-diverging.rs stdout ----
27           _4 = _1;                         // scope 0 at $DIR/inline-diverging.rs:+2:9: +2:10
28           _0 = move _4 as u32 (Misc);      // scope 0 at $DIR/inline-diverging.rs:+2:9: +2:17
29           StorageDead(_4);                 // scope 0 at $DIR/inline-diverging.rs:+2:16: +2:17
-           StorageDead(_2);                 // scope 0 at $DIR/inline-diverging.rs:+5:5: +5:6
+           StorageDead(_2);                 // scope 0 at $DIR/inline-diverging.rs:+1:12: +1:13
31           return;                          // scope 0 at $DIR/inline-diverging.rs:+6:2: +6:2
33   


thread '[mir-opt] src/test/mir-opt/inline/inline-diverging.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_diverging.g.Inline.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/issue-41888.rs stdout ----
75     }
76 
77     bb8: {
77     bb8: {
-         StorageDead(_2);                 // scope 1 at $DIR/issue-41888.rs:+8:5: +8:6
+         StorageDead(_2);                 // scope 1 at $DIR/issue-41888.rs:+2:13: +2:14
79         goto -> bb20;                    // scope 0 at $DIR/issue-41888.rs:+9:1: +9:2
81 


thread '[mir-opt] src/test/mir-opt/issue-41888.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_41888.main.ElaborateDrops.after.mir', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/loop_test.rs stdout ----
21 
22     bb1: {
22     bb1: {
23         _0 = const ();                   // scope 0 at $DIR/loop_test.rs:+5:9: +5:15
-         StorageDead(_2);                 // scope 0 at $DIR/loop_test.rs:+6:5: +6:6
+         StorageDead(_2);                 // scope 0 at $DIR/loop_test.rs:+4:11: +4:12
25         StorageDead(_1);                 // scope 0 at $DIR/loop_test.rs:+6:5: +6:6
26         return;                          // scope 0 at $DIR/loop_test.rs:+11:2: +11:2

28 
29     bb2: {
29     bb2: {
30         _1 = const ();                   // scope 0 at $DIR/loop_test.rs:+6:6: +6:6
-         StorageDead(_2);                 // scope 0 at $DIR/loop_test.rs:+6:5: +6:6
+         StorageDead(_2);                 // scope 0 at $DIR/loop_test.rs:+4:11: +4:12
32         StorageDead(_1);                 // scope 0 at $DIR/loop_test.rs:+6:5: +6:6
33         StorageLive(_4);                 // scope 0 at $DIR/loop_test.rs:+7:5: +10:6
34         goto -> bb3;                     // scope 0 at $DIR/loop_test.rs:+7:5: +10:6

thread '[mir-opt] src/test/mir-opt/loop_test.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/loop_test.main.SimplifyCfg-promote-consts.after.mir', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/lower_slice_len.rs stdout ----
56       }
57   
58       bb5: {
58       bb5: {
-           StorageDead(_3);                 // scope 0 at $DIR/lower_slice_len.rs:+5:5: +5:6
+           StorageDead(_3);                 // scope 0 at $DIR/lower_slice_len.rs:+1:26: +1:27
60           return;                          // scope 0 at $DIR/lower_slice_len.rs:+6:2: +6:2
62   }


thread '[mir-opt] src/test/mir-opt/lower_slice_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/lower_slice_len.bound.LowerSliceLenCalls.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs stdout ----
98     }
99 
100     bb6: {
100     bb6: {
-         StorageDead(_7);                 // bb6[0]: scope 3 at $DIR/region-subtyping-basic.rs:+8:5: +8:6
+         StorageDead(_7);                 // bb6[0]: scope 3 at $DIR/region-subtyping-basic.rs:+4:11: +4:12
102         StorageDead(_6);                 // bb6[1]: scope 2 at $DIR/region-subtyping-basic.rs:+9:1: +9:2
103         StorageDead(_3);                 // bb6[2]: scope 1 at $DIR/region-subtyping-basic.rs:+9:1: +9:2
104         StorageDead(_2);                 // bb6[3]: scope 1 at $DIR/region-subtyping-basic.rs:+9:1: +9:2

thread '[mir-opt] src/test/mir-opt/nll/region-subtyping-basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/nll/region_subtyping_basic.main.nll.0.64bit.mir', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/not_equal_false.rs stdout ----
28       }
29   
30       bb3: {
30       bb3: {
-           StorageDead(_2);                 // scope 0 at $DIR/not_equal_false.rs:+1:34: +1:35
+           StorageDead(_2);                 // scope 0 at $DIR/not_equal_false.rs:+1:17: +1:18
32           return;                          // scope 0 at $DIR/not_equal_false.rs:+2:2: +2:2
34   }


thread '[mir-opt] src/test/mir-opt/not_equal_false.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/not_equal_false.opt.InstCombine.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/matches_reduce_branches.rs stdout ----
20 -     bb1: {
20 -     bb1: {
21 +         StorageLive(_3);                 // scope 0 at $DIR/matches_reduce_branches.rs:+2:24: +2:28
22 +         _3 = move _2;                    // scope 0 at $DIR/matches_reduce_branches.rs:+2:24: +2:28
-           StorageDead(_2);                 // scope 0 at $DIR/matches_reduce_branches.rs:+2:51: +2:52
+           StorageDead(_2);                 // scope 0 at $DIR/matches_reduce_branches.rs:+2:27: +2:28
24 -         _1 = const true;                 // scope 0 at $DIR/matches_reduce_branches.rs:+8:13: +8:17
25 -         goto -> bb3;                     // scope 0 at $DIR/matches_reduce_branches.rs:+8:13: +8:17

27 - 
28 -     bb2: {
28 -     bb2: {
- -         StorageDead(_2);                 // scope 0 at $DIR/matches_reduce_branches.rs:+2:51: +2:52
+ -         StorageDead(_2);                 // scope 0 at $DIR/matches_reduce_branches.rs:+2:27: +2:28
30 -         _1 = const false;                // scope 0 at $DIR/matches_reduce_branches.rs:+10:14: +10:19
31 -         goto -> bb3;                     // scope 0 at $DIR/matches_reduce_branches.rs:+10:14: +10:19


thread '[mir-opt] src/test/mir-opt/matches_reduce_branches.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/matches_reduce_branches.match_nested_if.MatchBranchSimplification.64bit.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/lower_array_len.rs stdout ----
61       }
62   
63       bb5: {
63       bb5: {
-           StorageDead(_3);                 // scope 0 at $DIR/lower_array_len.rs:+5:5: +5:6
+           StorageDead(_3);                 // scope 0 at $DIR/lower_array_len.rs:+1:26: +1:27
65           return;                          // scope 0 at $DIR/lower_array_len.rs:+6:2: +6:2
67   }


thread '[mir-opt] src/test/mir-opt/lower_array_len.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/lower_array_len.array_bound.NormalizeArrayLen.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/match-arm-scopes.rs stdout ----
92 -     bb9: {
93 +     bb6: {
93 +     bb6: {
94           _0 = const 3_i32;                // scope 0 at $DIR/match-arm-scopes.rs:+2:59: +2:60
-           StorageDead(_10);                // scope 0 at $DIR/match-arm-scopes.rs:+2:72: +2:73
-           StorageDead(_9);                 // scope 0 at $DIR/match-arm-scopes.rs:+2:72: +2:73
+           StorageDead(_10);                // scope 0 at $DIR/match-arm-scopes.rs:+2:48: +2:49
+           StorageDead(_9);                 // scope 0 at $DIR/match-arm-scopes.rs:+2:48: +2:49
97 -         goto -> bb23;                    // scope 0 at no-location
98 +         goto -> bb20;                    // scope 0 at no-location

107   
108 -     bb11: {
109 +     bb8: {
109 +     bb8: {
-           StorageDead(_10);                // scope 0 at $DIR/match-arm-scopes.rs:+2:72: +2:73
-           StorageDead(_9);                 // scope 0 at $DIR/match-arm-scopes.rs:+2:72: +2:73
+           StorageDead(_10);                // scope 0 at $DIR/match-arm-scopes.rs:+2:48: +2:49
+           StorageDead(_9);                 // scope 0 at $DIR/match-arm-scopes.rs:+2:48: +2:49
112 -         FakeRead(ForMatchGuard, _3);     // scope 0 at $DIR/match-arm-scopes.rs:+2:72: +2:73
113 -         FakeRead(ForMatchGuard, _4);     // scope 0 at $DIR/match-arm-scopes.rs:+2:72: +2:73
114 -         FakeRead(ForGuardBinding, _6);   // scope 0 at $DIR/match-arm-scopes.rs:+2:72: +2:73
123   
124 -     bb12: {
125 +     bb9: {
125 +     bb9: {
-           StorageDead(_10);                // scope 0 at $DIR/match-arm-scopes.rs:+2:72: +2:73
-           StorageDead(_9);                 // scope 0 at $DIR/match-arm-scopes.rs:+2:72: +2:73
+           StorageDead(_10);                // scope 0 at $DIR/match-arm-scopes.rs:+2:48: +2:49
+           StorageDead(_9);                 // scope 0 at $DIR/match-arm-scopes.rs:+2:48: +2:49
128           StorageDead(_8);                 // scope 0 at $DIR/match-arm-scopes.rs:+2:77: +2:78
129           StorageDead(_6);                 // scope 0 at $DIR/match-arm-scopes.rs:+2:77: +2:78
130 -         falseEdge -> [real: bb2, imaginary: bb3]; // scope 0 at $DIR/match-arm-scopes.rs:+2:42: +2:73
149 -     bb14: {
150 +     bb11: {
150 +     bb11: {
151           _0 = const 3_i32;                // scope 0 at $DIR/match-arm-scopes.rs:+2:59: +2:60
-           StorageDead(_13);                // scope 0 at $DIR/match-arm-scopes.rs:+2:72: +2:73
-           StorageDead(_12);                // scope 0 at $DIR/match-arm-scopes.rs:+2:72: +2:73
+           StorageDead(_13);                // scope 0 at $DIR/match-arm-scopes.rs:+2:48: +2:49
+           StorageDead(_12);                // scope 0 at $DIR/match-arm-scopes.rs:+2:48: +2:49
154 -         goto -> bb23;                    // scope 0 at no-location
155 +         goto -> bb20;                    // scope 0 at no-location

164   
165 -     bb16: {
166 +     bb13: {
166 +     bb13: {
-           StorageDead(_13);                // scope 0 at $DIR/match-arm-scopes.rs:+2:72: +2:73
-           StorageDead(_12);                // scope 0 at $DIR/match-arm-scopes.rs:+2:72: +2:73
+           StorageDead(_13);                // scope 0 at $DIR/match-arm-scopes.rs:+2:48: +2:49
+           StorageDead(_12);                // scope 0 at $DIR/match-arm-scopes.rs:+2:48: +2:49
169 -         FakeRead(ForMatchGuard, _3);     // scope 0 at $DIR/match-arm-scopes.rs:+2:72: +2:73
170 -         FakeRead(ForMatchGuard, _4);     // scope 0 at $DIR/match-arm-scopes.rs:+2:72: +2:73
171 -         FakeRead(ForGuardBinding, _6);   // scope 0 at $DIR/match-arm-scopes.rs:+2:72: +2:73
180   
181 -     bb17: {
182 +     bb14: {
182 +     bb14: {
-           StorageDead(_13);                // scope 0 at $DIR/match-arm-scopes.rs:+2:72: +2:73
-           StorageDead(_12);                // scope 0 at $DIR/match-arm-scopes.rs:+2:72: +2:73
+           StorageDead(_13);                // scope 0 at $DIR/match-arm-scopes.rs:+2:48: +2:49
+           StorageDead(_12);                // scope 0 at $DIR/match-arm-scopes.rs:+2:48: +2:49
185           StorageDead(_8);                 // scope 0 at $DIR/match-arm-scopes.rs:+2:77: +2:78
186           StorageDead(_6);                 // scope 0 at $DIR/match-arm-scopes.rs:+2:77: +2:78
187 -         falseEdge -> [real: bb4, imaginary: bb5]; // scope 0 at $DIR/match-arm-scopes.rs:+2:42: +2:73

thread '[mir-opt] src/test/mir-opt/match-arm-scopes.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/match_arm_scopes.complicated_match.SimplifyCfg-initial.after-ElaborateDrops.after.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/simplify-locals-fixedpoint.rs stdout ----
---- [mir-opt] src/test/mir-opt/simplify-locals-fixedpoint.rs stdout ----
45 -         _8 = _6;                         // scope 1 at $DIR/simplify-locals-fixedpoint.rs:+2:12: +2:13
46 -         _7 = Gt(move _8, const 42_u8);   // scope 1 at $DIR/simplify-locals-fixedpoint.rs:+2:12: +2:20
47 -         StorageDead(_8);                 // scope 1 at $DIR/simplify-locals-fixedpoint.rs:+2:19: +2:20
- -         StorageDead(_7);                 // scope 1 at $DIR/simplify-locals-fixedpoint.rs:+4:9: +4:10
+ -         StorageDead(_7);                 // scope 1 at $DIR/simplify-locals-fixedpoint.rs:+2:19: +2:20
49           StorageDead(_6);                 // scope 0 at $DIR/simplify-locals-fixedpoint.rs:+5:5: +5:6
50           goto -> bb3;                     // scope 0 at $DIR/simplify-locals-fixedpoint.rs:+1:5: +5:6


thread '[mir-opt] src/test/mir-opt/simplify-locals-fixedpoint.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_locals_fixedpoint.foo.SimplifyLocals.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/simplify_if.rs stdout ----
33       }
34   
35       bb4: {
35       bb4: {
-           StorageDead(_1);                 // scope 0 at $DIR/simplify_if.rs:+3:5: +3:6
+           StorageDead(_1);                 // scope 0 at $DIR/simplify_if.rs:+1:12: +1:13
37           return;                          // scope 0 at $DIR/simplify_if.rs:+4:2: +4:2
39   }


thread '[mir-opt] src/test/mir-opt/simplify_if.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_if.main.SimplifyConstCondition-after-const-prop.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/simplify_cfg.rs stdout ----
32       bb4: {
32       bb4: {
33           _0 = const ();                   // scope 0 at $DIR/simplify_cfg.rs:+3:13: +3:18
34 -         goto -> bb10;                    // scope 0 at $DIR/simplify_cfg.rs:+3:13: +3:18
- +         StorageDead(_2);                 // scope 0 at $DIR/simplify_cfg.rs:+4:9: +4:10
+ +         StorageDead(_2);                 // scope 0 at $DIR/simplify_cfg.rs:+2:16: +2:17
36 +         return;                          // scope 0 at $DIR/simplify_cfg.rs:+6:2: +6:2
38   

54 -     }
55 - 
55 - 
56 -     bb9: {
-           StorageDead(_2);                 // scope 0 at $DIR/simplify_cfg.rs:+4:9: +4:10
+           StorageDead(_2);                 // scope 0 at $DIR/simplify_cfg.rs:+2:16: +2:17
58           goto -> bb1;                     // scope 0 at $DIR/simplify_cfg.rs:+1:5: +5:6
60   

61 -     bb10: {
61 -     bb10: {
- -         StorageDead(_2);                 // scope 0 at $DIR/simplify_cfg.rs:+4:9: +4:10
+ -         StorageDead(_2);                 // scope 0 at $DIR/simplify_cfg.rs:+2:16: +2:17
63 -         return;                          // scope 0 at $DIR/simplify_cfg.rs:+6:2: +6:2
65 - 


thread '[mir-opt] src/test/mir-opt/simplify_cfg.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_cfg.main.SimplifyCfg-initial.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/unreachable.rs stdout ----
54 -     }
55 - 
56 -     bb5: {
56 -     bb5: {
- -         StorageDead(_6);                 // scope 2 at $DIR/unreachable.rs:+8:9: +8:10
+ -         StorageDead(_6);                 // scope 2 at $DIR/unreachable.rs:+4:15: +4:16
58 -         StorageDead(_5);                 // scope 2 at $DIR/unreachable.rs:+8:9: +8:10
59 -         StorageLive(_7);                 // scope 2 at $DIR/unreachable.rs:+10:9: +10:21
60 -         unreachable;                     // scope 2 at $DIR/unreachable.rs:+10:15: +10:17

thread '[mir-opt] src/test/mir-opt/unreachable.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/unreachable.main.UnreachablePropagation.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/unreachable_diverging.rs stdout ----
57 -     }
58 - 
59 -     bb5: {
59 -     bb5: {
-           StorageDead(_6);                 // scope 2 at $DIR/unreachable_diverging.rs:+5:9: +5:10
+           StorageDead(_6);                 // scope 2 at $DIR/unreachable_diverging.rs:+3:12: +3:13
61           StorageDead(_5);                 // scope 2 at $DIR/unreachable_diverging.rs:+5:9: +5:10
62           StorageLive(_7);                 // scope 2 at $DIR/unreachable_diverging.rs:+6:9: +6:22
63           unreachable;                     // scope 2 at $DIR/unreachable_diverging.rs:+6:15: +6:19

thread '[mir-opt] src/test/mir-opt/unreachable_diverging.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/unreachable_diverging.main.UnreachablePropagation.diff', src/tools/compiletest/src/runtest.rs:3513:25
---- [mir-opt] src/test/mir-opt/while-storage.rs stdout ----
43     }
44 
45     bb5: {
45     bb5: {
-         StorageDead(_4);                 // scope 0 at $DIR/while-storage.rs:+4:9: +4:10
+         StorageDead(_4);                 // scope 0 at $DIR/while-storage.rs:+2:22: +2:23
47         goto -> bb8;                     // scope 0 at no-location
49 

50     bb6: {
50     bb6: {
-         StorageDead(_4);                 // scope 0 at $DIR/while-storage.rs:+4:9: +4:10
-         StorageDead(_2);                 // scope 0 at $DIR/while-storage.rs:+5:5: +5:6
+         StorageDead(_4);                 // scope 0 at $DIR/while-storage.rs:+2:22: +2:23
+         StorageDead(_2);                 // scope 0 at $DIR/while-storage.rs:+1:21: +1:22
53         goto -> bb1;                     // scope 0 at $DIR/while-storage.rs:+1:5: +5:6
55 

58     }
59 
59 
60     bb8: {
-         StorageDead(_2);                 // scope 0 at $DIR/while-storage.rs:+5:5: +5:6
+         StorageDead(_2);                 // scope 0 at $DIR/while-storage.rs:+1:21: +1:22
62         return;                          // scope 0 at $DIR/while-storage.rs:+6:2: +6:2
64 }


thread '[mir-opt] src/test/mir-opt/while-storage.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/while_storage.while_loop.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3513:25

failures:
    [mir-opt] src/test/mir-opt/bool_compare.rs
    [mir-opt] src/test/mir-opt/const_goto.rs
