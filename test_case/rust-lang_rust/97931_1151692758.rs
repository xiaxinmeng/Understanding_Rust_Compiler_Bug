plain
 finished in 0.673 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 178 tests
i.....F....F...F..........F.F..........i.............F....F.........F......F.F......F..F 88/178
......i.....F.F..F.....F.F....FFF.F..F.FF.F...F....F....Fi...FF....F........F.F..F...F.F 176/178
failures:

---- [mir-opt] src/test/mir-opt/const_goto.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_goto.rs stdout ----
7 -     let mut _2: bool;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
8 -     let mut _3: isize;                   // in scope 0 at $DIR/const_goto.rs:12:22: 12:28
9 +     let mut _2: isize;                   // in scope 0 at $DIR/const_goto.rs:12:22: 12:28
+       scope 1 {
10   
11       bb0: {
11       bb0: {
- -         StorageLive(_2);                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
- -         _3 = discriminant(_1);           // scope 0 at $DIR/const_goto.rs:12:17: 12:20
- -         switchInt(move _3) -> [1_isize: bb2, 2_isize: bb2, otherwise: bb1]; // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
- +         _2 = discriminant(_1);           // scope 0 at $DIR/const_goto.rs:12:17: 12:20
- +         switchInt(move _2) -> [1_isize: bb2, 2_isize: bb2, otherwise: bb1]; // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+ -         StorageLive(_2);                 // scope 1 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+ -         _3 = discriminant(_1);           // scope 1 at $DIR/const_goto.rs:12:17: 12:20
+ -         switchInt(move _3) -> [1_isize: bb2, 2_isize: bb2, otherwise: bb1]; // scope 1 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+ +         _2 = discriminant(_1);           // scope 1 at $DIR/const_goto.rs:12:17: 12:20
+ +         switchInt(move _2) -> [1_isize: bb2, 2_isize: bb2, otherwise: bb1]; // scope 1 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
18   
19       bb1: {


- -         _2 = const false;                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
- -         goto -> bb3;                     // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+ -         _2 = const false;                // scope 1 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+ -         goto -> bb3;                     // scope 1 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
22 +         _0 = const 42_u64;               // scope 0 at $DIR/const_goto.rs:12:53: 12:55
23 +         goto -> bb3;                     // scope 0 at $DIR/const_goto.rs:12:5: 12:57

25   
26       bb2: {
26       bb2: {
- -         _2 = const true;                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
- -         goto -> bb3;                     // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+ -         _2 = const true;                 // scope 1 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+ -         goto -> bb3;                     // scope 1 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
30 - 
31 -     bb3: {


- -         switchInt(move _2) -> [false: bb5, otherwise: bb4]; // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+ -         switchInt(move _2) -> [false: bb5, otherwise: bb4]; // scope 1 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
34 - 
35 -     bb4: {


-           _0 = const 23_u64;               // scope 0 at $DIR/const_goto.rs:12:41: 12:43
+           _0 = const 23_u64;               // scope 1 at $DIR/const_goto.rs:12:41: 12:43
37 -         goto -> bb6;                     // scope 0 at $DIR/const_goto.rs:12:5: 12:57
38 +         goto -> bb3;                     // scope 0 at $DIR/const_goto.rs:12:5: 12:57


thread '[mir-opt] src/test/mir-opt/const_goto.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_goto.issue_77355_opt.ConstGoto.diff', src/tools/compiletest/src/runtest.rs:3437:25

---- [mir-opt] src/test/mir-opt/bool_compare.rs stdout ----
---- [mir-opt] src/test/mir-opt/bool_compare.rs stdout ----
6       let mut _0: u32;                     // return place in scope 0 at $DIR/bool_compare.rs:2:21: 2:24
7       let mut _2: bool;                    // in scope 0 at $DIR/bool_compare.rs:3:8: 3:17
8       let mut _3: bool;                    // in scope 0 at $DIR/bool_compare.rs:3:8: 3:9
+       scope 1 {
9   
10       bb0: {
10       bb0: {
-           StorageLive(_2);                 // scope 0 at $DIR/bool_compare.rs:3:8: 3:17
-           StorageLive(_3);                 // scope 0 at $DIR/bool_compare.rs:3:8: 3:9
-           _3 = _1;                         // scope 0 at $DIR/bool_compare.rs:3:8: 3:9
- -         _2 = Ne(move _3, const true);    // scope 0 at $DIR/bool_compare.rs:3:8: 3:17
- +         _2 = Not(move _3);               // scope 0 at $DIR/bool_compare.rs:3:8: 3:17
-           StorageDead(_3);                 // scope 0 at $DIR/bool_compare.rs:3:16: 3:17
-           switchInt(move _2) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/bool_compare.rs:3:8: 3:17
+           StorageLive(_2);                 // scope 1 at $DIR/bool_compare.rs:3:8: 3:17
+           StorageLive(_3);                 // scope 1 at $DIR/bool_compare.rs:3:8: 3:9
+           _3 = _1;                         // scope 1 at $DIR/bool_compare.rs:3:8: 3:9
+ -         _2 = Ne(move _3, const true);    // scope 1 at $DIR/bool_compare.rs:3:8: 3:17
+ +         _2 = Not(move _3);               // scope 1 at $DIR/bool_compare.rs:3:8: 3:17
+           StorageDead(_3);                 // scope 1 at $DIR/bool_compare.rs:3:16: 3:17
+           switchInt(move _2) -> [false: bb2, otherwise: bb1]; // scope 1 at $DIR/bool_compare.rs:3:8: 3:17
19   
20       bb1: {


-           _0 = const 0_u32;                // scope 0 at $DIR/bool_compare.rs:3:20: 3:21
+           _0 = const 0_u32;                // scope 1 at $DIR/bool_compare.rs:3:20: 3:21
22           goto -> bb3;                     // scope 0 at $DIR/bool_compare.rs:3:5: 3:34
24   


thread '[mir-opt] src/test/mir-opt/bool_compare.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/bool_compare.opt1.InstCombine.diff', src/tools/compiletest/src/runtest.rs:3437:25
---- [mir-opt] src/test/mir-opt/const_goto_storage.rs stdout ----
13       scope 1 {
13       scope 1 {
14           debug val => _1;                 // in scope 1 at $DIR/const_goto_storage.rs:3:9: 3:12
+       scope 2 {
+           scope 3 {
+               scope 4 {
+               }
+               }
+           }
+       }
16   
17       bb0: {
18           StorageLive(_1);                 // scope 0 at $DIR/const_goto_storage.rs:3:9: 3:12

19 -         StorageLive(_2);                 // scope 0 at $DIR/const_goto_storage.rs:3:21: 3:23
20 -         nop;                             // scope 0 at $DIR/const_goto_storage.rs:3:21: 3:23
21 -         StorageLive(_3);                 // scope 0 at $DIR/const_goto_storage.rs:4:15: 8:10
- -         StorageLive(_4);                 // scope 0 at $DIR/const_goto_storage.rs:4:18: 4:76
- -         StorageLive(_5);                 // scope 0 at $DIR/const_goto_storage.rs:4:21: 4:52
- -         StorageLive(_6);                 // scope 0 at $DIR/const_goto_storage.rs:4:24: 4:28
- -         _6 = const true;                 // scope 0 at $DIR/const_goto_storage.rs:4:24: 4:28
- -         switchInt(move _6) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/const_goto_storage.rs:4:24: 4:28
- +         StorageLive(_2);                 // scope 0 at $DIR/const_goto_storage.rs:4:24: 4:28
- +         _2 = const true;                 // scope 0 at $DIR/const_goto_storage.rs:4:24: 4:28
- +         switchInt(move _2) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/const_goto_storage.rs:4:24: 4:28
+ -         StorageLive(_4);                 // scope 2 at $DIR/const_goto_storage.rs:4:18: 4:76
+ -         StorageLive(_5);                 // scope 3 at $DIR/const_goto_storage.rs:4:21: 4:52
+ -         StorageLive(_6);                 // scope 4 at $DIR/const_goto_storage.rs:4:24: 4:28
+ -         _6 = const true;                 // scope 4 at $DIR/const_goto_storage.rs:4:24: 4:28
+ -         switchInt(move _6) -> [false: bb2, otherwise: bb1]; // scope 4 at $DIR/const_goto_storage.rs:4:24: 4:28
+ +         StorageLive(_2);                 // scope 4 at $DIR/const_goto_storage.rs:4:24: 4:28
+ +         _2 = const true;                 // scope 4 at $DIR/const_goto_storage.rs:4:24: 4:28
+ +         switchInt(move _2) -> [false: bb2, otherwise: bb1]; // scope 4 at $DIR/const_goto_storage.rs:4:24: 4:28
31   
32       bb1: {


- -         _5 = const true;                 // scope 0 at $DIR/const_goto_storage.rs:4:31: 4:35
- -         goto -> bb3;                     // scope 0 at $DIR/const_goto_storage.rs:4:21: 4:52
+ -         _5 = const true;                 // scope 4 at $DIR/const_goto_storage.rs:4:31: 4:35
+ -         goto -> bb3;                     // scope 3 at $DIR/const_goto_storage.rs:4:21: 4:52
36 - 
37 -     bb2: {


- -         _5 = const false;                // scope 0 at $DIR/const_goto_storage.rs:4:45: 4:50
- -         goto -> bb3;                     // scope 0 at $DIR/const_goto_storage.rs:4:21: 4:52
+ -         _5 = const false;                // scope 3 at $DIR/const_goto_storage.rs:4:45: 4:50
+ -         goto -> bb3;                     // scope 3 at $DIR/const_goto_storage.rs:4:21: 4:52
41 - 
42 -     bb3: {


- -         StorageDead(_6);                 // scope 0 at $DIR/const_goto_storage.rs:4:51: 4:52
- -         switchInt(move _5) -> [false: bb5, otherwise: bb4]; // scope 0 at $DIR/const_goto_storage.rs:4:21: 4:52
+ -         StorageDead(_6);                 // scope 3 at $DIR/const_goto_storage.rs:4:51: 4:52
+ -         switchInt(move _5) -> [false: bb5, otherwise: bb4]; // scope 3 at $DIR/const_goto_storage.rs:4:21: 4:52
46 - 
47 -     bb4: {


- -         _4 = const true;                 // scope 0 at $DIR/const_goto_storage.rs:4:55: 4:59
- -         goto -> bb6;                     // scope 0 at $DIR/const_goto_storage.rs:4:18: 4:76
+ -         _4 = const true;                 // scope 3 at $DIR/const_goto_storage.rs:4:55: 4:59
+ -         goto -> bb6;                     // scope 2 at $DIR/const_goto_storage.rs:4:18: 4:76
51 - 
52 -     bb5: {


- -         _4 = const false;                // scope 0 at $DIR/const_goto_storage.rs:4:69: 4:74
- -         goto -> bb6;                     // scope 0 at $DIR/const_goto_storage.rs:4:18: 4:76
+ -         _4 = const false;                // scope 2 at $DIR/const_goto_storage.rs:4:69: 4:74
+ -         goto -> bb6;                     // scope 2 at $DIR/const_goto_storage.rs:4:18: 4:76
56 - 
57 -     bb6: {


- -         StorageDead(_5);                 // scope 0 at $DIR/const_goto_storage.rs:4:75: 4:76
- -         switchInt(move _4) -> [false: bb8, otherwise: bb7]; // scope 0 at $DIR/const_goto_storage.rs:4:18: 4:76
+ -         StorageDead(_5);                 // scope 2 at $DIR/const_goto_storage.rs:4:75: 4:76
+ -         switchInt(move _4) -> [false: bb8, otherwise: bb7]; // scope 2 at $DIR/const_goto_storage.rs:4:18: 4:76
61 - 
62 -     bb7: {


- -         _3 = const true;                 // scope 0 at $DIR/const_goto_storage.rs:5:13: 5:17
+ -         _3 = const true;                 // scope 2 at $DIR/const_goto_storage.rs:5:13: 5:17
64 -         goto -> bb9;                     // scope 0 at $DIR/const_goto_storage.rs:4:15: 8:10
66 - 

76 -     bb10: {
76 -     bb10: {
77 -         StorageDead(_4);                 // scope 0 at $DIR/const_goto_storage.rs:8:9: 8:10
78 -         StorageDead(_3);                 // scope 0 at $DIR/const_goto_storage.rs:8:9: 8:10
- +         StorageDead(_2);                 // scope 0 at $DIR/const_goto_storage.rs:4:51: 4:52
+ +         StorageDead(_2);                 // scope 3 at $DIR/const_goto_storage.rs:4:51: 4:52
80           _1 = const true;                 // scope 0 at $DIR/const_goto_storage.rs:10:17: 10:21
81 -         goto -> bb12;                    // scope 0 at $DIR/const_goto_storage.rs:10:17: 10:21
82 +         goto -> bb3;                     // scope 0 at $DIR/const_goto_storage.rs:10:17: 10:21

86 -         StorageDead(_4);                 // scope 0 at $DIR/const_goto_storage.rs:8:9: 8:10
87 -         StorageDead(_3);                 // scope 0 at $DIR/const_goto_storage.rs:8:9: 8:10
88 +     bb2: {
- +         StorageDead(_2);                 // scope 0 at $DIR/const_goto_storage.rs:4:51: 4:52
+ +         StorageDead(_2);                 // scope 3 at $DIR/const_goto_storage.rs:4:51: 4:52
90           _1 = const false;                // scope 0 at $DIR/const_goto_storage.rs:12:14: 12:19
91 -         goto -> bb12;                    // scope 0 at $DIR/const_goto_storage.rs:12:14: 12:19
92 +         goto -> bb3;                     // scope 0 at $DIR/const_goto_storage.rs:12:14: 12:19

thread '[mir-opt] src/test/mir-opt/const_goto_storage.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_goto_storage.match_nested_if.ConstGoto.diff', src/tools/compiletest/src/runtest.rs:3437:25
---- [mir-opt] src/test/mir-opt/const_prop/control-flow-simplification.rs stdout ----
---- [mir-opt] src/test/mir-opt/const_prop/control-flow-simplification.rs stdout ----
5       let mut _0: ();                      // return place in scope 0 at $DIR/control-flow-simplification.rs:11:14: 11:14
6       let mut _1: bool;                    // in scope 0 at $DIR/control-flow-simplification.rs:12:8: 12:21
7       let mut _2: !;                       // in scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
+       scope 1 {
8   
9       bb0: {
9       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/control-flow-simplification.rs:12:8: 12:21
- -         _1 = const <bool as NeedsDrop>::NEEDS; // scope 0 at $DIR/control-flow-simplification.rs:12:8: 12:21
- -         switchInt(move _1) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/control-flow-simplification.rs:12:8: 12:21
- +         _1 = const false;                // scope 0 at $DIR/control-flow-simplification.rs:12:8: 12:21
- +         switchInt(const false) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/control-flow-simplification.rs:12:8: 12:21
+           StorageLive(_1);                 // scope 1 at $DIR/control-flow-simplification.rs:12:8: 12:21
+ -         _1 = const <bool as NeedsDrop>::NEEDS; // scope 1 at $DIR/control-flow-simplification.rs:12:8: 12:21
+ -         switchInt(move _1) -> [false: bb2, otherwise: bb1]; // scope 1 at $DIR/control-flow-simplification.rs:12:8: 12:21
+ +         _1 = const false;                // scope 1 at $DIR/control-flow-simplification.rs:12:8: 12:21
+ +         switchInt(const false) -> [false: bb2, otherwise: bb1]; // scope 1 at $DIR/control-flow-simplification.rs:12:8: 12:21
16   
17       bb1: {


-           StorageLive(_2);                 // scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
-           _2 = begin_panic::<&str>(const "explicit panic"); // scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
+           StorageLive(_2);                 // scope 1 at $SRC_DIR/std/src/panic.rs:LL:COL
+           _2 = begin_panic::<&str>(const "explicit panic"); // scope 1 at $SRC_DIR/std/src/panic.rs:LL:COL
20                                            // mir::Constant
21                                            // + span: $SRC_DIR/std/src/panic.rs:LL:COL
22                                            // + literal: Const { ty: fn(&str) -> ! {begin_panic::<&str>}, val: Value(Scalar(<ZST>)) }

thread '[mir-opt] src/test/mir-opt/const_prop/control-flow-simplification.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/control_flow_simplification.hello.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3437:25
---- [mir-opt] src/test/mir-opt/const_prop/discriminant.rs stdout ----
10       scope 1 {
10       scope 1 {
11           debug x => _1;                   // in scope 1 at $DIR/discriminant.rs:11:9: 11:10
+       scope 2 {
+       }
13   
14       bb0: {
14       bb0: {
15           StorageLive(_1);                 // scope 0 at $DIR/discriminant.rs:11:9: 11:10

16           StorageLive(_2);                 // scope 0 at $DIR/discriminant.rs:11:13: 11:64
-           StorageLive(_3);                 // scope 0 at $DIR/discriminant.rs:11:34: 11:44
-           Deinit(_3);                      // scope 0 at $DIR/discriminant.rs:11:34: 11:44
-           ((_3 as Some).0: bool) = const true; // scope 0 at $DIR/discriminant.rs:11:34: 11:44
-           discriminant(_3) = 1;            // scope 0 at $DIR/discriminant.rs:11:34: 11:44
- -         _4 = discriminant(_3);           // scope 0 at $DIR/discriminant.rs:11:21: 11:31
- -         switchInt(move _4) -> [1_isize: bb1, otherwise: bb3]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
- +         _4 = const 1_isize;              // scope 0 at $DIR/discriminant.rs:11:21: 11:31
- +         switchInt(const 1_isize) -> [1_isize: bb1, otherwise: bb3]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
+           StorageLive(_3);                 // scope 2 at $DIR/discriminant.rs:11:34: 11:44
+           Deinit(_3);                      // scope 2 at $DIR/discriminant.rs:11:34: 11:44
+           ((_3 as Some).0: bool) = const true; // scope 2 at $DIR/discriminant.rs:11:34: 11:44
+           discriminant(_3) = 1;            // scope 2 at $DIR/discriminant.rs:11:34: 11:44
+ -         _4 = discriminant(_3);           // scope 2 at $DIR/discriminant.rs:11:21: 11:31
+ -         switchInt(move _4) -> [1_isize: bb1, otherwise: bb3]; // scope 2 at $DIR/discriminant.rs:11:21: 11:31
+ +         _4 = const 1_isize;              // scope 2 at $DIR/discriminant.rs:11:21: 11:31
+ +         switchInt(const 1_isize) -> [1_isize: bb1, otherwise: bb3]; // scope 2 at $DIR/discriminant.rs:11:21: 11:31
26   
27       bb1: {


-           switchInt(((_3 as Some).0: bool)) -> [false: bb3, otherwise: bb2]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
+           switchInt(((_3 as Some).0: bool)) -> [false: bb3, otherwise: bb2]; // scope 2 at $DIR/discriminant.rs:11:21: 11:31
30   
31       bb2: {


-           _2 = const 42_i32;               // scope 0 at $DIR/discriminant.rs:11:47: 11:49
+           _2 = const 42_i32;               // scope 2 at $DIR/discriminant.rs:11:47: 11:49
33           goto -> bb4;                     // scope 0 at $DIR/discriminant.rs:11:13: 11:64
35   


thread '[mir-opt] src/test/mir-opt/const_prop/discriminant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/discriminant.main.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3437:25
---- [mir-opt] src/test/mir-opt/dead-store-elimination/cycle.rs stdout ----
---- [mir-opt] src/test/mir-opt/dead-store-elimination/cycle.rs stdout ----
8       let mut _0: ();                      // return place in scope 0 at $DIR/cycle.rs:9:46: 9:46
9       let mut _4: ();                      // in scope 0 at $DIR/cycle.rs:9:1: 18:2
10       let mut _5: bool;                    // in scope 0 at $DIR/cycle.rs:12:11: 12:17
-       let _6: i32;                         // in scope 0 at $DIR/cycle.rs:13:13: 13:17
12       let mut _7: i32;                     // in scope 0 at $DIR/cycle.rs:14:13: 14:14
13       let mut _8: i32;                     // in scope 0 at $DIR/cycle.rs:15:13: 15:14
14       let mut _9: i32;                     // in scope 0 at $DIR/cycle.rs:16:13: 16:17

16       let _11: ();                         // in scope 0 at $DIR/cycle.rs:12:5: 17:6
17       let mut _12: !;                      // in scope 0 at $DIR/cycle.rs:12:5: 17:6
18       scope 1 {
-           debug temp => _6;                // in scope 1 at $DIR/cycle.rs:13:13: 13:17
+           let _6: i32;                     // in scope 1 at $DIR/cycle.rs:13:13: 13:17
+           scope 2 {
+               debug temp => _6;            // in scope 2 at $DIR/cycle.rs:13:13: 13:17
20       }
21   
22       bb0: {


24       }
25   
26       bb1: {
-           StorageLive(_5);                 // scope 0 at $DIR/cycle.rs:12:11: 12:17
-           _5 = cond() -> bb2;              // scope 0 at $DIR/cycle.rs:12:11: 12:17
+           StorageLive(_5);                 // scope 1 at $DIR/cycle.rs:12:11: 12:17
+           _5 = cond() -> bb2;              // scope 1 at $DIR/cycle.rs:12:11: 12:17
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
29                                            // mir::Constant
30                                            // + span: $DIR/cycle.rs:12:11: 12:15
31                                            // + literal: Const { ty: fn() -> bool {cond}, val: Value(Scalar(<ZST>)) }
32       }
33   
34       bb2: {
34       bb2: {
-           switchInt(move _5) -> [false: bb4, otherwise: bb3]; // scope 0 at $DIR/cycle.rs:12:11: 12:17
+           switchInt(move _5) -> [false: bb4, otherwise: bb3]; // scope 1 at $DIR/cycle.rs:12:11: 12:17
37   
38       bb3: {


-           StorageLive(_6);                 // scope 0 at $DIR/cycle.rs:13:13: 13:17
- -         _6 = _3;                         // scope 0 at $DIR/cycle.rs:13:20: 13:21
- +         nop;                             // scope 0 at $DIR/cycle.rs:13:20: 13:21
-           StorageLive(_7);                 // scope 1 at $DIR/cycle.rs:14:13: 14:14
- -         _7 = _2;                         // scope 1 at $DIR/cycle.rs:14:13: 14:14
- -         _3 = move _7;                    // scope 1 at $DIR/cycle.rs:14:9: 14:14
- +         nop;                             // scope 1 at $DIR/cycle.rs:14:13: 14:14
- +         nop;                             // scope 1 at $DIR/cycle.rs:14:9: 14:14
-           StorageDead(_7);                 // scope 1 at $DIR/cycle.rs:14:13: 14:14
-           StorageLive(_8);                 // scope 1 at $DIR/cycle.rs:15:13: 15:14
- -         _8 = _1;                         // scope 1 at $DIR/cycle.rs:15:13: 15:14
- -         _2 = move _8;                    // scope 1 at $DIR/cycle.rs:15:9: 15:14
- +         nop;                             // scope 1 at $DIR/cycle.rs:15:13: 15:14
- +         nop;                             // scope 1 at $DIR/cycle.rs:15:9: 15:14
-           StorageDead(_8);                 // scope 1 at $DIR/cycle.rs:15:13: 15:14
-           StorageLive(_9);                 // scope 1 at $DIR/cycle.rs:16:13: 16:17
- -         _9 = _6;                         // scope 1 at $DIR/cycle.rs:16:13: 16:17
- -         _1 = move _9;                    // scope 1 at $DIR/cycle.rs:16:9: 16:17
- +         nop;                             // scope 1 at $DIR/cycle.rs:16:13: 16:17
- +         nop;                             // scope 1 at $DIR/cycle.rs:16:9: 16:17
-           StorageDead(_9);                 // scope 1 at $DIR/cycle.rs:16:16: 16:17
- -         _4 = const ();                   // scope 0 at $DIR/cycle.rs:12:18: 17:6
- +         nop;                             // scope 0 at $DIR/cycle.rs:12:18: 17:6
-           StorageDead(_6);                 // scope 0 at $DIR/cycle.rs:17:5: 17:6
+           StorageLive(_6);                 // scope 1 at $DIR/cycle.rs:13:13: 13:17
+ -         _6 = _3;                         // scope 1 at $DIR/cycle.rs:13:20: 13:21
+ +         nop;                             // scope 1 at $DIR/cycle.rs:13:20: 13:21
+           StorageLive(_7);                 // scope 2 at $DIR/cycle.rs:14:13: 14:14
+ -         _7 = _2;                         // scope 2 at $DIR/cycle.rs:14:13: 14:14
+ -         _3 = move _7;                    // scope 2 at $DIR/cycle.rs:14:9: 14:14
+ +         nop;                             // scope 2 at $DIR/cycle.rs:14:13: 14:14
+ +         nop;                             // scope 2 at $DIR/cycle.rs:14:9: 14:14
+           StorageDead(_7);                 // scope 2 at $DIR/cycle.rs:14:13: 14:14
+           StorageLive(_8);                 // scope 2 at $DIR/cycle.rs:15:13: 15:14
+ -         _8 = _1;                         // scope 2 at $DIR/cycle.rs:15:13: 15:14
+ -         _2 = move _8;                    // scope 2 at $DIR/cycle.rs:15:9: 15:14
+ +         nop;                             // scope 2 at $DIR/cycle.rs:15:13: 15:14
+ +         nop;                             // scope 2 at $DIR/cycle.rs:15:9: 15:14
+           StorageDead(_8);                 // scope 2 at $DIR/cycle.rs:15:13: 15:14
+           StorageLive(_9);                 // scope 2 at $DIR/cycle.rs:16:13: 16:17
+ -         _9 = _6;                         // scope 2 at $DIR/cycle.rs:16:13: 16:17
+ -         _1 = move _9;                    // scope 2 at $DIR/cycle.rs:16:9: 16:17
+ +         nop;                             // scope 2 at $DIR/cycle.rs:16:13: 16:17
+ +         nop;                             // scope 2 at $DIR/cycle.rs:16:9: 16:17
+           StorageDead(_9);                 // scope 2 at $DIR/cycle.rs:16:16: 16:17
+ -         _4 = const ();                   // scope 1 at $DIR/cycle.rs:12:18: 17:6
+ +         nop;                             // scope 1 at $DIR/cycle.rs:12:18: 17:6
+           StorageDead(_6);                 // scope 1 at $DIR/cycle.rs:17:5: 17:6
63           StorageDead(_5);                 // scope 0 at $DIR/cycle.rs:17:5: 17:6
64           goto -> bb1;                     // scope 0 at $DIR/cycle.rs:12:5: 17:6


thread '[mir-opt] src/test/mir-opt/dead-store-elimination/cycle.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/dead-store-elimination/cycle.cycle.DeadStoreElimination.diff', src/tools/compiletest/src/runtest.rs:3437:25
---- [mir-opt] src/test/mir-opt/deaggregator_test_enum_2.rs stdout ----
---- [mir-opt] src/test/mir-opt/deaggregator_test_enum_2.rs stdout ----
8       let mut _3: bool;                    // in scope 0 at $DIR/deaggregator_test_enum_2.rs:10:8: 10:9
9       let mut _4: i32;                     // in scope 0 at $DIR/deaggregator_test_enum_2.rs:11:16: 11:17
10       let mut _5: i32;                     // in scope 0 at $DIR/deaggregator_test_enum_2.rs:13:16: 13:17
+       scope 1 {
11   
12       bb0: {
12       bb0: {
-           StorageLive(_3);                 // scope 0 at $DIR/deaggregator_test_enum_2.rs:10:8: 10:9
-           _3 = _1;                         // scope 0 at $DIR/deaggregator_test_enum_2.rs:10:8: 10:9
-           switchInt(move _3) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/deaggregator_test_enum_2.rs:10:8: 10:9
+           StorageLive(_3);                 // scope 1 at $DIR/deaggregator_test_enum_2.rs:10:8: 10:9
+           _3 = _1;                         // scope 1 at $DIR/deaggregator_test_enum_2.rs:10:8: 10:9
+           switchInt(move _3) -> [false: bb2, otherwise: bb1]; // scope 1 at $DIR/deaggregator_test_enum_2.rs:10:8: 10:9
17   
18       bb1: {


-           StorageLive(_4);                 // scope 0 at $DIR/deaggregator_test_enum_2.rs:11:16: 11:17
-           _4 = _2;                         // scope 0 at $DIR/deaggregator_test_enum_2.rs:11:16: 11:17
- -         _0 = Foo::A(move _4);            // scope 0 at $DIR/deaggregator_test_enum_2.rs:11:9: 11:18
- +         Deinit(_0);                      // scope 0 at $DIR/deaggregator_test_enum_2.rs:11:9: 11:18
- +         ((_0 as A).0: i32) = move _4;    // scope 0 at $DIR/deaggregator_test_enum_2.rs:11:9: 11:18
- +         discriminant(_0) = 0;            // scope 0 at $DIR/deaggregator_test_enum_2.rs:11:9: 11:18
-           StorageDead(_4);                 // scope 0 at $DIR/deaggregator_test_enum_2.rs:11:17: 11:18
+           StorageLive(_4);                 // scope 1 at $DIR/deaggregator_test_enum_2.rs:11:16: 11:17
+           _4 = _2;                         // scope 1 at $DIR/deaggregator_test_enum_2.rs:11:16: 11:17
+ -         _0 = Foo::A(move _4);            // scope 1 at $DIR/deaggregator_test_enum_2.rs:11:9: 11:18
+ +         Deinit(_0);                      // scope 1 at $DIR/deaggregator_test_enum_2.rs:11:9: 11:18
+ +         ((_0 as A).0: i32) = move _4;    // scope 1 at $DIR/deaggregator_test_enum_2.rs:11:9: 11:18
+ +         discriminant(_0) = 0;            // scope 1 at $DIR/deaggregator_test_enum_2.rs:11:9: 11:18
+           StorageDead(_4);                 // scope 1 at $DIR/deaggregator_test_enum_2.rs:11:17: 11:18
26           goto -> bb3;                     // scope 0 at $DIR/deaggregator_test_enum_2.rs:10:5: 14:6
28   


thread '[mir-opt] src/test/mir-opt/deaggregator_test_enum_2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/deaggregator_test_enum_2.test1.Deaggregator.diff', src/tools/compiletest/src/runtest.rs:3437:25
---- [mir-opt] src/test/mir-opt/dest-prop/branch.rs stdout ----
12           scope 2 {
12           scope 2 {
13               debug y => _2;               // in scope 2 at $DIR/branch.rs:15:9: 15:10
+           scope 3 {
+           }
15       }
16   
16   
17       bb0: {

24   
25       bb1: {
26           StorageLive(_2);                 // scope 1 at $DIR/branch.rs:15:9: 15:10
-           StorageLive(_3);                 // scope 1 at $DIR/branch.rs:15:16: 15:22
-           _3 = cond() -> bb2;              // scope 1 at $DIR/branch.rs:15:16: 15:22
+           StorageLive(_3);                 // scope 3 at $DIR/branch.rs:15:16: 15:22
+           _3 = cond() -> bb2;              // scope 3 at $DIR/branch.rs:15:16: 15:22
29                                            // mir::Constant
30                                            // + span: $DIR/branch.rs:15:16: 15:20
31                                            // + literal: Const { ty: fn() -> bool {cond}, val: Value(Scalar(<ZST>)) }
32       }
33   
34       bb2: {
34       bb2: {
-           switchInt(move _3) -> [false: bb4, otherwise: bb3]; // scope 1 at $DIR/branch.rs:15:16: 15:22
+           switchInt(move _3) -> [false: bb4, otherwise: bb3]; // scope 3 at $DIR/branch.rs:15:16: 15:22
37   
38       bb3: {


-           nop;                             // scope 1 at $DIR/branch.rs:16:9: 16:10
+           nop;                             // scope 3 at $DIR/branch.rs:16:9: 16:10
40           goto -> bb6;                     // scope 1 at $DIR/branch.rs:15:13: 20:6
42   


thread '[mir-opt] src/test/mir-opt/dest-prop/branch.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/dest-prop/branch.main.DestinationPropagation.diff', src/tools/compiletest/src/runtest.rs:3437:25
---- [mir-opt] src/test/mir-opt/equal_true.rs stdout ----
---- [mir-opt] src/test/mir-opt/equal_true.rs stdout ----
6       let mut _0: i32;                     // return place in scope 0 at $DIR/equal_true.rs:3:20: 3:23
7       let mut _2: bool;                    // in scope 0 at $DIR/equal_true.rs:4:8: 4:17
8       let mut _3: bool;                    // in scope 0 at $DIR/equal_true.rs:4:8: 4:9
+       scope 1 {
9   
10       bb0: {
10       bb0: {
-           StorageLive(_2);                 // scope 0 at $DIR/equal_true.rs:4:8: 4:17
-           StorageLive(_3);                 // scope 0 at $DIR/equal_true.rs:4:8: 4:9
-           _3 = _1;                         // scope 0 at $DIR/equal_true.rs:4:8: 4:9
- -         _2 = Eq(move _3, const true);    // scope 0 at $DIR/equal_true.rs:4:8: 4:17
- +         _2 = move _3;                    // scope 0 at $DIR/equal_true.rs:4:8: 4:17
-           StorageDead(_3);                 // scope 0 at $DIR/equal_true.rs:4:16: 4:17
-           switchInt(move _2) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/equal_true.rs:4:8: 4:17
+           StorageLive(_2);                 // scope 1 at $DIR/equal_true.rs:4:8: 4:17
+           StorageLive(_3);                 // scope 1 at $DIR/equal_true.rs:4:8: 4:9
+           _3 = _1;                         // scope 1 at $DIR/equal_true.rs:4:8: 4:9
+ -         _2 = Eq(move _3, const true);    // scope 1 at $DIR/equal_true.rs:4:8: 4:17
+ +         _2 = move _3;                    // scope 1 at $DIR/equal_true.rs:4:8: 4:17
+           StorageDead(_3);                 // scope 1 at $DIR/equal_true.rs:4:16: 4:17
+           switchInt(move _2) -> [false: bb2, otherwise: bb1]; // scope 1 at $DIR/equal_true.rs:4:8: 4:17
19   
20       bb1: {


-           _0 = const 0_i32;                // scope 0 at $DIR/equal_true.rs:4:20: 4:21
+           _0 = const 0_i32;                // scope 1 at $DIR/equal_true.rs:4:20: 4:21
22           goto -> bb3;                     // scope 0 at $DIR/equal_true.rs:4:5: 4:34
24   


thread '[mir-opt] src/test/mir-opt/equal_true.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/equal_true.opt.InstCombine.diff', src/tools/compiletest/src/runtest.rs:3437:25
---- [mir-opt] src/test/mir-opt/early_otherwise_branch_soundness.rs stdout ----
---- [mir-opt] src/test/mir-opt/early_otherwise_branch_soundness.rs stdout ----
7       let mut _2: isize;                   // in scope 0 at $DIR/early_otherwise_branch_soundness.rs:13:20: 13:30
8       let mut _3: isize;                   // in scope 0 at $DIR/early_otherwise_branch_soundness.rs:13:12: 13:31
9       let mut _4: &E;                      // in scope 0 at $DIR/early_otherwise_branch_soundness.rs:12:16: 12:17
+       scope 1 {
10   
11       bb0: {
11       bb0: {
