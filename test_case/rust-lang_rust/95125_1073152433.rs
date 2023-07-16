plain
Suite(test::src/test/mir-opt) not skipped for "bootstrap::test::MirOpt" -- not in [src/tools/tidy]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 168 tests
....F.........F...........F..........i................F.F...F........FF.........F...i..F............ 100/168
.......F....F......................i.FF...F.F.F..F..F....F.F......FF
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] mir-opt/const_debuginfo.rs stdout ----
---- [mir-opt] mir-opt/const_debuginfo.rs stdout ----
83           (_10.1: bool) = const false;     // scope 5 at $DIR/const_debuginfo.rs:16:13: 16:34
84           (_10.2: u32) = const 123_u32;    // scope 5 at $DIR/const_debuginfo.rs:16:13: 16:34
85           StorageLive(_11);                // scope 6 at $DIR/const_debuginfo.rs:18:9: 18:10
+           _11 = UninitVariant(1);          // scope 6 at $DIR/const_debuginfo.rs:18:13: 18:24
86           ((_11 as Some).0: u16) = const 99_u16; // scope 6 at $DIR/const_debuginfo.rs:18:13: 18:24
-           discriminant(_11) = 1;           // scope 6 at $DIR/const_debuginfo.rs:18:13: 18:24
88           StorageLive(_12);                // scope 7 at $DIR/const_debuginfo.rs:20:9: 20:10
89           (_12.0: u32) = const 32_u32;     // scope 7 at $DIR/const_debuginfo.rs:20:13: 20:35
90           (_12.1: u32) = const 32_u32;     // scope 7 at $DIR/const_debuginfo.rs:20:13: 20:35

thread '[mir-opt] mir-opt/const_debuginfo.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_debuginfo.main.ConstDebugInfo.diff', src/tools/compiletest/src/runtest.rs:3406:25

---- [mir-opt] mir-opt/76803_regression.rs stdout ----
17       }
18   
18   
19       bb2: {
-           discriminant(_0) = 1;            // scope 0 at $DIR/76803_regression.rs:12:20: 12:27
+           _0 = const Type::B;              // scope 0 at $DIR/76803_regression.rs:12:20: 12:27
+                                            // mir::Constant
+                                            // + span: $DIR/76803_regression.rs:12:20: 12:27
+                                            // + literal: Const { ty: Type, val: Value(Scalar(0x01)) }
21           goto -> bb3;                     // scope 0 at $DIR/76803_regression.rs:12:20: 12:27
23   


thread '[mir-opt] mir-opt/76803_regression.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/76803_regression.encode.SimplifyBranchSame.diff', src/tools/compiletest/src/runtest.rs:3406:25
---- [mir-opt] mir-opt/const_prop/discriminant.rs stdout ----
---- [mir-opt] mir-opt/const_prop/discriminant.rs stdout ----
15           StorageLive(_1);                 // scope 0 at $DIR/discriminant.rs:11:9: 11:10
16           StorageLive(_2);                 // scope 0 at $DIR/discriminant.rs:11:13: 11:64
17           StorageLive(_3);                 // scope 0 at $DIR/discriminant.rs:11:34: 11:44
+           _3 = UninitVariant(1);           // scope 0 at $DIR/discriminant.rs:11:34: 11:44
18           ((_3 as Some).0: bool) = const true; // scope 0 at $DIR/discriminant.rs:11:34: 11:44
-           discriminant(_3) = 1;            // scope 0 at $DIR/discriminant.rs:11:34: 11:44
20 -         _4 = discriminant(_3);           // scope 0 at $DIR/discriminant.rs:11:21: 11:31
21 -         switchInt(move _4) -> [1_isize: bb1, otherwise: bb3]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
22 +         _4 = const 1_isize;              // scope 0 at $DIR/discriminant.rs:11:21: 11:31

thread '[mir-opt] mir-opt/const_prop/discriminant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/discriminant.main.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3406:25
---- [mir-opt] mir-opt/deaggregator_test_multiple.rs stdout ----
---- [mir-opt] mir-opt/deaggregator_test_multiple.rs stdout ----
14           StorageLive(_3);                 // scope 0 at $DIR/deaggregator_test_multiple.rs:10:13: 10:14
15           _3 = _1;                         // scope 0 at $DIR/deaggregator_test_multiple.rs:10:13: 10:14
16 -         _2 = Foo::A(move _3);            // scope 0 at $DIR/deaggregator_test_multiple.rs:10:6: 10:15
+ +         _2 = UninitVariant(0);           // scope 0 at $DIR/deaggregator_test_multiple.rs:10:6: 10:15
17 +         ((_2 as A).0: i32) = move _3;    // scope 0 at $DIR/deaggregator_test_multiple.rs:10:6: 10:15
- +         discriminant(_2) = 0;            // scope 0 at $DIR/deaggregator_test_multiple.rs:10:6: 10:15
19           StorageDead(_3);                 // scope 0 at $DIR/deaggregator_test_multiple.rs:10:14: 10:15
20           StorageLive(_4);                 // scope 0 at $DIR/deaggregator_test_multiple.rs:10:17: 10:26
21           StorageLive(_5);                 // scope 0 at $DIR/deaggregator_test_multiple.rs:10:24: 10:25

22           _5 = _1;                         // scope 0 at $DIR/deaggregator_test_multiple.rs:10:24: 10:25
23 -         _4 = Foo::A(move _5);            // scope 0 at $DIR/deaggregator_test_multiple.rs:10:17: 10:26
+ +         _4 = UninitVariant(0);           // scope 0 at $DIR/deaggregator_test_multiple.rs:10:17: 10:26
24 +         ((_4 as A).0: i32) = move _5;    // scope 0 at $DIR/deaggregator_test_multiple.rs:10:17: 10:26
- +         discriminant(_4) = 0;            // scope 0 at $DIR/deaggregator_test_multiple.rs:10:17: 10:26
26           StorageDead(_5);                 // scope 0 at $DIR/deaggregator_test_multiple.rs:10:25: 10:26
27           _0 = [move _2, move _4];         // scope 0 at $DIR/deaggregator_test_multiple.rs:10:5: 10:27
28           StorageDead(_4);                 // scope 0 at $DIR/deaggregator_test_multiple.rs:10:26: 10:27

thread '[mir-opt] mir-opt/deaggregator_test_multiple.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/deaggregator_test_multiple.test.Deaggregator.diff', src/tools/compiletest/src/runtest.rs:3406:25
---- [mir-opt] mir-opt/deaggregator_test_enum_2.rs stdout ----
---- [mir-opt] mir-opt/deaggregator_test_enum_2.rs stdout ----
19           StorageLive(_4);                 // scope 0 at $DIR/deaggregator_test_enum_2.rs:11:16: 11:17
20           _4 = _2;                         // scope 0 at $DIR/deaggregator_test_enum_2.rs:11:16: 11:17
21 -         _0 = Foo::A(move _4);            // scope 0 at $DIR/deaggregator_test_enum_2.rs:11:9: 11:18
+ +         _0 = UninitVariant(0);           // scope 0 at $DIR/deaggregator_test_enum_2.rs:11:9: 11:18
22 +         ((_0 as A).0: i32) = move _4;    // scope 0 at $DIR/deaggregator_test_enum_2.rs:11:9: 11:18
- +         discriminant(_0) = 0;            // scope 0 at $DIR/deaggregator_test_enum_2.rs:11:9: 11:18
24           StorageDead(_4);                 // scope 0 at $DIR/deaggregator_test_enum_2.rs:11:17: 11:18
25           goto -> bb3;                     // scope 0 at $DIR/deaggregator_test_enum_2.rs:10:5: 14:6


29           StorageLive(_5);                 // scope 0 at $DIR/deaggregator_test_enum_2.rs:13:16: 13:17
30           _5 = _2;                         // scope 0 at $DIR/deaggregator_test_enum_2.rs:13:16: 13:17
31 -         _0 = Foo::B(move _5);            // scope 0 at $DIR/deaggregator_test_enum_2.rs:13:9: 13:18
+ +         _0 = UninitVariant(1);           // scope 0 at $DIR/deaggregator_test_enum_2.rs:13:9: 13:18
32 +         ((_0 as B).0: i32) = move _5;    // scope 0 at $DIR/deaggregator_test_enum_2.rs:13:9: 13:18
- +         discriminant(_0) = 1;            // scope 0 at $DIR/deaggregator_test_enum_2.rs:13:9: 13:18
34           StorageDead(_5);                 // scope 0 at $DIR/deaggregator_test_enum_2.rs:13:17: 13:18
35           goto -> bb3;                     // scope 0 at $DIR/deaggregator_test_enum_2.rs:10:5: 14:6


thread '[mir-opt] mir-opt/deaggregator_test_enum_2.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/deaggregator_test_enum_2.test1.Deaggregator.diff', src/tools/compiletest/src/runtest.rs:3406:25
---- [mir-opt] mir-opt/deaggregator_test_enum.rs stdout ----
---- [mir-opt] mir-opt/deaggregator_test_enum.rs stdout ----
10           StorageLive(_2);                 // scope 0 at $DIR/deaggregator_test_enum.rs:8:19: 8:20
11           _2 = _1;                         // scope 0 at $DIR/deaggregator_test_enum.rs:8:19: 8:20
12 -         _0 = Baz::Foo { x: move _2 };    // scope 0 at $DIR/deaggregator_test_enum.rs:8:5: 8:22
+ +         _0 = UninitVariant(1);           // scope 0 at $DIR/deaggregator_test_enum.rs:8:5: 8:22
13 +         ((_0 as Foo).0: usize) = move _2; // scope 0 at $DIR/deaggregator_test_enum.rs:8:5: 8:22
- +         discriminant(_0) = 1;            // scope 0 at $DIR/deaggregator_test_enum.rs:8:5: 8:22
15           StorageDead(_2);                 // scope 0 at $DIR/deaggregator_test_enum.rs:8:21: 8:22
16           return;                          // scope 0 at $DIR/deaggregator_test_enum.rs:9:2: 9:2


thread '[mir-opt] mir-opt/deaggregator_test_enum.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/deaggregator_test_enum.bar.Deaggregator.diff', src/tools/compiletest/src/runtest.rs:3406:25
---- [mir-opt] mir-opt/funky_arms.rs stdout ----
51       }
52   
53       bb2: {
53       bb2: {
-           discriminant(_6) = 1;            // scope 1 at $DIR/funky_arms.rs:21:17: 21:41
+ -         _6 = UninitVariant(1);           // scope 1 at $DIR/funky_arms.rs:21:17: 21:41
+ +         _6 = const MinusPlus;            // scope 1 at $DIR/funky_arms.rs:21:17: 21:41
+ +                                          // mir::Constant
+ +                                          // + span: $DIR/funky_arms.rs:21:17: 21:41
+ +                                          // + literal: Const { ty: Sign, val: Value(Scalar(0x01)) }
55           goto -> bb4;                     // scope 1 at $DIR/funky_arms.rs:21:17: 21:41
57   

58       bb3: {
58       bb3: {
-           discriminant(_6) = 0;            // scope 1 at $DIR/funky_arms.rs:20:18: 20:38
+ -         _6 = UninitVariant(0);           // scope 1 at $DIR/funky_arms.rs:20:18: 20:38
+ +         _6 = const Minus;                // scope 1 at $DIR/funky_arms.rs:20:18: 20:38
+ +                                          // mir::Constant
+ +                                          // + span: $DIR/funky_arms.rs:20:18: 20:38
+ +                                          // + literal: Const { ty: Sign, val: Value(Scalar(0x00)) }
60           goto -> bb4;                     // scope 1 at $DIR/funky_arms.rs:20:18: 20:38
62   


thread '[mir-opt] mir-opt/funky_arms.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/funky_arms.float_to_exponential_common.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3406:25
---- [mir-opt] mir-opt/early_otherwise_branch_68867.rs stdout ----
84 -     bb2: {
84 -     bb2: {
85 +         StorageDead(_35);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:26:25: 26:27
86           StorageLive(_33);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:26:25: 26:27
+           _0 = UninitVariant(1);           // scope 0 at $DIR/early_otherwise_branch_68867.rs:26:21: 26:28
87 -         nop;                             // scope 0 at $DIR/early_otherwise_branch_68867.rs:26:21: 26:28
-           discriminant(_0) = 1;            // scope 0 at $DIR/early_otherwise_branch_68867.rs:26:21: 26:28
89           StorageDead(_33);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:26:27: 26:28
90           StorageDead(_3);                 // scope 0 at $DIR/early_otherwise_branch_68867.rs:27:6: 27:7
91           StorageDead(_4);                 // scope 0 at $DIR/early_otherwise_branch_68867.rs:28:1: 28:2

121           _14 = Add(move _15, move _16);   // scope 1 at $DIR/early_otherwise_branch_68867.rs:22:38: 22:49
122           StorageDead(_16);                // scope 1 at $DIR/early_otherwise_branch_68867.rs:22:48: 22:49
123           StorageDead(_15);                // scope 1 at $DIR/early_otherwise_branch_68867.rs:22:48: 22:49
+           _3 = UninitVariant(0);           // scope 1 at $DIR/early_otherwise_branch_68867.rs:22:35: 22:50
124           ((_3 as Vw).0: f32) = move _14;  // scope 1 at $DIR/early_otherwise_branch_68867.rs:22:35: 22:50
-           discriminant(_3) = 0;            // scope 1 at $DIR/early_otherwise_branch_68867.rs:22:35: 22:50
126           StorageDead(_14);                // scope 1 at $DIR/early_otherwise_branch_68867.rs:22:49: 22:50
127           StorageDead(_13);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:22:49: 22:50
128           StorageDead(_12);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:22:49: 22:50

144           _19 = Add(move _20, move _21);   // scope 2 at $DIR/early_otherwise_branch_68867.rs:23:38: 23:49
145           StorageDead(_21);                // scope 2 at $DIR/early_otherwise_branch_68867.rs:23:48: 23:49
146           StorageDead(_20);                // scope 2 at $DIR/early_otherwise_branch_68867.rs:23:48: 23:49
+           _3 = UninitVariant(1);           // scope 2 at $DIR/early_otherwise_branch_68867.rs:23:35: 23:50
147           ((_3 as Vh).0: f32) = move _19;  // scope 2 at $DIR/early_otherwise_branch_68867.rs:23:35: 23:50
-           discriminant(_3) = 1;            // scope 2 at $DIR/early_otherwise_branch_68867.rs:23:35: 23:50
149           StorageDead(_19);                // scope 2 at $DIR/early_otherwise_branch_68867.rs:23:49: 23:50
150           StorageDead(_18);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:23:49: 23:50
151           StorageDead(_17);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:23:49: 23:50

167           _24 = Add(move _25, move _26);   // scope 3 at $DIR/early_otherwise_branch_68867.rs:24:44: 24:55
168           StorageDead(_26);                // scope 3 at $DIR/early_otherwise_branch_68867.rs:24:54: 24:55
169           StorageDead(_25);                // scope 3 at $DIR/early_otherwise_branch_68867.rs:24:54: 24:55
+           _3 = UninitVariant(2);           // scope 3 at $DIR/early_otherwise_branch_68867.rs:24:39: 24:56
170           ((_3 as Vmin).0: f32) = move _24; // scope 3 at $DIR/early_otherwise_branch_68867.rs:24:39: 24:56
-           discriminant(_3) = 2;            // scope 3 at $DIR/early_otherwise_branch_68867.rs:24:39: 24:56
172           StorageDead(_24);                // scope 3 at $DIR/early_otherwise_branch_68867.rs:24:55: 24:56
173           StorageDead(_23);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:24:55: 24:56
174           StorageDead(_22);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:24:55: 24:56

190           _29 = Add(move _30, move _31);   // scope 4 at $DIR/early_otherwise_branch_68867.rs:25:44: 25:55
191           StorageDead(_31);                // scope 4 at $DIR/early_otherwise_branch_68867.rs:25:54: 25:55
192           StorageDead(_30);                // scope 4 at $DIR/early_otherwise_branch_68867.rs:25:54: 25:55
+           _3 = UninitVariant(3);           // scope 4 at $DIR/early_otherwise_branch_68867.rs:25:39: 25:56
193           ((_3 as Vmax).0: f32) = move _29; // scope 4 at $DIR/early_otherwise_branch_68867.rs:25:39: 25:56
-           discriminant(_3) = 3;            // scope 4 at $DIR/early_otherwise_branch_68867.rs:25:39: 25:56
195           StorageDead(_29);                // scope 4 at $DIR/early_otherwise_branch_68867.rs:25:55: 25:56
196           StorageDead(_28);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:25:55: 25:56
197           StorageDead(_27);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:25:55: 25:56
201   
202 -     bb10: {
203 +     bb6: {
203 +     bb6: {
+           _0 = UninitVariant(0);           // scope 0 at $DIR/early_otherwise_branch_68867.rs:21:5: 27:7
204           ((_0 as Ok).0: ViewportPercentageLength) = move _3; // scope 0 at $DIR/early_otherwise_branch_68867.rs:21:5: 27:7
-           discriminant(_0) = 0;            // scope 0 at $DIR/early_otherwise_branch_68867.rs:21:5: 27:7
206           StorageDead(_3);                 // scope 0 at $DIR/early_otherwise_branch_68867.rs:27:6: 27:7
207           StorageDead(_4);                 // scope 0 at $DIR/early_otherwise_branch_68867.rs:28:1: 28:2
208           return;                          // scope 0 at $DIR/early_otherwise_branch_68867.rs:28:2: 28:2

thread '[mir-opt] mir-opt/early_otherwise_branch_68867.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/early_otherwise_branch_68867.try_sum.EarlyOtherwiseBranch.diff', src/tools/compiletest/src/runtest.rs:3406:25
---- [mir-opt] mir-opt/generator-tiny.rs stdout ----
41     bb2: {
41     bb2: {
42         StorageLive(_6);                 // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
43         StorageLive(_7);                 // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
+         _0 = UninitVariant(0);           // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
44         ((_0 as Yielded).0: ()) = move _7; // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
-         discriminant(_0) = 0;            // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
46         discriminant((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:19:16: 25:6]))) = 3; // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
47         return;                          // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18


thread '[mir-opt] mir-opt/generator-tiny.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/generator_tiny.main-{closure#0}.generator_resume.0.mir', src/tools/compiletest/src/runtest.rs:3406:25
---- [mir-opt] mir-opt/inline/inline-generator.rs stdout ----
113 + 
114 +     bb6: {
114 +     bb6: {
115 +         StorageDead(_9);                 // scope 6 at $DIR/inline-generator.rs:15:38: 15:39
+ +         _1 = UninitVariant(0);           // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
116 +         ((_1 as Yielded).0: i32) = move _8; // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
- +         discriminant(_1) = 0;            // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
118 +         discriminant((*(_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]))) = 3; // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
119 +         goto -> bb1;                     // scope 0 at $DIR/inline-generator.rs:15:11: 15:39


123 +         StorageLive(_8);                 // scope 6 at $DIR/inline-generator.rs:15:5: 15:41
124 +         _10 = move _7;                   // scope 6 at $DIR/inline-generator.rs:15:5: 15:41
125 +         StorageDead(_8);                 // scope 6 at $DIR/inline-generator.rs:15:38: 15:39
+ +         _1 = UninitVariant(1);           // scope 6 at $DIR/inline-generator.rs:15:41: 15:41
126 +         ((_1 as Complete).0: bool) = move _10; // scope 6 at $DIR/inline-generator.rs:15:41: 15:41
- +         discriminant(_1) = 1;            // scope 6 at $DIR/inline-generator.rs:15:41: 15:41
128 +         discriminant((*(_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]))) = 1; // scope 6 at $DIR/inline-generator.rs:15:41: 15:41
129 +         goto -> bb1;                     // scope 0 at $DIR/inline-generator.rs:15:41: 15:41


thread '[mir-opt] mir-opt/inline/inline-generator.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_generator.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3406:25
---- [mir-opt] mir-opt/issue-73223.rs stdout ----
51       bb0: {
51       bb0: {
52           StorageLive(_1);                 // scope 0 at $DIR/issue-73223.rs:2:9: 2:14
53           StorageLive(_2);                 // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
+           _2 = UninitVariant(1);           // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
54           ((_2 as Some).0: i32) = const 1_i32; // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
-           discriminant(_2) = 1;            // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
56           _3 = const 1_isize;              // scope 0 at $DIR/issue-73223.rs:2:23: 2:30
57           goto -> bb2;                     // scope 0 at $DIR/issue-73223.rs:2:17: 2:30


73           StorageLive(_6);                 // scope 1 at $DIR/issue-73223.rs:7:9: 7:14
74           StorageLive(_7);                 // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
75           _7 = _1;                         // scope 1 at $DIR/issue-73223.rs:7:22: 7:27
+           _6 = UninitVariant(1);           // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
76           ((_6 as Some).0: i32) = move _7; // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
-           discriminant(_6) = 1;            // scope 1 at $DIR/issue-73223.rs:7:17: 7:28
78           StorageDead(_7);                 // scope 1 at $DIR/issue-73223.rs:7:27: 7:28
79           StorageLive(_8);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
80           StorageLive(_9);                 // scope 3 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
110   
111       bb3: {
111       bb3: {
112           StorageLive(_20);                // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           discriminant(_20) = 0;           // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _20 = const core::panicking::AssertKind::Eq; // scope 4 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                                            // mir::Constant
+                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL
+                                            // + literal: Const { ty: core::panicking::AssertKind, val: Value(Scalar(0x00)) }
114           StorageLive(_21);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
115           StorageLive(_22);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
116           _22 = const core::panicking::AssertKind::Eq; // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL

126           _26 = _14;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
127           _25 = _26;                       // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
128           StorageLive(_27);                // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
-           discriminant(_27) = 0;           // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+           _27 = UninitVariant(0);          // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
130           core::panicking::assert_failed::<i32, i32>(const core::panicking::AssertKind::Eq, move _23, move _25, move _27); // scope 5 at $SRC_DIR/core/src/macros/mod.rs:LL:COL
131                                            // mir::Constant
132                                            // + span: $SRC_DIR/core/src/macros/mod.rs:LL:COL

thread '[mir-opt] mir-opt/issue-73223.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_73223.main.SimplifyArmIdentity.64bit.diff', src/tools/compiletest/src/runtest.rs:3406:25
---- [mir-opt] mir-opt/issues/issue-75439.rs stdout ----
67   
68       bb7: {
68       bb7: {
69           StorageDead(_6);                 // scope 4 at $DIR/issue-75439.rs:10:35: 10:36
+           _0 = UninitVariant(1);           // scope 1 at $DIR/issue-75439.rs:10:9: 10:39
70           ((_0 as Some).0: [u8; 4]) = move _5; // scope 1 at $DIR/issue-75439.rs:10:9: 10:39
-           discriminant(_0) = 1;            // scope 1 at $DIR/issue-75439.rs:10:9: 10:39
72           StorageDead(_5);                 // scope 1 at $DIR/issue-75439.rs:10:38: 10:39
73           StorageDead(_4);                 // scope 1 at $DIR/issue-75439.rs:11:5: 11:6
74           goto -> bb9;                     // scope 1 at $DIR/issue-75439.rs:9:5: 13:6
75       }
76   
77       bb8: {
77       bb8: {
-           discriminant(_0) = 0;            // scope 1 at $DIR/issue-75439.rs:12:9: 12:13
+           _0 = UninitVariant(0);           // scope 1 at $DIR/issue-75439.rs:12:9: 12:13
79           goto -> bb9;                     // scope 1 at $DIR/issue-75439.rs:9:5: 13:6
81   


thread '[mir-opt] mir-opt/issues/issue-75439.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issues/issue_75439.foo.MatchBranchSimplification.diff', src/tools/compiletest/src/runtest.rs:3406:25
---- [mir-opt] mir-opt/simplify-arm-identity.rs stdout ----
19   
20       bb0: {
20       bb0: {
21           StorageLive(_1);                 // scope 0 at $DIR/simplify-arm-identity.rs:18:9: 18:10
+           _1 = UninitVariant(0);           // scope 0 at $DIR/simplify-arm-identity.rs:18:18: 18:29
22           ((_1 as Foo).0: u8) = const 0_u8; // scope 0 at $DIR/simplify-arm-identity.rs:18:18: 18:29
-           discriminant(_1) = 0;            // scope 0 at $DIR/simplify-arm-identity.rs:18:18: 18:29
24           StorageLive(_2);                 // scope 1 at $DIR/simplify-arm-identity.rs:19:18: 22:6
25           _3 = const 0_isize;              // scope 1 at $DIR/simplify-arm-identity.rs:19:24: 19:25
26           goto -> bb3;                     // scope 1 at $DIR/simplify-arm-identity.rs:19:18: 19:25
27       }
28   
29       bb1: {
29       bb1: {
+           _2 = UninitVariant(0);           // scope 1 at $DIR/simplify-arm-identity.rs:21:21: 21:32
30           ((_2 as Foo).0: u8) = const 0_u8; // scope 1 at $DIR/simplify-arm-identity.rs:21:21: 21:32
-           discriminant(_2) = 0;            // scope 1 at $DIR/simplify-arm-identity.rs:21:21: 21:32
32           goto -> bb4;                     // scope 1 at $DIR/simplify-arm-identity.rs:21:21: 21:32
34   


41           _4 = ((_1 as Foo).0: u8);        // scope 1 at $DIR/simplify-arm-identity.rs:20:18: 20:19
42           StorageLive(_5);                 // scope 3 at $DIR/simplify-arm-identity.rs:20:33: 20:34
43           _5 = _4;                         // scope 3 at $DIR/simplify-arm-identity.rs:20:33: 20:34
+           _2 = UninitVariant(0);           // scope 3 at $DIR/simplify-arm-identity.rs:20:24: 20:35
44           ((_2 as Foo).0: u8) = move _5;   // scope 3 at $DIR/simplify-arm-identity.rs:20:24: 20:35
-           discriminant(_2) = 0;            // scope 3 at $DIR/simplify-arm-identity.rs:20:24: 20:35
46           StorageDead(_5);                 // scope 3 at $DIR/simplify-arm-identity.rs:20:34: 20:35
47           StorageDead(_4);                 // scope 1 at $DIR/simplify-arm-identity.rs:20:34: 20:35
48           goto -> bb4;                     // scope 1 at $DIR/simplify-arm-identity.rs:20:34: 20:35

thread '[mir-opt] mir-opt/simplify-arm-identity.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_arm_identity.main.SimplifyArmIdentity.64bit.diff', src/tools/compiletest/src/runtest.rs:3406:25
---- [mir-opt] mir-opt/separate_const_switch.rs stdout ----
---- [mir-opt] mir-opt/separate_const_switch.rs stdout ----
38           _6 = ((_1 as Err).0: usize);     // scope 0 at $DIR/separate_const_switch.rs:17:17: 17:18
39           StorageLive(_7);                 // scope 2 at $DIR/separate_const_switch.rs:17:42: 17:43
40           _7 = _6;                         // scope 2 at $DIR/separate_const_switch.rs:17:42: 17:43
+           _2 = UninitVariant(1);           // scope 2 at $DIR/separate_const_switch.rs:17:23: 17:44
41           ((_2 as Break).0: usize) = move _7; // scope 2 at $DIR/separate_const_switch.rs:17:23: 17:44
-           discriminant(_2) = 1;            // scope 2 at $DIR/separate_const_switch.rs:17:23: 17:44
43           StorageDead(_7);                 // scope 2 at $DIR/separate_const_switch.rs:17:43: 17:44
44           StorageDead(_6);                 // scope 0 at $DIR/separate_const_switch.rs:17:43: 17:44
- -         goto -> bb3;                     // scope 0 at $DIR/separate_const_switch.rs:17:43: 17:44
- +         _8 = discriminant(_2);           // scope 0 at $DIR/separate_const_switch.rs:14:11: 19:6
- +         switchInt(move _8) -> [0_isize: bb4, otherwise: bb3]; // scope 0 at $DIR/separate_const_switch.rs:14:5: 19:6
+           goto -> bb3;                     // scope 0 at $DIR/separate_const_switch.rs:17:43: 17:44
49   
50       bb2: {


52           _4 = ((_1 as Ok).0: i32);        // scope 0 at $DIR/separate_const_switch.rs:16:16: 16:17
53           StorageLive(_5);                 // scope 1 at $DIR/separate_const_switch.rs:16:44: 16:45
54           _5 = _4;                         // scope 1 at $DIR/separate_const_switch.rs:16:44: 16:45
+           _2 = UninitVariant(0);           // scope 1 at $DIR/separate_const_switch.rs:16:22: 16:46
55           ((_2 as Continue).0: i32) = move _5; // scope 1 at $DIR/separate_const_switch.rs:16:22: 16:46
-           discriminant(_2) = 0;            // scope 1 at $DIR/separate_const_switch.rs:16:22: 16:46
57           StorageDead(_5);                 // scope 1 at $DIR/separate_const_switch.rs:16:45: 16:46
58           StorageDead(_4);                 // scope 0 at $DIR/separate_const_switch.rs:16:45: 16:46
- -         goto -> bb3;                     // scope 0 at $DIR/separate_const_switch.rs:16:45: 16:46
- -     }
- -     bb3: {
- -     bb3: {
+           goto -> bb3;                     // scope 0 at $DIR/separate_const_switch.rs:16:45: 16:46
+   
+       bb3: {
+       bb3: {
63           _8 = discriminant(_2);           // scope 0 at $DIR/separate_const_switch.rs:14:11: 19:6
- -         switchInt(move _8) -> [0_isize: bb5, otherwise: bb4]; // scope 0 at $DIR/separate_const_switch.rs:14:5: 19:6
- +         switchInt(move _8) -> [0_isize: bb4, otherwise: bb3]; // scope 0 at $DIR/separate_const_switch.rs:14:5: 19:6
+           switchInt(move _8) -> [0_isize: bb5, otherwise: bb4]; // scope 0 at $DIR/separate_const_switch.rs:14:5: 19:6
67   
- -     bb4: {
- +     bb3: {
+       bb4: {
+       bb4: {
70           StorageLive(_11);                // scope 0 at $DIR/separate_const_switch.rs:21:28: 21:29
71           _11 = ((_2 as Break).0: usize);  // scope 0 at $DIR/separate_const_switch.rs:21:28: 21:29
-           discriminant(_0) = 0;            // scope 4 at $DIR/separate_const_switch.rs:21:34: 21:38
+           _0 = UninitVariant(0);           // scope 4 at $DIR/separate_const_switch.rs:21:34: 21:38
73           StorageDead(_11);                // scope 0 at $DIR/separate_const_switch.rs:21:37: 21:38
- -         goto -> bb6;                     // scope 0 at $DIR/separate_const_switch.rs:21:37: 21:38
- +         goto -> bb5;                     // scope 0 at $DIR/separate_const_switch.rs:21:37: 21:38
+           goto -> bb6;                     // scope 0 at $DIR/separate_const_switch.rs:21:37: 21:38
77   
- -     bb5: {
- +     bb4: {
+       bb5: {
+       bb5: {
80           StorageLive(_9);                 // scope 0 at $DIR/separate_const_switch.rs:20:31: 20:32
81           _9 = ((_2 as Continue).0: i32);  // scope 0 at $DIR/separate_const_switch.rs:20:31: 20:32
82           StorageLive(_10);                // scope 3 at $DIR/separate_const_switch.rs:20:42: 20:43

83           _10 = _9;                        // scope 3 at $DIR/separate_const_switch.rs:20:42: 20:43
+           _0 = UninitVariant(1);           // scope 3 at $DIR/separate_const_switch.rs:20:37: 20:44
84           ((_0 as Some).0: i32) = move _10; // scope 3 at $DIR/separate_const_switch.rs:20:37: 20:44
-           discriminant(_0) = 1;            // scope 3 at $DIR/separate_const_switch.rs:20:37: 20:44
86           StorageDead(_10);                // scope 3 at $DIR/separate_const_switch.rs:20:43: 20:44
87           StorageDead(_9);                 // scope 0 at $DIR/separate_const_switch.rs:20:43: 20:44
- -         goto -> bb6;                     // scope 0 at $DIR/separate_const_switch.rs:20:43: 20:44
- +         goto -> bb5;                     // scope 0 at $DIR/separate_const_switch.rs:20:43: 20:44
+           goto -> bb6;                     // scope 0 at $DIR/separate_const_switch.rs:20:43: 20:44
91   
- -     bb6: {
- +     bb5: {
+       bb6: {
+       bb6: {
94           StorageDead(_2);                 // scope 0 at $DIR/separate_const_switch.rs:23:1: 23:2
95           return;                          // scope 0 at $DIR/separate_const_switch.rs:23:2: 23:2


thread '[mir-opt] mir-opt/separate_const_switch.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/separate_const_switch.too_complex.SeparateConstSwitch.diff', src/tools/compiletest/src/runtest.rs:3406:25
---- [mir-opt] mir-opt/simplify-locals-fixedpoint.rs stdout ----
18       bb0: {
18       bb0: {
19           StorageLive(_1);                 // scope 0 at $DIR/simplify-locals-fixedpoint.rs:4:30: 4:69
20           StorageLive(_2);                 // scope 0 at $DIR/simplify-locals-fixedpoint.rs:4:31: 4:49
-           discriminant(_2) = 0;            // scope 0 at $DIR/simplify-locals-fixedpoint.rs:4:31: 4:49
+           _2 = UninitVariant(0);           // scope 0 at $DIR/simplify-locals-fixedpoint.rs:4:31: 4:49
22           StorageLive(_3);                 // scope 0 at $DIR/simplify-locals-fixedpoint.rs:4:51: 4:68
-           discriminant(_3) = 0;            // scope 0 at $DIR/simplify-locals-fixedpoint.rs:4:51: 4:68
+           _3 = UninitVariant(0);           // scope 0 at $DIR/simplify-locals-fixedpoint.rs:4:51: 4:68
24           (_1.0: std::option::Option<u8>) = move _2; // scope 0 at $DIR/simplify-locals-fixedpoint.rs:4:30: 4:69
25           (_1.1: std::option::Option<T>) = move _3; // scope 0 at $DIR/simplify-locals-fixedpoint.rs:4:30: 4:69
26           StorageDead(_3);                 // scope 0 at $DIR/simplify-locals-fixedpoint.rs:4:68: 4:69

thread '[mir-opt] mir-opt/simplify-locals-fixedpoint.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_locals_fixedpoint.foo.SimplifyLocals.diff', src/tools/compiletest/src/runtest.rs:3406:25
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
+           _0 = UninitVariant(1);           // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:20: 6:27
+ -         ((_0 as Some).0: std::boxed::Box<()>) = move _4; // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:20: 6:27
+ +         ((_0 as Some).0: std::boxed::Box<()>) = move _3; // scope 1 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:20: 6:27
+           goto -> bb3;                     // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:6:26: 6:27
+   
+       bb2: {
+       bb2: {
+           _0 = const Option::<Box<()>>::None; // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:5:17: 5:21
+                                            // mir::Constant
+                                            // + span: $DIR/simplify-locals-removes-unused-discriminant-reads.rs:5:17: 5:21
+                                            // + literal: Const { ty: Option<Box<()>>, val: Value(Scalar(0x0000000000000000)) }
+           goto -> bb3;                     // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:5:17: 5:21
+   
+       bb3: {
+       bb3: {
22 -         _6 = discriminant(_1);           // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:8:1: 8:2
23           return;                          // scope 0 at $DIR/simplify-locals-removes-unused-discriminant-reads.rs:8:2: 8:2


thread '[mir-opt] mir-opt/simplify-locals-removes-unused-discriminant-reads.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_locals_removes_unused_discriminant_reads.map.SimplifyLocals.64bit.diff', src/tools/compiletest/src/runtest.rs:3406:25
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

18       }
18       }
19   
20       bb1: {
-           discriminant(_0) = 0;            // scope 0 at $DIR/simplify-arm.rs:12:17: 12:21
+           _0 = UninitVariant(0);           // scope 0 at $DIR/simplify-arm.rs:12:17: 12:21
22           goto -> bb4;                     // scope 0 at $DIR/simplify-arm.rs:12:17: 12:21
24   

