plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 174 tests
......................................i...................................i............. 88/174
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
..i.............................F....................i..........F.....................

---- [mir-opt] src/test/mir-opt/issue-73223.rs stdout ----
---- [mir-opt] src/test/mir-opt/issue-73223.rs stdout ----
5       let mut _0: ();                      // return place in scope 0 at $DIR/issue-73223.rs:1:11: 1:11
6       let _1: i32;                         // in scope 0 at $DIR/issue-73223.rs:2:9: 2:14
7       let mut _2: std::option::Option<i32>; // in scope 0 at $DIR/issue-73223.rs:2:23: 2:30
-       let _3: i32;                         // in scope 0 at $DIR/issue-73223.rs:3:14: 3:15
-       let mut _5: i32;                     // in scope 0 at $DIR/issue-73223.rs:7:22: 7:27
-       let mut _6: (&i32, &i32);            // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _7: &i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _8: &i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _11: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _12: bool;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _13: i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _15: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let _16: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _17: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let _18: &i32;                       // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _19: std::option::Option<std::fmt::Arguments>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _4: (&i32, &i32);            // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _5: &i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _6: &i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _7: bool;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _8: bool;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _9: i32;                     // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _11: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _12: std::option::Option<std::fmt::Arguments>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
21       scope 1 {
22           debug split => _1;               // in scope 1 at $DIR/issue-73223.rs:2:9: 2:14
-           let _4: std::option::Option<i32>; // in scope 1 at $DIR/issue-73223.rs:7:9: 7:14
+           let _3: std::option::Option<i32>; // in scope 1 at $DIR/issue-73223.rs:7:9: 7:14
24           scope 3 {
-               debug _prev => _4;           // in scope 3 at $DIR/issue-73223.rs:7:9: 7:14
-               let _9: &i32;                // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-               let _10: &i32;               // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-               let mut _20: &i32;           // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+               debug _prev => _3;           // in scope 3 at $DIR/issue-73223.rs:7:9: 7:14
29               scope 4 {
-                   debug left_val => _9;    // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                   debug right_val => _10;  // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-                   let _14: core::panicking::AssertKind; // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                   debug left_val => _11;   // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                   debug right_val => (_4.1: &i32); // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                   let _10: core::panicking::AssertKind; // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
33                   scope 5 {
-                       debug kind => _14;   // in scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                       debug kind => _10;   // in scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
36               }
37           }

38       }
38       }
39       scope 2 {
-           debug v => _3;                   // in scope 2 at $DIR/issue-73223.rs:3:14: 3:15
+           debug v => ((_2 as Some).0: i32); // in scope 2 at $DIR/issue-73223.rs:3:14: 3:15
42   
43       bb0: {


44           StorageLive(_1);                 // scope 0 at $DIR/issue-73223.rs:2:9: 2:14
-           StorageLive(_2);                 // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
46           Deinit(_2);                      // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
47           ((_2 as Some).0: i32) = const 1_i32; // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
48           discriminant(_2) = 1;            // scope 0 at $DIR/issue-73223.rs:2:23: 2:30

-           StorageLive(_3);                 // scope 0 at $DIR/issue-73223.rs:3:14: 3:15
-           _3 = ((_2 as Some).0: i32);      // scope 0 at $DIR/issue-73223.rs:3:14: 3:15
-           _1 = _3;                         // scope 2 at $DIR/issue-73223.rs:3:20: 3:21
-           StorageDead(_3);                 // scope 0 at $DIR/issue-73223.rs:3:20: 3:21
-           StorageDead(_2);                 // scope 0 at $DIR/issue-73223.rs:5:6: 5:7
-           StorageLive(_4);                 // scope 1 at $DIR/issue-73223.rs:7:9: 7:14
-           StorageLive(_5);                 // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
-           _5 = _1;                         // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
-           Deinit(_4);                      // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
-           ((_4 as Some).0: i32) = move _5; // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
-           discriminant(_4) = 1;            // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
-           StorageDead(_5);                 // scope 1 at $DIR/issue-73223.rs:7:27: 7:28
-           StorageLive(_6);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_7);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _7 = &_1;                        // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _20 = const main::promoted[0];   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _1 = ((_2 as Some).0: i32);      // scope 2 at $DIR/issue-73223.rs:3:20: 3:21
+           ((_3 as Some).0: i32) = _1;      // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
+           StorageLive(_5);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _5 = &_1;                        // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _6 = const main::promoted[0];    // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
66                                            // mir::Constant
67                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
68                                            // + literal: Const { ty: &i32, val: Unevaluated(main, [], Some(promoted[0])) }

-           _8 = _20;                        // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           Deinit(_6);                      // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           (_6.0: &i32) = move _7;          // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           (_6.1: &i32) = move _8;          // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_7);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_9);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _9 = (_6.0: &i32);               // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_10);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _10 = (_6.1: &i32);              // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_11);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_12);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_13);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _13 = (*_9);                     // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _12 = Eq(move _13, const 1_i32); // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_13);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _11 = Not(move _12);             // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_12);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           switchInt(move _11) -> [false: bb2, otherwise: bb1]; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           Deinit(_4);                      // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           (_4.0: &i32) = move _5;          // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           (_4.1: &i32) = move _6;          // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_5);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _11 = (_4.0: &i32);              // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_7);                 // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_8);                 // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_9);                 // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _9 = (*_11);                     // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _8 = Eq(move _9, const 1_i32);   // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_9);                 // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _7 = Not(move _8);               // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageDead(_8);                 // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           switchInt(move _7) -> [false: bb2, otherwise: bb1]; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
89   
90       bb1: {


-           StorageLive(_14);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           Deinit(_14);                     // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           discriminant(_14) = 0;           // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_15);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_16);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _16 = _9;                        // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _15 = _16;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_17);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_18);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _18 = _10;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _17 = _18;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_19);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           Deinit(_19);                     // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           discriminant(_19) = 0;           // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _15, move _17, move _19); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_10);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           Deinit(_10);                     // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           discriminant(_10) = 0;           // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_12);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           Deinit(_12);                     // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           discriminant(_12) = 0;           // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _11, move (_4.1: &i32), move _12); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
106                                            // mir::Constant
107                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
108                                            // + literal: Const { ty: for<'r, 's, 't0> fn(core::panicking::AssertKind, &'r i32, &'s i32, Option<Arguments<'t0>>) -> ! {core::panicking::assert_failed::<i32, i32>}, val: Value(Scalar(<ZST>)) }
112       }
113   
114       bb2: {
114       bb2: {
-           StorageDead(_11);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_10);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_9);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_6);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageDead(_4);                 // scope 1 at $DIR/issue-73223.rs:9:1: 9:2
+           StorageDead(_7);                 // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
120           StorageDead(_1);                 // scope 0 at $DIR/issue-73223.rs:9:1: 9:2
121           return;                          // scope 0 at $DIR/issue-73223.rs:9:2: 9:2


thread '[mir-opt] src/test/mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.PreCodegen.32bit.diff', src/tools/compiletest/src/runtest.rs:3410:25

---- [mir-opt] src/test/mir-opt/simplify-locals-removes-unused-discriminant-reads.rs stdout ----
---- [mir-opt] src/test/mir-opt/simplify-locals-removes-unused-discriminant-reads.rs stdout ----
10 -     let mut _5: bool;                    // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:8:1: 8:2
11 -     let mut _6: isize;                   // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:8:1: 8:2
12 -     let mut _7: isize;                   // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:8:1: 8:2
+ +     let mut _3: std::boxed::Box<()>;     // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:25: 6:26
13       scope 1 {
-           debug x => ((_0 as Some).0: std::boxed::Box<()>); // in scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:14: 6:15
+ -         debug x => _4;                   // in scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:14: 6:15
+ +         debug x => _3;                   // in scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:14: 6:15
16   
17       bb0: {

22       }
22       }
23   
24       bb1: {
-           ((_0 as Some).0: std::boxed::Box<()>) = move ((_1 as Some).0: std::boxed::Box<()>); // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:14: 6:15
+ -         _4 = move ((_1 as Some).0: std::boxed::Box<()>); // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:14: 6:15
+ +         _3 = move ((_1 as Some).0: std::boxed::Box<()>); // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:14: 6:15
26           Deinit(_0);                      // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:20: 6:27
+ -         ((_0 as Some).0: std::boxed::Box<()>) = move _4; // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:20: 6:27
+ +         ((_0 as Some).0: std::boxed::Box<()>) = move _3; // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:20: 6:27
27           discriminant(_0) = 1;            // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:20: 6:27
28           goto -> bb3;                     // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:26: 6:27


thread '[mir-opt] src/test/mir-opt/simplify-locals-removes-unused-discriminant-reads.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_locals_removes_unused_discriminant_reads.map.SimplifyLocals.32bit.diff', src/tools/compiletest/src/runtest.rs:3410:25

failures:
    [mir-opt] src/test/mir-opt/issue-73223.rs
    [mir-opt] src/test/mir-opt/simplify-locals-removes-unused-discriminant-reads.rs
