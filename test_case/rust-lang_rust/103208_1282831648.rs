plain
 finished in 0.625 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 188 tests
.......................................i...............................F................ 88/188
..........i.............FF.F................F.F.....F.F..ii.....F.i...F................. 176/188
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
......F.....

---- [mir-opt] src/test/mir-opt/exponential-or.rs stdout ----
18     }
19 
19 
20     bb0: {
-         FakeRead(ForMatchedPlace(None), _1); // scope 0 at $DIR/exponential-or.rs:+1:11: +1:12
+         FakeRead(ForWildcard, _1);       // scope 0 at $DIR/exponential-or.rs:+1:11: +1:12
22         switchInt((_1.0: u32)) -> [1_u32: bb2, 4_u32: bb2, otherwise: bb1]; // scope 0 at $DIR/exponential-or.rs:+2:15: +2:20
24 


thread '[mir-opt] src/test/mir-opt/exponential-or.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/exponential_or.match_tuple.SimplifyCfg-initial.after.mir', src/tools/compiletest/src/runtest.rs:3515:25

---- [mir-opt] src/test/mir-opt/issue-101867.rs stdout ----
---- [mir-opt] src/test/mir-opt/issue-101867.rs stdout ----
25         FakeRead(ForLet(None), _1);      // scope 0 at $DIR/issue-101867.rs:+1:9: +1:10
26         AscribeUserType(_1, o, UserTypeProjection { base: UserType(1), projs: [] }); // scope 0 at $DIR/issue-101867.rs:+1:12: +1:22
27         StorageLive(_5);                 // scope 1 at $DIR/issue-101867.rs:+2:14: +2:15
-         FakeRead(ForMatchedPlace(None), _1); // scope 1 at $DIR/issue-101867.rs:+2:19: +2:20
+         FakeRead(ForWildcard, _1);       // scope 1 at $DIR/issue-101867.rs:+2:19: +2:20
29         _6 = discriminant(_1);           // scope 1 at $DIR/issue-101867.rs:+2:19: +2:20
30         switchInt(move _6) -> [1_isize: bb4, otherwise: bb3]; // scope 1 at $DIR/issue-101867.rs:+2:9: +2:16


thread '[mir-opt] src/test/mir-opt/issue-101867.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_101867.main.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3515:25
---- [mir-opt] src/test/mir-opt/issue-49232.rs stdout ----
---- [mir-opt] src/test/mir-opt/issue-49232.rs stdout ----
24         StorageLive(_2);                 // scope 0 at $DIR/issue-49232.rs:+2:13: +2:19
25         StorageLive(_3);                 // scope 0 at $DIR/issue-49232.rs:+3:19: +3:23
26         _3 = const true;                 // scope 0 at $DIR/issue-49232.rs:+3:19: +3:23
-         FakeRead(ForMatchedPlace(None), _3); // scope 0 at $DIR/issue-49232.rs:+3:19: +3:23
+         FakeRead(ForWildcard, _3);       // scope 0 at $DIR/issue-49232.rs:+3:19: +3:23
28         switchInt(_3) -> [false: bb3, otherwise: bb4]; // scope 0 at $DIR/issue-49232.rs:+3:13: +3:23
30 


thread '[mir-opt] src/test/mir-opt/issue-49232.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_49232.main.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3515:25
---- [mir-opt] src/test/mir-opt/issue-72181-1.rs stdout ----
9     bb0: {
9     bb0: {
10         StorageLive(_2);                 // scope 0 at $DIR/issue-72181-1.rs:+0:20: +2:2
11         StorageLive(_3);                 // scope 0 at $DIR/issue-72181-1.rs:+1:5: +1:15
+         FakeRead(ForWildcard, _1);       // scope 0 at $DIR/issue-72181-1.rs:+1:11: +1:12
12         FakeRead(ForMatchedPlace(None), _1); // scope 0 at $DIR/issue-72181-1.rs:+1:11: +1:12
13         unreachable;                     // scope 0 at $DIR/issue-72181-1.rs:+1:11: +1:12


thread '[mir-opt] src/test/mir-opt/issue-72181-1.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_72181_1.f.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3515:25
---- [mir-opt] src/test/mir-opt/match_test.rs stdout ----
---- [mir-opt] src/test/mir-opt/match_test.rs stdout ----
26         _2 = const true;                 // scope 1 at $DIR/match_test.rs:+2:13: +2:17
27         FakeRead(ForLet(None), _2);      // scope 1 at $DIR/match_test.rs:+2:9: +2:10
28         StorageLive(_3);                 // scope 2 at $DIR/match_test.rs:+6:5: +11:6
-         FakeRead(ForMatchedPlace(None), _1); // scope 2 at $DIR/match_test.rs:+6:11: +6:12
+         FakeRead(ForWildcard, _1);       // scope 2 at $DIR/match_test.rs:+6:11: +6:12
30         _6 = Le(const 0_i32, _1);        // scope 2 at $DIR/match_test.rs:+7:9: +7:14
31         switchInt(move _6) -> [false: bb4, otherwise: bb1]; // scope 2 at $DIR/match_test.rs:+7:9: +7:14


thread '[mir-opt] src/test/mir-opt/match_test.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/match_test.main.SimplifyCfg-initial.after.mir', src/tools/compiletest/src/runtest.rs:3515:25
---- [mir-opt] src/test/mir-opt/issue-99325.rs stdout ----
---- [mir-opt] src/test/mir-opt/issue-99325.rs stdout ----
87         _2 = (move _3, move _5);         // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
88         StorageDead(_5);                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
89         StorageDead(_3);                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         FakeRead(ForMatchedPlace(None), _2); // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         FakeRead(ForWildcard, _2);       // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
91         StorageLive(_8);                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
92         _8 = (_2.0: &&[u8]);             // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
93         StorageLive(_9);                 // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

196         _24 = (move _25, move _27);      // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
197         StorageDead(_27);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
198         StorageDead(_25);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-         FakeRead(ForMatchedPlace(None), _24); // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+         FakeRead(ForWildcard, _24);      // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
200         StorageLive(_29);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
201         _29 = (_24.0: &&[u8]);           // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
202         StorageLive(_30);                // scope 0 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

thread '[mir-opt] src/test/mir-opt/issue-99325.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_99325.main.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3515:25
---- [mir-opt] src/test/mir-opt/match_false_edges.rs stdout ----
---- [mir-opt] src/test/mir-opt/match_false_edges.rs stdout ----
27         StorageLive(_1);                 // scope 0 at $DIR/match_false_edges.rs:+1:13: +5:6
28         StorageLive(_2);                 // scope 0 at $DIR/match_false_edges.rs:+1:19: +1:27
29         _2 = Option::<i32>::Some(const 42_i32); // scope 0 at $DIR/match_false_edges.rs:+1:19: +1:27
-         FakeRead(ForMatchedPlace(None), _2); // scope 0 at $DIR/match_false_edges.rs:+1:19: +1:27
+         FakeRead(ForWildcard, _2);       // scope 0 at $DIR/match_false_edges.rs:+1:19: +1:27
31         _3 = discriminant(_2);           // scope 0 at $DIR/match_false_edges.rs:+1:19: +1:27
32         switchInt(move _3) -> [0_isize: bb1, 1_isize: bb2, otherwise: bb4]; // scope 0 at $DIR/match_false_edges.rs:+1:13: +1:27

46     }
47 
48     bb4: {
48     bb4: {
+         FakeRead(ForMatchedPlace(None), _2); // scope 0 at $DIR/match_false_edges.rs:+1:19: +1:27
49         unreachable;                     // scope 0 at $DIR/match_false_edges.rs:+1:19: +1:27
51 


thread '[mir-opt] src/test/mir-opt/match_false_edges.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/match_false_edges.full_tested_match.PromoteTemps.after.mir', src/tools/compiletest/src/runtest.rs:3515:25
---- [mir-opt] src/test/mir-opt/match-arm-scopes.rs stdout ----
31       }
32   
33       bb0: {
33       bb0: {
- -         FakeRead(ForMatchedPlace(None), _2); // scope 0 at $DIR/match-arm-scopes.rs:+1:11: +1:16
+ -         FakeRead(ForWildcard, _2);       // scope 0 at $DIR/match-arm-scopes.rs:+1:11: +1:16
35 -         switchInt((_2.0: bool)) -> [false: bb1, otherwise: bb2]; // scope 0 at $DIR/match-arm-scopes.rs:+1:5: +1:16
36 +         switchInt((_2.0: bool)) -> [false: bb5, otherwise: bb1]; // scope 0 at $DIR/match-arm-scopes.rs:+1:5: +1:16


thread '[mir-opt] src/test/mir-opt/match-arm-scopes.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/match_arm_scopes.complicated_match.SimplifyCfg-initial.after-ElaborateDrops.after.diff', src/tools/compiletest/src/runtest.rs:3515:25
---- [mir-opt] src/test/mir-opt/remove_fake_borrows.rs stdout ----
---- [mir-opt] src/test/mir-opt/remove_fake_borrows.rs stdout ----
13       let mut _8: bool;                    // in scope 0 at $DIR/remove_fake_borrows.rs:+2:20: +2:21
15       bb0: {
15       bb0: {
- -         FakeRead(ForMatchedPlace(None), _1); // scope 0 at $DIR/remove_fake_borrows.rs:+1:11: +1:12
+ -         FakeRead(ForWildcard, _1);       // scope 0 at $DIR/remove_fake_borrows.rs:+1:11: +1:12
17 +         nop;                             // scope 0 at $DIR/remove_fake_borrows.rs:+1:11: +1:12
18           _3 = discriminant(_1);           // scope 0 at $DIR/remove_fake_borrows.rs:+1:11: +1:12
19           switchInt(move _3) -> [1_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/remove_fake_borrows.rs:+1:5: +1:12

thread '[mir-opt] src/test/mir-opt/remove_fake_borrows.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/remove_fake_borrows.match_guard.CleanupNonCodegenStatements.diff', src/tools/compiletest/src/runtest.rs:3515:25
---- [mir-opt] src/test/mir-opt/simple-match.rs stdout ----
---- [mir-opt] src/test/mir-opt/simple-match.rs stdout ----
5     let mut _0: usize;                   // return place in scope 0 at $DIR/simple-match.rs:+0:27: +0:32
7     bb0: {
7     bb0: {
-         FakeRead(ForMatchedPlace(None), _1); // scope 0 at $DIR/simple-match.rs:+1:11: +1:12
+         FakeRead(ForWildcard, _1);       // scope 0 at $DIR/simple-match.rs:+1:11: +1:12
9         switchInt(_1) -> [false: bb2, otherwise: bb1]; // scope 0 at $DIR/simple-match.rs:+1:5: +1:12
11 


thread '[mir-opt] src/test/mir-opt/simple-match.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simple_match.match_bool.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3515:25
---- [mir-opt] src/test/mir-opt/uniform_array_move_out.rs stdout ----
77     bb6: {
77     bb6: {
78         StorageDead(_2);                 // scope 0 at $DIR/uniform_array_move_out.rs:+1:26: +1:27
79         FakeRead(ForLet(None), _1);      // scope 0 at $DIR/uniform_array_move_out.rs:+1:9: +1:10
+         FakeRead(ForWildcard, _1);       // scope 1 at $DIR/uniform_array_move_out.rs:+2:9: +2:17
80         StorageLive(_12);                // scope 1 at $DIR/uniform_array_move_out.rs:+2:14: +2:16
81         _12 = move _1[1 of 2];           // scope 1 at $DIR/uniform_array_move_out.rs:+2:14: +2:16
82         _0 = const ();                   // scope 0 at $DIR/uniform_array_move_out.rs:+0:24: +3:2

thread '[mir-opt] src/test/mir-opt/uniform_array_move_out.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/uniform_array_move_out.move_out_from_end.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3515:25

failures:
    [mir-opt] src/test/mir-opt/exponential-or.rs
    [mir-opt] src/test/mir-opt/issue-101867.rs
