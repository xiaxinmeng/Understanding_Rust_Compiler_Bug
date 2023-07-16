plain
...F.......................i................................
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
failures:

---- [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs stdout ----
9       let _5: usize;                       // in scope 0 at $DIR/optimizes_into_variable.rs:13:32: 13:33
10       let mut _6: usize;                   // in scope 0 at $DIR/optimizes_into_variable.rs:13:13: 13:34
11       let mut _7: bool;                    // in scope 0 at $DIR/optimizes_into_variable.rs:13:13: 13:34
-       let mut _9: Point;                   // in scope 0 at $DIR/optimizes_into_variable.rs:14:13: 14:36
+       let mut _9: u32;                     // in scope 0 at $DIR/optimizes_into_variable.rs:14:13: 14:36
13       scope 1 {
14           debug x => _1;                   // in scope 1 at $DIR/optimizes_into_variable.rs:12:9: 12:10
15           let _3: i32;                     // in scope 1 at $DIR/optimizes_into_variable.rs:13:9: 13:10

54           StorageDead(_5);                 // scope 1 at $DIR/optimizes_into_variable.rs:13:34: 13:35
55           StorageDead(_4);                 // scope 1 at $DIR/optimizes_into_variable.rs:13:34: 13:35
56           StorageLive(_8);                 // scope 2 at $DIR/optimizes_into_variable.rs:14:9: 14:10
-           StorageLive(_9);                 // scope 2 at $DIR/optimizes_into_variable.rs:14:13: 14:36
-           (_9.0: u32) = const 12_u32;      // scope 2 at $DIR/optimizes_into_variable.rs:14:13: 14:36
-           (_9.1: u32) = const 42_u32;      // scope 2 at $DIR/optimizes_into_variable.rs:14:13: 14:36
- -         _8 = (_9.1: u32);                // scope 2 at $DIR/optimizes_into_variable.rs:14:13: 14:38
+           _9 = const 42_u32;               // scope 2 at $DIR/optimizes_into_variable.rs:14:13: 14:36
+ -         _8 = _9;                         // scope 2 at $DIR/optimizes_into_variable.rs:14:13: 14:38
61 +         _8 = const 42_u32;               // scope 2 at $DIR/optimizes_into_variable.rs:14:13: 14:38
-           StorageDead(_9);                 // scope 2 at $DIR/optimizes_into_variable.rs:14:38: 14:39
63           nop;                             // scope 0 at $DIR/optimizes_into_variable.rs:11:11: 15:2
64           StorageDead(_8);                 // scope 2 at $DIR/optimizes_into_variable.rs:15:1: 15:2
65           StorageDead(_3);                 // scope 1 at $DIR/optimizes_into_variable.rs:15:1: 15:2

thread '[mir-opt] mir-opt/const_prop/optimizes_into_variable.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/optimizes_into_variable.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3558:25

---- [mir-opt] mir-opt/issue-73223.rs stdout ----
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
7       let mut _2: std::option::Option<i32>; // in scope 0 at $DIR/issue-73223.rs:2:23: 2:30
8       let mut _3: isize;                   // in scope 0 at $DIR/issue-73223.rs:3:9: 3:16
9       let _4: i32;                         // in scope 0 at $DIR/issue-73223.rs:3:14: 3:15
-       let mut _5: !;                       // in scope 0 at $DIR/issue-73223.rs:4:17: 4:23
-       let mut _7: i32;                     // in scope 0 at $DIR/issue-73223.rs:7:22: 7:27
-       let _8: ();                          // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _9: (&i32, &i32);            // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _10: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _11: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let _12: i32;                        // in scope 0 at $DIR/issue-73223.rs:8:23: 8:24
-       let mut _15: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _16: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _17: i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _18: i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _19: !;                      // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let _21: !;                          // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _22: core::panicking::AssertKind; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _6: i32;                     // in scope 0 at $DIR/issue-73223.rs:7:22: 7:27
+       let mut _7: &i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _8: &i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _11: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _12: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _13: i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _14: i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _16: core::panicking::AssertKind; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _17: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let _18: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _19: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let _20: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _21: std::option::Option<std::fmt::Arguments>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
24       let mut _23: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let _24: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _25: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let _26: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _27: std::option::Option<std::fmt::Arguments>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _24: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
29       scope 1 {
30           debug split => _1;               // in scope 1 at $DIR/issue-73223.rs:2:9: 2:14
-           let _6: std::option::Option<i32>; // in scope 1 at $DIR/issue-73223.rs:7:9: 7:14
+           let _5: std::option::Option<i32>; // in scope 1 at $DIR/issue-73223.rs:7:9: 7:14
32           scope 3 {
-               debug _prev => _6;           // in scope 3 at $DIR/issue-73223.rs:7:9: 7:14
-               let _13: &i32;               // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-               let _14: &i32;               // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-               let mut _28: &i32;           // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+               debug _prev => _5;           // in scope 3 at $DIR/issue-73223.rs:7:9: 7:14
+               let _9: &i32;                // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+               let _10: &i32;               // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+               let mut _22: &i32;           // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
37               scope 4 {
-                   debug left_val => _13;   // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                   debug right_val => _14;  // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                   let _20: core::panicking::AssertKind; // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                   debug left_val => _9;    // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                   debug right_val => _10;  // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                   let _15: core::panicking::AssertKind; // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
41                   scope 5 {
-                       debug kind => _20;   // in scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                       debug kind => _15;   // in scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
44               }
45           }


70           _1 = _4;                         // scope 2 at $DIR/issue-73223.rs:3:20: 3:21
71           StorageDead(_4);                 // scope 0 at $DIR/issue-73223.rs:3:20: 3:21
72           StorageDead(_2);                 // scope 0 at $DIR/issue-73223.rs:5:6: 5:7
-           StorageLive(_6);                 // scope 1 at $DIR/issue-73223.rs:7:9: 7:14
-           StorageLive(_7);                 // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
-           _7 = _1;                         // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
-           ((_6 as Some).0: i32) = move _7; // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
-           discriminant(_6) = 1;            // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
-           StorageDead(_7);                 // scope 1 at $DIR/issue-73223.rs:7:27: 7:28
+           StorageLive(_5);                 // scope 1 at $DIR/issue-73223.rs:7:9: 7:14
+           StorageLive(_6);                 // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
+           _6 = _1;                         // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
+           ((_5 as Some).0: i32) = move _6; // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
+           discriminant(_5) = 1;            // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
+           StorageDead(_6);                 // scope 1 at $DIR/issue-73223.rs:7:27: 7:28
+           StorageLive(_7);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _7 = &_1;                        // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
79           StorageLive(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_9);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_10);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _10 = &_1;                       // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_11);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _28 = const main::promoted[0];   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _22 = const main::promoted[0];   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
85                                            // ty::Const
86                                            // + ty: &i32
87                                            // + val: Unevaluated(main, [], Some(promoted[0]))
88                                            // mir::Constant
88                                            // mir::Constant
89                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
90                                            // + literal: Const { ty: &i32, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:3 ~ issue_73223[317d]::main), const_param_did: None }, substs: [], promoted: Some(promoted[0]) }) }
-           _11 = _28;                       // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           (_9.0: &i32) = move _10;         // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           (_9.1: &i32) = move _11;         // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_11);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_10);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_13);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _13 = (_9.0: &i32);              // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_14);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _14 = (_9.1: &i32);              // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_15);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_16);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_17);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _17 = (*_13);                    // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_18);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _18 = const 1_i32;               // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _16 = Eq(move _17, const 1_i32); // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_18);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_17);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _15 = Not(move _16);             // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_16);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           switchInt(move _15) -> [false: bb4, otherwise: bb3]; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _8 = _22;                        // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _23 = move _7;                   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _24 = move _8;                   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_7);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_9);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _9 = _23;                        // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_10);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _10 = _24;                       // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_11);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_12);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_13);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _13 = (*_9);                     // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_14);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _14 = const 1_i32;               // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _12 = Eq(move _13, const 1_i32); // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_14);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_13);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _11 = Not(move _12);             // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_12);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           switchInt(move _11) -> [false: bb4, otherwise: bb3]; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
113   
114       bb3: {


-           StorageLive(_20);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           discriminant(_20) = 0;           // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_21);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_22);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _22 = const core::panicking::AssertKind::Eq; // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_15);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           discriminant(_15) = 0;           // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_16);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _16 = const core::panicking::AssertKind::Eq; // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
120                                            // ty::Const
121                                            // + ty: core::panicking::AssertKind
122                                            // + val: Value(Scalar(0x00))
123                                            // mir::Constant
123                                            // mir::Constant
124                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
125                                            // + literal: Const { ty: core::panicking::AssertKind, val: Value(Scalar(0x00)) }
-           StorageLive(_23);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_24);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _24 = _13;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _23 = _24;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_25);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_26);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _26 = _14;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _25 = _26;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_27);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           discriminant(_27) = 0;           // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _23, move _25, move _27); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_17);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_18);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _18 = _9;                        // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _17 = _18;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_19);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_20);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _20 = _10;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _19 = _20;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_21);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           discriminant(_21) = 0;           // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _17, move _19, move _21); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
137                                            // mir::Constant
138                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
139                                            // + literal: Const { ty: for<'r, 's, 't0> fn(core::panicking::AssertKind, &'r i32, &'s i32, std::option::Option<std::fmt::Arguments<'t0>>) -> ! {core::panicking::assert_failed::<i32, i32>}, val: Value(Scalar(<ZST>)) }
146       }
147   
148       bb4: {
148       bb4: {
-           nop;                             // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_15);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_14);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_13);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_11);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_10);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
153           StorageDead(_9);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
155           nop;                             // scope 0 at $DIR/issue-73223.rs:1:11: 9:2
-           StorageDead(_6);                 // scope 1 at $DIR/issue-73223.rs:9:1: 9:2
+           StorageDead(_5);                 // scope 1 at $DIR/issue-73223.rs:9:1: 9:2
157           StorageDead(_1);                 // scope 0 at $DIR/issue-73223.rs:9:1: 9:2
158           return;                          // scope 0 at $DIR/issue-73223.rs:9:2: 9:2


thread '[mir-opt] mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.32bit.diff', src/tools/compiletest/src/runtest.rs:3558:25

failures:
    [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs
    [mir-opt] mir-opt/issue-73223.rs
    [mir-opt] mir-opt/issue-73223.rs

test result: FAILED. 154 passed; 2 failed; 4 ignored; 0 measured; 0 filtered out; finished in 2.11s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
Build completed unsuccessfully in 0:01:42
