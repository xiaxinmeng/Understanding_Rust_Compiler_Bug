plain
.................................................................................................... 9200/11475
.................................................................................................... 9300/11475
.................................................................................................... 9400/11475
.............................i......i............................................................... 9500/11475
....................................................................iiiiiii..iiiiii.i............... 9600/11475
.................................................................................................... 9800/11475
.................................................................................................... 9900/11475
.................................................................................................... 10000/11475
.................................................................................................... 10100/11475
---
Suite("src/test/mir-opt") not skipped for "bootstrap::test::MirOpt" -- not in ["src/tools/tidy"]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 154 tests
................................i...........................................i.....................F. 100/154
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.....................i.......F.......F................

---- [mir-opt] mir-opt/issue-62289.rs stdout ----
---- [mir-opt] mir-opt/issue-62289.rs stdout ----
4     let mut _0: std::option::Option<std::boxed::Box<u32>>; // return place in scope 0 at $DIR/issue-62289.rs:8:14: 8:30
5     let mut _1: std::boxed::Box<u32>;    // in scope 0 at $DIR/issue-62289.rs:9:10: 9:21
6     let mut _2: std::boxed::Box<u32>;    // in scope 0 at $DIR/issue-62289.rs:9:10: 9:21
-     let mut _3: std::result::Result<u32, std::option::NoneError>; // in scope 0 at $DIR/issue-62289.rs:9:15: 9:20
+     let mut _3: std::ops::ControlFlow<std::option::Option<!>, u32>; // in scope 0 at $DIR/issue-62289.rs:9:15: 9:20
8     let mut _4: std::option::Option<u32>; // in scope 0 at $DIR/issue-62289.rs:9:15: 9:19
-     let mut _5: isize;                   // in scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-     let _6: std::option::NoneError;      // in scope 0 at $DIR/issue-62289.rs:9:19: 9:20
+     let mut _5: isize;                   // in scope 0 at $DIR/issue-62289.rs:9:15: 9:20
+     let _6: std::option::Option<!>;      // in scope 0 at $DIR/issue-62289.rs:9:19: 9:20
11     let mut _7: !;                       // in scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-     let mut _8: std::option::NoneError;  // in scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-     let mut _9: std::option::NoneError;  // in scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-     let _10: u32;                        // in scope 0 at $DIR/issue-62289.rs:9:15: 9:20
+     let mut _8: std::option::Option<!>;  // in scope 0 at $DIR/issue-62289.rs:9:19: 9:20
+     let _9: u32;                         // in scope 0 at $DIR/issue-62289.rs:9:15: 9:20
15     scope 1 {
-         debug err => _6;                 // in scope 1 at $DIR/issue-62289.rs:9:19: 9:20
+         debug holder => _6;              // in scope 1 at $DIR/issue-62289.rs:9:19: 9:20
17         scope 2 {
19     }

20     scope 3 {
20     scope 3 {
-         debug val => _10;                // in scope 3 at $DIR/issue-62289.rs:9:15: 9:20
+         debug val => _9;                 // in scope 3 at $DIR/issue-62289.rs:9:15: 9:20
22         scope 4 {
24     }


30         StorageLive(_3);                 // scope 0 at $DIR/issue-62289.rs:9:15: 9:20
31         StorageLive(_4);                 // scope 0 at $DIR/issue-62289.rs:9:15: 9:19
32         _4 = Option::<u32>::None;        // scope 0 at $DIR/issue-62289.rs:9:15: 9:19
-         _3 = <Option<u32> as Try>::into_result(move _4) -> [return: bb1, unwind: bb12]; // scope 0 at $DIR/issue-62289.rs:9:15: 9:20
+         _3 = <Option<u32> as Try>::branch(move _4) -> [return: bb1, unwind: bb11]; // scope 0 at $DIR/issue-62289.rs:9:15: 9:20
34                                          // mir::Constant
35                                          // + span: $DIR/issue-62289.rs:9:15: 9:20
-                                          // + literal: Const { ty: fn(std::option::Option<u32>) -> std::result::Result<<std::option::Option<u32> as std::ops::Try>::Ok, <std::option::Option<u32> as std::ops::Try>::Error> {<std::option::Option<u32> as std::ops::Try>::into_result}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: fn(std::option::Option<u32>) -> std::ops::ControlFlow<<std::option::Option<u32> as std::ops::Try2021>::Residual, <std::option::Option<u32> as std::ops::Try2021>::Ok> {<std::option::Option<u32> as std::ops::Try2021>::branch}, val: Value(Scalar(<ZST>)) }
38 
39     bb1: {


40         StorageDead(_4);                 // scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-         _5 = discriminant(_3);           // scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-         switchInt(move _5) -> [0_isize: bb2, 1_isize: bb4, otherwise: bb3]; // scope 0 at $DIR/issue-62289.rs:9:19: 9:20
+         _5 = discriminant(_3);           // scope 0 at $DIR/issue-62289.rs:9:15: 9:20
+         switchInt(move _5) -> [0_isize: bb2, 1_isize: bb4, otherwise: bb3]; // scope 0 at $DIR/issue-62289.rs:9:15: 9:20
44 
45     bb2: {


-         StorageLive(_10);                // scope 0 at $DIR/issue-62289.rs:9:15: 9:20
-         _10 = ((_3 as Ok).0: u32);       // scope 0 at $DIR/issue-62289.rs:9:15: 9:20
-         (*_2) = _10;                     // scope 4 at $DIR/issue-62289.rs:9:15: 9:20
-         StorageDead(_10);                // scope 0 at $DIR/issue-62289.rs:9:19: 9:20
+         StorageLive(_9);                 // scope 0 at $DIR/issue-62289.rs:9:15: 9:20
+         _9 = ((_3 as Continue).0: u32);  // scope 0 at $DIR/issue-62289.rs:9:15: 9:20
+         (*_2) = _9;                      // scope 4 at $DIR/issue-62289.rs:9:15: 9:20
+         StorageDead(_9);                 // scope 0 at $DIR/issue-62289.rs:9:19: 9:20
50         _1 = move _2;                    // scope 0 at $DIR/issue-62289.rs:9:10: 9:21
-         drop(_2) -> [return: bb7, unwind: bb11]; // scope 0 at $DIR/issue-62289.rs:9:20: 9:21
+         drop(_2) -> [return: bb6, unwind: bb10]; // scope 0 at $DIR/issue-62289.rs:9:20: 9:21
53 
54     bb3: {

57 
57 
58     bb4: {
59         StorageLive(_6);                 // scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-         _6 = ((_3 as Err).0: std::option::NoneError); // scope 0 at $DIR/issue-62289.rs:9:19: 9:20
+         _6 = ((_3 as Break).0: std::option::Option<!>); // scope 0 at $DIR/issue-62289.rs:9:19: 9:20
61         StorageLive(_8);                 // scope 2 at $DIR/issue-62289.rs:9:19: 9:20
-         StorageLive(_9);                 // scope 2 at $DIR/issue-62289.rs:9:19: 9:20
-         _9 = _6;                         // scope 2 at $DIR/issue-62289.rs:9:19: 9:20
-         _8 = <NoneError as From<NoneError>>::from(move _9) -> [return: bb5, unwind: bb12]; // scope 2 at $DIR/issue-62289.rs:9:19: 9:20
+         _8 = _6;                         // scope 2 at $DIR/issue-62289.rs:9:19: 9:20
+         _0 = <Option<Box<u32>> as FromResidual<Option<!>>>::from_residual(move _8) -> [return: bb5, unwind: bb11]; // scope 2 at $DIR/issue-62289.rs:9:15: 9:20
65                                          // mir::Constant
-                                          // + span: $DIR/issue-62289.rs:9:19: 9:20
-                                          // + literal: Const { ty: fn(std::option::NoneError) -> std::option::NoneError {<std::option::NoneError as std::convert::From<std::option::NoneError>>::from}, val: Value(Scalar(<ZST>)) }
- 
-     bb5: {
-     bb5: {
-         StorageDead(_9);                 // scope 2 at $DIR/issue-62289.rs:9:19: 9:20
-         _0 = <Option<Box<u32>> as Try>::from_error(move _8) -> [return: bb6, unwind: bb12]; // scope 2 at $DIR/issue-62289.rs:9:15: 9:20
-                                          // mir::Constant
74                                          // + span: $DIR/issue-62289.rs:9:15: 9:20
-                                          // + literal: Const { ty: fn(<std::option::Option<std::boxed::Box<u32>> as std::ops::Try>::Error) -> std::option::Option<std::boxed::Box<u32>> {<std::option::Option<std::boxed::Box<u32>> as std::ops::Try>::from_error}, val: Value(Scalar(<ZST>)) }
+                                          // + literal: Const { ty: fn(std::option::Option<!>) -> std::option::Option<std::boxed::Box<u32>> {<std::option::Option<std::boxed::Box<u32>> as std::ops::FromResidual<std::option::Option<!>>>::from_residual}, val: Value(Scalar(<ZST>)) }
77 
-     bb6: {
+     bb5: {
+     bb5: {
79         StorageDead(_8);                 // scope 2 at $DIR/issue-62289.rs:9:19: 9:20
80         StorageDead(_6);                 // scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-         drop(_2) -> bb9;                 // scope 0 at $DIR/issue-62289.rs:9:20: 9:21
+         drop(_2) -> bb8;                 // scope 0 at $DIR/issue-62289.rs:9:20: 9:21
83 
-     bb7: {
+     bb6: {
+     bb6: {
85         StorageDead(_2);                 // scope 0 at $DIR/issue-62289.rs:9:20: 9:21
86         _0 = Option::<Box<u32>>::Some(move _1); // scope 0 at $DIR/issue-62289.rs:9:5: 9:22
-         drop(_1) -> bb8;                 // scope 0 at $DIR/issue-62289.rs:9:21: 9:22
+         drop(_1) -> bb7;                 // scope 0 at $DIR/issue-62289.rs:9:21: 9:22
89 
-     bb8: {
+     bb7: {
+     bb7: {
91         StorageDead(_1);                 // scope 0 at $DIR/issue-62289.rs:9:21: 9:22
92         StorageDead(_3);                 // scope 0 at $DIR/issue-62289.rs:10:1: 10:2
-         goto -> bb10;                    // scope 0 at $DIR/issue-62289.rs:10:2: 10:2
+         goto -> bb9;                     // scope 0 at $DIR/issue-62289.rs:10:2: 10:2
95 
-     bb9: {
+     bb8: {
+     bb8: {
97         StorageDead(_2);                 // scope 0 at $DIR/issue-62289.rs:9:20: 9:21
98         StorageDead(_1);                 // scope 0 at $DIR/issue-62289.rs:9:21: 9:22
99         StorageDead(_3);                 // scope 0 at $DIR/issue-62289.rs:10:1: 10:2

-         goto -> bb10;                    // scope 0 at $DIR/issue-62289.rs:10:2: 10:2
+         goto -> bb9;                     // scope 0 at $DIR/issue-62289.rs:10:2: 10:2
102 
-     bb10: {
+     bb9: {
+     bb9: {
104         return;                          // scope 0 at $DIR/issue-62289.rs:10:2: 10:2
106 


-     bb11 (cleanup): {
-         drop(_1) -> bb13;                // scope 0 at $DIR/issue-62289.rs:9:21: 9:22
+     bb10 (cleanup): {
+         drop(_1) -> bb12;                // scope 0 at $DIR/issue-62289.rs:9:21: 9:22
110 
110 
-     bb12 (cleanup): {
-         drop(_2) -> bb13;                // scope 0 at $DIR/issue-62289.rs:9:20: 9:21
+     bb11 (cleanup): {
+         drop(_2) -> bb12;                // scope 0 at $DIR/issue-62289.rs:9:20: 9:21
114 
114 
-     bb13 (cleanup): {
+     bb12 (cleanup): {
116         resume;                          // scope 0 at $DIR/issue-62289.rs:8:1: 10:2
118 }


thread '[mir-opt] mir-opt/issue-62289.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_62289.test.ElaborateDrops.before.mir', src/tools/compiletest/src/runtest.rs:3457:25

---- [mir-opt] mir-opt/simplify-arm.rs stdout ----
---- [mir-opt] mir-opt/simplify-arm.rs stdout ----
5       debug r => _1;                       // in scope 0 at $DIR/simplify-arm.rs:23:11: 23:12
6       let mut _0: std::result::Result<u8, i32>; // return place in scope 0 at $DIR/simplify-arm.rs:23:34: 23:49
7       let _2: u8;                          // in scope 0 at $DIR/simplify-arm.rs:24:9: 24:10
-       let mut _3: std::result::Result<u8, i32>; // in scope 0 at $DIR/simplify-arm.rs:24:13: 24:15
+       let mut _3: std::ops::ControlFlow<std::result::Result<!, i32>, u8>; // in scope 0 at $DIR/simplify-arm.rs:24:13: 24:15
9       let mut _4: std::result::Result<u8, i32>; // in scope 0 at $DIR/simplify-arm.rs:24:13: 24:14
-       let mut _5: isize;                   // in scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
-       let _6: i32;                         // in scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
+       let mut _5: isize;                   // in scope 0 at $DIR/simplify-arm.rs:24:13: 24:15
+       let _6: std::result::Result<!, i32>; // in scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
12       let mut _7: !;                       // in scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
-       let mut _8: i32;                     // in scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
-       let mut _9: i32;                     // in scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
-       let _10: u8;                         // in scope 0 at $DIR/simplify-arm.rs:24:13: 24:15
-       let mut _11: u8;                     // in scope 0 at $DIR/simplify-arm.rs:25:8: 25:9
+       let mut _8: std::result::Result<!, i32>; // in scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
+       let _9: u8;                          // in scope 0 at $DIR/simplify-arm.rs:24:13: 24:15
+       let mut _10: u8;                     // in scope 0 at $DIR/simplify-arm.rs:25:8: 25:9
17       scope 1 {
- -         debug x => _2;                   // in scope 1 at $DIR/simplify-arm.rs:24:9: 24:10
- +         debug x => ((_0 as Ok).0: u8);   // in scope 1 at $DIR/simplify-arm.rs:24:9: 24:10
+           debug x => _2;                   // in scope 1 at $DIR/simplify-arm.rs:24:9: 24:10
21       scope 2 {
21       scope 2 {
- -         debug err => _6;                 // in scope 2 at $DIR/simplify-arm.rs:24:14: 24:15
- +         debug err => ((_0 as Err).0: i32); // in scope 2 at $DIR/simplify-arm.rs:24:14: 24:15
+           debug holder => _6;              // in scope 2 at $DIR/simplify-arm.rs:24:14: 24:15
24           scope 3 {
-               scope 7 (inlined <i32 as From<i32>>::from) { // at $DIR/simplify-arm.rs:24:14: 24:15
- -                 debug t => _9;           // in scope 7 at $DIR/simplify-arm.rs:24:14: 24:15
- +                 debug t => ((_0 as Err).0: i32); // in scope 7 at $DIR/simplify-arm.rs:24:14: 24:15
+               scope 9 (inlined <Result<u8, i32> as FromResidual<Result<!, i32>>>::from_residual) { // at $DIR/simplify-arm.rs:24:13: 24:15
+                   debug x => _8;           // in scope 9 at $DIR/simplify-arm.rs:24:13: 24:15
+                   let _17: i32;            // in scope 9 at $DIR/simplify-arm.rs:24:13: 24:15
+                   let mut _18: i32;        // in scope 9 at $DIR/simplify-arm.rs:24:13: 24:15
+                   let mut _19: i32;        // in scope 9 at $DIR/simplify-arm.rs:24:13: 24:15
+                   scope 10 {
+                       debug e => _17;      // in scope 10 at $DIR/simplify-arm.rs:24:13: 24:15
+                       scope 11 (inlined <i32 as From<i32>>::from) { // at $DIR/simplify-arm.rs:24:13: 24:15
+                           debug t => _19;  // in scope 11 at $DIR/simplify-arm.rs:24:13: 24:15
+                   }
28               }
28               }
-               scope 8 (inlined <Result<u8, i32> as Try>::from_error) { // at $DIR/simplify-arm.rs:24:13: 24:15
- -                 debug v => _8;           // in scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
- +                 debug v => ((_0 as Err).0: i32); // in scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
-                   let mut _12: i32;        // in scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
34           }
35       }
36       scope 4 {


- -         debug val => _10;                // in scope 4 at $DIR/simplify-arm.rs:24:13: 24:15
- +         debug val => ((_0 as Ok).0: u8); // in scope 4 at $DIR/simplify-arm.rs:24:13: 24:15
+           debug val => _9;                 // in scope 4 at $DIR/simplify-arm.rs:24:13: 24:15
39           scope 5 {
41       }


-       scope 6 (inlined <Result<u8, i32> as Try>::into_result) { // at $DIR/simplify-arm.rs:24:13: 24:15
+       scope 6 (inlined <Result<u8, i32> as Try>::branch) { // at $DIR/simplify-arm.rs:24:13: 24:15
43           debug self => _4;                // in scope 6 at $DIR/simplify-arm.rs:24:13: 24:15
+           let mut _11: isize;              // in scope 6 at $DIR/simplify-arm.rs:24:13: 24:15
+           let _12: u8;                     // in scope 6 at $DIR/simplify-arm.rs:24:13: 24:15
+           let mut _13: u8;                 // in scope 6 at $DIR/simplify-arm.rs:24:13: 24:15
+           let _14: i32;                    // in scope 6 at $DIR/simplify-arm.rs:24:13: 24:15
+           let mut _15: std::result::Result<!, i32>; // in scope 6 at $DIR/simplify-arm.rs:24:13: 24:15
+           let mut _16: i32;                // in scope 6 at $DIR/simplify-arm.rs:24:13: 24:15
+           scope 7 {
+               debug c => _12;              // in scope 7 at $DIR/simplify-arm.rs:24:13: 24:15
+           scope 8 {
+           scope 8 {
+               debug e => _14;              // in scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
44       }
45   
46       bb0: {


48           StorageLive(_3);                 // scope 0 at $DIR/simplify-arm.rs:24:13: 24:15
49           StorageLive(_4);                 // scope 0 at $DIR/simplify-arm.rs:24:13: 24:14
50           _4 = _1;                         // scope 0 at $DIR/simplify-arm.rs:24:13: 24:14
-           _3 = move _4;                    // scope 6 at $DIR/simplify-arm.rs:24:13: 24:15
-           StorageDead(_4);                 // scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
-           _5 = discriminant(_3);           // scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
-           switchInt(move _5) -> [0_isize: bb1, 1_isize: bb3, otherwise: bb2]; // scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
+           StorageLive(_11);                // scope 0 at $DIR/simplify-arm.rs:24:13: 24:15
+           _11 = discriminant(_4);          // scope 6 at $DIR/simplify-arm.rs:24:13: 24:15
+           switchInt(move _11) -> [0_isize: bb8, 1_isize: bb6, otherwise: bb7]; // scope 6 at $DIR/simplify-arm.rs:24:13: 24:15
56   
57       bb1: {


- -         StorageLive(_10);                // scope 0 at $DIR/simplify-arm.rs:24:13: 24:15
- -         _10 = ((_3 as Ok).0: u8);        // scope 0 at $DIR/simplify-arm.rs:24:13: 24:15
- -         _2 = _10;                        // scope 5 at $DIR/simplify-arm.rs:24:13: 24:15
- -         StorageDead(_10);                // scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
- +         _0 = move _3;                    // scope 1 at $DIR/simplify-arm.rs:25:5: 25:10
+           StorageDead(_11);                // scope 0 at $DIR/simplify-arm.rs:24:13: 24:15
+           StorageDead(_4);                 // scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
+           _5 = discriminant(_3);           // scope 0 at $DIR/simplify-arm.rs:24:13: 24:15
+           switchInt(move _5) -> [0_isize: bb2, 1_isize: bb4, otherwise: bb3]; // scope 0 at $DIR/simplify-arm.rs:24:13: 24:15
+   
+       bb2: {
+       bb2: {
+           StorageLive(_9);                 // scope 0 at $DIR/simplify-arm.rs:24:13: 24:15
+           _9 = ((_3 as Continue).0: u8);   // scope 0 at $DIR/simplify-arm.rs:24:13: 24:15
+           _2 = _9;                         // scope 5 at $DIR/simplify-arm.rs:24:13: 24:15
+           StorageDead(_9);                 // scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
63           StorageDead(_3);                 // scope 0 at $DIR/simplify-arm.rs:24:15: 24:16
- -         StorageLive(_11);                // scope 1 at $DIR/simplify-arm.rs:25:8: 25:9
- -         _11 = _2;                        // scope 1 at $DIR/simplify-arm.rs:25:8: 25:9
- -         ((_0 as Ok).0: u8) = move _11;   // scope 1 at $DIR/simplify-arm.rs:25:5: 25:10
- -         discriminant(_0) = 0;            // scope 1 at $DIR/simplify-arm.rs:25:5: 25:10
- -         StorageDead(_11);                // scope 1 at $DIR/simplify-arm.rs:25:9: 25:10
+           StorageLive(_10);                // scope 1 at $DIR/simplify-arm.rs:25:8: 25:9
+           _10 = _2;                        // scope 1 at $DIR/simplify-arm.rs:25:8: 25:9
+           ((_0 as Ok).0: u8) = move _10;   // scope 1 at $DIR/simplify-arm.rs:25:5: 25:10
+           discriminant(_0) = 0;            // scope 1 at $DIR/simplify-arm.rs:25:5: 25:10
+           StorageDead(_10);                // scope 1 at $DIR/simplify-arm.rs:25:9: 25:10
69           StorageDead(_2);                 // scope 0 at $DIR/simplify-arm.rs:26:1: 26:2
-           goto -> bb4;                     // scope 0 at $DIR/simplify-arm.rs:26:2: 26:2
+           goto -> bb5;                     // scope 0 at $DIR/simplify-arm.rs:26:2: 26:2
72   
-       bb2: {
+       bb3: {
+       bb3: {
74           unreachable;                     // scope 0 at $DIR/simplify-arm.rs:24:13: 24:15
76   

-       bb3: {
-       bb3: {
- -         StorageLive(_6);                 // scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
- -         _6 = ((_3 as Err).0: i32);       // scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
- -         StorageLive(_8);                 // scope 3 at $DIR/simplify-arm.rs:24:14: 24:15
- -         StorageLive(_9);                 // scope 3 at $DIR/simplify-arm.rs:24:14: 24:15
- -         _9 = _6;                         // scope 3 at $DIR/simplify-arm.rs:24:14: 24:15
- -         _8 = move _9;                    // scope 7 at $DIR/simplify-arm.rs:24:14: 24:15
- -         StorageDead(_9);                 // scope 3 at $DIR/simplify-arm.rs:24:14: 24:15
- -         StorageLive(_12);                // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
- -         _12 = move _8;                   // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
- -         ((_0 as Err).0: i32) = move _12; // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
- -         discriminant(_0) = 1;            // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
- -         StorageDead(_12);                // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
- -         StorageDead(_8);                 // scope 3 at $DIR/simplify-arm.rs:24:14: 24:15
- -         StorageDead(_6);                 // scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
- +         _0 = move _3;                    // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
+       bb4: {
+           StorageLive(_6);                 // scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
+           _6 = ((_3 as Break).0: std::result::Result<!, i32>); // scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
+           StorageLive(_8);                 // scope 3 at $DIR/simplify-arm.rs:24:14: 24:15
+           _8 = _6;                         // scope 3 at $DIR/simplify-arm.rs:24:14: 24:15
+           StorageLive(_17);                // scope 9 at $DIR/simplify-arm.rs:24:13: 24:15
+           _17 = move ((_8 as Err).0: i32); // scope 9 at $DIR/simplify-arm.rs:24:13: 24:15
+           StorageLive(_18);                // scope 10 at $DIR/simplify-arm.rs:24:13: 24:15
+           StorageLive(_19);                // scope 10 at $DIR/simplify-arm.rs:24:13: 24:15
+           _19 = move _17;                  // scope 10 at $DIR/simplify-arm.rs:24:13: 24:15
+           _18 = move _19;                  // scope 11 at $DIR/simplify-arm.rs:24:13: 24:15
+           StorageDead(_19);                // scope 10 at $DIR/simplify-arm.rs:24:13: 24:15
+           ((_0 as Err).0: i32) = move _18; // scope 10 at $DIR/simplify-arm.rs:24:13: 24:15
+           discriminant(_0) = 1;            // scope 10 at $DIR/simplify-arm.rs:24:13: 24:15
+           StorageDead(_18);                // scope 10 at $DIR/simplify-arm.rs:24:13: 24:15
+           StorageDead(_17);                // scope 9 at $DIR/simplify-arm.rs:24:13: 24:15
+           StorageDead(_8);                 // scope 3 at $DIR/simplify-arm.rs:24:14: 24:15
+           StorageDead(_6);                 // scope 0 at $DIR/simplify-arm.rs:24:14: 24:15
93           StorageDead(_3);                 // scope 0 at $DIR/simplify-arm.rs:24:15: 24:16
94           StorageDead(_2);                 // scope 0 at $DIR/simplify-arm.rs:26:1: 26:2
-           goto -> bb4;                     // scope 0 at $DIR/simplify-arm.rs:26:2: 26:2
+           goto -> bb5;                     // scope 0 at $DIR/simplify-arm.rs:26:2: 26:2
97   
-       bb4: {
+       bb5: {
+       bb5: {
99           return;                          // scope 0 at $DIR/simplify-arm.rs:26:2: 26:2
+   
+       bb6: {
+       bb6: {
+           StorageLive(_14);                // scope 6 at $DIR/simplify-arm.rs:24:13: 24:15
+           _14 = move ((_4 as Err).0: i32); // scope 6 at $DIR/simplify-arm.rs:24:13: 24:15
+           StorageLive(_15);                // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
+           StorageLive(_16);                // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
+           _16 = move _14;                  // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
+           ((_15 as Err).0: i32) = move _16; // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
+           discriminant(_15) = 1;           // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
+           StorageDead(_16);                // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
+           ((_3 as Break).0: std::result::Result<!, i32>) = move _15; // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
+           discriminant(_3) = 1;            // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
+           StorageDead(_15);                // scope 8 at $DIR/simplify-arm.rs:24:13: 24:15
+           StorageDead(_14);                // scope 6 at $DIR/simplify-arm.rs:24:13: 24:15
+           goto -> bb1;                     // scope 6 at $DIR/simplify-arm.rs:24:13: 24:15
+   
+       bb7: {
+       bb7: {
+           unreachable;                     // scope 6 at $DIR/simplify-arm.rs:24:13: 24:15
+   
+       bb8: {
+       bb8: {
+           StorageLive(_12);                // scope 6 at $DIR/simplify-arm.rs:24:13: 24:15
+           _12 = move ((_4 as Ok).0: u8);   // scope 6 at $DIR/simplify-arm.rs:24:13: 24:15
+           StorageLive(_13);                // scope 7 at $DIR/simplify-arm.rs:24:13: 24:15
+           _13 = move _12;                  // scope 7 at $DIR/simplify-arm.rs:24:13: 24:15
+           ((_3 as Continue).0: u8) = move _13; // scope 7 at $DIR/simplify-arm.rs:24:13: 24:15
+           discriminant(_3) = 0;            // scope 7 at $DIR/simplify-arm.rs:24:13: 24:15
+           StorageDead(_13);                // scope 7 at $DIR/simplify-arm.rs:24:13: 24:15
+           StorageDead(_12);                // scope 6 at $DIR/simplify-arm.rs:24:13: 24:15
+           goto -> bb1;                     // scope 6 at $DIR/simplify-arm.rs:24:13: 24:15
101   }
102   


thread '[mir-opt] mir-opt/simplify-arm.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_arm.id_try.SimplifyArmIdentity.diff', src/tools/compiletest/src/runtest.rs:3457:25
---- [mir-opt] mir-opt/simplify_try.rs stdout ----
---- [mir-opt] mir-opt/simplify_try.rs stdout ----
5       debug x => _1;                       // in scope 0 at $DIR/simplify_try.rs:7:17: 7:18
6       let mut _0: std::result::Result<u32, i32>; // return place in scope 0 at $DIR/simplify_try.rs:7:41: 7:57
7       let _2: u32;                         // in scope 0 at $DIR/simplify_try.rs:8:9: 8:10
-       let mut _3: std::result::Result<u32, i32>; // in scope 0 at $DIR/simplify_try.rs:8:13: 8:15
+       let mut _3: std::ops::ControlFlow<std::result::Result<!, i32>, u32>; // in scope 0 at $DIR/simplify_try.rs:8:13: 8:15
9       let mut _4: std::result::Result<u32, i32>; // in scope 0 at $DIR/simplify_try.rs:8:13: 8:14
-       let mut _5: isize;                   // in scope 0 at $DIR/simplify_try.rs:8:14: 8:15
-       let _6: i32;                         // in scope 0 at $DIR/simplify_try.rs:8:14: 8:15
+       let mut _5: isize;                   // in scope 0 at $DIR/simplify_try.rs:8:13: 8:15
+       let _6: std::result::Result<!, i32>; // in scope 0 at $DIR/simplify_try.rs:8:14: 8:15
12       let mut _7: !;                       // in scope 0 at $DIR/simplify_try.rs:8:14: 8:15
-       let mut _8: i32;                     // in scope 0 at $DIR/simplify_try.rs:8:14: 8:15
-       let mut _9: i32;                     // in scope 0 at $DIR/simplify_try.rs:8:14: 8:15
-       let _10: u32;                        // in scope 0 at $DIR/simplify_try.rs:8:13: 8:15
-       let mut _11: u32;                    // in scope 0 at $DIR/simplify_try.rs:9:8: 9:9
+       let mut _8: std::result::Result<!, i32>; // in scope 0 at $DIR/simplify_try.rs:8:14: 8:15
+       let _9: u32;                         // in scope 0 at $DIR/simplify_try.rs:8:13: 8:15
+       let mut _10: u32;                    // in scope 0 at $DIR/simplify_try.rs:9:8: 9:9
17       scope 1 {
- -         debug y => _2;                   // in scope 1 at $DIR/simplify_try.rs:8:9: 8:10
- +         debug y => ((_0 as Ok).0: u32);  // in scope 1 at $DIR/simplify_try.rs:8:9: 8:10
+           debug y => _2;                   // in scope 1 at $DIR/simplify_try.rs:8:9: 8:10
21       scope 2 {
21       scope 2 {
- -         debug err => _6;                 // in scope 2 at $DIR/simplify_try.rs:8:14: 8:15
- +         debug err => ((_0 as Err).0: i32); // in scope 2 at $DIR/simplify_try.rs:8:14: 8:15
+           debug holder => _6;              // in scope 2 at $DIR/simplify_try.rs:8:14: 8:15
24           scope 3 {
-               scope 7 (inlined <i32 as From<i32>>::from) { // at $DIR/simplify_try.rs:8:14: 8:15
- -                 debug t => _9;           // in scope 7 at $DIR/simplify_try.rs:8:14: 8:15
- +                 debug t => ((_0 as Err).0: i32); // in scope 7 at $DIR/simplify_try.rs:8:14: 8:15
+               scope 9 (inlined <Result<u32, i32> as FromResidual<Result<!, i32>>>::from_residual) { // at $DIR/simplify_try.rs:8:13: 8:15
+                   debug x => _8;           // in scope 9 at $DIR/simplify_try.rs:8:13: 8:15
+                   let _17: i32;            // in scope 9 at $DIR/simplify_try.rs:8:13: 8:15
+                   let mut _18: i32;        // in scope 9 at $DIR/simplify_try.rs:8:13: 8:15
+                   let mut _19: i32;        // in scope 9 at $DIR/simplify_try.rs:8:13: 8:15
+                   scope 10 {
+                       debug e => _17;      // in scope 10 at $DIR/simplify_try.rs:8:13: 8:15
+                       scope 11 (inlined <i32 as From<i32>>::from) { // at $DIR/simplify_try.rs:8:13: 8:15
+                           debug t => _19;  // in scope 11 at $DIR/simplify_try.rs:8:13: 8:15
+                   }
28               }
28               }
-               scope 8 (inlined <Result<u32, i32> as Try>::from_error) { // at $DIR/simplify_try.rs:8:13: 8:15
- -                 debug v => _8;           // in scope 8 at $DIR/simplify_try.rs:8:13: 8:15
- +                 debug v => ((_0 as Err).0: i32); // in scope 8 at $DIR/simplify_try.rs:8:13: 8:15
-                   let mut _12: i32;        // in scope 8 at $DIR/simplify_try.rs:8:13: 8:15
34           }
35       }
36       scope 4 {


- -         debug val => _10;                // in scope 4 at $DIR/simplify_try.rs:8:13: 8:15
- +         debug val => ((_0 as Ok).0: u32); // in scope 4 at $DIR/simplify_try.rs:8:13: 8:15
+           debug val => _9;                 // in scope 4 at $DIR/simplify_try.rs:8:13: 8:15
39           scope 5 {
41       }


-       scope 6 (inlined <Result<u32, i32> as Try>::into_result) { // at $DIR/simplify_try.rs:8:13: 8:15
+       scope 6 (inlined <Result<u32, i32> as Try>::branch) { // at $DIR/simplify_try.rs:8:13: 8:15
43           debug self => _4;                // in scope 6 at $DIR/simplify_try.rs:8:13: 8:15
+           let mut _11: isize;              // in scope 6 at $DIR/simplify_try.rs:8:13: 8:15
+           let _12: u32;                    // in scope 6 at $DIR/simplify_try.rs:8:13: 8:15
+           let mut _13: u32;                // in scope 6 at $DIR/simplify_try.rs:8:13: 8:15
+           let _14: i32;                    // in scope 6 at $DIR/simplify_try.rs:8:13: 8:15
+           let mut _15: std::result::Result<!, i32>; // in scope 6 at $DIR/simplify_try.rs:8:13: 8:15
+           let mut _16: i32;                // in scope 6 at $DIR/simplify_try.rs:8:13: 8:15
+           scope 7 {
+               debug c => _12;              // in scope 7 at $DIR/simplify_try.rs:8:13: 8:15
+           scope 8 {
+           scope 8 {
+               debug e => _14;              // in scope 8 at $DIR/simplify_try.rs:8:13: 8:15
44       }
45   
46       bb0: {


48           StorageLive(_3);                 // scope 0 at $DIR/simplify_try.rs:8:13: 8:15
49           StorageLive(_4);                 // scope 0 at $DIR/simplify_try.rs:8:13: 8:14
50           _4 = _1;                         // scope 0 at $DIR/simplify_try.rs:8:13: 8:14
-           _3 = move _4;                    // scope 6 at $DIR/simplify_try.rs:8:13: 8:15
-           StorageDead(_4);                 // scope 0 at $DIR/simplify_try.rs:8:14: 8:15
-           _5 = discriminant(_3);           // scope 0 at $DIR/simplify_try.rs:8:14: 8:15
-           switchInt(move _5) -> [0_isize: bb1, otherwise: bb2]; // scope 0 at $DIR/simplify_try.rs:8:14: 8:15
+           StorageLive(_11);                // scope 0 at $DIR/simplify_try.rs:8:13: 8:15
+           _11 = discriminant(_4);          // scope 6 at $DIR/simplify_try.rs:8:13: 8:15
+           switchInt(move _11) -> [0_isize: bb6, 1_isize: bb4, otherwise: bb5]; // scope 6 at $DIR/simplify_try.rs:8:13: 8:15
56   
57       bb1: {


- -         StorageLive(_10);                // scope 0 at $DIR/simplify_try.rs:8:13: 8:15
- -         _10 = ((_3 as Ok).0: u32);       // scope 0 at $DIR/simplify_try.rs:8:13: 8:15
- -         _2 = _10;                        // scope 5 at $DIR/simplify_try.rs:8:13: 8:15
- -         StorageDead(_10);                // scope 0 at $DIR/simplify_try.rs:8:14: 8:15
- +         _0 = move _3;                    // scope 1 at $DIR/simplify_try.rs:9:5: 9:10
+           StorageDead(_11);                // scope 0 at $DIR/simplify_try.rs:8:13: 8:15
+           StorageDead(_4);                 // scope 0 at $DIR/simplify_try.rs:8:14: 8:15
+           _5 = discriminant(_3);           // scope 0 at $DIR/simplify_try.rs:8:13: 8:15
+           switchInt(move _5) -> [0_isize: bb2, otherwise: bb3]; // scope 0 at $DIR/simplify_try.rs:8:13: 8:15
+   
+       bb2: {
+       bb2: {
+           StorageLive(_9);                 // scope 0 at $DIR/simplify_try.rs:8:13: 8:15
+           _9 = ((_3 as Continue).0: u32);  // scope 0 at $DIR/simplify_try.rs:8:13: 8:15
+           _2 = _9;                         // scope 5 at $DIR/simplify_try.rs:8:13: 8:15
+           StorageDead(_9);                 // scope 0 at $DIR/simplify_try.rs:8:14: 8:15
63           StorageDead(_3);                 // scope 0 at $DIR/simplify_try.rs:8:15: 8:16
- -         StorageLive(_11);                // scope 1 at $DIR/simplify_try.rs:9:8: 9:9
- -         _11 = _2;                        // scope 1 at $DIR/simplify_try.rs:9:8: 9:9
- -         ((_0 as Ok).0: u32) = move _11;  // scope 1 at $DIR/simplify_try.rs:9:5: 9:10
- -         discriminant(_0) = 0;            // scope 1 at $DIR/simplify_try.rs:9:5: 9:10
- -         StorageDead(_11);                // scope 1 at $DIR/simplify_try.rs:9:9: 9:10
+           StorageLive(_10);                // scope 1 at $DIR/simplify_try.rs:9:8: 9:9
+           _10 = _2;                        // scope 1 at $DIR/simplify_try.rs:9:8: 9:9
+           ((_0 as Ok).0: u32) = move _10;  // scope 1 at $DIR/simplify_try.rs:9:5: 9:10
+           discriminant(_0) = 0;            // scope 1 at $DIR/simplify_try.rs:9:5: 9:10
+           StorageDead(_10);                // scope 1 at $DIR/simplify_try.rs:9:9: 9:10
69           StorageDead(_2);                 // scope 0 at $DIR/simplify_try.rs:10:1: 10:2
70           return;                          // scope 0 at $DIR/simplify_try.rs:10:2: 10:2

72   
-       bb2: {
