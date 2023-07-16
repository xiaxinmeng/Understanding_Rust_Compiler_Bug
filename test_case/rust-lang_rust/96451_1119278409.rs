plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 174 tests
......................................i...................................i............. 88/174
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
..i............................F.....................i............F...................

---- [mir-opt] src/test/mir-opt/issue-73223.rs stdout ----
---- [mir-opt] src/test/mir-opt/issue-73223.rs stdout ----
7       let mut _2: std::option::Option<i32>; // in scope 0 at $DIR/issue-73223.rs:2:23: 2:30
8       let mut _4: (&i32, &i32);            // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
9       let mut _5: &i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _6: &i32;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
11       let mut _7: bool;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
12       let mut _8: bool;                    // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
13       let mut _9: i32;                     // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

-       let mut _11: &i32;                   // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-       let mut _12: std::option::Option<std::fmt::Arguments>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+       let mut _11: std::option::Option<std::fmt::Arguments>; // in scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
16       scope 1 {
17           debug split => _1;               // in scope 1 at $DIR/issue-73223.rs:2:9: 2:14
18           let _3: std::option::Option<i32>; // in scope 1 at $DIR/issue-73223.rs:7:9: 7:14
19           scope 3 {
19           scope 3 {
20               debug _prev => _3;           // in scope 3 at $DIR/issue-73223.rs:7:9: 7:14
+               let _6: &i32;                // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+               let mut _12: &i32;           // in scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
21               scope 4 {
-                   debug left_val => _11;   // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                   debug left_val => _6;    // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
23                   debug right_val => (_4.1: &i32); // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
24                   let _10: core::panicking::AssertKind; // in scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
25                   scope 5 {

41           ((_3 as Some).0: i32) = _1;      // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
42           StorageLive(_5);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
43           _5 = &_1;                        // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _6 = const main::promoted[0];    // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _12 = const main::promoted[0];   // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
45                                            // mir::Constant
46                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
47                                            // + literal: Const { ty: &i32, val: Unevaluated(main, [], Some(promoted[0])) }

48           Deinit(_4);                      // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
49           (_4.0: &i32) = move _5;          // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           (_4.1: &i32) = move _6;          // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           (_4.1: &i32) = move _12;         // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
51           StorageDead(_5);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           _11 = (_4.0: &i32);              // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _6 = (_4.0: &i32);               // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
53           StorageLive(_7);                 // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
54           StorageLive(_8);                 // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
55           StorageLive(_9);                 // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

-           _9 = (*_11);                     // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _9 = (*_6);                      // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
57           _8 = Eq(move _9, const 1_i32);   // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
58           StorageDead(_9);                 // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
59           _7 = Not(move _8);               // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

65           StorageLive(_10);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
66           Deinit(_10);                     // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
67           discriminant(_10) = 0;           // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           StorageLive(_12);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           Deinit(_12);                     // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           discriminant(_12) = 0;           // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _11, move (_4.1: &i32), move _12); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           StorageLive(_11);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           Deinit(_11);                     // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           discriminant(_11) = 0;           // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _6, move (_4.1: &i32), move _11); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
72                                            // mir::Constant
73                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
74                                            // + literal: Const { ty: for<'r, 's, 't0> fn(core::panicking::AssertKind, &'r i32, &'s i32, Option<Arguments<'t0>>) -> ! {core::panicking::assert_failed::<i32, i32>}, val: Value(Scalar(<ZST>)) }

thread '[mir-opt] src/test/mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.PreCodegen.32bit.diff', src/tools/compiletest/src/runtest.rs:3410:25

---- [mir-opt] src/test/mir-opt/simplify-locals-removes-unused-discriminant-reads.rs stdout ----
---- [mir-opt] src/test/mir-opt/simplify-locals-removes-unused-discriminant-reads.rs stdout ----
5       debug x => _1;                       // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:3:8: 3:9
6       let mut _0: std::option::Option<std::boxed::Box<()>>; // return place in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:3:31: 3:46
7       let mut _2: isize;                   // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:5:9: 5:13
- -     let _3: std::boxed::Box<()>;         // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:14: 6:15
+       let _3: std::boxed::Box<()>;         // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:14: 6:15
9 -     let mut _4: std::boxed::Box<()>;     // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:25: 6:26
10 -     let mut _5: bool;                    // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:8:1: 8:2
11 -     let mut _6: isize;                   // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:8:1: 8:2

12 -     let mut _7: isize;                   // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:8:1: 8:2
- +     let mut _3: std::boxed::Box<()>;     // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:25: 6:26
14       scope 1 {
- -         debug x => _4;                   // in scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:14: 6:15
- +         debug x => _3;                   // in scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:14: 6:15
+           debug x => _3;                   // in scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:14: 6:15
18   
19       bb0: {

24       }
24       }
25   
26       bb1: {
- -         _4 = move ((_1 as Some).0: std::boxed::Box<()>); // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:14: 6:15
- +         _3 = move ((_1 as Some).0: std::boxed::Box<()>); // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:14: 6:15
+           _3 = move ((_1 as Some).0: std::boxed::Box<()>); // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:14: 6:15
29           Deinit(_0);                      // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:20: 6:27
- -         ((_0 as Some).0: std::boxed::Box<()>) = move _4; // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:20: 6:27
- +         ((_0 as Some).0: std::boxed::Box<()>) = move _3; // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:20: 6:27
+           ((_0 as Some).0: std::boxed::Box<()>) = move _3; // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:20: 6:27
32           discriminant(_0) = 1;            // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:20: 6:27
33           goto -> bb3;                     // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:26: 6:27


thread '[mir-opt] src/test/mir-opt/simplify-locals-removes-unused-discriminant-reads.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_locals_removes_unused_discriminant_reads.map.SimplifyLocals.32bit.diff', src/tools/compiletest/src/runtest.rs:3410:25

failures:
    [mir-opt] src/test/mir-opt/issue-73223.rs
    [mir-opt] src/test/mir-opt/simplify-locals-removes-unused-discriminant-reads.rs
