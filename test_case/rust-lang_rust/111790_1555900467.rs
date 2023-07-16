plain
 finished in 0.627 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 255 tests
................F..................F.............F....F.FF......................i.......  88/255
............F........................................F...iFFF....F.FFiiiFF.........F.FF. 176/255
.F...F..F.........iii.iiiii..F......ii...FF..Fi...F..FFFFF..........F...F......
failures:

---- [mir-opt] tests/mir-opt/casts.rs stdout ----
---- [mir-opt] tests/mir-opt/casts.rs stdout ----
8       let mut _3: *const &u8;              // in scope 0 at $DIR/casts.rs:+1:36: +1:37
9       scope 1 (inlined generic_cast::<&u8, &u8>) { // at $DIR/casts.rs:6:5: 6:38
10           debug x => _3;                   // in scope 1 at $DIR/casts.rs:10:23: 10:24
+           let mut _4: *const &u8;          // in scope 1 at $DIR/casts.rs:11:5: 11:6
12   
13       bb0: {


14           StorageLive(_2);                 // scope 0 at $DIR/casts.rs:+1:5: +1:55
15           StorageLive(_3);                 // scope 0 at $DIR/casts.rs:+1:36: +1:37
16           _3 = _1;                         // scope 0 at $DIR/casts.rs:+1:36: +1:37
+           StorageLive(_4);                 // scope 0 at $DIR/casts.rs:+1:5: +1:38
17 -         _2 = _3 as *const &u8 (PtrToPtr); // scope 1 at $DIR/casts.rs:11:5: 11:18
18 +         _2 = _3;                         // scope 1 at $DIR/casts.rs:11:5: 11:18
+           StorageDead(_4);                 // scope 0 at $DIR/casts.rs:+1:5: +1:38
19           StorageDead(_3);                 // scope 0 at $DIR/casts.rs:+1:37: +1:38
20           _0 = _2;                         // scope 0 at $DIR/casts.rs:+1:5: +1:55
21           StorageDead(_2);                 // scope 0 at $DIR/casts.rs:+2:1: +2:2

thread '[mir-opt] tests/mir-opt/casts.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/casts.redundant.InstSimplify.diff', src/tools/compiletest/src/runtest.rs:3639:21

---- [mir-opt] tests/mir-opt/const_debuginfo.rs stdout ----
---- [mir-opt] tests/mir-opt/const_debuginfo.rs stdout ----
8       let mut _6: u8;                      // in scope 0 at $DIR/const_debuginfo.rs:+4:15: +4:16
9       let mut _7: u8;                      // in scope 0 at $DIR/const_debuginfo.rs:+4:19: +4:20
10       let mut _8: u8;                      // in scope 0 at $DIR/const_debuginfo.rs:+4:23: +4:24
-       let mut _12: u32;                    // in scope 0 at $DIR/const_debuginfo.rs:+13:13: +13:16
-       let mut _13: u32;                    // in scope 0 at $DIR/const_debuginfo.rs:+13:19: +13:22
+       let mut _14: u32;                    // in scope 0 at $DIR/const_debuginfo.rs:+13:13: +13:16
+       let mut _15: u32;                    // in scope 0 at $DIR/const_debuginfo.rs:+13:19: +13:22
13       scope 1 {
14 -         debug x => _1;                   // in scope 1 at $DIR/const_debuginfo.rs:+1:9: +1:10
15 +         debug x => const 1_u8;           // in scope 1 at $DIR/const_debuginfo.rs:+1:9: +1:10
29                       scope 5 {
29                       scope 5 {
30 -                         debug s => _9;   // in scope 5 at $DIR/const_debuginfo.rs:+6:9: +6:10
31 +                         debug s => const "hello, world!"; // in scope 5 at $DIR/const_debuginfo.rs:+6:9: +6:10
-                           let _14: bool;   // in scope 5 at $DIR/const_debuginfo.rs:+8:9: +8:10
-                           let _15: bool;   // in scope 5 at $DIR/const_debuginfo.rs:+8:9: +8:10
-                           let _16: u32;    // in scope 5 at $DIR/const_debuginfo.rs:+8:9: +8:10
+                           let _10: (bool, bool, u32); // in scope 5 at $DIR/const_debuginfo.rs:+8:9: +8:10
+                           let _16: bool;   // in scope 5 at $DIR/const_debuginfo.rs:+8:9: +8:10
+                           let _17: bool;   // in scope 5 at $DIR/const_debuginfo.rs:+8:9: +8:10
+                           let _18: u32;    // in scope 5 at $DIR/const_debuginfo.rs:+8:9: +8:10
35                           scope 6 {
-                               debug f => (bool, bool, u32){ .0 => _14, .1 => _15, .2 => _16, }; // in scope 6 at $DIR/const_debuginfo.rs:+8:9: +8:10
-                               let _10: std::option::Option<u16>; // in scope 6 at $DIR/const_debuginfo.rs:+10:9: +10:10
+                               debug f => (bool, bool, u32){ .0 => _16, .1 => _17, .2 => _18, }; // in scope 6 at $DIR/const_debuginfo.rs:+8:9: +8:10
Build completed unsuccessfully in 0:13:21
+                               let _11: std::option::Option<u16>; // in scope 6 at $DIR/const_debuginfo.rs:+10:9: +10:10
38                               scope 7 {
-                                   debug o => _10; // in scope 7 at $DIR/const_debuginfo.rs:+10:9: +10:10
-                                   let _17: u32; // in scope 7 at $DIR/const_debuginfo.rs:+12:9: +12:10
-                                   let _18: u32; // in scope 7 at $DIR/const_debuginfo.rs:+12:9: +12:10
+                                   debug o => _11; // in scope 7 at $DIR/const_debuginfo.rs:+10:9: +10:10
+                                   let _12: Point; // in scope 7 at $DIR/const_debuginfo.rs:+12:9: +12:10
+                                   let _19: u32; // in scope 7 at $DIR/const_debuginfo.rs:+12:9: +12:10
+                                   let _20: u32; // in scope 7 at $DIR/const_debuginfo.rs:+12:9: +12:10
42                                   scope 8 {
-                                       debug p => Point{ .0 => _17, .1 => _18, }; // in scope 8 at $DIR/const_debuginfo.rs:+12:9: +12:10
-                                       let _11: u32; // in scope 8 at $DIR/const_debuginfo.rs:+13:9: +13:10
+                                       debug p => Point{ .0 => _19, .1 => _20, }; // in scope 8 at $DIR/const_debuginfo.rs:+12:9: +12:10
+                                       let _13: u32; // in scope 8 at $DIR/const_debuginfo.rs:+13:9: +13:10
45                                       scope 9 {
- -                                         debug a => _11; // in scope 9 at $DIR/const_debuginfo.rs:+13:9: +13:10
+ -                                         debug a => _13; // in scope 9 at $DIR/const_debuginfo.rs:+13:9: +13:10
47 +                                         debug a => const 64_u32; // in scope 9 at $DIR/const_debuginfo.rs:+13:9: +13:10
49                                   }

69                                            // mir::Constant
69                                            // mir::Constant
70                                            // + span: $DIR/const_debuginfo.rs:14:13: 14:28
71                                            // + literal: Const { ty: &str, val: Value(Slice(..)) }
-           StorageLive(_14);                // scope 5 at $DIR/const_debuginfo.rs:+8:9: +8:10
-           StorageLive(_15);                // scope 5 at $DIR/const_debuginfo.rs:+8:9: +8:10
74           StorageLive(_16);                // scope 5 at $DIR/const_debuginfo.rs:+8:9: +8:10
-           _14 = const true;                // scope 5 at $DIR/const_debuginfo.rs:+8:13: +8:34
-           _15 = const false;               // scope 5 at $DIR/const_debuginfo.rs:+8:13: +8:34
-           _16 = const 123_u32;             // scope 5 at $DIR/const_debuginfo.rs:+8:13: +8:34
-           StorageLive(_10);                // scope 6 at $DIR/const_debuginfo.rs:+10:9: +10:10
-           _10 = Option::<u16>::Some(const 99_u16); // scope 6 at $DIR/const_debuginfo.rs:+10:13: +10:24
-           _17 = const 32_u32;              // scope 7 at $DIR/const_debuginfo.rs:+12:13: +12:35
-           _18 = const 32_u32;              // scope 7 at $DIR/const_debuginfo.rs:+12:13: +12:35
-           StorageLive(_11);                // scope 8 at $DIR/const_debuginfo.rs:+13:9: +13:10
-           _11 = const 64_u32;              // scope 8 at $DIR/const_debuginfo.rs:+13:13: +13:22
-           StorageDead(_11);                // scope 8 at $DIR/const_debuginfo.rs:+14:1: +14:2
-           StorageDead(_10);                // scope 6 at $DIR/const_debuginfo.rs:+14:1: +14:2
-           StorageDead(_14);                // scope 5 at $DIR/const_debuginfo.rs:+14:1: +14:2
-           StorageDead(_15);                // scope 5 at $DIR/const_debuginfo.rs:+14:1: +14:2
+           StorageLive(_17);                // scope 5 at $DIR/const_debuginfo.rs:+8:9: +8:10
+           StorageLive(_18);                // scope 5 at $DIR/const_debuginfo.rs:+8:9: +8:10
+           _16 = const true;                // scope 5 at $DIR/const_debuginfo.rs:+8:13: +8:34
+           _17 = const false;               // scope 5 at $DIR/const_debuginfo.rs:+8:13: +8:34
+           _18 = const 123_u32;             // scope 5 at $DIR/const_debuginfo.rs:+8:13: +8:34
+           StorageLive(_11);                // scope 6 at $DIR/const_debuginfo.rs:+10:9: +10:10
+           _11 = Option::<u16>::Some(const 99_u16); // scope 6 at $DIR/const_debuginfo.rs:+10:13: +10:24
+           _19 = const 32_u32;              // scope 7 at $DIR/const_debuginfo.rs:+12:13: +12:35
+           _20 = const 32_u32;              // scope 7 at $DIR/const_debuginfo.rs:+12:13: +12:35
+           StorageLive(_13);                // scope 8 at $DIR/const_debuginfo.rs:+13:9: +13:10
+           _13 = const 64_u32;              // scope 8 at $DIR/const_debuginfo.rs:+13:13: +13:22
+           StorageDead(_13);                // scope 8 at $DIR/const_debuginfo.rs:+14:1: +14:2
+           StorageDead(_11);                // scope 6 at $DIR/const_debuginfo.rs:+14:1: +14:2
88           StorageDead(_16);                // scope 5 at $DIR/const_debuginfo.rs:+14:1: +14:2
+           StorageDead(_17);                // scope 5 at $DIR/const_debuginfo.rs:+14:1: +14:2
+           StorageDead(_18);                // scope 5 at $DIR/const_debuginfo.rs:+14:1: +14:2
89           StorageDead(_9);                 // scope 4 at $DIR/const_debuginfo.rs:+14:1: +14:2
90           StorageDead(_4);                 // scope 3 at $DIR/const_debuginfo.rs:+14:1: +14:2
91           return;                          // scope 0 at $DIR/const_debuginfo.rs:+14:2: +14:2

thread '[mir-opt] tests/mir-opt/const_debuginfo.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_debuginfo.main.ConstDebugInfo.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/const_prop/control_flow_simplification.rs stdout ----
14       }
15   
16       bb1: {
16       bb1: {
+           nop;                             // scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
17           _2 = begin_panic::<&str>(const "explicit panic"); // scope 0 at $SRC_DIR/std/src/panic.rs:LL:COL
18                                            // mir::Constant
19                                            // + span: $SRC_DIR/std/src/panic.rs:LL:COL
24       }
25   
26       bb2: {
26       bb2: {
+           nop;                             // scope 0 at $DIR/control_flow_simplification.rs:+3:6: +3:6
27           StorageDead(_1);                 // scope 0 at $DIR/control_flow_simplification.rs:+3:5: +3:6
28           return;                          // scope 0 at $DIR/control_flow_simplification.rs:+4:2: +4:2


thread '[mir-opt] tests/mir-opt/const_prop/control_flow_simplification.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/control_flow_simplification.hello.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3639:21
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
+           StorageLive(_4);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
+           StorageLive(_5);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
+ -         _6 = CheckedAdd(_2, _3);         // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ -         assert(!move (_6.1: bool), "attempt to compute `{} + {}`, which would overflow", _2, _3) -> bb1; // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ +         _6 = const (0_u8, true);         // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
26 +         assert(!const true, "attempt to compute `{} + {}`, which would overflow", _2, _3) -> bb1; // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
28   

29       bb1: {
29       bb1: {
- -         _1 = move (_4.0: u8);            // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ -         _1 = move (_6.0: u8);            // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
31 +         _1 = const 0_u8;                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           StorageDead(_5);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
+           StorageDead(_4);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
32           StorageDead(_3);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
33           StorageDead(_2);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
34           StorageDead(_1);                 // scope 0 at $DIR/inherit_overflow.rs:+3:47: +3:48

thread '[mir-opt] tests/mir-opt/const_prop/inherit_overflow.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/inherit_overflow.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/const_prop/issue_66971.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/issue_66971.rs stdout ----
5       let mut _0: ();                      // return place in scope 0 at $DIR/issue_66971.rs:+0:11: +0:11
6       let _1: ();                          // in scope 0 at $DIR/issue_66971.rs:+1:5: +1:23
7       let mut _2: ((), u8, u8);            // in scope 0 at $DIR/issue_66971.rs:+1:12: +1:22
+       let mut _3: ();                      // in scope 0 at $DIR/issue_66971.rs:+1:13: +1:15
9       bb0: {
9       bb0: {
+           nop;                             // scope 0 at $DIR/issue_66971.rs:+1:5: +1:23
10           StorageLive(_2);                 // scope 0 at $DIR/issue_66971.rs:+1:12: +1:22
+           nop;                             // scope 0 at $DIR/issue_66971.rs:+1:13: +1:15
+           nop;                             // scope 0 at $DIR/issue_66971.rs:+1:13: +1:15
11           _2 = (const (), const 0_u8, const 0_u8); // scope 0 at $DIR/issue_66971.rs:+1:12: +1:22
+           nop;                             // scope 0 at $DIR/issue_66971.rs:+1:21: +1:22
12           _1 = encode(move _2) -> bb1;     // scope 0 at $DIR/issue_66971.rs:+1:5: +1:23
13                                            // mir::Constant
14                                            // + span: $DIR/issue_66971.rs:18:5: 18:11
17   
18       bb1: {
18       bb1: {
19           StorageDead(_2);                 // scope 0 at $DIR/issue_66971.rs:+1:22: +1:23
+           nop;                             // scope 0 at $DIR/issue_66971.rs:+1:23: +1:24
+           nop;                             // scope 0 at $DIR/issue_66971.rs:+0:11: +2:2
20           return;                          // scope 0 at $DIR/issue_66971.rs:+2:2: +2:2
22   }


thread '[mir-opt] tests/mir-opt/const_prop/issue_66971.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/issue_66971.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/const_prop/issue_67019.rs stdout ----
---- [mir-opt] tests/mir-opt/const_prop/issue_67019.rs stdout ----
8       let mut _3: (u8, u8);                // in scope 0 at $DIR/issue_67019.rs:+1:11: +1:17
10       bb0: {
10       bb0: {
+           nop;                             // scope 0 at $DIR/issue_67019.rs:+1:5: +1:20
11           StorageLive(_2);                 // scope 0 at $DIR/issue_67019.rs:+1:10: +1:19
12           StorageLive(_3);                 // scope 0 at $DIR/issue_67019.rs:+1:11: +1:17
13 -         _3 = (const 1_u8, const 2_u8);   // scope 0 at $DIR/issue_67019.rs:+1:11: +1:17
22   
23       bb1: {
23       bb1: {
24           StorageDead(_2);                 // scope 0 at $DIR/issue_67019.rs:+1:19: +1:20
+           nop;                             // scope 0 at $DIR/issue_67019.rs:+1:20: +1:21
+           nop;                             // scope 0 at $DIR/issue_67019.rs:+0:11: +2:2
25           return;                          // scope 0 at $DIR/issue_67019.rs:+2:2: +2:2
27   }


thread '[mir-opt] tests/mir-opt/const_prop/issue_67019.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/issue_67019.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3639:21
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
+           StorageLive(_4);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
+           StorageLive(_5);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
+ -         _6 = CheckedAdd(_2, _3);         // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ -         assert(!move (_6.1: bool), "attempt to compute `{} + {}`, which would overflow", _2, _3) -> bb1; // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ +         _6 = CheckedAdd(const u8::MAX, const 1_u8); // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
26 +         assert(!const true, "attempt to compute `{} + {}`, which would overflow", const u8::MAX, const 1_u8) -> bb1; // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
28   

29       bb1: {
29       bb1: {
- -         _1 = move (_4.0: u8);            // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+ -         _1 = move (_6.0: u8);            // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
31 +         _1 = const 0_u8;                 // scope 2 at $SRC_DIR/core/src/ops/arith.rs:LL:COL
+           StorageDead(_5);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
+           StorageDead(_4);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
32           StorageDead(_3);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
33           StorageDead(_2);                 // scope 0 at $DIR/inherit_overflow.rs:+3:13: +3:47
34           StorageDead(_1);                 // scope 0 at $DIR/inherit_overflow.rs:+3:47: +3:48

thread '[mir-opt] tests/mir-opt/dataflow-const-prop/inherit_overflow.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/dataflow-const-prop/inherit_overflow.main.DataflowConstProp.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/asm_unwind.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/asm_unwind.rs stdout ----
6       let _1: ();                          // in scope 0 at $DIR/asm_unwind.rs:+1:5: +1:10
7 +     scope 1 (inlined foo) {              // at $DIR/asm_unwind.rs:21:5: 21:10
8 +         let _2: D;                       // in scope 1 at $DIR/asm_unwind.rs:15:9: 15:11
+ +         let _3: ();                      // in scope 1 at $DIR/asm_unwind.rs:16:14: 16:54
9 +         scope 2 {
10 +             debug _d => const D;         // in scope 2 at $DIR/asm_unwind.rs:15:9: 15:11
11 +             scope 3 {

20 -                                          // + span: $DIR/asm_unwind.rs:21:5: 21:8
21 -                                          // + literal: Const { ty: fn() {foo}, val: Value(<ZST>) }
22 +         StorageLive(_2);                 // scope 0 at $DIR/asm_unwind.rs:+1:5: +1:10
+ +         StorageLive(_3);                 // scope 0 at $DIR/asm_unwind.rs:+1:5: +1:10
23 +         asm!("", options(MAY_UNWIND)) -> [return: bb2, unwind: bb3]; // scope 3 at $DIR/asm_unwind.rs:16:14: 16:54
25   

26       bb1: {
26       bb1: {
+ +         StorageDead(_3);                 // scope 0 at $DIR/asm_unwind.rs:+1:5: +1:10
27 +         StorageDead(_2);                 // scope 0 at $DIR/asm_unwind.rs:+1:5: +1:10
28           StorageDead(_1);                 // scope 0 at $DIR/asm_unwind.rs:+1:10: +1:11
29           _0 = const ();                   // scope 0 at $DIR/asm_unwind.rs:+0:15: +2:2

thread '[mir-opt] tests/mir-opt/inline/asm_unwind.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/asm_unwind.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/cycle.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/cycle.rs stdout ----
5       let mut _0: ();                      // return place in scope 0 at $DIR/cycle.rs:+0:8: +0:8
6       let _1: ();                          // in scope 0 at $DIR/cycle.rs:+1:5: +1:12
7 +     let mut _2: fn() {main};             // in scope 0 at $DIR/cycle.rs:+1:5: +1:12
- +     let mut _5: ();                      // in scope 0 at $DIR/cycle.rs:6:5: 6:8
+ +     let mut _6: ();                      // in scope 0 at $DIR/cycle.rs:6:5: 6:8
9 +     scope 1 (inlined f::<fn() {main}>) { // at $DIR/cycle.rs:12:5: 12:12
10 +         debug g => _2;                   // in scope 1 at $DIR/cycle.rs:5:6: 5:7
11 +         let _3: ();                      // in scope 1 at $DIR/cycle.rs:6:5: 6:8

12 +         let mut _4: &fn() {main};        // in scope 1 at $DIR/cycle.rs:6:5: 6:6
+ +         let mut _5: ();                  // in scope 1 at $DIR/cycle.rs:6:5: 6:8
13 +         scope 2 (inlined <fn() {main} as Fn<()>>::call - shim(fn() {main})) { // at $DIR/cycle.rs:6:5: 6:8
15 +     }


26                                            // + span: $DIR/cycle.rs:12:7: 12:11
27                                            // + literal: Const { ty: fn() {main}, val: Value(<ZST>) }
28 +         StorageLive(_3);                 // scope 0 at $DIR/cycle.rs:+1:5: +1:12
+ +         StorageLive(_5);                 // scope 0 at $DIR/cycle.rs:+1:5: +1:12
29 +         StorageLive(_4);                 // scope 1 at $DIR/cycle.rs:6:5: 6:6
30 +         _4 = &_2;                        // scope 1 at $DIR/cycle.rs:6:5: 6:6
- +         StorageLive(_5);                 // scope 1 at $DIR/cycle.rs:6:5: 6:8
- +         _5 = const ();                   // scope 1 at $DIR/cycle.rs:6:5: 6:8
+ +         StorageLive(_6);                 // scope 1 at $DIR/cycle.rs:6:5: 6:8
+ +         _6 = const ();                   // scope 1 at $DIR/cycle.rs:6:5: 6:8
33 +         _3 = move (*_4)() -> [return: bb4, unwind: bb2]; // scope 2 at $SRC_DIR/core/src/ops/function.rs:LL:COL
35   

36       bb1: {
36       bb1: {
+ +         StorageDead(_5);                 // scope 0 at $DIR/cycle.rs:+1:5: +1:12
37 +         StorageDead(_3);                 // scope 0 at $DIR/cycle.rs:+1:5: +1:12
38 +         StorageDead(_2);                 // scope 0 at $DIR/cycle.rs:+1:5: +1:12
39           StorageDead(_1);                 // scope 0 at $DIR/cycle.rs:+1:12: +1:13
50 +     }
51 + 
52 +     bb4: {
52 +     bb4: {
- +         StorageDead(_5);                 // scope 1 at $DIR/cycle.rs:6:5: 6:8
+ +         StorageDead(_6);                 // scope 1 at $DIR/cycle.rs:6:5: 6:8
54 +         StorageDead(_4);                 // scope 1 at $DIR/cycle.rs:6:7: 6:8
55 +         drop(_2) -> bb1;                 // scope 1 at $DIR/cycle.rs:7:1: 7:2


thread '[mir-opt] tests/mir-opt/inline/cycle.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/cycle.g.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/dyn_trait.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/dyn_trait.rs stdout ----
8       let mut _3: &C;                      // in scope 0 at $DIR/dyn_trait.rs:+1:14: +1:15
9 +     scope 1 (inlined mk_cycle::<<C as Cache>::V>) { // at $DIR/dyn_trait.rs:28:5: 28:16
10 +         debug c => _2;                   // in scope 1 at $DIR/dyn_trait.rs:21:27: 21:28
+ +         let mut _4: &dyn Cache<V = <C as Cache>::V>; // in scope 1 at $DIR/dyn_trait.rs:22:5: 22:22
12   
13       bb0: {


17           _2 = move _3 as &dyn Cache<V = <C as Cache>::V> (Pointer(Unsize)); // scope 0 at $DIR/dyn_trait.rs:+1:14: +1:15
18           StorageDead(_3);                 // scope 0 at $DIR/dyn_trait.rs:+1:14: +1:15
19 -         _0 = mk_cycle::<<C as Cache>::V>(move _2) -> bb1; // scope 0 at $DIR/dyn_trait.rs:+1:5: +1:16
+ +         StorageLive(_4);                 // scope 0 at $DIR/dyn_trait.rs:+1:5: +1:16
20 +         _0 = <dyn Cache<V = <C as Cache>::V> as Cache>::store_nocache(_2) -> bb1; // scope 1 at $DIR/dyn_trait.rs:22:5: 22:22
21                                            // mir::Constant
22 -                                          // + span: $DIR/dyn_trait.rs:28:5: 28:13
26       }
27   
28       bb1: {
28       bb1: {
+ +         StorageDead(_4);                 // scope 0 at $DIR/dyn_trait.rs:+1:5: +1:16
29           StorageDead(_2);                 // scope 0 at $DIR/dyn_trait.rs:+1:15: +1:16
30           return;                          // scope 0 at $DIR/dyn_trait.rs:+2:2: +2:2


thread '[mir-opt] tests/mir-opt/inline/dyn_trait.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/dyn_trait.try_execute_query.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/inline_instruction_set.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_instruction_set.rs stdout ----
10 +     scope 1 (inlined instruction_set_default) { // at $DIR/inline_instruction_set.rs:59:5: 59:30
11 +     }
12 +     scope 2 (inlined inline_always_and_using_inline_asm) { // at $DIR/inline_instruction_set.rs:60:5: 60:41
+ +         let _5: ();                      // in scope 2 at $DIR/inline_instruction_set.rs:43:14: 43:38
13 +         scope 3 {
15 +     }

47 -                                          // mir::Constant
47 -                                          // mir::Constant
48 -                                          // + span: $DIR/inline_instruction_set.rs:60:5: 60:39
49 -                                          // + literal: Const { ty: fn() {inline_always_and_using_inline_asm}, val: Value(<ZST>) }
+ +         StorageLive(_5);                 // scope 0 at $DIR/inline_instruction_set.rs:+4:5: +4:41
50 +         asm!("/* do nothing */", options((empty))) -> [return: bb3, unwind unreachable]; // scope 3 at $DIR/inline_instruction_set.rs:43:14: 43:38
52   

53 -     bb4: {
54 +     bb3: {
54 +     bb3: {
+ +         StorageDead(_5);                 // scope 0 at $DIR/inline_instruction_set.rs:+4:5: +4:41
55           StorageDead(_4);                 // scope 0 at $DIR/inline_instruction_set.rs:+4:41: +4:42
56           _0 = const ();                   // scope 0 at $DIR/inline_instruction_set.rs:+0:18: +5:2
57           return;                          // scope 0 at $DIR/inline_instruction_set.rs:+5:2: +5:2

thread '[mir-opt] tests/mir-opt/inline/inline_instruction_set.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_instruction_set.default.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/inline_any_operand.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_any_operand.rs stdout ----
11         scope 2 (inlined foo) {          // at $DIR/inline_any_operand.rs:12:5: 12:13
12             debug x => _3;               // in scope 2 at $DIR/inline_any_operand.rs:16:8: 16:9
13             debug y => _4;               // in scope 2 at $DIR/inline_any_operand.rs:16:16: 16:17
+             let mut _5: i32;             // in scope 2 at $DIR/inline_any_operand.rs:17:5: 17:6
+             let mut _6: i32;             // in scope 2 at $DIR/inline_any_operand.rs:17:10: 17:11
15     }
16 


26         _3 = const 1_i32;                // scope 1 at $DIR/inline_any_operand.rs:+2:5: +2:13
27         StorageLive(_4);                 // scope 1 at $DIR/inline_any_operand.rs:+2:5: +2:13
28         _4 = const -1_i32;               // scope 1 at $DIR/inline_any_operand.rs:+2:5: +2:13
+         StorageLive(_5);                 // scope 1 at $DIR/inline_any_operand.rs:+2:5: +2:13
+         StorageLive(_6);                 // scope 1 at $DIR/inline_any_operand.rs:+2:5: +2:13
29         _0 = Eq(_3, _4);                 // scope 2 at $DIR/inline_any_operand.rs:17:5: 17:11
+         StorageDead(_6);                 // scope 1 at $DIR/inline_any_operand.rs:+2:5: +2:13
+         StorageDead(_5);                 // scope 1 at $DIR/inline_any_operand.rs:+2:5: +2:13
30         StorageDead(_4);                 // scope 1 at $DIR/inline_any_operand.rs:+2:5: +2:13
31         StorageDead(_3);                 // scope 1 at $DIR/inline_any_operand.rs:+2:5: +2:13
32         StorageDead(_2);                 // scope 1 at $DIR/inline_any_operand.rs:+2:12: +2:13

thread '[mir-opt] tests/mir-opt/inline/inline_any_operand.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_any_operand.bar.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/inline_closure_borrows_arg.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_closure_borrows_arg.rs stdout ----
16         scope 2 (inlined foo::<T>::{closure#0}) { // at $DIR/inline_closure_borrows_arg.rs:16:5: 16:12
17             debug r => _8;               // in scope 2 at $DIR/inline_closure_borrows_arg.rs:+1:14: +1:15
18             debug _s => _9;              // in scope 2 at $DIR/inline_closure_borrows_arg.rs:+1:23: +1:25
+             let _10: &i32;               // in scope 2 at $DIR/inline_closure_borrows_arg.rs:+2:13: +2:21
19             scope 3 {
20                 debug variable => _8;    // in scope 3 at $DIR/inline_closure_borrows_arg.rs:+2:13: +2:21


45         _8 = move (_5.0: &i32);          // scope 1 at $DIR/inline_closure_borrows_arg.rs:+5:5: +5:12
46         StorageLive(_9);                 // scope 1 at $DIR/inline_closure_borrows_arg.rs:+5:5: +5:12
47         _9 = move (_5.1: &i32);          // scope 1 at $DIR/inline_closure_borrows_arg.rs:+5:5: +5:12
+         StorageLive(_10);                // scope 1 at $DIR/inline_closure_borrows_arg.rs:+5:5: +5:12
48         _0 = (*_8);                      // scope 3 at $DIR/inline_closure_borrows_arg.rs:+3:9: +3:18
+         StorageDead(_10);                // scope 1 at $DIR/inline_closure_borrows_arg.rs:+5:5: +5:12
49         StorageDead(_9);                 // scope 1 at $DIR/inline_closure_borrows_arg.rs:+5:5: +5:12
50         StorageDead(_8);                 // scope 1 at $DIR/inline_closure_borrows_arg.rs:+5:5: +5:12
51         StorageDead(_7);                 // scope 1 at $DIR/inline_closure_borrows_arg.rs:+5:11: +5:12

thread '[mir-opt] tests/mir-opt/inline/inline_closure_borrows_arg.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_closure_borrows_arg.foo.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/inline_diverging.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_diverging.rs stdout ----
6       let mut _1: !;                       // in scope 0 at $DIR/inline_diverging.rs:+0:12: +2:2
7       let _2: !;                           // in scope 0 at $DIR/inline_diverging.rs:+1:5: +1:12
8 +     scope 1 (inlined sleep) {            // at $DIR/inline_diverging.rs:8:5: 8:12
+ +         let mut _3: !;                   // in scope 1 at $DIR/inline_diverging.rs:38:17: 40:2
+ +         let mut _4: !;                   // in scope 1 at $DIR/inline_diverging.rs:39:5: 39:12
+ +         let mut _5: ();                  // in scope 1 at $DIR/inline_diverging.rs:38:1: 40:2
10   
11       bb0: {

14 -                                          // mir::Constant
14 -                                          // mir::Constant
15 -                                          // + span: $DIR/inline_diverging.rs:8:5: 8:10
16 -                                          // + literal: Const { ty: fn() -> ! {sleep}, val: Value(<ZST>) }
+ +         StorageLive(_3);                 // scope 0 at $DIR/inline_diverging.rs:+1:5: +1:12
+ +         StorageLive(_4);                 // scope 0 at $DIR/inline_diverging.rs:+1:5: +1:12
17 +         goto -> bb1;                     // scope 0 at $DIR/inline_diverging.rs:+1:5: +1:12
19 + 


thread '[mir-opt] tests/mir-opt/inline/inline_diverging.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_diverging.f.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/inline_cycle.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_cycle.rs stdout ----
5       let mut _0: ();                      // return place in scope 0 at $DIR/inline_cycle.rs:+0:10: +0:10
6       let _1: ();                          // in scope 0 at $DIR/inline_cycle.rs:+1:5: +1:12
7 +     let mut _2: fn() {f};                // in scope 0 at $DIR/inline_cycle.rs:+1:5: +1:12
- +     let mut _4: ();                      // in scope 0 at $DIR/inline_cycle.rs:55:5: 55:8
+ +     let mut _6: ();                      // in scope 0 at $DIR/inline_cycle.rs:55:5: 55:8
9 +     scope 1 (inlined call::<fn() {f}>) { // at $DIR/inline_cycle.rs:50:5: 50:12
10 +         debug f => _2;                   // in scope 1 at $DIR/inline_cycle.rs:54:22: 54:23
11 +         let _3: ();                      // in scope 1 at $DIR/inline_cycle.rs:55:5: 55:8

+ +         let mut _4: fn() {f};            // in scope 1 at $DIR/inline_cycle.rs:55:5: 55:6
+ +         let mut _5: ();                  // in scope 1 at $DIR/inline_cycle.rs:55:5: 55:8
12 +         scope 2 (inlined <fn() {f} as FnOnce<()>>::call_once - shim(fn() {f})) { // at $DIR/inline_cycle.rs:55:5: 55:8
14 +     }


25                                            // + span: $DIR/inline_cycle.rs:50:10: 50:11
26                                            // + literal: Const { ty: fn() {f}, val: Value(<ZST>) }
27 +         StorageLive(_3);                 // scope 0 at $DIR/inline_cycle.rs:+1:5: +1:12
- +         StorageLive(_4);                 // scope 1 at $DIR/inline_cycle.rs:55:5: 55:8
- +         _4 = const ();                   // scope 1 at $DIR/inline_cycle.rs:55:5: 55:8
+ +         StorageLive(_4);                 // scope 0 at $DIR/inline_cycle.rs:+1:5: +1:12
+ +         StorageLive(_5);                 // scope 0 at $DIR/inline_cycle.rs:+1:5: +1:12
+ +         StorageLive(_6);                 // scope 1 at $DIR/inline_cycle.rs:55:5: 55:8
+ +         _6 = const ();                   // scope 1 at $DIR/inline_cycle.rs:55:5: 55:8
30 +         _3 = move _2() -> bb1;           // scope 2 at $SRC_DIR/core/src/ops/function.rs:LL:COL
32   

33       bb1: {
33       bb1: {
- +         StorageDead(_4);                 // scope 1 at $DIR/inline_cycle.rs:55:5: 55:8
+ +         StorageDead(_6);                 // scope 1 at $DIR/inline_cycle.rs:55:5: 55:8
+ +         StorageDead(_5);                 // scope 0 at $DIR/inline_cycle.rs:+1:5: +1:12
+ +         StorageDead(_4);                 // scope 0 at $DIR/inline_cycle.rs:+1:5: +1:12
35 +         StorageDead(_3);                 // scope 0 at $DIR/inline_cycle.rs:+1:5: +1:12
36 +         StorageDead(_2);                 // scope 0 at $DIR/inline_cycle.rs:+1:5: +1:12
37           StorageDead(_1);                 // scope 0 at $DIR/inline_cycle.rs:+1:12: +1:13

thread '[mir-opt] tests/mir-opt/inline/inline_cycle.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_cycle.two.Inline.diff', src/tools/compiletest/src/runtest.rs:3639:21
---- [mir-opt] tests/mir-opt/inline/inline_generator.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_generator.rs stdout ----
