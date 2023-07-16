plain
.................................................................................................... 9500/11836
.................................................................................................... 9600/11836
.......................................................i.....i...................................... 9700/11836
.................................................................................................... 9800/11836
.iiiiiii.iiiiii.i................................................................................... 9900/11836
.................................................................................................... 10100/11836
.................................................................................................... 10200/11836
.................................................................................................... 10300/11836
.................................................................................................... 10400/11836
---
Suite("src/test/mir-opt") not skipped for "bootstrap::test::MirOpt" -- not in ["src/tools/tidy"]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 160 tests
..................................i............................................i.....F......F...F... 100/160
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...FF......................i...........F.........F.......F..

---- [mir-opt] mir-opt/inline/inline-diverging.rs stdout ----
---- [mir-opt] mir-opt/inline/inline-diverging.rs stdout ----
4   fn f() -> () {
5       let mut _0: ();                      // return place in scope 0 at $DIR/inline-diverging.rs:7:12: 7:12
6       let mut _1: !;                       // in scope 0 at $DIR/inline-diverging.rs:7:12: 9:2
-       let _2: !;                           // in scope 0 at $DIR/inline-diverging.rs:8:5: 8:12
- +     let mut _3: !;                       // in scope 0 at $DIR/inline-diverging.rs:8:5: 8:12
+       let _2: ();                          // in scope 0 at $DIR/inline-diverging.rs:8:5: 8:12
+       let mut _3: !;                       // in scope 0 at $DIR/inline-diverging.rs:8:5: 8:12
+ +     let mut _4: !;                       // in scope 0 at $DIR/inline-diverging.rs:8:5: 8:12
9 +     scope 1 (inlined sleep) {            // at $DIR/inline-diverging.rs:8:5: 8:12
10 +     }

12       bb0: {
12       bb0: {
13           StorageLive(_2);                 // scope 0 at $DIR/inline-diverging.rs:8:5: 8:12
+           StorageLive(_3);                 // scope 0 at $DIR/inline-diverging.rs:8:5: 8:12
14 -         sleep();                         // scope 0 at $DIR/inline-diverging.rs:8:5: 8:12
15 -                                          // mir::Constant
16 -                                          // + span: $DIR/inline-diverging.rs:8:5: 8:10

17 -                                          // + literal: Const { ty: fn() -> ! {sleep}, val: Value(Scalar(<ZST>)) }
- +         StorageLive(_3);                 // scope 0 at $DIR/inline-diverging.rs:8:5: 8:12
+ +         StorageLive(_4);                 // scope 0 at $DIR/inline-diverging.rs:8:5: 8:12
19 +         goto -> bb1;                     // scope 0 at $DIR/inline-diverging.rs:8:5: 8:12
20 +     }


thread '[mir-opt] mir-opt/inline/inline-diverging.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_diverging.f.Inline.diff', src/tools/compiletest/src/runtest.rs:3554:25

---- [mir-opt] mir-opt/issue-38669.rs stdout ----
---- [mir-opt] mir-opt/issue-38669.rs stdout ----
7     let _3: ();                          // in scope 0 at $DIR/issue-38669.rs:7:9: 9:10
8     let mut _4: bool;                    // in scope 0 at $DIR/issue-38669.rs:7:12: 7:24
9     let mut _5: !;                       // in scope 0 at $DIR/issue-38669.rs:7:25: 9:10
+     let _6: ();                          // in scope 0 at $DIR/issue-38669.rs:8:13: 8:18
+     let mut _7: !;                       // in scope 0 at $DIR/issue-38669.rs:8:13: 8:18
10     scope 1 {
11         debug should_break => _1;        // in scope 1 at $DIR/issue-38669.rs:5:9: 5:25

30     }
31 
32     bb3: {
32     bb3: {
+         StorageLive(_6);                 // scope 1 at $DIR/issue-38669.rs:8:13: 8:18
33         _0 = const ();                   // scope 1 at $DIR/issue-38669.rs:8:13: 8:18
+         StorageDead(_6);                 // scope 1 at $DIR/issue-38669.rs:8:18: 8:19
34         StorageDead(_4);                 // scope 1 at $DIR/issue-38669.rs:9:9: 9:10
35         StorageDead(_3);                 // scope 1 at $DIR/issue-38669.rs:9:9: 9:10
36         StorageDead(_1);                 // scope 0 at $DIR/issue-38669.rs:12:1: 12:2

thread '[mir-opt] mir-opt/issue-38669.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_38669.main.SimplifyCfg-initial.after.mir', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/issue-72181-1.rs stdout ----
---- [mir-opt] mir-opt/issue-72181-1.rs stdout ----
9     let mut _1: !;                       // in scope 0 at $DIR/issue-72181-1.rs:15:11: 21:2
10     let _2: Void as UserTypeProjection { base: UserType(0), projs: [] }; // in scope 0 at $DIR/issue-72181-1.rs:16:9: 16:10
11     let mut _3: ();                      // in scope 0 at $DIR/issue-72181-1.rs:17:41: 17:43
-     let _4: !;                           // in scope 0 at $DIR/issue-72181-1.rs:20:5: 20:9
-     let mut _5: Void;                    // in scope 0 at $DIR/issue-72181-1.rs:20:7: 20:8
+     let _4: ();                          // in scope 0 at $DIR/issue-72181-1.rs:20:5: 20:9
+     let mut _5: !;                       // in scope 0 at $DIR/issue-72181-1.rs:20:5: 20:9
+     let mut _6: Void;                    // in scope 0 at $DIR/issue-72181-1.rs:20:7: 20:8
14     scope 1 {
15         debug v => _2;                   // in scope 1 at $DIR/issue-72181-1.rs:16:9: 16:10


32         FakeRead(ForLet(None), _2);      // scope 0 at $DIR/issue-72181-1.rs:16:9: 16:10
33         AscribeUserType(_2, o, UserTypeProjection { base: UserType(1), projs: [] }); // scope 0 at $DIR/issue-72181-1.rs:16:12: 16:16
34         StorageLive(_4);                 // scope 1 at $DIR/issue-72181-1.rs:20:5: 20:9
-         StorageLive(_5);                 // scope 1 at $DIR/issue-72181-1.rs:20:7: 20:8
-         _5 = move _2;                    // scope 1 at $DIR/issue-72181-1.rs:20:7: 20:8
-         f(move _5) -> bb4;               // scope 1 at $DIR/issue-72181-1.rs:20:5: 20:9
+         StorageLive(_5);                 // scope 1 at $DIR/issue-72181-1.rs:20:5: 20:9
+         StorageLive(_6);                 // scope 1 at $DIR/issue-72181-1.rs:20:7: 20:8
+         _6 = move _2;                    // scope 1 at $DIR/issue-72181-1.rs:20:7: 20:8
+         f(move _6) -> bb4;               // scope 1 at $DIR/issue-72181-1.rs:20:5: 20:9
38                                          // mir::Constant
39                                          // + span: $DIR/issue-72181-1.rs:20:5: 20:6
40                                          // + literal: Const { ty: fn(Void) -> ! {f}, val: Value(Scalar(<ZST>)) }
41     }
42 
43     bb2: {
43     bb2: {
+         StorageDead(_6);                 // scope 1 at $DIR/issue-72181-1.rs:20:8: 20:9
44         StorageDead(_5);                 // scope 1 at $DIR/issue-72181-1.rs:20:8: 20:9
45         StorageDead(_4);                 // scope 1 at $DIR/issue-72181-1.rs:20:9: 20:10
46         StorageDead(_2);                 // scope 0 at $DIR/issue-72181-1.rs:21:1: 21:2

thread '[mir-opt] mir-opt/issue-72181-1.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_72181_1.main.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/loop_test.rs stdout ----
---- [mir-opt] mir-opt/loop_test.rs stdout ----
5     let _1: ();                          // in scope 0 at $DIR/loop_test.rs:10:5: 12:6
6     let mut _2: bool;                    // in scope 0 at $DIR/loop_test.rs:10:8: 10:12
7     let mut _3: !;                       // in scope 0 at $DIR/loop_test.rs:10:13: 12:6
-     let mut _4: !;                       // in scope 0 at $DIR/loop_test.rs:13:5: 16:6
-     let mut _5: ();                      // in scope 0 at $DIR/loop_test.rs:6:1: 17:2
-     let _6: i32;                         // in scope 0 at $DIR/loop_test.rs:14:13: 14:14
+     let _4: ();                          // in scope 0 at $DIR/loop_test.rs:11:9: 11:15
+     let mut _5: !;                       // in scope 0 at $DIR/loop_test.rs:11:9: 11:15
+     let mut _6: !;                       // in scope 0 at $DIR/loop_test.rs:13:5: 16:6
+     let mut _7: ();                      // in scope 0 at $DIR/loop_test.rs:6:1: 17:2
+     let _8: i32;                         // in scope 0 at $DIR/loop_test.rs:14:13: 14:14
+     let _9: ();                          // in scope 0 at $DIR/loop_test.rs:15:9: 15:17
+     let mut _10: !;                      // in scope 0 at $DIR/loop_test.rs:15:9: 15:17
11     scope 1 {
-         debug x => _6;                   // in scope 1 at $DIR/loop_test.rs:14:13: 14:14
+         debug x => _8;                   // in scope 1 at $DIR/loop_test.rs:14:13: 14:14
14 
15     bb0: {

20     }
20     }
21 
22     bb1: {
+         StorageLive(_4);                 // scope 0 at $DIR/loop_test.rs:11:9: 11:15
23         _0 = const ();                   // scope 0 at $DIR/loop_test.rs:11:9: 11:15
+         StorageDead(_4);                 // scope 0 at $DIR/loop_test.rs:11:15: 11:16
24         StorageDead(_2);                 // scope 0 at $DIR/loop_test.rs:12:5: 12:6
25         StorageDead(_1);                 // scope 0 at $DIR/loop_test.rs:12:5: 12:6
26         return;                          // scope 0 at $DIR/loop_test.rs:17:2: 17:2

30         _1 = const ();                   // scope 0 at $DIR/loop_test.rs:12:6: 12:6
31         StorageDead(_2);                 // scope 0 at $DIR/loop_test.rs:12:5: 12:6
32         StorageDead(_1);                 // scope 0 at $DIR/loop_test.rs:12:5: 12:6
-         StorageLive(_4);                 // scope 0 at $DIR/loop_test.rs:13:5: 16:6
+         StorageLive(_6);                 // scope 0 at $DIR/loop_test.rs:13:5: 16:6
34         goto -> bb3;                     // scope 0 at $DIR/loop_test.rs:13:5: 16:6
36 

39     }
40 
40 
41     bb4: {
-         StorageLive(_6);                 // scope 0 at $DIR/loop_test.rs:14:13: 14:14
-         _6 = const 1_i32;                // scope 0 at $DIR/loop_test.rs:14:17: 14:18
-         FakeRead(ForLet(None), _6);      // scope 0 at $DIR/loop_test.rs:14:13: 14:14
-         StorageDead(_6);                 // scope 0 at $DIR/loop_test.rs:16:5: 16:6
+         StorageLive(_8);                 // scope 0 at $DIR/loop_test.rs:14:13: 14:14
+         _8 = const 1_i32;                // scope 0 at $DIR/loop_test.rs:14:17: 14:18
+         FakeRead(ForLet(None), _8);      // scope 0 at $DIR/loop_test.rs:14:13: 14:14
+         StorageLive(_9);                 // scope 1 at $DIR/loop_test.rs:15:9: 15:17
+         StorageDead(_9);                 // scope 1 at $DIR/loop_test.rs:15:17: 15:18
+         StorageDead(_8);                 // scope 0 at $DIR/loop_test.rs:16:5: 16:6
46         goto -> bb3;                     // scope 0 at $DIR/loop_test.rs:1:1: 1:1
48 


thread '[mir-opt] mir-opt/loop_test.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/loop_test.main.SimplifyCfg-promote-consts.after.mir', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
19       let mut _17: i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
20       let mut _18: i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
21       let mut _19: !;                      // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let _21: !;                          // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _22: core::panicking::AssertKind; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _23: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let _24: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _25: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let _26: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _27: std::option::Option<std::fmt::Arguments>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let _21: ();                         // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _22: !;                      // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _23: core::panicking::AssertKind; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _24: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let _25: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _26: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let _27: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _28: std::option::Option<std::fmt::Arguments>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
29       scope 1 {
30           debug split => _1;               // in scope 1 at $DIR/issue-73223.rs:2:9: 2:14
31           let _6: std::option::Option<i32>; // in scope 1 at $DIR/issue-73223.rs:7:9: 7:14

33               debug _prev => _6;           // in scope 3 at $DIR/issue-73223.rs:7:9: 7:14
34               let _13: &i32;               // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
35               let _14: &i32;               // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-               let mut _28: &i32;           // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+               let mut _29: &i32;           // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
37               scope 4 {
38                   debug left_val => _13;   // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
39                   debug right_val => _14;  // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

81           StorageLive(_10);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
82           _10 = &_1;                       // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
83           StorageLive(_11);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _28 = const main::promoted[0];   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _29 = const main::promoted[0];   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
85                                            // ty::Const
86                                            // + ty: &i32
87                                            // + val: Unevaluated(main, [], Some(promoted[0]))
88                                            // mir::Constant
88                                            // mir::Constant
89                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
90                                            // + literal: Const { ty: &i32, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ issue_73223[317d]::main), const_param_did: None }, substs: [], promoted: Some(promoted[0]) }) }
-           _11 = _28;                       // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _11 = _29;                       // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
92           (_9.0: &i32) = move _10;         // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
93           (_9.1: &i32) = move _11;         // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
94           StorageDead(_11);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

116           discriminant(_20) = 0;           // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
117           StorageLive(_21);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
118           StorageLive(_22);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _22 = const core::panicking::AssertKind::Eq; // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_23);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _23 = const core::panicking::AssertKind::Eq; // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
120                                            // ty::Const
121                                            // + ty: core::panicking::AssertKind
122                                            // + val: Value(Scalar(0x00))
123                                            // mir::Constant
123                                            // mir::Constant
124                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
125                                            // + literal: Const { ty: core::panicking::AssertKind, val: Value(Scalar(0x00)) }
-           StorageLive(_23);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
127           StorageLive(_24);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _24 = _13;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _23 = _24;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
130           StorageLive(_25);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _25 = _13;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _24 = _25;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
131           StorageLive(_26);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _26 = _14;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _25 = _26;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
134           StorageLive(_27);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           discriminant(_27) = 0;           // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _23, move _25, move _27); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _27 = _14;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _26 = _27;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_28);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           discriminant(_28) = 0;           // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _24, move _26, move _28); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
137                                            // mir::Constant
138                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
139                                            // + literal: Const { ty: for<'r, 's, 't0> fn(core::panicking::AssertKind, &'r i32, &'s i32, std::option::Option<std::fmt::Arguments<'t0>>) -> ! {core::panicking::assert_failed::<i32, i32>}, val: Value(Scalar(<ZST>)) }

thread '[mir-opt] mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.64bit.diff', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/simplify_cfg.rs stdout ----
---- [mir-opt] mir-opt/simplify_cfg.rs stdout ----
6       let mut _1: ();                      // in scope 0 at $DIR/simplify_cfg.rs:5:1: 11:2
7       let mut _2: bool;                    // in scope 0 at $DIR/simplify_cfg.rs:7:12: 7:17
8       let mut _3: !;                       // in scope 0 at $DIR/simplify_cfg.rs:7:18: 9:10
+       let _4: ();                          // in scope 0 at $DIR/simplify_cfg.rs:8:13: 8:18
+       let mut _5: !;                       // in scope 0 at $DIR/simplify_cfg.rs:8:13: 8:18
10       bb0: {
10       bb0: {
11 -         goto -> bb1;                     // scope 0 at $DIR/simplify_cfg.rs:6:5: 10:6
13       }
14   
15       bb1: {
15       bb1: {
- -         falseUnwind -> [real: bb2, cleanup: bb10]; // scope 0 at $DIR/simplify_cfg.rs:6:5: 10:6
+ -         falseUnwind -> [real: bb2, cleanup: bb11]; // scope 0 at $DIR/simplify_cfg.rs:6:5: 10:6
17 -     }
19 -     bb2: {


20           StorageLive(_2);                 // scope 0 at $DIR/simplify_cfg.rs:7:12: 7:17
- -         _2 = bar() -> [return: bb3, unwind: bb10]; // scope 0 at $DIR/simplify_cfg.rs:7:12: 7:17
+ -         _2 = bar() -> [return: bb3, unwind: bb11]; // scope 0 at $DIR/simplify_cfg.rs:7:12: 7:17
22 +         _2 = bar() -> [return: bb2, unwind: bb5]; // scope 0 at $DIR/simplify_cfg.rs:7:12: 7:17
23                                            // mir::Constant
24                                            // + span: $DIR/simplify_cfg.rs:7:12: 7:15
33   
34 -     bb4: {
35 +     bb3: {
35 +     bb3: {
+           StorageLive(_4);                 // scope 0 at $DIR/simplify_cfg.rs:8:13: 8:18
36           _0 = const ();                   // scope 0 at $DIR/simplify_cfg.rs:8:13: 8:18
- -         goto -> bb9;                     // scope 0 at $DIR/simplify_cfg.rs:8:13: 8:18
- +         StorageDead(_2);                 // scope 0 at $DIR/simplify_cfg.rs:9:9: 9:10
- +         return;                          // scope 0 at $DIR/simplify_cfg.rs:11:2: 11:2
-   
-   
+ -         goto -> bb10;                    // scope 0 at $DIR/simplify_cfg.rs:8:13: 8:18
+ -     }
42 -     bb5: {
- +     bb4: {
- +     bb4: {
-           _1 = const ();                   // scope 0 at $DIR/simplify_cfg.rs:9:10: 9:10
- -         goto -> bb8;                     // scope 0 at $DIR/simplify_cfg.rs:7:9: 9:10
+ -         _1 = const ();                   // scope 0 at $DIR/simplify_cfg.rs:9:10: 9:10
+ -         goto -> bb9;                     // scope 0 at $DIR/simplify_cfg.rs:7:9: 9:10
46 -     }
48 -     bb6: {


- -         unreachable;                     // scope 0 at $DIR/simplify_cfg.rs:7:18: 9:10
+ -         unreachable;                     // scope 0 at $DIR/simplify_cfg.rs:8:13: 8:18
50 -     }
52 -     bb7: {


- -         goto -> bb8;                     // scope 0 at $DIR/simplify_cfg.rs:7:9: 9:10
+           StorageDead(_4);                 // scope 0 at $DIR/simplify_cfg.rs:8:18: 8:19
+ -         unreachable;                     // scope 0 at $DIR/simplify_cfg.rs:7:18: 9:10
54 -     }
56 -     bb8: {


+ -         goto -> bb9;                     // scope 0 at $DIR/simplify_cfg.rs:7:9: 9:10
+ -     }
+ -     bb9: {
+ -     bb9: {
57           StorageDead(_2);                 // scope 0 at $DIR/simplify_cfg.rs:9:9: 9:10
58 -         goto -> bb1;                     // scope 0 at $DIR/simplify_cfg.rs:6:5: 10:6
- +         goto -> bb0;                     // scope 0 at $DIR/simplify_cfg.rs:6:5: 10:6
+ +         return;                          // scope 0 at $DIR/simplify_cfg.rs:11:2: 11:2
61   
- -     bb9: {
- -     bb9: {
- -         StorageDead(_2);                 // scope 0 at $DIR/simplify_cfg.rs:9:9: 9:10
+ -     bb10: {
+ -         StorageDead(_4);                 // scope 0 at $DIR/simplify_cfg.rs:8:18: 8:19
+ +     bb4: {
+ +         _1 = const ();                   // scope 0 at $DIR/simplify_cfg.rs:9:10: 9:10
+           StorageDead(_2);                 // scope 0 at $DIR/simplify_cfg.rs:9:9: 9:10
64 -         return;                          // scope 0 at $DIR/simplify_cfg.rs:11:2: 11:2
- -     }
- - 
- -     bb10 (cleanup): {
+ +         goto -> bb0;                     // scope 0 at $DIR/simplify_cfg.rs:6:5: 10:6
+   
+   
+ -     bb11 (cleanup): {
68 +     bb5 (cleanup): {
69           resume;                          // scope 0 at $DIR/simplify_cfg.rs:5:1: 11:2


thread '[mir-opt] mir-opt/simplify_cfg.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_cfg.main.SimplifyCfg-initial.diff', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/uninhabited-enum.rs stdout ----
---- [mir-opt] mir-opt/uninhabited-enum.rs stdout ----
3 fn process_never(_1: *const !) -> () {
4     debug input => _1;                   // in scope 0 at $DIR/uninhabited-enum.rs:7:22: 7:27
5     let mut _0: ();                      // return place in scope 0 at $DIR/uninhabited-enum.rs:7:39: 7:39
-     let _2: &!;                          // in scope 0 at $DIR/uninhabited-enum.rs:8:8: 8:14
+     let _2: &();                         // in scope 0 at $DIR/uninhabited-enum.rs:8:8: 8:14
7     scope 1 {
8         debug _input => _2;              // in scope 1 at $DIR/uninhabited-enum.rs:8:8: 8:14

12 
13     bb0: {
13     bb0: {
14         StorageLive(_2);                 // scope 0 at $DIR/uninhabited-enum.rs:8:8: 8:14
-         _2 = &(*_1);                     // scope 2 at $DIR/uninhabited-enum.rs:8:26: 8:33
-         StorageDead(_2);                 // scope 0 at $DIR/uninhabited-enum.rs:9:1: 9:2
-         unreachable;                     // scope 0 at $DIR/uninhabited-enum.rs:7:39: 9:2
+         unreachable;                     // scope 2 at $DIR/uninhabited-enum.rs:8:27: 8:33
19 }
20 


thread '[mir-opt] mir-opt/uninhabited-enum.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/uninhabited_enum.process_never.SimplifyLocals.after.mir', src/tools/compiletest/src/runtest.rs:3554:25
---- [mir-opt] mir-opt/while_let_loops.rs stdout ----
---- [mir-opt] mir-opt/while_let_loops.rs stdout ----
8       let mut _3: std::option::Option<u32>; // in scope 0 at $DIR/while_let_loops.rs:7:28: 7:32
9       let mut _4: isize;                   // in scope 0 at $DIR/while_let_loops.rs:7:15: 7:25
10       let mut _5: !;                       // in scope 0 at $DIR/while_let_loops.rs:7:33: 10:6
-       let mut _6: !;                       // in scope 0 at $DIR/while_let_loops.rs:7:5: 10:6
+       let _6: ();                          // in scope 0 at $DIR/while_let_loops.rs:9:9: 9:14
+       let mut _7: !;                       // in scope 0 at $DIR/while_let_loops.rs:9:9: 9:14
+       let mut _8: !;                       // in scope 0 at $DIR/while_let_loops.rs:7:5: 10:6
12       scope 1 {
13           debug _x => _1;                  // in scope 1 at $DIR/while_let_loops.rs:6:9: 6:15

35   
36       bb3: {
36       bb3: {
37           _1 = const 1_i32;                // scope 1 at $DIR/while_let_loops.rs:8:9: 8:15
+           StorageLive(_6);                 // scope 1 at $DIR/while_let_loops.rs:9:9: 9:14
38           nop;                             // scope 1 at $DIR/while_let_loops.rs:9:9: 9:14
-           goto -> bb4;                     // scope 1 at $DIR/while_let_loops.rs:9:9: 9:14
+           StorageDead(_6);                 // scope 1 at $DIR/while_let_loops.rs:9:14: 9:15
+           goto -> bb4;                     // scope 1 at $DIR/while_let_loops.rs:1:1: 1:1
41   
42       bb4: {


thread '[mir-opt] mir-opt/while_let_loops.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/while_let_loops.change_loop_body.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3554:25

failures:
    [mir-opt] mir-opt/inline/inline-diverging.rs
    [mir-opt] mir-opt/issue-38669.rs
---
test result: FAILED. 149 passed; 8 failed; 3 ignored; 0 measured; 0 filtered out; finished in 2.05s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:53
