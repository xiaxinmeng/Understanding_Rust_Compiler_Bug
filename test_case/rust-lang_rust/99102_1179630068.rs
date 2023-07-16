plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
....
failures:

---- [mir-opt] src/test/mir-opt/generator-tiny.rs stdout ----
38     bb1: {
39         _10 = move _2;                   // scope 0 at $DIR/generator-tiny.rs:19:16: 19:24
40         nop;                             // scope 0 at $DIR/generator-tiny.rs:20:13: 20:15
-         Deinit((((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:19:16: 19:24])) as variant#3).0: HasDrop)); // scope 0 at $DIR/generator-tiny.rs:20:18: 20:25
+         (((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:19:16: 19:24])) as variant#3).0: HasDrop) = HasDrop; // scope 0 at $DIR/generator-tiny.rs:20:18: 20:25
42         StorageLive(_4);                 // scope 1 at $DIR/generator-tiny.rs:21:9: 24:10
43         goto -> bb2;                     // scope 1 at $DIR/generator-tiny.rs:21:9: 24:10

46     bb2: {
46     bb2: {
47         StorageLive(_6);                 // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
48         StorageLive(_7);                 // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
-         Deinit(_7);                      // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
+         _7 = ();                         // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
50         Deinit(_0);                      // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
51         ((_0 as Yielded).0: ()) = move _7; // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
52         discriminant(_0) = 0;            // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18

thread '[mir-opt] src/test/mir-opt/generator-tiny.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/generator_tiny.main-{closure#0}.generator_resume.0.mir', src/tools/compiletest/src/runtest.rs:3466:25

---- [mir-opt] src/test/mir-opt/generator-storage-dead-unwind.rs stdout ----
21 
22     bb0: {
22     bb0: {
23         StorageLive(_3);                 // scope 0 at $DIR/generator-storage-dead-unwind.rs:23:13: 23:14
-         Deinit(_3);                      // scope 0 at $DIR/generator-storage-dead-unwind.rs:23:17: 23:23
-         (_3.0: i32) = const 5_i32;       // scope 0 at $DIR/generator-storage-dead-unwind.rs:23:17: 23:23
+         _3 = Foo(const 5_i32);           // scope 0 at $DIR/generator-storage-dead-unwind.rs:23:17: 23:23
26         StorageLive(_4);                 // scope 1 at $DIR/generator-storage-dead-unwind.rs:24:13: 24:14
-         Deinit(_4);                      // scope 1 at $DIR/generator-storage-dead-unwind.rs:24:17: 24:23
-         (_4.0: i32) = const 6_i32;       // scope 1 at $DIR/generator-storage-dead-unwind.rs:24:17: 24:23
+         _4 = Bar(const 6_i32);           // scope 1 at $DIR/generator-storage-dead-unwind.rs:24:17: 24:23
29         StorageLive(_5);                 // scope 2 at $DIR/generator-storage-dead-unwind.rs:25:9: 25:14
30         StorageLive(_6);                 // scope 2 at $DIR/generator-storage-dead-unwind.rs:25:9: 25:14
-         Deinit(_6);                      // scope 2 at $DIR/generator-storage-dead-unwind.rs:25:9: 25:14
-         _5 = yield(move _6) -> [resume: bb1, drop: bb5]; // scope 2 at $DIR/generator-storage-dead-unwind.rs:25:9: 25:14
+         _6 = ();                         // scope 2 at $DIR/generator-storage-dead-unwind.rs:25:9: 25:14
+         _5 = yield(move _6) -> [resume: bb1, drop: bb6]; // scope 2 at $DIR/generator-storage-dead-unwind.rs:25:9: 25:14
34 
35     bb1: {


38         StorageLive(_7);                 // scope 2 at $DIR/generator-storage-dead-unwind.rs:26:9: 26:16
39         StorageLive(_8);                 // scope 2 at $DIR/generator-storage-dead-unwind.rs:26:14: 26:15
40         _8 = move _3;                    // scope 2 at $DIR/generator-storage-dead-unwind.rs:26:14: 26:15
-         _7 = take::<Foo>(move _8) -> [return: bb2, unwind: bb9]; // scope 2 at $DIR/generator-storage-dead-unwind.rs:26:9: 26:16
+         _7 = take::<Foo>(move _8) -> [return: bb2, unwind: bb10]; // scope 2 at $DIR/generator-storage-dead-unwind.rs:26:9: 26:16
42                                          // mir::Constant
43                                          // + span: $DIR/generator-storage-dead-unwind.rs:26:9: 26:13
44                                          // + literal: Const { ty: fn(Foo) {take::<Foo>}, val: Value(<ZST>) }

50         StorageLive(_9);                 // scope 2 at $DIR/generator-storage-dead-unwind.rs:27:9: 27:16
51         StorageLive(_10);                // scope 2 at $DIR/generator-storage-dead-unwind.rs:27:14: 27:15
52         _10 = move _4;                   // scope 2 at $DIR/generator-storage-dead-unwind.rs:27:14: 27:15
-         _9 = take::<Bar>(move _10) -> [return: bb3, unwind: bb8]; // scope 2 at $DIR/generator-storage-dead-unwind.rs:27:9: 27:16
+         _9 = take::<Bar>(move _10) -> [return: bb3, unwind: bb9]; // scope 2 at $DIR/generator-storage-dead-unwind.rs:27:9: 27:16
54                                          // mir::Constant
55                                          // + span: $DIR/generator-storage-dead-unwind.rs:27:9: 27:13
56                                          // + literal: Const { ty: fn(Bar) {take::<Bar>}, val: Value(<ZST>) }

61         StorageDead(_9);                 // scope 2 at $DIR/generator-storage-dead-unwind.rs:27:16: 27:17
62         _0 = const ();                   // scope 0 at $DIR/generator-storage-dead-unwind.rs:22:19: 28:6
63         StorageDead(_4);                 // scope 1 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6
-         StorageDead(_3);                 // scope 0 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6
-         drop(_1) -> [return: bb4, unwind: bb11]; // scope 0 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6
+         goto -> bb4;                     // scope 0 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6
67 
68     bb4: {


-         return;                          // scope 0 at $DIR/generator-storage-dead-unwind.rs:22:18: 22:18
+         StorageDead(_3);                 // scope 0 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6
+         drop(_1) -> [return: bb5, unwind: bb14]; // scope 0 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6
71 
72     bb5: {


+         return;                          // scope 0 at $DIR/generator-storage-dead-unwind.rs:22:18: 22:18
+ 
+     bb6: {
+     bb6: {
73         StorageDead(_6);                 // scope 2 at $DIR/generator-storage-dead-unwind.rs:25:13: 25:14
74         StorageDead(_5);                 // scope 2 at $DIR/generator-storage-dead-unwind.rs:25:14: 25:15
75         StorageDead(_4);                 // scope 1 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6

-         drop(_3) -> [return: bb6, unwind: bb12]; // scope 0 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6
+         drop(_3) -> [return: bb7, unwind: bb15]; // scope 0 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6
78 
-     bb6: {
+     bb7: {
+     bb7: {
80         StorageDead(_3);                 // scope 0 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6
-         drop(_1) -> [return: bb7, unwind: bb11]; // scope 0 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6
+         drop(_1) -> [return: bb8, unwind: bb14]; // scope 0 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6
83 
-     bb7: {
+     bb8: {
+     bb8: {
85         generator_drop;                  // scope 0 at $DIR/generator-storage-dead-unwind.rs:22:16: 22:18
87 


-     bb8 (cleanup): {
+     bb9 (cleanup): {
89         StorageDead(_10);                // scope 2 at $DIR/generator-storage-dead-unwind.rs:27:15: 27:16
90         StorageDead(_9);                 // scope 2 at $DIR/generator-storage-dead-unwind.rs:27:16: 27:17
-         goto -> bb10;                    // scope 2 at no-location
+         goto -> bb12;                    // scope 2 at no-location
93 
93 
-     bb9 (cleanup): {
+     bb10 (cleanup): {
+         goto -> bb11;                    // scope 2 at $DIR/generator-storage-dead-unwind.rs:26:15: 26:16
+ 
+ 
+     bb11 (cleanup): {
95         StorageDead(_8);                 // scope 2 at $DIR/generator-storage-dead-unwind.rs:26:15: 26:16
96         StorageDead(_7);                 // scope 2 at $DIR/generator-storage-dead-unwind.rs:26:16: 26:17
-         goto -> bb10;                    // scope 2 at no-location
+         goto -> bb12;                    // scope 2 at no-location
99 
99 
-     bb10 (cleanup): {
+     bb12 (cleanup): {
101         StorageDead(_4);                 // scope 1 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6
+         goto -> bb13;                    // scope 0 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6
+ 
+ 
+     bb13 (cleanup): {
102         StorageDead(_3);                 // scope 0 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6
-         drop(_1) -> bb11;                // scope 0 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6
+         drop(_1) -> bb14;                // scope 0 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6
105 
105 
-     bb11 (cleanup): {
+     bb14 (cleanup): {
107         resume;                          // scope 0 at $DIR/generator-storage-dead-unwind.rs:22:16: 22:18
109 


-     bb12 (cleanup): {
+     bb15 (cleanup): {
111         StorageDead(_3);                 // scope 0 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6
-         drop(_1) -> bb11;                // scope 0 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6
+         drop(_1) -> bb14;                // scope 0 at $DIR/generator-storage-dead-unwind.rs:28:5: 28:6
114 }
115 


thread '[mir-opt] src/test/mir-opt/generator-storage-dead-unwind.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/generator_storage_dead_unwind.main-{closure#0}.StateTransform.before.mir', src/tools/compiletest/src/runtest.rs:3466:25
---- [mir-opt] src/test/mir-opt/inline/inline-generator.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/inline-generator.rs stdout ----
80 +         _13 = move (_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
81 +         _12 = discriminant((*_13));      // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
82 +         StorageDead(_13);                // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
- +         switchInt(move _12) -> [0_u32: bb3, 1_u32: bb8, 3_u32: bb7, otherwise: bb9]; // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
+ +         switchInt(move _12) -> [0_u32: bb3, 1_u32: bb8, otherwise: bb7]; // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
85   
86 -     bb3: {

147 + 
147 + 
148 +     bb8: {
149 +         assert(const false, "generator resumed after completion") -> [success: bb8, unwind: bb2]; // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
- +     }
- +     bb9: {
- +     bb9: {
- +         unreachable;                     // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
155   }
156   


thread '[mir-opt] src/test/mir-opt/inline/inline-generator.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_generator.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3466:25

failures:
    [mir-opt] src/test/mir-opt/generator-storage-dead-unwind.rs
    [mir-opt] src/test/mir-opt/generator-tiny.rs
