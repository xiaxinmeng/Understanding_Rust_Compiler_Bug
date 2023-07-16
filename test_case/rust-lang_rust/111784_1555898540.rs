plain
 finished in 0.586 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 255 tests
...................F.............F.............F.....F..........................iFF.FFFF  88/255
FFFF.FFFF....F...........................F..............Fi......F....iii...F..........F. 176/255
...F.F...F..F.....iii.iiiii.........ii...F....i....F..F...F.......F......F.....
failures:

---- [mir-opt] tests/mir-opt/casts.rs stdout ----
---- [mir-opt] tests/mir-opt/casts.rs stdout ----
14           StorageLive(_2);                 // scope 0 at $DIR/casts.rs:+1:5: +1:55
15           StorageLive(_3);                 // scope 0 at $DIR/casts.rs:+1:36: +1:37
16           _3 = _1;                         // scope 0 at $DIR/casts.rs:+1:36: +1:37
- -         _2 = _3 as *const &u8 (PtrToPtr); // scope 1 at $DIR/casts.rs:11:5: 11:18
- +         _2 = _3;                         // scope 1 at $DIR/casts.rs:11:5: 11:18
+ -         _2 = move _3 as *const &u8 (PtrToPtr); // scope 1 at $DIR/casts.rs:11:5: 11:18
+ +         _2 = move _3;                    // scope 1 at $DIR/casts.rs:11:5: 11:18
19           StorageDead(_3);                 // scope 0 at $DIR/casts.rs:+1:37: +1:38
20           _0 = _2;                         // scope 0 at $DIR/casts.rs:+1:5: +1:55
21           StorageDead(_2);                 // scope 0 at $DIR/casts.rs:+2:1: +2:2

thread '[mir-opt] tests/mir-opt/casts.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/casts.redundant.InstSimplify.diff', src/tools/compiletest/src/runtest.rs:3639:21

---- [mir-opt] tests/mir-opt/const_debuginfo.rs stdout ----
56       }
57   
57   
58       bb0: {
+           StorageLive(_1);                 // scope 0 at $DIR/const_debuginfo.rs:+1:9: +1:10
59           _1 = const 1_u8;                 // scope 0 at $DIR/const_debuginfo.rs:+1:13: +1:16
+           StorageLive(_2);                 // scope 1 at $DIR/const_debuginfo.rs:+2:9: +2:10
60           _2 = const 2_u8;                 // scope 1 at $DIR/const_debuginfo.rs:+2:13: +2:16
+           StorageLive(_3);                 // scope 2 at $DIR/const_debuginfo.rs:+3:9: +3:10
61           _3 = const 3_u8;                 // scope 2 at $DIR/const_debuginfo.rs:+3:13: +3:16
62           StorageLive(_4);                 // scope 3 at $DIR/const_debuginfo.rs:+4:9: +4:12
63           StorageLive(_5);                 // scope 3 at $DIR/const_debuginfo.rs:+4:15: +4:20

+           StorageLive(_6);                 // scope 3 at $DIR/const_debuginfo.rs:+4:15: +4:16
+           _6 = const 1_u8;                 // scope 3 at $DIR/const_debuginfo.rs:+4:15: +4:16
+           StorageLive(_7);                 // scope 3 at $DIR/const_debuginfo.rs:+4:19: +4:20
+           _7 = const 2_u8;                 // scope 3 at $DIR/const_debuginfo.rs:+4:19: +4:20
64           _5 = const 3_u8;                 // scope 3 at $DIR/const_debuginfo.rs:+4:15: +4:20
+           StorageDead(_7);                 // scope 3 at $DIR/const_debuginfo.rs:+4:19: +4:20
+           StorageDead(_6);                 // scope 3 at $DIR/const_debuginfo.rs:+4:19: +4:20
+           StorageLive(_8);                 // scope 3 at $DIR/const_debuginfo.rs:+4:23: +4:24
+           _8 = const 3_u8;                 // scope 3 at $DIR/const_debuginfo.rs:+4:23: +4:24
65           _4 = const 6_u8;                 // scope 3 at $DIR/const_debuginfo.rs:+4:15: +4:24
+           StorageDead(_8);                 // scope 3 at $DIR/const_debuginfo.rs:+4:23: +4:24
66           StorageDead(_5);                 // scope 3 at $DIR/const_debuginfo.rs:+4:23: +4:24
67           StorageLive(_9);                 // scope 4 at $DIR/const_debuginfo.rs:+6:9: +6:10
68           _9 = const "hello, world!";      // scope 4 at $DIR/const_debuginfo.rs:+6:13: +6:28

77           _16 = const 123_u32;             // scope 5 at $DIR/const_debuginfo.rs:+8:13: +8:34
78           StorageLive(_10);                // scope 6 at $DIR/const_debuginfo.rs:+10:9: +10:10
79           _10 = Option::<u16>::Some(const 99_u16); // scope 6 at $DIR/const_debuginfo.rs:+10:13: +10:24
+           StorageLive(_17);                // scope 7 at $DIR/const_debuginfo.rs:+12:9: +12:10
+           StorageLive(_18);                // scope 7 at $DIR/const_debuginfo.rs:+12:9: +12:10
80           _17 = const 32_u32;              // scope 7 at $DIR/const_debuginfo.rs:+12:13: +12:35
81           _18 = const 32_u32;              // scope 7 at $DIR/const_debuginfo.rs:+12:13: +12:35
82           StorageLive(_11);                // scope 8 at $DIR/const_debuginfo.rs:+13:9: +13:10

+           StorageLive(_12);                // scope 8 at $DIR/const_debuginfo.rs:+13:13: +13:16
+           _12 = const 32_u32;              // scope 8 at $DIR/const_debuginfo.rs:+13:13: +13:16
+           StorageLive(_13);                // scope 8 at $DIR/const_debuginfo.rs:+13:19: +13:22
+           _13 = const 32_u32;              // scope 8 at $DIR/const_debuginfo.rs:+13:19: +13:22
83           _11 = const 64_u32;              // scope 8 at $DIR/const_debuginfo.rs:+13:13: +13:22
+           StorageDead(_13);                // scope 8 at $DIR/const_debuginfo.rs:+13:21: +13:22
+           StorageDead(_12);                // scope 8 at $DIR/const_debuginfo.rs:+13:21: +13:22
84           StorageDead(_11);                // scope 8 at $DIR/const_debuginfo.rs:+14:1: +14:2
+           StorageDead(_17);                // scope 7 at $DIR/const_debuginfo.rs:+14:1: +14:2
+           StorageDead(_18);                // scope 7 at $DIR/const_debuginfo.rs:+14:1: +14:2
85           StorageDead(_10);                // scope 6 at $DIR/const_debuginfo.rs:+14:1: +14:2
86           StorageDead(_14);                // scope 5 at $DIR/const_debuginfo.rs:+14:1: +14:2
87           StorageDead(_15);                // scope 5 at $DIR/const_debuginfo.rs:+14:1: +14:2

88           StorageDead(_16);                // scope 5 at $DIR/const_debuginfo.rs:+14:1: +14:2
89           StorageDead(_9);                 // scope 4 at $DIR/const_debuginfo.rs:+14:1: +14:2
90           StorageDead(_4);                 // scope 3 at $DIR/const_debuginfo.rs:+14:1: +14:2
+           StorageDead(_3);                 // scope 2 at $DIR/const_debuginfo.rs:+14:1: +14:2
+           StorageDead(_2);                 // scope 1 at $DIR/const_debuginfo.rs:+14:1: +14:2
+           StorageDead(_1);                 // scope 0 at $DIR/const_debuginfo.rs:+14:1: +14:2
91           return;                          // scope 0 at $DIR/const_debuginfo.rs:+14:2: +14:2
93   }


thread '[mir-opt] tests/mir-opt/const_debuginfo.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_debuginfo.main.ConstDebugInfo.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/const_prop/boolean_identities.rs stdout ----
12   
13       bb0: {
13       bb0: {
14           StorageLive(_3);                 // scope 0 at $DIR/boolean_identities.rs:+1:5: +1:15
- -         _3 = BitOr(_2, const true);      // scope 0 at $DIR/boolean_identities.rs:+1:5: +1:15
+           StorageLive(_4);                 // scope 0 at $DIR/boolean_identities.rs:+1:6: +1:7
+           _4 = _2;                         // scope 0 at $DIR/boolean_identities.rs:+1:6: +1:7
+ -         _3 = BitOr(move _4, const true); // scope 0 at $DIR/boolean_identities.rs:+1:5: +1:15
16 +         _3 = const true;                 // scope 0 at $DIR/boolean_identities.rs:+1:5: +1:15
+           StorageDead(_4);                 // scope 0 at $DIR/boolean_identities.rs:+1:14: +1:15
17           StorageLive(_5);                 // scope 0 at $DIR/boolean_identities.rs:+1:18: +1:29
- -         _5 = BitAnd(_1, const false);    // scope 0 at $DIR/boolean_identities.rs:+1:18: +1:29
- -         _0 = BitAnd(move _3, move _5);   // scope 0 at $DIR/boolean_identities.rs:+1:5: +1:29
+           StorageLive(_6);                 // scope 0 at $DIR/boolean_identities.rs:+1:19: +1:20
+           _6 = _1;                         // scope 0 at $DIR/boolean_identities.rs:+1:19: +1:20
+ -         _5 = BitAnd(move _6, const false); // scope 0 at $DIR/boolean_identities.rs:+1:18: +1:29
20 +         _5 = const false;                // scope 0 at $DIR/boolean_identities.rs:+1:18: +1:29
+           StorageDead(_6);                 // scope 0 at $DIR/boolean_identities.rs:+1:28: +1:29
+ -         _0 = BitAnd(move _3, move _5);   // scope 0 at $DIR/boolean_identities.rs:+1:5: +1:29
21 +         _0 = const false;                // scope 0 at $DIR/boolean_identities.rs:+1:5: +1:29
22           StorageDead(_5);                 // scope 0 at $DIR/boolean_identities.rs:+1:28: +1:29
23           StorageDead(_3);                 // scope 0 at $DIR/boolean_identities.rs:+1:28: +1:29

thread '[mir-opt] tests/mir-opt/const_prop/boolean_identities.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/boolean_identities.test.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/const_prop/inherit_overflow.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/inherit_overflow.rs stdout ----
11       scope 2 (inlined <u8 as Add>::add) { // at $DIR/inherit_overflow.rs:9:13: 9:47
12           debug self => _2;                // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
13           debug other => _3;               // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
-           let mut _4: (u8, bool);          // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           let mut _4: u8;                  // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           let mut _5: u8;                  // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           let mut _6: (u8, bool);          // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
16   
17       bb0: {


20           _2 = const u8::MAX;              // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
21           StorageLive(_3);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
22           _3 = const 1_u8;                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
- -         _4 = CheckedAdd(_2, _3);         // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
- -         assert(!move (_4.1: bool), "attempt to compute `{} + {}`, which would overflow", _2, _3) -> bb1; // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
- +         _4 = const (0_u8, true);         // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
- +         assert(!const true, "attempt to compute `{} + {}`, which would overflow", _2, _3) -> bb1; // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           StorageLive(_4);                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ -         _4 = _2;                         // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ +         _4 = const u8::MAX;              // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           StorageLive(_5);                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ -         _5 = _3;                         // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ -         _6 = CheckedAdd(_4, _5);         // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ -         assert(!move (_6.1: bool), "attempt to compute `{} + {}`, which would overflow", move _4, move _5) -> bb1; // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ +         _5 = const 1_u8;                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ +         _6 = const (0_u8, true);         // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ +         assert(!const true, "attempt to compute `{} + {}`, which would overflow", move _4, move _5) -> bb1; // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
28   
29       bb1: {


- -         _1 = move (_4.0: u8);            // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ -         _1 = move (_6.0: u8);            // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
31 +         _1 = const 0_u8;                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           StorageDead(_5);                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           StorageDead(_4);                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
32           StorageDead(_3);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
33           StorageDead(_2);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
34           StorageDead(_1);                 // scope 0 at $DIR/inherit_overflow.rs:+3:47: +3:48

thread '[mir-opt] tests/mir-opt/const_prop/inherit_overflow.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/inherit_overflow.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/copy-prop/borrowed_local.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/borrowed_local.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/borrowed_local.rs' panicked at 'the mir dump file for borrowed_local.f.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/borrowed_local.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/copy-prop/branch.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/branch.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/branch.rs' panicked at 'the mir dump file for branch.foo.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/branch.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/copy-prop/dead_stores_79191.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/dead_stores_79191.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/dead_stores_79191.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/copy-prop/dead_stores_79191/dead_stores_79191.f.CopyProp.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/copy-prop/dead_stores_79191`', src/tools/compiletest/src/runtest.rs:3618:21
---- [mir-opt] tests/mir-opt/copy-prop/copy_propagation_arg.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/copy_propagation_arg.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/copy_propagation_arg.rs' panicked at 'the mir dump file for copy_propagation_arg.foo.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/copy_propagation_arg.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/copy-prop/cycle.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/cycle.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/cycle.rs' panicked at 'the mir dump file for cycle.main.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/cycle.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/copy-prop/move_arg.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/move_arg.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/move_arg.rs' panicked at 'the mir dump file for move_arg.f.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/move_arg.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/copy-prop/custom_move_arg.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/custom_move_arg.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/custom_move_arg.rs' panicked at 'the mir dump file for custom_move_arg.f.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/custom_move_arg.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/copy-prop/dead_stores_better.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/dead_stores_better.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/dead_stores_better.rs' panicked at 'Output file `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/copy-prop/dead_stores_better/dead_stores_better.f.CopyProp.after.mir` from test does not exist, available files are in `/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/copy-prop/dead_stores_better`', src/tools/compiletest/src/runtest.rs:3618:21
---- [mir-opt] tests/mir-opt/copy-prop/reborrow.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/reborrow.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/reborrow.rs' panicked at 'the mir dump file for reborrow.remut.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/reborrow.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/copy-prop/move_projection.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/move_projection.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/move_projection.rs' panicked at 'the mir dump file for move_projection.f.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/move_projection.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/copy-prop/mutate_through_pointer.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/mutate_through_pointer.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/mutate_through_pointer.rs' panicked at 'the mir dump file for mutate_through_pointer.f.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/mutate_through_pointer.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/copy-prop/non_dominate.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/non_dominate.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/non_dominate.rs' panicked at 'the mir dump file for non_dominate.f.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/non_dominate.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/copy-prop/issue_107511.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/issue_107511.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/issue_107511.rs' panicked at 'the mir dump file for issue_107511.main.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/issue_107511.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/copy-prop/partial_init.rs stdout ----
---- [mir-opt] tests/mir-opt/copy-prop/partial_init.rs stdout ----
thread '[mir-opt] tests/mir-opt/copy-prop/partial_init.rs' panicked at 'the mir dump file for partial_init.main.CopyProp.before.mir does not exist (requested in /checkout/tests/mir-opt/copy-prop/partial_init.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/dataflow-const-prop/inherit_overflow.rs stdout ----
---- [mir-opt] tests/mir-opt/dataflow-const-prop/inherit_overflow.rs stdout ----
11       scope 2 (inlined <u8 as Add>::add) { // at $DIR/inherit_overflow.rs:9:13: 9:47
12           debug self => _2;                // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
13           debug other => _3;               // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
-           let mut _4: (u8, bool);          // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           let mut _4: u8;                  // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           let mut _5: u8;                  // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           let mut _6: (u8, bool);          // in scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
16   
17       bb0: {


20           _2 = const u8::MAX;              // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
21           StorageLive(_3);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
22           _3 = const 1_u8;                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
- -         _4 = CheckedAdd(_2, _3);         // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
- -         assert(!move (_4.1: bool), "attempt to compute `{} + {}`, which would overflow", _2, _3) -> bb1; // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
- +         _4 = CheckedAdd(const u8::MAX, const 1_u8); // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           StorageLive(_4);                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ -         _4 = _2;                         // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ +         _4 = const u8::MAX;              // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           StorageLive(_5);                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ -         _5 = _3;                         // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ -         _6 = CheckedAdd(_4, _5);         // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ -         assert(!move (_6.1: bool), "attempt to compute `{} + {}`, which would overflow", move _4, move _5) -> bb1; // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ +         _5 = const 1_u8;                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ +         _6 = CheckedAdd(const u8::MAX, const 1_u8); // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
26 +         assert(!const true, "attempt to compute `{} + {}`, which would overflow", const u8::MAX, const 1_u8) -> bb1; // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
28   

29       bb1: {
29       bb1: {
- -         _1 = move (_4.0: u8);            // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ -         _1 = move (_6.0: u8);            // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
31 +         _1 = const 0_u8;                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           StorageDead(_5);                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           StorageDead(_4);                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
32           StorageDead(_3);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
33           StorageDead(_2);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
34           StorageDead(_1);                 // scope 0 at $DIR/inherit_overflow.rs:+3:47: +3:48

thread '[mir-opt] tests/mir-opt/dataflow-const-prop/inherit_overflow.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/dataflow-const-prop/inherit_overflow.main.DataflowConstProp.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/funky_arms.rs stdout ----
85       }
86   
87       bb6: {
87       bb6: {
+           StorageLive(_10);                // scope 3 at $DIR/funky_arms.rs:+13:17: +13:26
88           _10 = ((_7 as Some).0: usize);   // scope 3 at $DIR/funky_arms.rs:+13:17: +13:26
+           StorageLive(_11);                // scope 3 at $DIR/funky_arms.rs:+15:43: +15:46
+           _11 = _1;                        // scope 3 at $DIR/funky_arms.rs:+15:43: +15:46
+           StorageLive(_12);                // scope 3 at $DIR/funky_arms.rs:+15:48: +15:51
+           _12 = _2;                        // scope 3 at $DIR/funky_arms.rs:+15:48: +15:51
89           StorageLive(_13);                // scope 3 at $DIR/funky_arms.rs:+15:53: +15:57
90           _13 = _6;                        // scope 3 at $DIR/funky_arms.rs:+15:53: +15:57
91           StorageLive(_14);                // scope 3 at $DIR/funky_arms.rs:+15:59: +15:79

92           StorageLive(_15);                // scope 3 at $DIR/funky_arms.rs:+15:59: +15:75
-           _15 = _10 as u32 (IntToInt);     // scope 3 at $DIR/funky_arms.rs:+15:59: +15:75
+           StorageLive(_16);                // scope 3 at $DIR/funky_arms.rs:+15:59: +15:68
+           _16 = _10;                       // scope 3 at $DIR/funky_arms.rs:+15:59: +15:68
+           _15 = move _16 as u32 (IntToInt); // scope 3 at $DIR/funky_arms.rs:+15:59: +15:75
+           StorageDead(_16);                // scope 3 at $DIR/funky_arms.rs:+15:74: +15:75
94           _14 = Add(move _15, const 1_u32); // scope 3 at $DIR/funky_arms.rs:+15:59: +15:79
95           StorageDead(_15);                // scope 3 at $DIR/funky_arms.rs:+15:78: +15:79
-           _0 = float_to_exponential_common_exact::<T>(_1, _2, move _13, move _14, _3) -> bb7; // scope 3 at $DIR/funky_arms.rs:+15:9: +15:87
+           StorageLive(_17);                // scope 3 at $DIR/funky_arms.rs:+15:81: +15:86
+           _17 = _3;                        // scope 3 at $DIR/funky_arms.rs:+15:81: +15:86
+           _0 = float_to_exponential_common_exact::<T>(move _11, move _12, move _13, move _14, move _17) -> bb7; // scope 3 at $DIR/funky_arms.rs:+15:9: +15:87
97                                            // mir::Constant
98                                            // + span: $DIR/funky_arms.rs:27:9: 27:42
99                                            // + literal: Const { ty: for<'a, 'b, 'c> fn(&'a mut Formatter<'b>, &'c T, Sign, u32, bool) -> Result<(), std::fmt::Error> {float_to_exponential_common_exact::<T>}, val: Value(<ZST>) }
100       }
101   
102       bb7: {
102       bb7: {
+           StorageDead(_17);                // scope 3 at $DIR/funky_arms.rs:+15:86: +15:87
103           StorageDead(_14);                // scope 3 at $DIR/funky_arms.rs:+15:86: +15:87
104           StorageDead(_13);                // scope 3 at $DIR/funky_arms.rs:+15:86: +15:87
+           StorageDead(_12);                // scope 3 at $DIR/funky_arms.rs:+15:86: +15:87
+           StorageDead(_11);                // scope 3 at $DIR/funky_arms.rs:+15:86: +15:87
+           StorageDead(_10);                // scope 2 at $DIR/funky_arms.rs:+16:5: +16:6
105           goto -> bb10;                    // scope 2 at $DIR/funky_arms.rs:+13:5: +18:6
107   

108       bb8: {
108       bb8: {
+           StorageLive(_18);                // scope 2 at $DIR/funky_arms.rs:+17:46: +17:49
+           _18 = _1;                        // scope 2 at $DIR/funky_arms.rs:+17:46: +17:49
+           StorageLive(_19);                // scope 2 at $DIR/funky_arms.rs:+17:51: +17:54
+           _19 = _2;                        // scope 2 at $DIR/funky_arms.rs:+17:51: +17:54
109           StorageLive(_20);                // scope 2 at $DIR/funky_arms.rs:+17:56: +17:60
110           _20 = _6;                        // scope 2 at $DIR/funky_arms.rs:+17:56: +17:60
-           _0 = float_to_exponential_common_shortest::<T>(_1, _2, move _20, _3) -> bb9; // scope 2 at $DIR/funky_arms.rs:+17:9: +17:68
+           StorageLive(_21);                // scope 2 at $DIR/funky_arms.rs:+17:62: +17:67
+           _21 = _3;                        // scope 2 at $DIR/funky_arms.rs:+17:62: +17:67
Build completed unsuccessfully in 0:12:39
+           _0 = float_to_exponential_common_shortest::<T>(move _18, move _19, move _20, move _21) -> bb9; // scope 2 at $DIR/funky_arms.rs:+17:9: +17:68
112                                            // mir::Constant
113                                            // + span: $DIR/funky_arms.rs:29:9: 29:45
114                                            // + literal: Const { ty: for<'a, 'b, 'c> fn(&'a mut Formatter<'b>, &'c T, Sign, bool) -> Result<(), std::fmt::Error> {float_to_exponential_common_shortest::<T>}, val: Value(<ZST>) }
115       }
116   
117       bb9: {
117       bb9: {
+           StorageDead(_21);                // scope 2 at $DIR/funky_arms.rs:+17:67: +17:68
118           StorageDead(_20);                // scope 2 at $DIR/funky_arms.rs:+17:67: +17:68
+           StorageDead(_19);                // scope 2 at $DIR/funky_arms.rs:+17:67: +17:68
+           StorageDead(_18);                // scope 2 at $DIR/funky_arms.rs:+17:67: +17:68
119           goto -> bb10;                    // scope 2 at $DIR/funky_arms.rs:+13:5: +18:6
121   


thread '[mir-opt] tests/mir-opt/funky_arms.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/funky_arms.float_to_exponential_common.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/dyn_trait.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/dyn_trait.rs stdout ----
17           _2 = move _3 as &dyn Cache<V = <C as Cache>::V> (Pointer(Unsize)); // scope 0 at $DIR/dyn_trait.rs:+1:14: +1:15
18           StorageDead(_3);                 // scope 0 at $DIR/dyn_trait.rs:+1:14: +1:15
19 -         _0 = mk_cycle::<<C as Cache>::V>(move _2) -> bb1; // scope 0 at $DIR/dyn_trait.rs:+1:5: +1:16
- +         _0 = <dyn Cache<V = <C as Cache>::V> as Cache>::store_nocache(_2) -> bb1; // scope 1 at $DIR/dyn_trait.rs:22:5: 22:22
+ +         _0 = <dyn Cache<V = <C as Cache>::V> as Cache>::store_nocache(move _2) -> bb1; // scope 1 at $DIR/dyn_trait.rs:22:5: 22:22
21                                            // mir::Constant
22 -                                          // + span: $DIR/dyn_trait.rs:28:5: 28:13
23 -                                          // + literal: Const { ty: for<'a> fn(&'a (dyn Cache<V = <C as Cache>::V> + 'a)) {mk_cycle::<<C as Cache>::V>}, val: Value(<ZST>) }

thread '[mir-opt] tests/mir-opt/inline/dyn_trait.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/dyn_trait.try_execute_query.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/inline_any_operand.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_any_operand.rs stdout ----
26         _3 = const 1_i32;                // scope 1 at $DIR/inline_any_operand.rs:+2:5: +2:13
27         StorageLive(_4);                 // scope 1 at $DIR/inline_any_operand.rs:+2:5: +2:13
28         _4 = const -1_i32;               // scope 1 at $DIR/inline_any_operand.rs:+2:5: +2:13
-         _0 = Eq(_3, _4);                 // scope 2 at $DIR/inline_any_operand.rs:17:5: 17:11
+         _0 = Eq(move _3, move _4);       // scope 2 at $DIR/inline_any_operand.rs:17:5: 17:11
30         StorageDead(_4);                 // scope 1 at $DIR/inline_any_operand.rs:+2:5: +2:13
31         StorageDead(_3);                 // scope 1 at $DIR/inline_any_operand.rs:+2:5: +2:13
32         StorageDead(_2);                 // scope 1 at $DIR/inline_any_operand.rs:+2:12: +2:13

thread '[mir-opt] tests/mir-opt/inline/inline_any_operand.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_any_operand.bar.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/inline_generator.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_generator.rs stdout ----
7       let mut _2: std::pin::Pin<&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]>; // in scope 0 at $DIR/inline_generator.rs:+1:14: +1:32
8       let mut _3: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 0 at $DIR/inline_generator.rs:+1:23: +1:31
9       let mut _4: [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 0 at $DIR/inline_generator.rs:+1:28: +1:31
- +     let mut _5: bool;                    // in scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
+ +     let mut _7: bool;                    // in scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
11       scope 1 {
12           debug _r => _1;                  // in scope 1 at $DIR/inline_generator.rs:+1:9: +1:11

15 +     }
15 +     }
16 +     scope 3 (inlined Pin::<&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]>::new) { // at $DIR/inline_generator.rs:9:14: 9:32
17 +         debug pointer => _3;             // in scope 3 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +         let mut _5: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 3 at $SRC_DIR/core/src/pin.rs:LL:COL
18 +         scope 4 {
19 +             scope 5 (inlined Pin::<&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]>::new_unchecked) { // at $SRC_DIR/core/src/pin.rs:LL:COL
- +                 debug pointer => _3;     // in scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +                 debug pointer => _5;     // in scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +                 let mut _6: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
22 +         }
23 +     }


24 +     scope 6 (inlined g::{closure#0}) {   // at $DIR/inline_generator.rs:9:33: 9:46
- +         debug a => _5;                   // in scope 6 at $DIR/inline_generator.rs:15:6: 15:7
- +         let mut _6: i32;                 // in scope 6 at $DIR/inline_generator.rs:15:17: 15:39
- +         let mut _7: u32;                 // in scope 6 at $DIR/inline_generator.rs:15:5: 15:41
- +         let mut _8: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 6 at $DIR/inline_generator.rs:15:5: 15:41
- +         let mut _9: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         debug a => _7;                   // in scope 6 at $DIR/inline_generator.rs:15:6: 15:7
+ +         let mut _8: i32;                 // in scope 6 at $DIR/inline_generator.rs:15:17: 15:39
+ +         let mut _9: u32;                 // in scope 6 at $DIR/inline_generator.rs:15:5: 15:41
30 +         let mut _10: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         let mut _11: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         let mut _12: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]; // in scope 6 at $DIR/inline_generator.rs:15:5: 15:41
32   
33       bb0: {

62 -     }
62 -     }
63 - 
64 -     bb2: {
- +         _2 = Pin::<&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]> { pointer: move _3 }; // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +         StorageLive(_5);                 // scope 4 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +         _5 = move _3;                    // scope 4 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +         StorageLive(_6);                 // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +         _6 = move _5;                    // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +         _2 = Pin::<&mut [generator@$DIR/inline_generator.rs:15:5: 15:8]> { pointer: move _6 }; // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +         StorageDead(_6);                 // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +         StorageDead(_5);                 // scope 4 at $SRC_DIR/core/src/pin.rs:LL:COL
66           StorageDead(_3);                 // scope 0 at $DIR/inline_generator.rs:+1:31: +1:32
67 -         _1 = <[generator@$DIR/inline_generator.rs:15:5: 15:8] as Generator<bool>>::resume(move _2, const false) -> [return: bb3, unwind: bb4]; // scope 0 at $DIR/inline_generator.rs:+1:14: +1:46
68 -                                          // mir::Constant

69 -                                          // + span: $DIR/inline_generator.rs:9:33: 9:39
70 -                                          // + literal: Const { ty: for<'a> fn(Pin<&'a mut [generator@$DIR/inline_generator.rs:15:5: 15:8]>, bool) -> GeneratorState<<[generator@$DIR/inline_generator.rs:15:5: 15:8] as Generator<bool>>::Yield, <[generator@$DIR/inline_generator.rs:15:5: 15:8] as Generator<bool>>::Return> {<[generator@$DIR/inline_generator.rs:15:5: 15:8] as Generator<bool>>::resume}, val: Value(<ZST>) }
- +         StorageLive(_5);                 // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
- +         _5 = const false;                // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
- +         _8 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
- +         _7 = discriminant((*_8));        // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
- +         switchInt(move _7) -> [0: bb3, 1: bb8, 3: bb7, otherwise: bb9]; // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         StorageLive(_7);                 // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
+ +         _7 = const false;                // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
+ +         _10 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         _9 = discriminant((*_10));       // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         switchInt(move _9) -> [0: bb3, 1: bb8, 3: bb7, otherwise: bb9]; // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
77   
78 -     bb3: {

79 +     bb1: {
79 +     bb1: {
- +         StorageDead(_5);                 // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
+ +         StorageDead(_7);                 // scope 0 at $DIR/inline_generator.rs:+1:33: +1:46
81           StorageDead(_2);                 // scope 0 at $DIR/inline_generator.rs:+1:45: +1:46
82           StorageDead(_4);                 // scope 0 at $DIR/inline_generator.rs:+1:46: +1:47
83           _0 = const ();                   // scope 0 at $DIR/inline_generator.rs:+0:11: +2:2
91 +     }
92 + 
93 +     bb3: {
93 +     bb3: {
- +         StorageLive(_6);                 // scope 6 at $DIR/inline_generator.rs:15:17: 15:39
- +         switchInt(_5) -> [0: bb5, otherwise: bb4]; // scope 6 at $DIR/inline_generator.rs:15:20: 15:21
+ +         StorageLive(_8);                 // scope 6 at $DIR/inline_generator.rs:15:17: 15:39
+ +         switchInt(move _7) -> [0: bb5, otherwise: bb4]; // scope 6 at $DIR/inline_generator.rs:15:20: 15:21
97 + 
98 +     bb4: {


- +         _6 = const 7_i32;                // scope 6 at $DIR/inline_generator.rs:15:24: 15:25
+ +         _8 = const 7_i32;                // scope 6 at $DIR/inline_generator.rs:15:24: 15:25
100 +         goto -> bb6;                     // scope 6 at $DIR/inline_generator.rs:15:17: 15:39
102 + 

103 +     bb5: {
103 +     bb5: {
- +         _6 = const 13_i32;               // scope 6 at $DIR/inline_generator.rs:15:35: 15:37
+ +         _8 = const 13_i32;               // scope 6 at $DIR/inline_generator.rs:15:35: 15:37
105 +         goto -> bb6;                     // scope 6 at $DIR/inline_generator.rs:15:17: 15:39
107 + 

108 +     bb6: {
108 +     bb6: {
- +         _1 = GeneratorState::<i32, bool>::Yielded(move _6); // scope 6 at $DIR/inline_generator.rs:15:11: 15:39
- +         _9 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:11: 15:39
- +         discriminant((*_9)) = 3;         // scope 6 at $DIR/inline_generator.rs:15:11: 15:39
+ +         _1 = GeneratorState::<i32, bool>::Yielded(move _8); // scope 6 at $DIR/inline_generator.rs:15:11: 15:39
+ +         _11 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:11: 15:39
+ +         discriminant((*_11)) = 3;        // scope 6 at $DIR/inline_generator.rs:15:11: 15:39
112 +         goto -> bb1;                     // scope 0 at $DIR/inline_generator.rs:15:11: 15:39
114 + 

115 +     bb7: {
115 +     bb7: {
- +         StorageLive(_6);                 // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
- +         StorageDead(_6);                 // scope 6 at $DIR/inline_generator.rs:15:38: 15:39
- +         _1 = GeneratorState::<i32, bool>::Complete(_5); // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
- +         _10 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
- +         discriminant((*_10)) = 1;        // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
+ +         StorageLive(_8);                 // scope 6 at $DIR/inline_generator.rs:15:5: 15:41
+ +         StorageDead(_8);                 // scope 6 at $DIR/inline_generator.rs:15:38: 15:39
+ +         _1 = GeneratorState::<i32, bool>::Complete(move _7); // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
+ +         _12 = deref_copy (_2.0: &mut [generator@$DIR/inline_generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
+ +         discriminant((*_12)) = 1;        // scope 6 at $DIR/inline_generator.rs:15:41: 15:41
121 +         goto -> bb1;                     // scope 0 at $DIR/inline_generator.rs:15:41: 15:41
123 + 


thread '[mir-opt] tests/mir-opt/inline/inline_generator.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_generator.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/inline_trait_method_2.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_trait_method_2.rs stdout ----
15         _3 = &(*_1);                     // scope 0 at $DIR/inline_trait_method_2.rs:+1:10: +1:11
16         _2 = move _3 as &dyn X (Pointer(Unsize)); // scope 0 at $DIR/inline_trait_method_2.rs:+1:10: +1:11
17         StorageDead(_3);                 // scope 0 at $DIR/inline_trait_method_2.rs:+1:10: +1:11
-         _0 = <dyn X as X>::y(_2) -> bb1; // scope 1 at $DIR/inline_trait_method_2.rs:11:5: 11:10
+         _0 = <dyn X as X>::y(move _2) -> bb1; // scope 1 at $DIR/inline_trait_method_2.rs:11:5: 11:10
19                                          // mir::Constant
20                                          // + span: $DIR/inline_trait_method_2.rs:11:7: 11:8
21                                          // + literal: Const { ty: for<'a> fn(&'a dyn X) -> bool {<dyn X as X>::y}, val: Value(<ZST>) }

thread '[mir-opt] tests/mir-opt/inline/inline_trait_method_2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_trait_method_2.test2.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/issue_101973.rs stdout ----
---- [mir-opt] tests/mir-opt/issue_101973.rs stdout ----
17       let mut _12: u32;                    // in scope 0 at $DIR/issue_101973.rs:+1:31: +1:57
18       let mut _13: bool;                   // in scope 0 at $DIR/issue_101973.rs:+1:31: +1:57
19       scope 1 (inlined imm8) {             // at $DIR/issue_101973.rs:15:5: 15:17
-           debug x => _1;                   // in scope 1 at $DIR/issue_101973.rs:6:13: 6:14
+           debug x => _5;                   // in scope 1 at $DIR/issue_101973.rs:6:13: 6:14
21           let mut _14: u32;                // in scope 1 at $DIR/issue_101973.rs:8:12: 8:27
22           let mut _15: u32;                // in scope 1 at $DIR/issue_101973.rs:8:12: 8:20
23           scope 2 {

27       scope 3 (inlined core::num::<impl u32>::rotate_right) { // at $DIR/issue_101973.rs:15:18: 15:58
28           debug self => _4;                // in scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
29           debug n => _6;                   // in scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+           let mut _16: u32;                // in scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+           let mut _17: u32;                // in scope 3 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
31   
32       bb0: {


33           StorageLive(_2);                 // scope 0 at $DIR/issue_101973.rs:+1:5: +1:65
34           StorageLive(_3);                 // scope 0 at $DIR/issue_101973.rs:+1:5: +1:58
