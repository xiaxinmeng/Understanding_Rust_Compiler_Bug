plain
Warning: Skipping "src/test/mir-opt": not a regular file or directory
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 164 tests
.........................F.........i.............................i..............i................... 100/164
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
............F....F............i............................F....

---- [mir-opt] mir-opt/const_prop/discriminant.rs stdout ----
---- [mir-opt] mir-opt/const_prop/discriminant.rs stdout ----
18           ((_3 as Some).0: bool) = const true; // scope 0 at $DIR/discriminant.rs:11:34: 11:44
19           discriminant(_3) = 1;            // scope 0 at $DIR/discriminant.rs:11:34: 11:44
20 -         _4 = discriminant(_3);           // scope 0 at $DIR/discriminant.rs:11:21: 11:31
- -         switchInt(move _4) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
+ -         switchInt(move _4) -> [1_isize: bb1, otherwise: bb3]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
22 +         _4 = const 1_isize;              // scope 0 at $DIR/discriminant.rs:11:21: 11:31
- +         switchInt(const 1_isize) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
+ +         switchInt(const 1_isize) -> [1_isize: bb1, otherwise: bb3]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
25   
26       bb1: {


-           _2 = const 10_i32;               // scope 0 at $DIR/discriminant.rs:11:59: 11:61
-           goto -> bb4;                     // scope 0 at $DIR/discriminant.rs:11:13: 11:64
+           switchInt(((_3 as Some).0: bool)) -> [false: bb3, otherwise: bb2]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
30   
31       bb2: {


-           switchInt(((_3 as Some).0: bool)) -> [false: bb1, otherwise: bb3]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
+           _2 = const 42_i32;               // scope 0 at $DIR/discriminant.rs:11:47: 11:49
+           goto -> bb4;                     // scope 0 at $DIR/discriminant.rs:11:13: 11:64
34   
35       bb3: {


-           _2 = const 42_i32;               // scope 0 at $DIR/discriminant.rs:11:47: 11:49
+           _2 = const 10_i32;               // scope 0 at $DIR/discriminant.rs:11:59: 11:61
37           goto -> bb4;                     // scope 0 at $DIR/discriminant.rs:11:13: 11:64
39   


thread '[mir-opt] mir-opt/const_prop/discriminant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/discriminant.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3588:25

---- [mir-opt] mir-opt/matches_reduce_branches.rs stdout ----
4   fn match_nested_if() -> bool {
4   fn match_nested_if() -> bool {
5       let mut _0: bool;                    // return place in scope 0 at $DIR/matches_reduce_branches.rs:39:25: 39:29
6       let _1: bool;                        // in scope 0 at $DIR/matches_reduce_branches.rs:40:9: 40:12
-       let mut _2: ();                      // in scope 0 at $DIR/matches_reduce_branches.rs:40:21: 40:23
-       let mut _3: bool;                    // in scope 0 at $DIR/matches_reduce_branches.rs:41:15: 45:10
-       let mut _4: bool;                    // in scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:76
-       let mut _5: bool;                    // in scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:52
-       let mut _6: bool;                    // in scope 0 at $DIR/matches_reduce_branches.rs:41:24: 41:28
- +     let mut _7: bool;                    // in scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:52
- +     let mut _8: bool;                    // in scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:76
- +     let mut _9: bool;                    // in scope 0 at $DIR/matches_reduce_branches.rs:41:15: 45:10
- +     let mut _10: bool;                   // in scope 0 at $DIR/matches_reduce_branches.rs:41:15: 45:10
+       let mut _2: bool;                    // in scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:76
+       let mut _3: bool;                    // in scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:52
+       let mut _4: bool;                    // in scope 0 at $DIR/matches_reduce_branches.rs:41:24: 41:28
+ +     let mut _5: bool;                    // in scope 0 at $DIR/matches_reduce_branches.rs:41:24: 41:28
+ +     let mut _6: bool;                    // in scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:52
+ +     let mut _7: bool;                    // in scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:76
16       scope 1 {
17           debug val => _1;                 // in scope 1 at $DIR/matches_reduce_branches.rs:40:9: 40:12

19   
20       bb0: {
20       bb0: {
21           StorageLive(_1);                 // scope 0 at $DIR/matches_reduce_branches.rs:40:9: 40:12
-           StorageLive(_2);                 // scope 0 at $DIR/matches_reduce_branches.rs:40:21: 40:23
-           StorageLive(_3);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:15: 45:10
-           StorageLive(_4);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:76
-           StorageLive(_5);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:52
-           StorageLive(_6);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:24: 41:28
-           _6 = const true;                 // scope 0 at $DIR/matches_reduce_branches.rs:41:24: 41:28
- -         switchInt(move _6) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:52
+           StorageLive(_2);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:76
+           StorageLive(_3);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:52
+           StorageLive(_4);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:24: 41:28
+           _4 = const true;                 // scope 0 at $DIR/matches_reduce_branches.rs:41:24: 41:28
+ -         switchInt(move _4) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/matches_reduce_branches.rs:41:24: 41:28
29 -     }
31 -     bb1: {


- -         _5 = const true;                 // scope 0 at $DIR/matches_reduce_branches.rs:41:31: 41:35
+ -         _3 = const true;                 // scope 0 at $DIR/matches_reduce_branches.rs:41:31: 41:35
33 -         goto -> bb3;                     // scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:52
34 -     }

36 -     bb2: {
36 -     bb2: {
- -         _5 = const false;                // scope 0 at $DIR/matches_reduce_branches.rs:41:45: 41:50
+ -         _3 = const false;                // scope 0 at $DIR/matches_reduce_branches.rs:41:45: 41:50
38 -         goto -> bb3;                     // scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:52
39 -     }

41 -     bb3: {
41 -     bb3: {
- +         StorageLive(_7);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:52
- +         _7 = move _6;                    // scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:52
- +         _5 = Ne(_7, const false);        // scope 0 at $DIR/matches_reduce_branches.rs:41:45: 41:50
- +         StorageDead(_7);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:52
-           StorageDead(_6);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:51: 41:52
- -         switchInt(move _5) -> [false: bb5, otherwise: bb4]; // scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:76
+ +         StorageLive(_5);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:24: 41:28
+ +         _5 = move _4;                    // scope 0 at $DIR/matches_reduce_branches.rs:41:24: 41:28
+ +         _3 = Ne(_5, const false);        // scope 0 at $DIR/matches_reduce_branches.rs:41:45: 41:50
+ +         StorageDead(_5);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:24: 41:28
+           StorageDead(_4);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:51: 41:52
+ -         switchInt(move _3) -> [false: bb5, otherwise: bb4]; // scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:52
48 -     }
50 -     bb4: {


- -         _4 = const true;                 // scope 0 at $DIR/matches_reduce_branches.rs:41:55: 41:59
+ -         _2 = const true;                 // scope 0 at $DIR/matches_reduce_branches.rs:41:55: 41:59
52 -         goto -> bb6;                     // scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:76
53 -     }

55 -     bb5: {
55 -     bb5: {
- -         _4 = const false;                // scope 0 at $DIR/matches_reduce_branches.rs:41:69: 41:74
+ -         _2 = const false;                // scope 0 at $DIR/matches_reduce_branches.rs:41:69: 41:74
57 -         goto -> bb6;                     // scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:76
58 -     }

60 -     bb6: {
60 -     bb6: {
- +         StorageLive(_8);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:76
- +         _8 = move _5;                    // scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:76
- +         _4 = Ne(_8, const false);        // scope 0 at $DIR/matches_reduce_branches.rs:41:69: 41:74
- +         StorageDead(_8);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:76
-           StorageDead(_5);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:75: 41:76
- -         switchInt(move _4) -> [false: bb8, otherwise: bb7]; // scope 0 at $DIR/matches_reduce_branches.rs:41:15: 45:10
+ +         StorageLive(_6);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:52
+ +         _6 = move _3;                    // scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:52
+ +         _2 = Ne(_6, const false);        // scope 0 at $DIR/matches_reduce_branches.rs:41:69: 41:74
+ +         StorageDead(_6);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:21: 41:52
+           StorageDead(_3);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:75: 41:76
+ -         switchInt(move _2) -> [false: bb8, otherwise: bb7]; // scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:76
67 -     }
69 -     bb7: {


- -         _3 = const true;                 // scope 0 at $DIR/matches_reduce_branches.rs:42:13: 42:17
- -         goto -> bb9;                     // scope 0 at $DIR/matches_reduce_branches.rs:41:15: 45:10
- -     }
- -     bb8: {
- -     bb8: {
- -         _3 = const false;                // scope 0 at $DIR/matches_reduce_branches.rs:44:13: 44:18
- -         goto -> bb9;                     // scope 0 at $DIR/matches_reduce_branches.rs:41:15: 45:10
- -     }
- -     bb9: {
- -     bb9: {
- +         StorageLive(_9);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:15: 45:10
- +         _9 = move _4;                    // scope 0 at $DIR/matches_reduce_branches.rs:41:15: 45:10
- +         _3 = Ne(_9, const false);        // scope 0 at $DIR/matches_reduce_branches.rs:44:13: 44:18
- +         StorageDead(_9);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:15: 45:10
-           StorageDead(_4);                 // scope 0 at $DIR/matches_reduce_branches.rs:45:9: 45:10
- -         switchInt(move _3) -> [false: bb11, otherwise: bb10]; // scope 0 at $DIR/matches_reduce_branches.rs:41:15: 45:10
- -     }
- -     bb10: {
- -     bb10: {
- +         StorageLive(_10);                // scope 0 at $DIR/matches_reduce_branches.rs:41:15: 45:10
- +         _10 = move _3;                   // scope 0 at $DIR/matches_reduce_branches.rs:41:15: 45:10
-           StorageDead(_3);                 // scope 0 at $DIR/matches_reduce_branches.rs:48:9: 48:10
+ +         StorageLive(_7);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:76
+ +         _7 = move _2;                    // scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:76
+           StorageDead(_2);                 // scope 0 at $DIR/matches_reduce_branches.rs:45:9: 45:10
92 -         _1 = const true;                 // scope 0 at $DIR/matches_reduce_branches.rs:47:13: 47:17
- -         goto -> bb12;                    // scope 0 at $DIR/matches_reduce_branches.rs:47:13: 47:17
+ -         goto -> bb9;                     // scope 0 at $DIR/matches_reduce_branches.rs:48:9: 48:10
94 -     }
- -     bb11: {
- -     bb11: {
- -         StorageDead(_3);                 // scope 0 at $DIR/matches_reduce_branches.rs:48:9: 48:10
+ -     bb8: {
+ -         StorageDead(_2);                 // scope 0 at $DIR/matches_reduce_branches.rs:45:9: 45:10
98 -         _1 = const false;                // scope 0 at $DIR/matches_reduce_branches.rs:49:14: 49:19
- -         goto -> bb12;                    // scope 0 at $DIR/matches_reduce_branches.rs:49:14: 49:19
+ -         goto -> bb9;                     // scope 0 at $DIR/matches_reduce_branches.rs:49:14: 49:19
100 -     }
- -     bb12: {
- -     bb12: {
- +         _1 = Ne(_10, const false);       // scope 0 at $DIR/matches_reduce_branches.rs:49:14: 49:19
- +         StorageDead(_10);                // scope 0 at $DIR/matches_reduce_branches.rs:41:15: 45:10
-           StorageDead(_2);                 // scope 0 at $DIR/matches_reduce_branches.rs:50:6: 50:7
+ -     bb9: {
+ +         _1 = Ne(_7, const false);        // scope 0 at $DIR/matches_reduce_branches.rs:49:14: 49:19
+ +         StorageDead(_7);                 // scope 0 at $DIR/matches_reduce_branches.rs:41:18: 41:76
106           _0 = _1;                         // scope 1 at $DIR/matches_reduce_branches.rs:51:5: 51:8
107           StorageDead(_1);                 // scope 0 at $DIR/matches_reduce_branches.rs:52:1: 52:2
108           return;                          // scope 0 at $DIR/matches_reduce_branches.rs:52:2: 52:2

thread '[mir-opt] mir-opt/matches_reduce_branches.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/matches_reduce_branches.match_nested_if.MatchBranchSimplification.32bit.diff', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
5 | '_#1r | Local | ['_#1r]
7 | Inferred Region Values
7 | Inferred Region Values
- | '_#0r | U0 | {bb0[0..=8], bb1[0..=7], bb2[0..=3], bb3[0..=1], bb4[0..=3], bb5[0..=2], bb6[0..=5], bb7[0], '_#0r, '_#1r}
- | '_#1r | U0 | {bb0[0..=8], bb1[0..=7], bb2[0..=3], bb3[0..=1], bb4[0..=3], bb5[0..=2], bb6[0..=5], bb7[0], '_#1r}
+ | '_#0r | U0 | {bb0[0..=8], bb1[0..=7], bb2[0..=3], bb3[0..=3], bb4[0..=1], bb5[0..=2], bb6[0..=5], bb7[0], '_#0r, '_#1r}
+ | '_#1r | U0 | {bb0[0..=8], bb1[0..=7], bb2[0..=3], bb3[0..=3], bb4[0..=1], bb5[0..=2], bb6[0..=5], bb7[0], '_#1r}
10 | '_#2r | U0 | {}
11 | '_#3r | U0 | {bb1[0..=7], bb2[0..=2]}
12 | '_#4r | U0 | {bb1[1..=7], bb2[0..=2]}

13 | '_#5r | U0 | {bb1[4..=7], bb2[0..=2]}
15 | Inference Constraints
15 | Inference Constraints
- | '_#0r live at {bb0[0..=8], bb1[0..=7], bb2[0..=3], bb3[0..=1], bb4[0..=3], bb5[0..=2], bb6[0..=5], bb7[0]}
- | '_#1r live at {bb0[0..=8], bb1[0..=7], bb2[0..=3], bb3[0..=1], bb4[0..=3], bb5[0..=2], bb6[0..=5], bb7[0]}
+ | '_#0r live at {bb0[0..=8], bb1[0..=7], bb2[0..=3], bb3[0..=3], bb4[0..=1], bb5[0..=2], bb6[0..=5], bb7[0]}
+ | '_#1r live at {bb0[0..=8], bb1[0..=7], bb2[0..=3], bb3[0..=3], bb4[0..=1], bb5[0..=2], bb6[0..=5], bb7[0]}
18 | '_#3r live at {bb1[0]}
19 | '_#4r live at {bb1[1..=3]}
20 | '_#5r live at {bb1[4..=7], bb2[0..=2]}

63         FakeRead(ForLet(None), _6);      // bb1[4]: scope 2 at $DIR/region-subtyping-basic.rs:19:9: 19:10
64         StorageLive(_7);                 // bb1[5]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
65         _7 = const Const(Value(Scalar(0x01)): bool); // bb1[6]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
-         switchInt(move _7) -> [Const(Value(Scalar(0x00)): bool): bb3, otherwise: bb2]; // bb1[7]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
+         switchInt(move _7) -> [Const(Value(Scalar(0x00)): bool): bb4, otherwise: bb2]; // bb1[7]: scope 3 at $DIR/region-subtyping-basic.rs:20:8: 20:12
68 
69     bb2: {


70         StorageLive(_8);                 // bb2[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
71         StorageLive(_9);                 // bb2[1]: scope 3 at $DIR/region-subtyping-basic.rs:21:15: 21:17
72         _9 = (*_6);                      // bb2[2]: scope 3 at $DIR/region-subtyping-basic.rs:21:15: 21:17
-         _8 = Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(move _9) -> [return: bb4, unwind: bb7]; // bb2[3]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
+         _8 = Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(move _9) -> [return: bb3, unwind: bb7]; // bb2[3]: scope 3 at $DIR/region-subtyping-basic.rs:21:9: 21:18
74                                          // mir::Constant
75                                          // + span: $DIR/region-subtyping-basic.rs:21:9: 21:14
76                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(Scalar(<ZST>)) }
77     }
78 
79     bb3: {
79     bb3: {
-         StorageLive(_10);                // bb3[0]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
-         _10 = Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(const Const(Value(Scalar(0x00000016)): usize)) -> [return: bb5, unwind: bb7]; // bb3[1]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
-                                          // mir::Constant
-                                          // + span: $DIR/region-subtyping-basic.rs:23:9: 23:14
-                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(Scalar(<ZST>)) }
+         StorageDead(_9);                 // bb3[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:17: 21:18
+         StorageDead(_8);                 // bb3[1]: scope 3 at $DIR/region-subtyping-basic.rs:21:18: 21:19
+         _0 = const Const(Value(Scalar(<ZST>)): ()); // bb3[2]: scope 3 at $DIR/region-subtyping-basic.rs:20:13: 22:6
+         goto -> bb6;                     // bb3[3]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
86 
87     bb4: {


-         StorageDead(_9);                 // bb4[0]: scope 3 at $DIR/region-subtyping-basic.rs:21:17: 21:18
-         StorageDead(_8);                 // bb4[1]: scope 3 at $DIR/region-subtyping-basic.rs:21:18: 21:19
-         _0 = const Const(Value(Scalar(<ZST>)): ()); // bb4[2]: scope 3 at $DIR/region-subtyping-basic.rs:20:13: 22:6
-         goto -> bb6;                     // bb4[3]: scope 3 at $DIR/region-subtyping-basic.rs:20:5: 24:6
+         StorageLive(_10);                // bb4[0]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
+         _10 = Const(Value(Scalar(<ZST>)): fn(usize) -> bool {use_x})(const Const(Value(Scalar(0x00000016)): usize)) -> [return: bb5, unwind: bb7]; // bb4[1]: scope 3 at $DIR/region-subtyping-basic.rs:23:9: 23:18
+                                          // mir::Constant
+                                          // + span: $DIR/region-subtyping-basic.rs:23:9: 23:14
+                                          // + literal: Const { ty: fn(usize) -> bool {use_x}, val: Value(Scalar(<ZST>)) }
93 
94     bb5: {


thread '[mir-opt] mir-opt/nll/region-subtyping-basic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/nll/region_subtyping_basic.main.nll.0.32bit.mir', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/while_let_loops.rs stdout ----
---- [mir-opt] mir-opt/while_let_loops.rs stdout ----
21           StorageLive(_3);                 // scope 1 at $DIR/while_let_loops.rs:7:28: 7:32
22           discriminant(_3) = 0;            // scope 1 at $DIR/while_let_loops.rs:7:28: 7:32
23 -         _4 = discriminant(_3);           // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
- -         switchInt(move _4) -> [1_isize: bb2, otherwise: bb1]; // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
+ -         switchInt(move _4) -> [1_isize: bb1, otherwise: bb3]; // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
25 +         _4 = const 0_isize;              // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
- +         switchInt(const 0_isize) -> [1_isize: bb2, otherwise: bb1]; // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
+ +         switchInt(const 0_isize) -> [1_isize: bb1, otherwise: bb3]; // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
28   
29       bb1: {


-           StorageLive(_7);                 // scope 1 at $DIR/while_let_loops.rs:7:5: 10:6
-           nop;                             // scope 1 at $DIR/while_let_loops.rs:7:5: 10:6
-           StorageDead(_7);                 // scope 1 at $DIR/while_let_loops.rs:10:5: 10:6
-           goto -> bb4;                     // scope 1 at no-location
+           switchInt(((_3 as Some).0: u32)) -> [0_u32: bb2, otherwise: bb3]; // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
35   
36       bb2: {


-           switchInt(((_3 as Some).0: u32)) -> [0_u32: bb3, otherwise: bb1]; // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
-   
-       bb3: {
-       bb3: {
41           _1 = const 1_i32;                // scope 1 at $DIR/while_let_loops.rs:8:9: 8:15
42           nop;                             // scope 1 at $DIR/while_let_loops.rs:9:9: 9:14
43           goto -> bb4;                     // scope 1 at $DIR/while_let_loops.rs:9:9: 9:14
+       }
+   
+       bb3: {
+       bb3: {
+           StorageLive(_7);                 // scope 1 at $DIR/while_let_loops.rs:7:5: 10:6
+           nop;                             // scope 1 at $DIR/while_let_loops.rs:7:5: 10:6
+           StorageDead(_7);                 // scope 1 at $DIR/while_let_loops.rs:10:5: 10:6
+           goto -> bb4;                     // scope 1 at no-location
45   
46       bb4: {


thread '[mir-opt] mir-opt/while_let_loops.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/while_let_loops.change_loop_body.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3588:25

failures:
    [mir-opt] mir-opt/const_prop/discriminant.rs
    [mir-opt] mir-opt/matches_reduce_branches.rs
    [mir-opt] mir-opt/matches_reduce_branches.rs
    [mir-opt] mir-opt/nll/region-subtyping-basic.rs
    [mir-opt] mir-opt/while_let_loops.rs

test result: FAILED. 156 passed; 4 failed; 4 ignored; 0 measured; 0 filtered out; finished in 2.15s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:01:44
