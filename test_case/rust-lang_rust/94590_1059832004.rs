plain
Suite(test::src/test/mir-opt) not skipped for "bootstrap::test::MirOpt" -- not in [src/tools/tidy]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 167 tests
....................................i..................................F...........i................ 100/167
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
........F.........................i..F....F.F.....F.F..............

---- [mir-opt] mir-opt/early_otherwise_branch_68867.rs stdout ----
84 -     bb2: {
84 -     bb2: {
85 +         StorageDead(_35);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:26:25: 26:27
86           StorageLive(_33);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:26:25: 26:27
- -         nop;                             // scope 0 at $DIR/early_otherwise_branch_68867.rs:26:21: 26:28
88           discriminant(_0) = 1;            // scope 0 at $DIR/early_otherwise_branch_68867.rs:26:21: 26:28
+ -         nop;                             // scope 0 at $DIR/early_otherwise_branch_68867.rs:26:21: 26:28
89           StorageDead(_33);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:26:27: 26:28
90           StorageDead(_3);                 // scope 0 at $DIR/early_otherwise_branch_68867.rs:27:6: 27:7
91           StorageDead(_4);                 // scope 0 at $DIR/early_otherwise_branch_68867.rs:28:1: 28:2

121           _14 = Add(move _15, move _16);   // scope 1 at $DIR/early_otherwise_branch_68867.rs:22:38: 22:49
122           StorageDead(_16);                // scope 1 at $DIR/early_otherwise_branch_68867.rs:22:48: 22:49
123           StorageDead(_15);                // scope 1 at $DIR/early_otherwise_branch_68867.rs:22:48: 22:49
-           ((_3 as Vw).0: f32) = move _14;  // scope 1 at $DIR/early_otherwise_branch_68867.rs:22:35: 22:50
125           discriminant(_3) = 0;            // scope 1 at $DIR/early_otherwise_branch_68867.rs:22:35: 22:50
+           ((_3 as Vw).0: f32) = move _14;  // scope 1 at $DIR/early_otherwise_branch_68867.rs:22:35: 22:50
126           StorageDead(_14);                // scope 1 at $DIR/early_otherwise_branch_68867.rs:22:49: 22:50
127           StorageDead(_13);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:22:49: 22:50
128           StorageDead(_12);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:22:49: 22:50

144           _19 = Add(move _20, move _21);   // scope 2 at $DIR/early_otherwise_branch_68867.rs:23:38: 23:49
145           StorageDead(_21);                // scope 2 at $DIR/early_otherwise_branch_68867.rs:23:48: 23:49
146           StorageDead(_20);                // scope 2 at $DIR/early_otherwise_branch_68867.rs:23:48: 23:49
-           ((_3 as Vh).0: f32) = move _19;  // scope 2 at $DIR/early_otherwise_branch_68867.rs:23:35: 23:50
148           discriminant(_3) = 1;            // scope 2 at $DIR/early_otherwise_branch_68867.rs:23:35: 23:50
+           ((_3 as Vh).0: f32) = move _19;  // scope 2 at $DIR/early_otherwise_branch_68867.rs:23:35: 23:50
149           StorageDead(_19);                // scope 2 at $DIR/early_otherwise_branch_68867.rs:23:49: 23:50
150           StorageDead(_18);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:23:49: 23:50
151           StorageDead(_17);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:23:49: 23:50

167           _24 = Add(move _25, move _26);   // scope 3 at $DIR/early_otherwise_branch_68867.rs:24:44: 24:55
168           StorageDead(_26);                // scope 3 at $DIR/early_otherwise_branch_68867.rs:24:54: 24:55
169           StorageDead(_25);                // scope 3 at $DIR/early_otherwise_branch_68867.rs:24:54: 24:55
-           ((_3 as Vmin).0: f32) = move _24; // scope 3 at $DIR/early_otherwise_branch_68867.rs:24:39: 24:56
171           discriminant(_3) = 2;            // scope 3 at $DIR/early_otherwise_branch_68867.rs:24:39: 24:56
+           ((_3 as Vmin).0: f32) = move _24; // scope 3 at $DIR/early_otherwise_branch_68867.rs:24:39: 24:56
172           StorageDead(_24);                // scope 3 at $DIR/early_otherwise_branch_68867.rs:24:55: 24:56
173           StorageDead(_23);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:24:55: 24:56
174           StorageDead(_22);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:24:55: 24:56

190           _29 = Add(move _30, move _31);   // scope 4 at $DIR/early_otherwise_branch_68867.rs:25:44: 25:55
191           StorageDead(_31);                // scope 4 at $DIR/early_otherwise_branch_68867.rs:25:54: 25:55
192           StorageDead(_30);                // scope 4 at $DIR/early_otherwise_branch_68867.rs:25:54: 25:55
-           ((_3 as Vmax).0: f32) = move _29; // scope 4 at $DIR/early_otherwise_branch_68867.rs:25:39: 25:56
194           discriminant(_3) = 3;            // scope 4 at $DIR/early_otherwise_branch_68867.rs:25:39: 25:56
+           ((_3 as Vmax).0: f32) = move _29; // scope 4 at $DIR/early_otherwise_branch_68867.rs:25:39: 25:56
195           StorageDead(_29);                // scope 4 at $DIR/early_otherwise_branch_68867.rs:25:55: 25:56
196           StorageDead(_28);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:25:55: 25:56
197           StorageDead(_27);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:25:55: 25:56
201   
202 -     bb10: {
203 +     bb6: {
203 +     bb6: {
-           ((_0 as Ok).0: ViewportPercentageLength) = move _3; // scope 0 at $DIR/early_otherwise_branch_68867.rs:21:5: 27:7
205           discriminant(_0) = 0;            // scope 0 at $DIR/early_otherwise_branch_68867.rs:21:5: 27:7
+           ((_0 as Ok).0: ViewportPercentageLength) = move _3; // scope 0 at $DIR/early_otherwise_branch_68867.rs:21:5: 27:7
206           StorageDead(_3);                 // scope 0 at $DIR/early_otherwise_branch_68867.rs:27:6: 27:7
207           StorageDead(_4);                 // scope 0 at $DIR/early_otherwise_branch_68867.rs:28:1: 28:2
208           return;                          // scope 0 at $DIR/early_otherwise_branch_68867.rs:28:2: 28:2

thread '[mir-opt] mir-opt/early_otherwise_branch_68867.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/early_otherwise_branch_68867.try_sum.EarlyOtherwiseBranch.diff', src/tools/compiletest/src/runtest.rs:3393:25

---- [mir-opt] mir-opt/issue-73223.rs stdout ----
51       bb0: {
51       bb0: {
52           StorageLive(_1);                 // scope 0 at $DIR/issue-73223.rs:2:9: 2:14
53           StorageLive(_2);                 // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
-           ((_2 as Some).0: i32) = const 1_i32; // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
55           discriminant(_2) = 1;            // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
+           ((_2 as Some).0: i32) = const 1_i32; // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
56           _3 = const 1_isize;              // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
57           goto -> bb2;                     // scope 0 at $DIR/issue-73223.rs:2:17: 2:30


73           StorageLive(_6);                 // scope 1 at $DIR/issue-73223.rs:7:9: 7:14
74           StorageLive(_7);                 // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
75           _7 = _1;                         // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
-           ((_6 as Some).0: i32) = move _7; // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
77           discriminant(_6) = 1;            // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
+           ((_6 as Some).0: i32) = move _7; // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
78           StorageDead(_7);                 // scope 1 at $DIR/issue-73223.rs:7:27: 7:28
79           StorageLive(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
80           StorageLive(_9);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

thread '[mir-opt] mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.64bit.diff', src/tools/compiletest/src/runtest.rs:3393:25
---- [mir-opt] mir-opt/simplify-arm-identity.rs stdout ----
19   
20       bb0: {
20       bb0: {
21           StorageLive(_1);                 // scope 0 at $DIR/simplify-arm-identity.rs:18:9: 18:10
-           ((_1 as Foo).0: u8) = const 0_u8; // scope 0 at $DIR/simplify-arm-identity.rs:18:18: 18:29
23           discriminant(_1) = 0;            // scope 0 at $DIR/simplify-arm-identity.rs:18:18: 18:29
+           ((_1 as Foo).0: u8) = const 0_u8; // scope 0 at $DIR/simplify-arm-identity.rs:18:18: 18:29
24           StorageLive(_2);                 // scope 1 at $DIR/simplify-arm-identity.rs:19:18: 22:6
25           _3 = const 0_isize;              // scope 1 at $DIR/simplify-arm-identity.rs:19:24: 19:25
26           goto -> bb3;                     // scope 1 at $DIR/simplify-arm-identity.rs:19:18: 19:25
27       }
28   
29       bb1: {
29       bb1: {
-           ((_2 as Foo).0: u8) = const 0_u8; // scope 1 at $DIR/simplify-arm-identity.rs:21:21: 21:32
31           discriminant(_2) = 0;            // scope 1 at $DIR/simplify-arm-identity.rs:21:21: 21:32
+           ((_2 as Foo).0: u8) = const 0_u8; // scope 1 at $DIR/simplify-arm-identity.rs:21:21: 21:32
32           goto -> bb4;                     // scope 1 at $DIR/simplify-arm-identity.rs:21:21: 21:32
34   


41           _4 = ((_1 as Foo).0: u8);        // scope 1 at $DIR/simplify-arm-identity.rs:20:18: 20:19
42           StorageLive(_5);                 // scope 3 at $DIR/simplify-arm-identity.rs:20:33: 20:34
43           _5 = _4;                         // scope 3 at $DIR/simplify-arm-identity.rs:20:33: 20:34
-           ((_2 as Foo).0: u8) = move _5;   // scope 3 at $DIR/simplify-arm-identity.rs:20:24: 20:35
45           discriminant(_2) = 0;            // scope 3 at $DIR/simplify-arm-identity.rs:20:24: 20:35
+           ((_2 as Foo).0: u8) = move _5;   // scope 3 at $DIR/simplify-arm-identity.rs:20:24: 20:35
46           StorageDead(_5);                 // scope 3 at $DIR/simplify-arm-identity.rs:20:34: 20:35
47           StorageDead(_4);                 // scope 1 at $DIR/simplify-arm-identity.rs:20:34: 20:35
48           goto -> bb4;                     // scope 1 at $DIR/simplify-arm-identity.rs:20:34: 20:35

thread '[mir-opt] mir-opt/simplify-arm-identity.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_arm_identity.main.SimplifyArmIdentity.64bit.diff', src/tools/compiletest/src/runtest.rs:3393:25
---- [mir-opt] mir-opt/separate_const_switch.rs stdout ----
---- [mir-opt] mir-opt/separate_const_switch.rs stdout ----
38           _6 = ((_1 as Err).0: usize);     // scope 0 at $DIR/separate_const_switch.rs:17:17: 17:18
39           StorageLive(_7);                 // scope 2 at $DIR/separate_const_switch.rs:17:42: 17:43
40           _7 = _6;                         // scope 2 at $DIR/separate_const_switch.rs:17:42: 17:43
-           ((_2 as Break).0: usize) = move _7; // scope 2 at $DIR/separate_const_switch.rs:17:23: 17:44
42           discriminant(_2) = 1;            // scope 2 at $DIR/separate_const_switch.rs:17:23: 17:44
+           ((_2 as Break).0: usize) = move _7; // scope 2 at $DIR/separate_const_switch.rs:17:23: 17:44
43           StorageDead(_7);                 // scope 2 at $DIR/separate_const_switch.rs:17:43: 17:44
44           StorageDead(_6);                 // scope 0 at $DIR/separate_const_switch.rs:17:43: 17:44
45 -         goto -> bb3;                     // scope 0 at $DIR/separate_const_switch.rs:17:43: 17:44

52           _4 = ((_1 as Ok).0: i32);        // scope 0 at $DIR/separate_const_switch.rs:16:16: 16:17
53           StorageLive(_5);                 // scope 1 at $DIR/separate_const_switch.rs:16:44: 16:45
54           _5 = _4;                         // scope 1 at $DIR/separate_const_switch.rs:16:44: 16:45
-           ((_2 as Continue).0: i32) = move _5; // scope 1 at $DIR/separate_const_switch.rs:16:22: 16:46
56           discriminant(_2) = 0;            // scope 1 at $DIR/separate_const_switch.rs:16:22: 16:46
+           ((_2 as Continue).0: i32) = move _5; // scope 1 at $DIR/separate_const_switch.rs:16:22: 16:46
57           StorageDead(_5);                 // scope 1 at $DIR/separate_const_switch.rs:16:45: 16:46
58           StorageDead(_4);                 // scope 0 at $DIR/separate_const_switch.rs:16:45: 16:46
59 -         goto -> bb3;                     // scope 0 at $DIR/separate_const_switch.rs:16:45: 16:46

81           _9 = ((_2 as Continue).0: i32);  // scope 0 at $DIR/separate_const_switch.rs:20:31: 20:32
82           StorageLive(_10);                // scope 3 at $DIR/separate_const_switch.rs:20:42: 20:43
83           _10 = _9;                        // scope 3 at $DIR/separate_const_switch.rs:20:42: 20:43
-           ((_0 as Some).0: i32) = move _10; // scope 3 at $DIR/separate_const_switch.rs:20:37: 20:44
85           discriminant(_0) = 1;            // scope 3 at $DIR/separate_const_switch.rs:20:37: 20:44
+           ((_0 as Some).0: i32) = move _10; // scope 3 at $DIR/separate_const_switch.rs:20:37: 20:44
86           StorageDead(_10);                // scope 3 at $DIR/separate_const_switch.rs:20:43: 20:44
87           StorageDead(_9);                 // scope 0 at $DIR/separate_const_switch.rs:20:43: 20:44
88 -         goto -> bb6;                     // scope 0 at $DIR/separate_const_switch.rs:20:43: 20:44

thread '[mir-opt] mir-opt/separate_const_switch.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/separate_const_switch.too_complex.SeparateConstSwitch.diff', src/tools/compiletest/src/runtest.rs:3393:25
---- [mir-opt] mir-opt/simplify-locals-removes-unused-discriminant-reads.rs stdout ----
---- [mir-opt] mir-opt/simplify-locals-removes-unused-discriminant-reads.rs stdout ----
4   fn map(_1: Option<Box<()>>) -> Option<Box<()>> {
5       debug x => _1;                       // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:3:8: 3:9
6       let mut _0: std::option::Option<std::boxed::Box<()>>; // return place in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:3:31: 3:46
- -     let mut _2: isize;                   // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:5:9: 5:13
+       let mut _2: isize;                   // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:5:9: 5:13
8 -     let _3: std::boxed::Box<()>;         // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:14: 6:15
9 -     let mut _4: std::boxed::Box<()>;     // in scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:25: 6:26
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


18 -         _5 = const false;                // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:4:11: 4:12
19 -         _5 = const true;                 // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:4:11: 4:12
- -         _2 = discriminant(_1);           // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:4:11: 4:12
-           _0 = move _1;                    // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:20: 6:27
+           _2 = discriminant(_1);           // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:4:11: 4:12
+           switchInt(move _2) -> [0_isize: bb2, otherwise: bb1]; // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:4:5: 4:12
+   
+       bb1: {
+       bb1: {
+ -         _4 = move ((_1 as Some).0: std::boxed::Box<()>); // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:14: 6:15
+ +         _3 = move ((_1 as Some).0: std::boxed::Box<()>); // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:14: 6:15
+           discriminant(_0) = 1;            // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:20: 6:27
+ -         ((_0 as Some).0: std::boxed::Box<()>) = move _4; // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:20: 6:27
+ +         ((_0 as Some).0: std::boxed::Box<()>) = move _3; // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:20: 6:27
+           goto -> bb3;                     // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:26: 6:27
+   
+       bb2: {
+       bb2: {
+           discriminant(_0) = 0;            // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:5:17: 5:21
+           goto -> bb3;                     // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:5:17: 5:21
+   
+       bb3: {
+       bb3: {
22 -         _6 = discriminant(_1);           // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:8:1: 8:2
23           return;                          // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:8:2: 8:2


thread '[mir-opt] mir-opt/simplify-locals-removes-unused-discriminant-reads.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_locals_removes_unused_discriminant_reads.map.SimplifyLocals.64bit.diff', src/tools/compiletest/src/runtest.rs:3393:25
---- [mir-opt] mir-opt/simplify-arm.rs stdout ----
---- [mir-opt] mir-opt/simplify-arm.rs stdout ----
8       let _3: u8;                          // in scope 0 at $DIR/simplify-arm.rs:11:14: 11:15
9       let mut _4: u8;                      // in scope 0 at $DIR/simplify-arm.rs:11:25: 11:26
10       scope 1 {
- -         debug v => _3;                   // in scope 1 at $DIR/simplify-arm.rs:11:14: 11:15
- +         debug v => ((_0 as Some).0: u8); // in scope 1 at $DIR/simplify-arm.rs:11:14: 11:15
+           debug v => _3;                   // in scope 1 at $DIR/simplify-arm.rs:11:14: 11:15
14   
15       bb0: {

27       }
27       }
28   
29       bb3: {
- -         StorageLive(_3);                 // scope 0 at $DIR/simplify-arm.rs:11:14: 11:15
- -         _3 = ((_1 as Some).0: u8);       // scope 0 at $DIR/simplify-arm.rs:11:14: 11:15
- -         StorageLive(_4);                 // scope 1 at $DIR/simplify-arm.rs:11:25: 11:26
- -         _4 = _3;                         // scope 1 at $DIR/simplify-arm.rs:11:25: 11:26
- -         ((_0 as Some).0: u8) = move _4;  // scope 1 at $DIR/simplify-arm.rs:11:20: 11:27
- -         discriminant(_0) = 1;            // scope 1 at $DIR/simplify-arm.rs:11:20: 11:27
- -         StorageDead(_4);                 // scope 1 at $DIR/simplify-arm.rs:11:26: 11:27
- -         StorageDead(_3);                 // scope 0 at $DIR/simplify-arm.rs:11:26: 11:27
- +         _0 = move _1;                    // scope 1 at $DIR/simplify-arm.rs:11:20: 11:27
+           StorageLive(_3);                 // scope 0 at $DIR/simplify-arm.rs:11:14: 11:15
+           _3 = ((_1 as Some).0: u8);       // scope 0 at $DIR/simplify-arm.rs:11:14: 11:15
+           StorageLive(_4);                 // scope 1 at $DIR/simplify-arm.rs:11:25: 11:26
+           _4 = _3;                         // scope 1 at $DIR/simplify-arm.rs:11:25: 11:26
+           discriminant(_0) = 1;            // scope 1 at $DIR/simplify-arm.rs:11:20: 11:27
+           ((_0 as Some).0: u8) = move _4;  // scope 1 at $DIR/simplify-arm.rs:11:20: 11:27
+           StorageDead(_4);                 // scope 1 at $DIR/simplify-arm.rs:11:26: 11:27
+           StorageDead(_3);                 // scope 0 at $DIR/simplify-arm.rs:11:26: 11:27
39           goto -> bb4;                     // scope 0 at $DIR/simplify-arm.rs:11:26: 11:27
41   


thread '[mir-opt] mir-opt/simplify-arm.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_arm.id.SimplifyArmIdentity.diff', src/tools/compiletest/src/runtest.rs:3393:25
---- [mir-opt] mir-opt/simplify_try.rs stdout ----
---- [mir-opt] mir-opt/simplify_try.rs stdout ----
15       let _10: u32;                        // in scope 0 at $DIR/simplify_try.rs:23:12: 23:13
16       let mut _11: u32;                    // in scope 0 at $DIR/simplify_try.rs:25:8: 25:9
17       scope 1 {
- -         debug y => _2;                   // in scope 1 at $DIR/simplify_try.rs:21:9: 21:10
- +         debug y => ((_0 as Ok).0: u32);  // in scope 1 at $DIR/simplify_try.rs:21:9: 21:10
+           debug y => _2;                   // in scope 1 at $DIR/simplify_try.rs:21:9: 21:10
21       scope 2 {
21       scope 2 {
- -         debug e => _6;                   // in scope 2 at $DIR/simplify_try.rs:22:13: 22:14
- +         debug e => ((_0 as Err).0: i32); // in scope 2 at $DIR/simplify_try.rs:22:13: 22:14
+           debug e => _6;                   // in scope 2 at $DIR/simplify_try.rs:22:13: 22:14
24           scope 5 (inlined <i32 as From<i32>>::from) { // at $DIR/simplify_try.rs:22:37: 22:50
- -             debug t => _9;               // in scope 5 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
- +             debug t => ((_0 as Err).0: i32); // in scope 5 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
+               debug t => _9;               // in scope 5 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
27           }
28           scope 6 (inlined from_error::<u32, i32>) { // at $DIR/simplify_try.rs:22:26: 22:51
- -             debug e => _8;               // in scope 6 at $DIR/simplify_try.rs:12:21: 12:22
- +             debug e => ((_0 as Err).0: i32); // in scope 6 at $DIR/simplify_try.rs:12:21: 12:22
+               debug e => _8;               // in scope 6 at $DIR/simplify_try.rs:12:21: 12:22
+               let mut _12: i32;            // in scope 6 at $DIR/simplify_try.rs:13:9: 13:10
32       }
33       scope 3 {


- -         debug v => _10;                  // in scope 3 at $DIR/simplify_try.rs:23:12: 23:13
- +         debug v => ((_0 as Ok).0: u32);  // in scope 3 at $DIR/simplify_try.rs:23:12: 23:13
+           debug v => _10;                  // in scope 3 at $DIR/simplify_try.rs:23:12: 23:13
36       }
37       scope 4 (inlined into_result::<u32, i32>) { // at $DIR/simplify_try.rs:21:19: 21:33
38           debug r => _4;                   // in scope 4 at $DIR/simplify_try.rs:8:22: 8:23
50       }
51   
52       bb1: {
52       bb1: {
- -         StorageLive(_10);                // scope 0 at $DIR/simplify_try.rs:23:12: 23:13
- -         _10 = ((_3 as Ok).0: u32);       // scope 0 at $DIR/simplify_try.rs:23:12: 23:13
- -         _2 = _10;                        // scope 3 at $DIR/simplify_try.rs:23:18: 23:19
- -         StorageDead(_10);                // scope 0 at $DIR/simplify_try.rs:23:18: 23:19
- +         _0 = move _3;                    // scope 1 at $DIR/simplify_try.rs:25:5: 25:10
+           StorageLive(_10);                // scope 0 at $DIR/simplify_try.rs:23:12: 23:13
+           _10 = ((_3 as Ok).0: u32);       // scope 0 at $DIR/simplify_try.rs:23:12: 23:13
+           _2 = _10;                        // scope 3 at $DIR/simplify_try.rs:23:18: 23:19
+           StorageDead(_10);                // scope 0 at $DIR/simplify_try.rs:23:18: 23:19
58           StorageDead(_3);                 // scope 0 at $DIR/simplify_try.rs:24:6: 24:7
- -         StorageLive(_11);                // scope 1 at $DIR/simplify_try.rs:25:8: 25:9
- -         _11 = _2;                        // scope 1 at $DIR/simplify_try.rs:25:8: 25:9
- -         ((_0 as Ok).0: u32) = move _11;  // scope 1 at $DIR/simplify_try.rs:25:5: 25:10
- -         discriminant(_0) = 0;            // scope 1 at $DIR/simplify_try.rs:25:5: 25:10
- -         StorageDead(_11);                // scope 1 at $DIR/simplify_try.rs:25:9: 25:10
+           StorageLive(_11);                // scope 1 at $DIR/simplify_try.rs:25:8: 25:9
+           _11 = _2;                        // scope 1 at $DIR/simplify_try.rs:25:8: 25:9
+           discriminant(_0) = 0;            // scope 1 at $DIR/simplify_try.rs:25:5: 25:10
+           ((_0 as Ok).0: u32) = move _11;  // scope 1 at $DIR/simplify_try.rs:25:5: 25:10
+           StorageDead(_11);                // scope 1 at $DIR/simplify_try.rs:25:9: 25:10
64           StorageDead(_2);                 // scope 0 at $DIR/simplify_try.rs:26:1: 26:2
65           return;                          // scope 0 at $DIR/simplify_try.rs:26:2: 26:2

67   
68       bb2: {
68       bb2: {
- -         StorageLive(_6);                 // scope 0 at $DIR/simplify_try.rs:22:13: 22:14
- -         _6 = ((_3 as Err).0: i32);       // scope 0 at $DIR/simplify_try.rs:22:13: 22:14
- -         StorageLive(_8);                 // scope 2 at $DIR/simplify_try.rs:22:37: 22:50
- -         StorageLive(_9);                 // scope 2 at $DIR/simplify_try.rs:22:48: 22:49
- -         _9 = _6;                         // scope 2 at $DIR/simplify_try.rs:22:48: 22:49
- -         _8 = move _9;                    // scope 5 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
- -         StorageDead(_9);                 // scope 2 at $DIR/simplify_try.rs:22:49: 22:50
- -         ((_0 as Err).0: i32) = move _8;  // scope 6 at $DIR/simplify_try.rs:13:9: 13:10
- -         discriminant(_0) = 1;            // scope 6 at $DIR/simplify_try.rs:13:5: 13:11
- -         StorageDead(_8);                 // scope 2 at $DIR/simplify_try.rs:22:50: 22:51
- -         StorageDead(_6);                 // scope 0 at $DIR/simplify_try.rs:22:50: 22:51
- +         _0 = move _3;                    // scope 6 at $DIR/simplify_try.rs:13:5: 13:11
+           StorageLive(_6);                 // scope 0 at $DIR/simplify_try.rs:22:13: 22:14
+           _6 = ((_3 as Err).0: i32);       // scope 0 at $DIR/simplify_try.rs:22:13: 22:14
+           StorageLive(_8);                 // scope 2 at $DIR/simplify_try.rs:22:37: 22:50
+           StorageLive(_9);                 // scope 2 at $DIR/simplify_try.rs:22:48: 22:49
+           _9 = _6;                         // scope 2 at $DIR/simplify_try.rs:22:48: 22:49
+           _8 = move _9;                    // scope 5 at $SRC_DIR/core/src/convert/mod.rs:LL:COL
+           StorageDead(_9);                 // scope 2 at $DIR/simplify_try.rs:22:49: 22:50
+           StorageLive(_12);                // scope 6 at $DIR/simplify_try.rs:13:9: 13:10
+           _12 = move _8;                   // scope 6 at $DIR/simplify_try.rs:13:9: 13:10
+           discriminant(_0) = 1;            // scope 6 at $DIR/simplify_try.rs:13:5: 13:11
+           ((_0 as Err).0: i32) = move _12; // scope 6 at $DIR/simplify_try.rs:13:5: 13:11
+           StorageDead(_12);                // scope 6 at $DIR/simplify_try.rs:13:10: 13:11
+           StorageDead(_8);                 // scope 2 at $DIR/simplify_try.rs:22:50: 22:51
+           StorageDead(_6);                 // scope 0 at $DIR/simplify_try.rs:22:50: 22:51
81           StorageDead(_3);                 // scope 0 at $DIR/simplify_try.rs:24:6: 24:7
82           StorageDead(_2);                 // scope 0 at $DIR/simplify_try.rs:26:1: 26:2
83           return;                          // scope 0 at $DIR/simplify_try.rs:26:2: 26:2

thread '[mir-opt] mir-opt/simplify_try.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_try.try_identity.SimplifyArmIdentity.diff', src/tools/compiletest/src/runtest.rs:3393:25

failures:
    [mir-opt] mir-opt/early_otherwise_branch_68867.rs
    [mir-opt] mir-opt/issue-73223.rs
