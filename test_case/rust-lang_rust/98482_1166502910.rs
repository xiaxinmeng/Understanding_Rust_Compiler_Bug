plain
 finished in 0.618 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 178 tests
i......................................i............................................FF.F 88/178
......iF...F.......F.....................................i.....F........................ 176/178
failures:
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] src/test/mir-opt/generator-storage-dead-unwind.rs stdout ----
---- [mir-opt] src/test/mir-opt/generator-storage-dead-unwind.rs stdout ----
1 // MIR for `main::{closure#0}` before StateTransform
2 
- fn main::{closure#0}(_1: [generator@$DIR/generator-storage-dead-unwind.rs:22:16: 28:6], _2: ()) -> ()
+ fn main::{closure#0}(_1: [generator@$DIR/generator-storage-dead-unwind.rs:22:16: 22:18], _2: ()) -> ()
4 yields ()
5  {
6     let mut _0: ();                      // return place in scope 0 at $DIR/generator-storage-dead-unwind.rs:22:19: 22:19
66     }
67 
68     bb4: {
68     bb4: {
-         return;                          // scope 0 at $DIR/generator-storage-dead-unwind.rs:28:6: 28:6
+         return;                          // scope 0 at $DIR/generator-storage-dead-unwind.rs:22:18: 22:18
71 
72     bb5: {

82     }
82     }
83 
84     bb7: {
-         generator_drop;                  // scope 0 at $DIR/generator-storage-dead-unwind.rs:22:16: 28:6
+         generator_drop;                  // scope 0 at $DIR/generator-storage-dead-unwind.rs:22:16: 22:18
87 
87 
88     bb8 (cleanup): {
104     }
105 
105 
106     bb11 (cleanup): {
-         resume;                          // scope 0 at $DIR/generator-storage-dead-unwind.rs:22:16: 28:6
+         resume;                          // scope 0 at $DIR/generator-storage-dead-unwind.rs:22:16: 22:18
109 
109 
110     bb12 (cleanup): {

thread '[mir-opt] src/test/mir-opt/generator-storage-dead-unwind.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/generator_storage_dead_unwind.main-{closure#0}.StateTransform.before.mir', src/tools/compiletest/src/runtest.rs:3439:25

---- [mir-opt] src/test/mir-opt/generator-drop-cleanup.rs stdout ----
14     },
15 } */
15 } */
16 
- fn main::{closure#0}(_1: *mut [generator@$DIR/generator-drop-cleanup.rs:10:15: 13:6]) -> () {
-     let mut _0: ();                      // return place in scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
-     let mut _2: ();                      // in scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
+ fn main::{closure#0}(_1: *mut [generator@$DIR/generator-drop-cleanup.rs:10:15: 10:17]) -> () {
+     let mut _0: ();                      // return place in scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 10:17
+     let mut _2: ();                      // in scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 10:17
20     let _3: std::string::String;         // in scope 0 at $DIR/generator-drop-cleanup.rs:11:13: 11:15
21     let _4: ();                          // in scope 0 at $DIR/generator-drop-cleanup.rs:12:9: 12:14
22     let mut _5: ();                      // in scope 0 at $DIR/generator-drop-cleanup.rs:12:9: 12:14

23     let mut _6: ();                      // in scope 0 at $DIR/generator-drop-cleanup.rs:10:18: 10:18
-     let mut _7: ();                      // in scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
-     let mut _8: u32;                     // in scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
+     let mut _7: ();                      // in scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 10:17
+     let mut _8: u32;                     // in scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 10:17
26     scope 1 {
27         debug _s => (((*_1) as variant#3).0: std::string::String); // in scope 1 at $DIR/generator-drop-cleanup.rs:11:13: 11:15

29 
30     bb0: {
30     bb0: {
-         _8 = discriminant((*_1));        // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
-         switchInt(move _8) -> [0_u32: bb7, 3_u32: bb10, otherwise: bb11]; // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
+         _8 = discriminant((*_1));        // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 10:17
+         switchInt(move _8) -> [0_u32: bb7, 3_u32: bb10, otherwise: bb11]; // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 10:17
34 
35     bb1: {

44     }
44     }
45 
46     bb3: {
-         return;                          // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
+         return;                          // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 10:17
49 
49 
50     bb4 (cleanup): {

-         resume;                          // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
+         resume;                          // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 10:17
53 
53 
54     bb5 (cleanup): {
57     }
58 
59     bb6: {
59     bb6: {
-         return;                          // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
+         return;                          // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 10:17
62 
63     bb7: {


-         goto -> bb9;                     // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
+         goto -> bb9;                     // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 10:17
66 
67     bb8: {

69     }
69     }
70 
71     bb9: {
-         goto -> bb6;                     // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
+         goto -> bb6;                     // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 10:17
74 
75     bb10: {


-         StorageLive(_4);                 // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
-         StorageLive(_5);                 // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
-         goto -> bb1;                     // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
+         StorageLive(_4);                 // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 10:17
+         StorageLive(_5);                 // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 10:17
+         goto -> bb1;                     // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 10:17
80 
81     bb11: {


-         return;                          // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 13:6
+         return;                          // scope 0 at $DIR/generator-drop-cleanup.rs:10:15: 10:17
84 }
85 


thread '[mir-opt] src/test/mir-opt/generator-drop-cleanup.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/generator_drop_cleanup.main-{closure#0}.generator_drop.0.mir', src/tools/compiletest/src/runtest.rs:3439:25
---- [mir-opt] src/test/mir-opt/generator-tiny.rs stdout ----
14     },
15 } */
16 
16 
- fn main::{closure#0}(_1: Pin<&mut [generator@$DIR/generator-tiny.rs:19:16: 25:6]>, _2: u8) -> GeneratorState<(), ()> {
+ fn main::{closure#0}(_1: Pin<&mut [generator@$DIR/generator-tiny.rs:19:16: 19:24]>, _2: u8) -> GeneratorState<(), ()> {
18     debug _x => _10;                     // in scope 0 at $DIR/generator-tiny.rs:19:17: 19:19
-     let mut _0: std::ops::GeneratorState<(), ()>; // return place in scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
+     let mut _0: std::ops::GeneratorState<(), ()>; // return place in scope 0 at $DIR/generator-tiny.rs:19:16: 19:24
20     let _3: HasDrop;                     // in scope 0 at $DIR/generator-tiny.rs:20:13: 20:15
21     let mut _4: !;                       // in scope 0 at $DIR/generator-tiny.rs:21:9: 24:10
-     let mut _5: ();                      // in scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
+     let mut _5: ();                      // in scope 0 at $DIR/generator-tiny.rs:19:16: 19:24
23     let _6: u8;                          // in scope 0 at $DIR/generator-tiny.rs:22:13: 22:18
24     let mut _7: ();                      // in scope 0 at $DIR/generator-tiny.rs:22:13: 22:18
25     let _8: ();                          // in scope 0 at $DIR/generator-tiny.rs:23:13: 23:21

26     let mut _9: ();                      // in scope 0 at $DIR/generator-tiny.rs:19:25: 19:25
27     let _10: u8;                         // in scope 0 at $DIR/generator-tiny.rs:19:17: 19:19
-     let mut _11: u32;                    // in scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
+     let mut _11: u32;                    // in scope 0 at $DIR/generator-tiny.rs:19:16: 19:24
29     scope 1 {
-         debug _d => (((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:19:16: 25:6])) as variant#3).0: HasDrop); // in scope 1 at $DIR/generator-tiny.rs:20:13: 20:15
+         debug _d => (((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:19:16: 19:24])) as variant#3).0: HasDrop); // in scope 1 at $DIR/generator-tiny.rs:20:13: 20:15
32 
33     bb0: {


-         _11 = discriminant((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:19:16: 25:6]))); // scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
-         switchInt(move _11) -> [0_u32: bb1, 3_u32: bb5, otherwise: bb6]; // scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
+         _11 = discriminant((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:19:16: 19:24]))); // scope 0 at $DIR/generator-tiny.rs:19:16: 19:24
+         switchInt(move _11) -> [0_u32: bb1, 3_u32: bb5, otherwise: bb6]; // scope 0 at $DIR/generator-tiny.rs:19:16: 19:24
37 
38     bb1: {


-         _10 = move _2;                   // scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
+         _10 = move _2;                   // scope 0 at $DIR/generator-tiny.rs:19:16: 19:24
40         nop;                             // scope 0 at $DIR/generator-tiny.rs:20:13: 20:15
-         Deinit((((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:19:16: 25:6])) as variant#3).0: HasDrop)); // scope 0 at $DIR/generator-tiny.rs:20:18: 20:25
+         Deinit((((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:19:16: 19:24])) as variant#3).0: HasDrop)); // scope 0 at $DIR/generator-tiny.rs:20:18: 20:25
42         StorageLive(_4);                 // scope 1 at $DIR/generator-tiny.rs:21:9: 24:10
43         goto -> bb2;                     // scope 1 at $DIR/generator-tiny.rs:21:9: 24:10


50         Deinit(_0);                      // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
51         ((_0 as Yielded).0: ()) = move _7; // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
52         discriminant(_0) = 0;            // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
-         discriminant((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:19:16: 25:6]))) = 3; // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
+         discriminant((*(_1.0: &mut [generator@$DIR/generator-tiny.rs:19:16: 19:24]))) = 3; // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
54         return;                          // scope 1 at $DIR/generator-tiny.rs:22:13: 22:18
56 

71     }
72 
72 
73     bb5: {
-         StorageLive(_4);                 // scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
-         StorageLive(_6);                 // scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
-         StorageLive(_7);                 // scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
-         _6 = move _2;                    // scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
-         goto -> bb3;                     // scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
+         StorageLive(_4);                 // scope 0 at $DIR/generator-tiny.rs:19:16: 19:24
+         StorageLive(_6);                 // scope 0 at $DIR/generator-tiny.rs:19:16: 19:24
+         StorageLive(_7);                 // scope 0 at $DIR/generator-tiny.rs:19:16: 19:24
+         _6 = move _2;                    // scope 0 at $DIR/generator-tiny.rs:19:16: 19:24
+         goto -> bb3;                     // scope 0 at $DIR/generator-tiny.rs:19:16: 19:24
80 
81     bb6: {


-         unreachable;                     // scope 0 at $DIR/generator-tiny.rs:19:16: 25:6
+         unreachable;                     // scope 0 at $DIR/generator-tiny.rs:19:16: 19:24
84 }
85 


thread '[mir-opt] src/test/mir-opt/generator-tiny.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/generator_tiny.main-{closure#0}.generator_resume.0.mir', src/tools/compiletest/src/runtest.rs:3439:25
---- [mir-opt] src/test/mir-opt/inline/inline-closure-captures.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/inline-closure-captures.rs stdout ----
19             debug t => (*((*_6).1: &T)); // in scope 2 at $DIR/inline-closure-captures.rs:10:17: 10:18
20             let mut _10: i32;            // in scope 2 at $DIR/inline-closure-captures.rs:11:19: 11:20
21             let mut _11: T;              // in scope 2 at $DIR/inline-closure-captures.rs:11:22: 11:23
-             let mut _12: &i32;           // in scope 2 at $DIR/inline-closure-captures.rs:11:13: 11:24
-             let mut _13: &T;             // in scope 2 at $DIR/inline-closure-captures.rs:11:13: 11:24
+             let mut _12: &i32;           // in scope 2 at $DIR/inline-closure-captures.rs:11:13: 11:17
+             let mut _13: &T;             // in scope 2 at $DIR/inline-closure-captures.rs:11:13: 11:17
25     }
26 


33         Deinit(_3);                      // scope 0 at $DIR/inline-closure-captures.rs:11:13: 11:24
34         (_3.0: &i32) = move _4;          // scope 0 at $DIR/inline-closure-captures.rs:11:13: 11:24
35         (_3.1: &T) = move _5;            // scope 0 at $DIR/inline-closure-captures.rs:11:13: 11:24
-         StorageDead(_5);                 // scope 0 at $DIR/inline-closure-captures.rs:11:23: 11:24
-         StorageDead(_4);                 // scope 0 at $DIR/inline-closure-captures.rs:11:23: 11:24
+         StorageDead(_5);                 // scope 0 at $DIR/inline-closure-captures.rs:11:16: 11:17
+         StorageDead(_4);                 // scope 0 at $DIR/inline-closure-captures.rs:11:16: 11:17
38         StorageLive(_6);                 // scope 1 at $DIR/inline-closure-captures.rs:12:5: 12:6
39         _6 = &_3;                        // scope 1 at $DIR/inline-closure-captures.rs:12:5: 12:6
40         StorageLive(_7);                 // scope 1 at $DIR/inline-closure-captures.rs:12:5: 12:9

thread '[mir-opt] src/test/mir-opt/inline/inline-closure-captures.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_closure_captures.foo.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3439:25
---- [mir-opt] src/test/mir-opt/inline/inline-generator.rs stdout ----
4   fn main() -> () {
4   fn main() -> () {
5       let mut _0: ();                      // return place in scope 0 at $DIR/inline-generator.rs:8:11: 8:11
6       let _1: std::ops::GeneratorState<i32, bool>; // in scope 0 at $DIR/inline-generator.rs:9:9: 9:11
-       let mut _2: std::pin::Pin<&mut [generator@$DIR/inline-generator.rs:15:5: 15:41]>; // in scope 0 at $DIR/inline-generator.rs:9:14: 9:32
-       let mut _3: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]; // in scope 0 at $DIR/inline-generator.rs:9:23: 9:31
-       let mut _4: [generator@$DIR/inline-generator.rs:15:5: 15:41]; // in scope 0 at $DIR/inline-generator.rs:9:28: 9:31
+       let mut _2: std::pin::Pin<&mut [generator@$DIR/inline-generator.rs:15:5: 15:8]>; // in scope 0 at $DIR/inline-generator.rs:9:14: 9:32
+       let mut _3: &mut [generator@$DIR/inline-generator.rs:15:5: 15:8]; // in scope 0 at $DIR/inline-generator.rs:9:23: 9:31
+       let mut _4: [generator@$DIR/inline-generator.rs:15:5: 15:8]; // in scope 0 at $DIR/inline-generator.rs:9:28: 9:31
10 +     let mut _7: bool;                    // in scope 0 at $DIR/inline-generator.rs:9:14: 9:46
11       scope 1 {
12           debug _r => _1;                  // in scope 1 at $DIR/inline-generator.rs:9:9: 9:11
13       }
13       }
14 +     scope 2 (inlined g) {                // at $DIR/inline-generator.rs:9:28: 9:31
15 +     }
- +     scope 3 (inlined Pin::<&mut [generator@$DIR/inline-generator.rs:15:5: 15:41]>::new) { // at $DIR/inline-generator.rs:9:14: 9:32
+ +     scope 3 (inlined Pin::<&mut [generator@$DIR/inline-generator.rs:15:5: 15:8]>::new) { // at $DIR/inline-generator.rs:9:14: 9:32
17 +         debug pointer => _3;             // in scope 3 at $SRC_DIR/core/src/pin.rs:LL:COL
- +         let mut _5: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]; // in scope 3 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +         let mut _5: &mut [generator@$DIR/inline-generator.rs:15:5: 15:8]; // in scope 3 at $SRC_DIR/core/src/pin.rs:LL:COL
19 +         scope 4 {
- +             scope 5 (inlined Pin::<&mut [generator@$DIR/inline-generator.rs:15:5: 15:41]>::new_unchecked) { // at $SRC_DIR/core/src/pin.rs:LL:COL
+ +             scope 5 (inlined Pin::<&mut [generator@$DIR/inline-generator.rs:15:5: 15:8]>::new_unchecked) { // at $SRC_DIR/core/src/pin.rs:LL:COL
21 +                 debug pointer => _5;     // in scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
- +                 let mut _6: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]; // in scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +                 let mut _6: &mut [generator@$DIR/inline-generator.rs:15:5: 15:8]; // in scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
24 +         }
25 +     }


29 +         let mut _9: bool;                // in scope 6 at $DIR/inline-generator.rs:15:20: 15:21
30 +         let mut _10: bool;               // in scope 6 at $DIR/inline-generator.rs:15:9: 15:9
31 +         let _11: bool;                   // in scope 6 at $DIR/inline-generator.rs:15:6: 15:7
- +         let mut _12: u32;                // in scope 6 at $DIR/inline-generator.rs:15:5: 15:41
- +         let mut _13: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]; // in scope 6 at $DIR/inline-generator.rs:15:5: 15:41
- +         let mut _14: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]; // in scope 6 at $DIR/inline-generator.rs:15:5: 15:41
- +         let mut _15: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]; // in scope 6 at $DIR/inline-generator.rs:15:5: 15:41
+ +         let mut _12: u32;                // in scope 6 at $DIR/inline-generator.rs:15:5: 15:8
+ +         let mut _13: &mut [generator@$DIR/inline-generator.rs:15:5: 15:8]; // in scope 6 at $DIR/inline-generator.rs:15:5: 15:8
+ +         let mut _14: &mut [generator@$DIR/inline-generator.rs:15:5: 15:8]; // in scope 6 at $DIR/inline-generator.rs:15:5: 15:8
+ +         let mut _15: &mut [generator@$DIR/inline-generator.rs:15:5: 15:8]; // in scope 6 at $DIR/inline-generator.rs:15:5: 15:8
37   
38       bb0: {


50 +         Deinit(_4);                      // scope 2 at $DIR/inline-generator.rs:15:5: 15:41
51 +         discriminant(_4) = 0;            // scope 2 at $DIR/inline-generator.rs:15:5: 15:41
52           _3 = &mut _4;                    // scope 0 at $DIR/inline-generator.rs:9:23: 9:31
- -         _2 = Pin::<&mut [generator@$DIR/inline-generator.rs:15:5: 15:41]>::new(move _3) -> [return: bb2, unwind: bb4]; // scope 0 at $DIR/inline-generator.rs:9:14: 9:32
+ -         _2 = Pin::<&mut [generator@$DIR/inline-generator.rs:15:5: 15:8]>::new(move _3) -> [return: bb2, unwind: bb4]; // scope 0 at $DIR/inline-generator.rs:9:14: 9:32
54 -                                          // mir::Constant
55 -                                          // + span: $DIR/inline-generator.rs:9:14: 9:22
56 -                                          // + user_ty: UserType(0)

- -                                          // + literal: Const { ty: fn(&mut [generator@$DIR/inline-generator.rs:15:5: 15:41]) -> Pin<&mut [generator@$DIR/inline-generator.rs:15:5: 15:41]> {Pin::<&mut [generator@$DIR/inline-generator.rs:15:5: 15:41]>::new}, val: Value(Scalar(<ZST>)) }
+ -                                          // + literal: Const { ty: fn(&mut [generator@$DIR/inline-generator.rs:15:5: 15:8]) -> Pin<&mut [generator@$DIR/inline-generator.rs:15:5: 15:8]> {Pin::<&mut [generator@$DIR/inline-generator.rs:15:5: 15:8]>::new}, val: Value(Scalar(<ZST>)) }
59 - 
60 -     bb2: {


63 +         StorageLive(_6);                 // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
64 +         _6 = move _5;                    // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
65 +         Deinit(_2);                      // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
- +         (_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]) = move _6; // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
+ +         (_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:8]) = move _6; // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
67 +         StorageDead(_6);                 // scope 5 at $SRC_DIR/core/src/pin.rs:LL:COL
68 +         StorageDead(_5);                 // scope 4 at $SRC_DIR/core/src/pin.rs:LL:COL
69           StorageDead(_3);                 // scope 0 at $DIR/inline-generator.rs:9:31: 9:32

- -         _1 = <[generator@$DIR/inline-generator.rs:15:5: 15:41] as Generator<bool>>::resume(move _2, const false) -> [return: bb3, unwind: bb4]; // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
+ -         _1 = <[generator@$DIR/inline-generator.rs:15:5: 15:8] as Generator<bool>>::resume(move _2, const false) -> [return: bb3, unwind: bb4]; // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
71 -                                          // mir::Constant
72 -                                          // + span: $DIR/inline-generator.rs:9:33: 9:39
- -                                          // + literal: Const { ty: for<'r> fn(Pin<&'r mut [generator@$DIR/inline-generator.rs:15:5: 15:41]>, bool) -> GeneratorState<<[generator@$DIR/inline-generator.rs:15:5: 15:41] as Generator<bool>>::Yield, <[generator@$DIR/inline-generator.rs:15:5: 15:41] as Generator<bool>>::Return> {<[generator@$DIR/inline-generator.rs:15:5: 15:41] as Generator<bool>>::resume}, val: Value(Scalar(<ZST>)) }
+ -                                          // + literal: Const { ty: for<'r> fn(Pin<&'r mut [generator@$DIR/inline-generator.rs:15:5: 15:8]>, bool) -> GeneratorState<<[generator@$DIR/inline-generator.rs:15:5: 15:8] as Generator<bool>>::Yield, <[generator@$DIR/inline-generator.rs:15:5: 15:8] as Generator<bool>>::Return> {<[generator@$DIR/inline-generator.rs:15:5: 15:8] as Generator<bool>>::resume}, val: Value(Scalar(<ZST>)) }
74 +         StorageLive(_7);                 // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
75 +         _7 = const false;                // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
76 +         StorageLive(_10);                // scope 0 at $DIR/inline-generator.rs:9:14: 9:46

77 +         StorageLive(_11);                // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
78 +         StorageLive(_12);                // scope 0 at $DIR/inline-generator.rs:9:14: 9:46
- +         StorageLive(_13);                // scope 6 at $DIR/inline-generator.rs:15:5: 15:41
- +         _13 = move (_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]); // scope 6 at $DIR/inline-generator.rs:15:5: 15:41
- +         _12 = discriminant((*_13));      // scope 6 at $DIR/inline-generator.rs:15:5: 15:41
- +         StorageDead(_13);                // scope 6 at $DIR/inline-generator.rs:15:5: 15:41
- +         switchInt(move _12) -> [0_u32: bb3, 1_u32: bb8, 3_u32: bb7, otherwise: bb9]; // scope 6 at $DIR/inline-generator.rs:15:5: 15:41
+ +         StorageLive(_13);                // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
+ +         _13 = move (_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
+ +         _12 = discriminant((*_13));      // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
+ +         StorageDead(_13);                // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
+ +         switchInt(move _12) -> [0_u32: bb3, 1_u32: bb8, 3_u32: bb7, otherwise: bb9]; // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
85   
86 -     bb3: {

102 +     }
102 +     }
103 + 
104 +     bb3: {
- +         _11 = move _7;                   // scope 6 at $DIR/inline-generator.rs:15:5: 15:41
+ +         _11 = move _7;                   // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
106 +         StorageLive(_8);                 // scope 6 at $DIR/inline-generator.rs:15:17: 15:39
107 +         StorageLive(_9);                 // scope 6 at $DIR/inline-generator.rs:15:20: 15:21
108 +         _9 = _11;                        // scope 6 at $DIR/inline-generator.rs:15:20: 15:21

125 +         ((_1 as Yielded).0: i32) = move _8; // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
126 +         discriminant(_1) = 0;            // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
127 +         StorageLive(_14);                // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
- +         _14 = move (_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]); // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
+ +         _14 = move (_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
129 +         discriminant((*_14)) = 3;        // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
130 +         StorageDead(_14);                // scope 6 at $DIR/inline-generator.rs:15:11: 15:39
131 +         goto -> bb1;                     // scope 0 at $DIR/inline-generator.rs:15:11: 15:39
132 +     }
133 + 
134 +     bb7: {
134 +     bb7: {
- +         StorageLive(_8);                 // scope 6 at $DIR/inline-generator.rs:15:5: 15:41
- +         _10 = move _7;                   // scope 6 at $DIR/inline-generator.rs:15:5: 15:41
+ +         StorageLive(_8);                 // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
+ +         _10 = move _7;                   // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
137 +         StorageDead(_8);                 // scope 6 at $DIR/inline-generator.rs:15:38: 15:39
- +         Deinit(_1);                      // scope 6 at $DIR/inline-generator.rs:15:41: 15:41
- +         ((_1 as Complete).0: bool) = move _10; // scope 6 at $DIR/inline-generator.rs:15:41: 15:41
- +         discriminant(_1) = 1;            // scope 6 at $DIR/inline-generator.rs:15:41: 15:41
- +         StorageLive(_15);                // scope 6 at $DIR/inline-generator.rs:15:41: 15:41
- +         _15 = move (_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:41]); // scope 6 at $DIR/inline-generator.rs:15:41: 15:41
- +         discriminant((*_15)) = 1;        // scope 6 at $DIR/inline-generator.rs:15:41: 15:41
- +         StorageDead(_15);                // scope 6 at $DIR/inline-generator.rs:15:41: 15:41
- +         goto -> bb1;                     // scope 0 at $DIR/inline-generator.rs:15:41: 15:41
+ +         Deinit(_1);                      // scope 6 at $DIR/inline-generator.rs:15:8: 15:8
+ +         ((_1 as Complete).0: bool) = move _10; // scope 6 at $DIR/inline-generator.rs:15:8: 15:8
+ +         discriminant(_1) = 1;            // scope 6 at $DIR/inline-generator.rs:15:8: 15:8
+ +         StorageLive(_15);                // scope 6 at $DIR/inline-generator.rs:15:8: 15:8
+ +         _15 = move (_2.0: &mut [generator@$DIR/inline-generator.rs:15:5: 15:8]); // scope 6 at $DIR/inline-generator.rs:15:8: 15:8
+ +         discriminant((*_15)) = 1;        // scope 6 at $DIR/inline-generator.rs:15:8: 15:8
+ +         StorageDead(_15);                // scope 6 at $DIR/inline-generator.rs:15:8: 15:8
+ +         goto -> bb1;                     // scope 0 at $DIR/inline-generator.rs:15:8: 15:8
147 + 
148 +     bb8: {


- +         assert(const false, "generator resumed after completion") -> [success: bb8, unwind: bb2]; // scope 6 at $DIR/inline-generator.rs:15:5: 15:41
+ +         assert(const false, "generator resumed after completion") -> [success: bb8, unwind: bb2]; // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
151 + 
152 +     bb9: {


- +         unreachable;                     // scope 6 at $DIR/inline-generator.rs:15:5: 15:41
+ +         unreachable;                     // scope 6 at $DIR/inline-generator.rs:15:5: 15:8
155   }
156   


thread '[mir-opt] src/test/mir-opt/inline/inline-generator.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_generator.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3439:25
---- [mir-opt] src/test/mir-opt/inline/issue-76997-inline-scopes-parenting.rs stdout ----
2 
3 fn main() -> () {
3 fn main() -> () {
4     let mut _0: ();                      // return place in scope 0 at $DIR/issue-76997-inline-scopes-parenting.rs:4:11: 4:11
-     let _1: [closure@$DIR/issue-76997-inline-scopes-parenting.rs:5:13: 5:33]; // in scope 0 at $DIR/issue-76997-inline-scopes-parenting.rs:5:9: 5:10
-     let mut _2: &[closure@$DIR/issue-76997-inline-scopes-parenting.rs:5:13: 5:33]; // in scope 0 at $DIR/issue-76997-inline-scopes-parenting.rs:6:5: 6:6
+     let _1: [closure@$DIR/issue-76997-inline-scopes-parenting.rs:5:13: 5:16]; // in scope 0 at $DIR/issue-76997-inline-scopes-parenting.rs:5:9: 5:10
+     let mut _2: &[closure@$DIR/issue-76997-inline-scopes-parenting.rs:5:13: 5:16]; // in scope 0 at $DIR/issue-76997-inline-scopes-parenting.rs:6:5: 6:6
7     let mut _3: ((),);                   // in scope 0 at $DIR/issue-76997-inline-scopes-parenting.rs:6:5: 6:10
8     let mut _4: ();                      // in scope 0 at $DIR/issue-76997-inline-scopes-parenting.rs:6:7: 6:9
9     let mut _5: ();                      // in scope 0 at $DIR/issue-76997-inline-scopes-parenting.rs:6:5: 6:10

thread '[mir-opt] src/test/mir-opt/inline/issue-76997-inline-scopes-parenting.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/issue_76997_inline_scopes_parenting.main.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3439:25
---- [mir-opt] src/test/mir-opt/retag.rs stdout ----
121                                          // ]
121                                          // ]
122         Retag(_14);                      // scope 1 at $DIR/retag.rs:40:31: 43:6
123         _13 = move _14 as for<'r> fn(&'r i32) -> &'r i32 (Pointer(ClosureFnPointer(Normal))); // scope 1 at $DIR/retag.rs:40:31: 43:6
-         StorageDead(_14);                // scope 1 at $DIR/retag.rs:43:5: 43:6
+         StorageDead(_14);                // scope 1 at $DIR/retag.rs:40:47: 40:48
125         StorageLive(_15);                // scope 6 at $DIR/retag.rs:44:9: 44:11
126         StorageLive(_16);                // scope 6 at $DIR/retag.rs:44:14: 44:15
127         _16 = _13;                       // scope 6 at $DIR/retag.rs:44:14: 44:15

thread '[mir-opt] src/test/mir-opt/retag.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/retag.main.SimplifyCfg-elaborate-drops.after.mir', src/tools/compiletest/src/runtest.rs:3439:25

failures:
    [mir-opt] src/test/mir-opt/generator-drop-cleanup.rs
    [mir-opt] src/test/mir-opt/generator-storage-dead-unwind.rs
