plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 180 tests
i......................................i........................................i....... 88/180
........i..............................F...................i.F.........F................ 176/180
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
failures:

---- [mir-opt] src/test/mir-opt/issue-73223.rs stdout ----
---- [mir-opt] src/test/mir-opt/issue-73223.rs stdout ----
55           ((_2 as Some).0: i32) = const 1_i32; // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
56           discriminant(_2) = 1;            // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
57           _3 = const 1_isize;              // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
-           goto -> bb2;                     // scope 0 at $DIR/issue-73223.rs:2:17: 2:30
+           goto -> bb3;                     // scope 0 at $DIR/issue-73223.rs:2:17: 2:30
60   
61       bb1: {

66       }
66       }
67   
68       bb2: {
+           unreachable;                     // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
+   
+       bb3: {
+       bb3: {
69           StorageLive(_4);                 // scope 0 at $DIR/issue-73223.rs:3:14: 3:15
70           _4 = ((_2 as Some).0: i32);      // scope 0 at $DIR/issue-73223.rs:3:14: 3:15
71           _1 = _4;                         // scope 2 at $DIR/issue-73223.rs:3:20: 3:21

108           StorageDead(_17);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
109           _15 = Not(move _16);             // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
110           StorageDead(_16);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           switchInt(move _15) -> [false: bb4, otherwise: bb3]; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           switchInt(move _15) -> [false: bb5, otherwise: bb4]; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
113   
-       bb3: {
+       bb4: {
+       bb4: {
115           StorageLive(_20);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
116           Deinit(_20);                     // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
117           discriminant(_20) = 0;           // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

141                                            // + literal: Const { ty: core::panicking::AssertKind, val: Value(Scalar(0x00)) }
143   
-       bb4: {
+       bb5: {
+       bb5: {
145           nop;                             // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
146           StorageDead(_15);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
147           StorageDead(_14);                // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

thread '[mir-opt] src/test/mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.32bit.diff', src/tools/compiletest/src/runtest.rs:3512:25

---- [mir-opt] src/test/mir-opt/matches_u8.rs stdout ----
8   
9       bb0: {
9       bb0: {
10           _2 = discriminant(_1);           // scope 0 at $DIR/matches_u8.rs:12:11: 12:12
-           switchInt(move _2) -> [0_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/matches_u8.rs:12:5: 12:12
+           switchInt(move _2) -> [0_isize: bb3, 1_isize: bb1, otherwise: bb2]; // scope 0 at $DIR/matches_u8.rs:12:5: 12:12
13   
14       bb1: {


15           _0 = const 1_u8;                 // scope 0 at $DIR/matches_u8.rs:14:17: 14:18
-           goto -> bb3;                     // scope 0 at $DIR/matches_u8.rs:14:17: 14:18
+           goto -> bb4;                     // scope 0 at $DIR/matches_u8.rs:14:17: 14:18
18   
19       bb2: {


-           _0 = const 0_u8;                 // scope 0 at $DIR/matches_u8.rs:13:17: 13:18
-           goto -> bb3;                     // scope 0 at $DIR/matches_u8.rs:13:17: 13:18
+           unreachable;                     // scope 0 at $DIR/matches_u8.rs:12:11: 12:12
23   
24       bb3: {


+           _0 = const 0_u8;                 // scope 0 at $DIR/matches_u8.rs:13:17: 13:18
+           goto -> bb4;                     // scope 0 at $DIR/matches_u8.rs:13:17: 13:18
+   
+       bb4: {
+       bb4: {
25           return;                          // scope 0 at $DIR/matches_u8.rs:16:2: 16:2
27   }


thread '[mir-opt] src/test/mir-opt/matches_u8.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/matches_u8.exhaustive_match.MatchBranchSimplification.32bit.diff', src/tools/compiletest/src/runtest.rs:3512:25
---- [mir-opt] src/test/mir-opt/simplify-locals-removes-unused-discriminant-reads.rs stdout ----
16   
17       bb0: {
17       bb0: {
18           _2 = discriminant(_1);           // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:4:11: 4:12
-           switchInt(move _2) -> [0_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:4:5: 4:12
+           switchInt(move _2) -> [0_isize: bb3, 1_isize: bb1, otherwise: bb2]; // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:4:5: 4:12
21   
22       bb1: {


23           ((_0 as Some).0: std::boxed::Box<()>) = move ((_1 as Some).0: std::boxed::Box<()>); // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:14: 6:15
24           Deinit(_0);                      // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:20: 6:27
25           discriminant(_0) = 1;            // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:20: 6:27
-           goto -> bb3;                     // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:26: 6:27
+           goto -> bb4;                     // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:26: 6:27
28   
29       bb2: {


+           unreachable;                     // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:4:11: 4:12
+   
+       bb3: {
+       bb3: {
30           Deinit(_0);                      // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:5:17: 5:21
31           discriminant(_0) = 0;            // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:5:17: 5:21
-           goto -> bb3;                     // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:5:17: 5:21
+           goto -> bb4;                     // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:5:17: 5:21
34   
-       bb3: {
+       bb4: {
+       bb4: {
36           return;                          // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:8:2: 8:2
38   }


thread '[mir-opt] src/test/mir-opt/simplify-locals-removes-unused-discriminant-reads.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_locals_removes_unused_discriminant_reads.map.SimplifyLocals.32bit.diff', src/tools/compiletest/src/runtest.rs:3512:25

failures:
    [mir-opt] src/test/mir-opt/issue-73223.rs
    [mir-opt] src/test/mir-opt/matches_u8.rs
