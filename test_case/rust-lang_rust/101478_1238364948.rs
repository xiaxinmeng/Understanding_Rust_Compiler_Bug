plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 185 tests
.......................................i................................................ 88/185
.........i..F....F....................................ii.......i........................ 176/185
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] src/test/mir-opt/inline/inline-generator.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/inline-generator.rs stdout ----
50 +         Deinit(_4);                      // scope 2 at $DIR/inline-generator.rs:15:5: 15:41
51 +         discriminant(_4) = 0;            // scope 2 at $DIR/inline-generator.rs:15:5: 15:41
52           _3 = &mut _4;                    // scope 0 at $DIR/inline-generator.rs:+1:23: +1:31
- -         _2 = Pin::<&mut [generator@$DIR/inline-generator.rs:15:5: 15:8]>::new(move _3) -> [return: bb2, unwind: bb4]; // scope 0 at $DIR/inline-generator.rs:+1:14: +1:32
+ -         _2 = Pin::<&mut [generator@$DIR/inline-generator.rs:15:5: 15:8]>::new(move _3) -> [return: bb2, unwind: bb7]; // scope 0 at $DIR/inline-generator.rs:+1:14: +1:32
54 -                                          // mir::Constant
55 -                                          // + span: $DIR/inline-generator.rs:9:14: 9:22
56 -                                          // + user_ty: UserType(0)

67 +         StorageDead(_6);                 // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
68 +         StorageDead(_5);                 // scope 4 at $SRC_DIR/core/src/pin.rs:LL:COL
69           StorageDead(_3);                 // scope 0 at $DIR/inline-generator.rs:+1:31: +1:32
- -         _1 = <[generator@$DIR/inline-generator.rs:15:5: 15:8] as Generator<bool>>::resume(move _2, const false) -> [return: bb3, unwind: bb4]; // scope 0 at $DIR/inline-generator.rs:+1:14: +1:46
+ -         _1 = <[generator@$DIR/inline-generator.rs:15:5: 15:8] as Generator<bool>>::resume(move _2, const false) -> [return: bb3, unwind: bb7]; // scope 0 at $DIR/inline-generator.rs:+1:14: +1:46
71 -                                          // mir::Constant
72 -                                          // + span: $DIR/inline-generator.rs:9:33: 9:39
73 -                                          // + literal: Const { ty: for<'r> fn(Pin<&'r mut [generator@$DIR/inline-generator.rs:15:5: 15:8]>, bool) -> GeneratorState<<[generator@$DIR/inline-generator.rs:15:5: 15:8] as Generator<bool>>::Yield, <[generator@$DIR/inline-generator.rs:15:5: 15:8] as Generator<bool>>::Return> {<[generator@$DIR/inline-generator.rs:15:5: 15:8] as Generator<bool>>::resume}, val: Value(<ZST>) }

77 +         StorageLive(_11);                // scope 0 at $DIR/inline-generator.rs:+1:14: +1:46
78 +         _13 = deref_copy (_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
79 +         _12 = discriminant((*_13));      // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
- +         switchInt(move _12) -> [0_u32: bb3, 1_u32: bb8, 3_u32: bb7, otherwise: bb9]; // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
+ +         switchInt(move _12) -> [0_u32: bb7, 1_u32: bb12, 3_u32: bb11, otherwise: bb13]; // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
82   
83 -     bb3: {


86 +         StorageDead(_10);                // scope 0 at $DIR/inline-generator.rs:+1:14: +1:46
87 +         StorageDead(_7);                 // scope 0 at $DIR/inline-generator.rs:+1:14: +1:46
88           StorageDead(_2);                 // scope 0 at $DIR/inline-generator.rs:+1:45: +1:46
+ -         drop(_4) -> [return: bb4, unwind: bb6]; // scope 0 at $DIR/inline-generator.rs:+1:46: +1:47
+ +         drop(_4) -> [return: bb2, unwind: bb4]; // scope 0 at $DIR/inline-generator.rs:+1:46: +1:47
+   
+ -     bb4: {
+ +     bb2: {
+ +     bb2: {
89           StorageDead(_4);                 // scope 0 at $DIR/inline-generator.rs:+1:46: +1:47
90           _0 = const ();                   // scope 0 at $DIR/inline-generator.rs:+0:11: +2:2
+ -         drop(_1) -> [return: bb5, unwind: bb8]; // scope 0 at $DIR/inline-generator.rs:+2:1: +2:2
+ +         drop(_1) -> [return: bb3, unwind: bb6]; // scope 0 at $DIR/inline-generator.rs:+2:1: +2:2
+   
+ -     bb5: {
+ +     bb3: {
+ +     bb3: {
91           StorageDead(_1);                 // scope 0 at $DIR/inline-generator.rs:+2:1: +2:2
92           return;                          // scope 0 at $DIR/inline-generator.rs:+2:2: +2:2

94   
94   
- -     bb4 (cleanup): {
- +     bb2 (cleanup): {
+ -     bb6 (cleanup): {
+ -         drop(_1) -> bb8;                 // scope 0 at $DIR/inline-generator.rs:+2:1: +2:2
+ +     bb4 (cleanup): {
+ +         drop(_1) -> bb6;                 // scope 0 at $DIR/inline-generator.rs:+2:1: +2:2
+   
+   
+ -     bb7 (cleanup): {
+ -         drop(_4) -> bb8;                 // scope 0 at $DIR/inline-generator.rs:+1:46: +1:47
+ +     bb5 (cleanup): {
+ +         drop(_4) -> bb6;                 // scope 0 at $DIR/inline-generator.rs:+1:46: +1:47
+   
+   
+ -     bb8 (cleanup): {
+ +     bb6 (cleanup): {
97           resume;                          // scope 0 at $DIR/inline-generator.rs:+0:1: +2:2
99 + 

- +     bb3: {
+ +     bb7: {
+ +     bb7: {
101 +         _11 = move _7;                   // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
102 +         StorageLive(_8);                 // scope 6 at $DIR/inline-generator.rs:15:17: 15:39
103 +         StorageLive(_9);                 // scope 6 at $DIR/inline-generator.rs:15:20: 15:21

104 +         _9 = _11;                        // scope 6 at $DIR/inline-generator.rs:15:20: 15:21
- +         switchInt(move _9) -> [false: bb5, otherwise: bb4]; // scope 6 at $DIR/inline-generator.rs:15:20: 15:21
+ +         switchInt(move _9) -> [false: bb9, otherwise: bb8]; // scope 6 at $DIR/inline-generator.rs:15:20: 15:21
107 + 
- +     bb4: {
+ +     bb8: {
+ +     bb8: {
109 +         _8 = const 7_i32;                // scope 6 at $DIR/inline-generator.rs:15:24: 15:25
- +         goto -> bb6;                     // scope 6 at $DIR/inline-generator.rs:15:17: 15:39
+ +         goto -> bb10;                    // scope 6 at $DIR/inline-generator.rs:15:17: 15:39
112 + 
- +     bb5: {
+ +     bb9: {
+ +     bb9: {
114 +         _8 = const 13_i32;               // scope 6 at $DIR/inline-generator.rs:15:35: 15:37
- +         goto -> bb6;                     // scope 6 at $DIR/inline-generator.rs:15:17: 15:39
+ +         goto -> bb10;                    // scope 6 at $DIR/inline-generator.rs:15:17: 15:39
117 + 
- +     bb6: {
+ +     bb10: {
+ +     bb10: {
119 +         StorageDead(_9);                 // scope 6 at $DIR/inline-generator.rs:15:38: 15:39
120 +         Deinit(_1);                      // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
121 +         ((_1 as Yielded).0: i32) = move _8; // scope 6 at $DIR/inline-generator.rs:15:11: 15:39

125 +         goto -> bb1;                     // scope 0 at $DIR/inline-generator.rs:15:11: 15:39
127 + 
- +     bb7: {
+ +     bb11: {
+ +     bb11: {
129 +         StorageLive(_8);                 // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
130 +         _10 = move _7;                   // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
131 +         StorageDead(_8);                 // scope 6 at $DIR/inline-generator.rs:15:38: 15:39

137 +         goto -> bb1;                     // scope 0 at $DIR/inline-generator.rs:15:8: 15:8
139 + 
- +     bb8: {
- +     bb8: {
- +         assert(const false, "generator resumed after completion") -> [success: bb8, unwind: bb2]; // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
+ +     bb12: {
+ +         assert(const false, "generator resumed after completion") -> [success: bb12, unwind: bb5]; // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
143 + 
- +     bb9: {
+ +     bb13: {
+ +     bb13: {
145 +         unreachable;                     // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
147   }


thread '[mir-opt] src/test/mir-opt/inline/inline-generator.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_generator.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3516:25

---- [mir-opt] src/test/mir-opt/inline/issue-78442.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/issue-78442.rs stdout ----
15           StorageLive(_2);                 // scope 0 at $DIR/issue-78442.rs:+4:5: +4:17
16           StorageLive(_3);                 // scope 0 at $DIR/issue-78442.rs:+4:5: +4:15
17           StorageLive(_4);                 // scope 0 at $DIR/issue-78442.rs:+4:5: +4:15
-           _4 = hide_foo() -> [return: bb1, unwind: bb4]; // scope 0 at $DIR/issue-78442.rs:+4:5: +4:15
+           _4 = hide_foo() -> [return: bb1, unwind: bb6]; // scope 0 at $DIR/issue-78442.rs:+4:5: +4:15
19                                            // mir::Constant
20                                            // + span: $DIR/issue-78442.rs:11:5: 11:13
21                                            // + literal: Const { ty: fn() -> impl Fn() {hide_foo}, val: Value(<ZST>) }

25           _3 = &_4;                        // scope 0 at $DIR/issue-78442.rs:+4:5: +4:15
26           StorageLive(_5);                 // scope 0 at $DIR/issue-78442.rs:+4:5: +4:17
27           Deinit(_5);                      // scope 0 at $DIR/issue-78442.rs:+4:5: +4:17
- -         _2 = <impl Fn() as Fn<()>>::call(move _3, move _5) -> [return: bb2, unwind: bb4]; // scope 0 at $DIR/issue-78442.rs:+4:5: +4:17
- +         _2 = <fn() {foo} as Fn<()>>::call(move _3, move _5) -> [return: bb2, unwind: bb4]; // scope 0 at $DIR/issue-78442.rs:+4:5: +4:17
+ -         _2 = <impl Fn() as Fn<()>>::call(move _3, move _5) -> [return: bb2, unwind: bb5]; // scope 0 at $DIR/issue-78442.rs:+4:5: +4:17
+ +         _2 = <fn() {foo} as Fn<()>>::call(move _3, move _5) -> [return: bb2, unwind: bb5]; // scope 0 at $DIR/issue-78442.rs:+4:5: +4:17
30                                            // mir::Constant
31                                            // + span: $DIR/issue-78442.rs:11:5: 11:15
32 -                                          // + literal: Const { ty: for<'r> extern "rust-call" fn(&'r impl Fn(), ()) -> <impl Fn() as FnOnce<()>>::Output {<impl Fn() as Fn<()>>::call}, val: Value(<ZST>) }
36       bb2: {
36       bb2: {
37           StorageDead(_5);                 // scope 0 at $DIR/issue-78442.rs:+4:16: +4:17
38           StorageDead(_3);                 // scope 0 at $DIR/issue-78442.rs:+4:16: +4:17
+           drop(_4) -> [return: bb3, unwind: bb6]; // scope 0 at $DIR/issue-78442.rs:+4:17: +4:18
+   
+       bb3: {
+       bb3: {
39           StorageDead(_4);                 // scope 0 at $DIR/issue-78442.rs:+4:17: +4:18
40           StorageDead(_2);                 // scope 0 at $DIR/issue-78442.rs:+4:17: +4:18
41           _0 = const ();                   // scope 0 at $DIR/issue-78442.rs:+3:3: +5:2

-           drop(_1) -> [return: bb3, unwind: bb5]; // scope 0 at $DIR/issue-78442.rs:+5:1: +5:2
+           drop(_1) -> [return: bb4, unwind: bb7]; // scope 0 at $DIR/issue-78442.rs:+5:1: +5:2
44   
-       bb3: {
+       bb4: {
+       bb4: {
46           return;                          // scope 0 at $DIR/issue-78442.rs:+5:2: +5:2
48   


-       bb4 (cleanup): {
-           drop(_1) -> bb5;                 // scope 0 at $DIR/issue-78442.rs:+5:1: +5:2
+       bb5 (cleanup): {
+           drop(_4) -> bb6;                 // scope 0 at $DIR/issue-78442.rs:+4:17: +4:18
52   
52   
-       bb5 (cleanup): {
+       bb6 (cleanup): {
+           drop(_1) -> bb7;                 // scope 0 at $DIR/issue-78442.rs:+5:1: +5:2
+   
+   
+       bb7 (cleanup): {
54           resume;                          // scope 0 at $DIR/issue-78442.rs:+0:1: +5:2
56   }


thread '[mir-opt] src/test/mir-opt/inline/issue-78442.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/issue_78442.bar.RevealAll.diff', src/tools/compiletest/src/runtest.rs:3516:25

failures:
    [mir-opt] src/test/mir-opt/inline/inline-generator.rs
    [mir-opt] src/test/mir-opt/inline/issue-78442.rs
