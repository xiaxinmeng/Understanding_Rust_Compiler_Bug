plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 164 tests
...................................i.............................................i.................. 100/164
.F............................i................F...........F....
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] mir-opt/issue-62289.rs stdout ----
---- [mir-opt] mir-opt/issue-62289.rs stdout ----
1 // MIR for `test` before ElaborateDrops
2 
3 fn test() -> Option<Box<u32>> {
-     let mut _0: std::option::Option<std::boxed::Box<u32>>; // return place in scope 0 at $DIR/issue-62289.rs:8:14: 8:30
-     let mut _1: std::boxed::Box<u32>;    // in scope 0 at $DIR/issue-62289.rs:9:10: 9:21
-     let mut _2: std::boxed::Box<u32>;    // in scope 0 at $DIR/issue-62289.rs:9:10: 9:21
-     let mut _3: std::ops::ControlFlow<std::option::Option<std::convert::Infallible>, u32>; // in scope 0 at $DIR/issue-62289.rs:9:15: 9:20
-     let mut _4: std::option::Option<u32>; // in scope 0 at $DIR/issue-62289.rs:9:15: 9:19
-     let mut _5: isize;                   // in scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-     let _6: std::option::Option<std::convert::Infallible>; // in scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-     let mut _7: !;                       // in scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-     let mut _8: std::option::Option<std::convert::Infallible>; // in scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-     let _9: u32;                         // in scope 0 at $DIR/issue-62289.rs:9:15: 9:20
+     let mut _0: std::option::Option<std::boxed::Box<u32>>; // return place in scope 0 at $DIR/issue-62289.rs:6:14: 6:30
+     let mut _1: std::boxed::Box<u32>;    // in scope 0 at $DIR/issue-62289.rs:7:10: 7:25
+     let mut _2: u32;                     // in scope 0 at $DIR/issue-62289.rs:7:19: 7:24
+     let mut _3: std::ops::ControlFlow<std::option::Option<std::convert::Infallible>, u32>; // in scope 0 at $DIR/issue-62289.rs:7:19: 7:24
+     let mut _4: std::option::Option<u32>; // in scope 0 at $DIR/issue-62289.rs:7:19: 7:23
+     let mut _5: isize;                   // in scope 0 at $DIR/issue-62289.rs:7:23: 7:24
+     let _6: std::option::Option<std::convert::Infallible>; // in scope 0 at $DIR/issue-62289.rs:7:23: 7:24
+     let mut _7: !;                       // in scope 0 at $DIR/issue-62289.rs:7:23: 7:24
+     let mut _8: std::option::Option<std::convert::Infallible>; // in scope 0 at $DIR/issue-62289.rs:7:23: 7:24
+     let _9: u32;                         // in scope 0 at $DIR/issue-62289.rs:7:19: 7:24
14     scope 1 {
-         debug residual => _6;            // in scope 1 at $DIR/issue-62289.rs:9:19: 9:20
+         debug residual => _6;            // in scope 1 at $DIR/issue-62289.rs:7:23: 7:24
16         scope 2 {
18     }

19     scope 3 {
19     scope 3 {
-         debug val => _9;                 // in scope 3 at $DIR/issue-62289.rs:9:15: 9:20
+         debug val => _9;                 // in scope 3 at $DIR/issue-62289.rs:7:19: 7:24
21         scope 4 {
23     }

24 
25     bb0: {
25     bb0: {
-         StorageLive(_1);                 // scope 0 at $DIR/issue-62289.rs:9:10: 9:21
-         StorageLive(_2);                 // scope 0 at $DIR/issue-62289.rs:9:10: 9:21
-         _2 = Box(u32);                   // scope 0 at $DIR/issue-62289.rs:9:10: 9:21
-         StorageLive(_3);                 // scope 0 at $DIR/issue-62289.rs:9:15: 9:20
-         StorageLive(_4);                 // scope 0 at $DIR/issue-62289.rs:9:15: 9:19
-         _4 = Option::<u32>::None;        // scope 0 at $DIR/issue-62289.rs:9:15: 9:19
-         _3 = <Option<u32> as Try>::branch(move _4) -> [return: bb1, unwind: bb11]; // scope 0 at $DIR/issue-62289.rs:9:15: 9:20
+         StorageLive(_1);                 // scope 0 at $DIR/issue-62289.rs:7:10: 7:25
+         StorageLive(_2);                 // scope 0 at $DIR/issue-62289.rs:7:19: 7:24
+         StorageLive(_3);                 // scope 0 at $DIR/issue-62289.rs:7:19: 7:24
+         StorageLive(_4);                 // scope 0 at $DIR/issue-62289.rs:7:19: 7:23
+         _4 = Option::<u32>::None;        // scope 0 at $DIR/issue-62289.rs:7:19: 7:23
+         _3 = <Option<u32> as Try>::branch(move _4) -> bb1; // scope 0 at $DIR/issue-62289.rs:7:19: 7:24
33                                          // mir::Constant
-                                          // + span: $DIR/issue-62289.rs:9:15: 9:20
+                                          // + span: $DIR/issue-62289.rs:7:19: 7:24
35                                          // + literal: Const { ty: fn(std::option::Option<u32>) -> std::ops::ControlFlow<<std::option::Option<u32> as std::ops::Try>::Residual, <std::option::Option<u32> as std::ops::Try>::Output> {<std::option::Option<u32> as std::ops::Try>::branch}, val: Value(Scalar(<ZST>)) }
37 

38     bb1: {
38     bb1: {
-         StorageDead(_4);                 // scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-         _5 = discriminant(_3);           // scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-         switchInt(move _5) -> [0_isize: bb2, 1_isize: bb4, otherwise: bb3]; // scope 0 at $DIR/issue-62289.rs:9:19: 9:20
+         StorageDead(_4);                 // scope 0 at $DIR/issue-62289.rs:7:23: 7:24
+         _5 = discriminant(_3);           // scope 0 at $DIR/issue-62289.rs:7:23: 7:24
+         switchInt(move _5) -> [0_isize: bb2, 1_isize: bb4, otherwise: bb3]; // scope 0 at $DIR/issue-62289.rs:7:23: 7:24
43 
44     bb2: {


-         StorageLive(_9);                 // scope 0 at $DIR/issue-62289.rs:9:15: 9:20
-         _9 = ((_3 as Continue).0: u32);  // scope 0 at $DIR/issue-62289.rs:9:15: 9:20
-         (*_2) = _9;                      // scope 4 at $DIR/issue-62289.rs:9:15: 9:20
-         StorageDead(_9);                 // scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-         _1 = move _2;                    // scope 0 at $DIR/issue-62289.rs:9:10: 9:21
-         drop(_2) -> [return: bb6, unwind: bb10]; // scope 0 at $DIR/issue-62289.rs:9:20: 9:21
+         StorageLive(_9);                 // scope 0 at $DIR/issue-62289.rs:7:19: 7:24
+         _9 = ((_3 as Continue).0: u32);  // scope 0 at $DIR/issue-62289.rs:7:19: 7:24
+         _2 = _9;                         // scope 4 at $DIR/issue-62289.rs:7:19: 7:24
+         StorageDead(_9);                 // scope 0 at $DIR/issue-62289.rs:7:23: 7:24
+         _1 = Box::<u32>::new(move _2) -> bb6; // scope 0 at $DIR/issue-62289.rs:7:10: 7:25
+                                          // mir::Constant
+                                          // + span: $DIR/issue-62289.rs:7:10: 7:18
+                                          // + user_ty: UserType(0)
+                                          // + literal: Const { ty: fn(u32) -> std::boxed::Box<u32> {std::boxed::Box::<u32>::new}, val: Value(Scalar(<ZST>)) }
52 
53     bb3: {


-         unreachable;                     // scope 0 at $DIR/issue-62289.rs:9:15: 9:20
+         unreachable;                     // scope 0 at $DIR/issue-62289.rs:7:19: 7:24
56 
57     bb4: {


-         StorageLive(_6);                 // scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-         _6 = ((_3 as Break).0: std::option::Option<std::convert::Infallible>); // scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-         StorageLive(_8);                 // scope 2 at $DIR/issue-62289.rs:9:19: 9:20
-         _8 = _6;                         // scope 2 at $DIR/issue-62289.rs:9:19: 9:20
-         _0 = <Option<Box<u32>> as FromResidual<Option<Infallible>>>::from_residual(move _8) -> [return: bb5, unwind: bb11]; // scope 2 at $DIR/issue-62289.rs:9:15: 9:20
+         StorageLive(_6);                 // scope 0 at $DIR/issue-62289.rs:7:23: 7:24
+         _6 = ((_3 as Break).0: std::option::Option<std::convert::Infallible>); // scope 0 at $DIR/issue-62289.rs:7:23: 7:24
+         StorageLive(_8);                 // scope 2 at $DIR/issue-62289.rs:7:23: 7:24
+         _8 = _6;                         // scope 2 at $DIR/issue-62289.rs:7:23: 7:24
+         _0 = <Option<Box<u32>> as FromResidual<Option<Infallible>>>::from_residual(move _8) -> bb5; // scope 2 at $DIR/issue-62289.rs:7:19: 7:24
63                                          // mir::Constant
-                                          // + span: $DIR/issue-62289.rs:9:19: 9:20
+                                          // + span: $DIR/issue-62289.rs:7:23: 7:24
65                                          // + literal: Const { ty: fn(std::option::Option<std::convert::Infallible>) -> std::option::Option<std::boxed::Box<u32>> {<std::option::Option<std::boxed::Box<u32>> as std::ops::FromResidual<std::option::Option<std::convert::Infallible>>>::from_residual}, val: Value(Scalar(<ZST>)) }
67 

68     bb5: {
68     bb5: {
-         StorageDead(_8);                 // scope 2 at $DIR/issue-62289.rs:9:19: 9:20
-         StorageDead(_6);                 // scope 0 at $DIR/issue-62289.rs:9:19: 9:20
-         drop(_2) -> bb8;                 // scope 0 at $DIR/issue-62289.rs:9:20: 9:21
+         StorageDead(_8);                 // scope 2 at $DIR/issue-62289.rs:7:23: 7:24
+         StorageDead(_6);                 // scope 0 at $DIR/issue-62289.rs:7:23: 7:24
+         StorageDead(_2);                 // scope 0 at $DIR/issue-62289.rs:7:24: 7:25
+         StorageDead(_1);                 // scope 0 at $DIR/issue-62289.rs:7:25: 7:26
+         StorageDead(_3);                 // scope 0 at $DIR/issue-62289.rs:8:1: 8:2
+         goto -> bb8;                     // scope 0 at $DIR/issue-62289.rs:8:2: 8:2
73 
74     bb6: {


-         StorageDead(_2);                 // scope 0 at $DIR/issue-62289.rs:9:20: 9:21
-         _0 = Option::<Box<u32>>::Some(move _1); // scope 0 at $DIR/issue-62289.rs:9:5: 9:22
-         drop(_1) -> bb7;                 // scope 0 at $DIR/issue-62289.rs:9:21: 9:22
+         StorageDead(_2);                 // scope 0 at $DIR/issue-62289.rs:7:24: 7:25
+         _0 = Option::<Box<u32>>::Some(move _1); // scope 0 at $DIR/issue-62289.rs:7:5: 7:26
+         drop(_1) -> bb7;                 // scope 0 at $DIR/issue-62289.rs:7:25: 7:26
79 
80     bb7: {


-         StorageDead(_1);                 // scope 0 at $DIR/issue-62289.rs:9:21: 9:22
-         StorageDead(_3);                 // scope 0 at $DIR/issue-62289.rs:10:1: 10:2
-         goto -> bb9;                     // scope 0 at $DIR/issue-62289.rs:10:2: 10:2
+         StorageDead(_1);                 // scope 0 at $DIR/issue-62289.rs:7:25: 7:26
+         StorageDead(_3);                 // scope 0 at $DIR/issue-62289.rs:8:1: 8:2
+         goto -> bb8;                     // scope 0 at $DIR/issue-62289.rs:8:2: 8:2
85 
86     bb8: {


-         StorageDead(_2);                 // scope 0 at $DIR/issue-62289.rs:9:20: 9:21
-         StorageDead(_1);                 // scope 0 at $DIR/issue-62289.rs:9:21: 9:22
-         StorageDead(_3);                 // scope 0 at $DIR/issue-62289.rs:10:1: 10:2
-         goto -> bb9;                     // scope 0 at $DIR/issue-62289.rs:10:2: 10:2
- 
-     bb9: {
-     bb9: {
-         return;                          // scope 0 at $DIR/issue-62289.rs:10:2: 10:2
- 
- 
-     bb10 (cleanup): {
-         drop(_1) -> bb12;                // scope 0 at $DIR/issue-62289.rs:9:21: 9:22
- 
- 
-     bb11 (cleanup): {
-         drop(_2) -> bb12;                // scope 0 at $DIR/issue-62289.rs:9:20: 9:21
- 
- 
-     bb12 (cleanup): {
-         resume;                          // scope 0 at $DIR/issue-62289.rs:8:1: 10:2
+         return;                          // scope 0 at $DIR/issue-62289.rs:8:2: 8:2
108 }
109 


thread '[mir-opt] mir-opt/issue-62289.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/issue_62289.test.ElaborateDrops.before.mir', src/tools/compiletest/src/runtest.rs:3588:25

---- [mir-opt] mir-opt/simplify-locals.rs stdout ----
---- [mir-opt] mir-opt/simplify-locals.rs stdout ----
2 + // MIR for `c` after SimplifyLocals
4   fn c() -> () {
4   fn c() -> () {
-       let mut _0: ();                      // return place in scope 0 at $DIR/simplify-locals.rs:13:8: 13:8
-       let _1: [u8; 10];                    // in scope 0 at $DIR/simplify-locals.rs:14:9: 14:14
- -     let mut _2: &[u8];                   // in scope 0 at $DIR/simplify-locals.rs:16:20: 16:26
- -     let mut _3: &[u8; 10];               // in scope 0 at $DIR/simplify-locals.rs:16:20: 16:26
- -     let _4: &[u8; 10];                   // in scope 0 at $DIR/simplify-locals.rs:16:20: 16:26
+       let mut _0: ();                      // return place in scope 0 at $DIR/simplify-locals.rs:12:8: 12:8
+       let _1: [u8; 10];                    // in scope 0 at $DIR/simplify-locals.rs:13:9: 13:14
+ -     let mut _2: &[u8];                   // in scope 0 at $DIR/simplify-locals.rs:15:20: 15:26
+ -     let mut _3: &[u8; 10];               // in scope 0 at $DIR/simplify-locals.rs:15:20: 15:26
+ -     let _4: &[u8; 10];                   // in scope 0 at $DIR/simplify-locals.rs:15:20: 15:26
10       scope 1 {
-           debug bytes => _1;               // in scope 1 at $DIR/simplify-locals.rs:14:9: 14:14
+           debug bytes => _1;               // in scope 1 at $DIR/simplify-locals.rs:13:9: 13:14
12           scope 2 {
14       }

15   
16       bb0: {
16       bb0: {
-           StorageLive(_1);                 // scope 0 at $DIR/simplify-locals.rs:14:9: 14:14
-           _1 = [const 0_u8; 10];           // scope 0 at $DIR/simplify-locals.rs:14:17: 14:26
- -         StorageLive(_2);                 // scope 1 at $DIR/simplify-locals.rs:16:20: 16:26
- -         StorageLive(_3);                 // scope 1 at $DIR/simplify-locals.rs:16:20: 16:26
- -         StorageLive(_4);                 // scope 1 at $DIR/simplify-locals.rs:16:20: 16:26
- -         _4 = &_1;                        // scope 1 at $DIR/simplify-locals.rs:16:20: 16:26
- -         _3 = _4;                         // scope 1 at $DIR/simplify-locals.rs:16:20: 16:26
- -         _2 = move _3 as &[u8] (Pointer(Unsize)); // scope 1 at $DIR/simplify-locals.rs:16:20: 16:26
- -         StorageDead(_3);                 // scope 1 at $DIR/simplify-locals.rs:16:25: 16:26
- -         StorageDead(_4);                 // scope 1 at $DIR/simplify-locals.rs:16:26: 16:27
- -         StorageDead(_2);                 // scope 1 at $DIR/simplify-locals.rs:16:26: 16:27
-           StorageDead(_1);                 // scope 0 at $DIR/simplify-locals.rs:17:1: 17:2
-           return;                          // scope 0 at $DIR/simplify-locals.rs:17:2: 17:2
+           StorageLive(_1);                 // scope 0 at $DIR/simplify-locals.rs:13:9: 13:14
+           _1 = [const 0_u8; 10];           // scope 0 at $DIR/simplify-locals.rs:13:17: 13:26
+ -         StorageLive(_2);                 // scope 1 at $DIR/simplify-locals.rs:15:20: 15:26
+ -         StorageLive(_3);                 // scope 1 at $DIR/simplify-locals.rs:15:20: 15:26
+ -         StorageLive(_4);                 // scope 1 at $DIR/simplify-locals.rs:15:20: 15:26
+ -         _4 = &_1;                        // scope 1 at $DIR/simplify-locals.rs:15:20: 15:26
+ -         _3 = _4;                         // scope 1 at $DIR/simplify-locals.rs:15:20: 15:26
+ -         _2 = move _3 as &[u8] (Pointer(Unsize)); // scope 1 at $DIR/simplify-locals.rs:15:20: 15:26
+ -         StorageDead(_3);                 // scope 1 at $DIR/simplify-locals.rs:15:25: 15:26
+ -         StorageDead(_4);                 // scope 1 at $DIR/simplify-locals.rs:15:26: 15:27
+ -         StorageDead(_2);                 // scope 1 at $DIR/simplify-locals.rs:15:26: 15:27
+           StorageDead(_1);                 // scope 0 at $DIR/simplify-locals.rs:16:1: 16:2
+           return;                          // scope 0 at $DIR/simplify-locals.rs:16:2: 16:2
31   }
32   


thread '[mir-opt] mir-opt/simplify-locals.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/simplify_locals.c.SimplifyLocals.diff', src/tools/compiletest/src/runtest.rs:3588:25
---- [mir-opt] mir-opt/uniform_array_move_out.rs stdout ----
---- [mir-opt] mir-opt/uniform_array_move_out.rs stdout ----
1 // MIR for `move_out_from_end` 0 mir_map
+ | User Type Annotations
+ | User Type Annotations
+ | 0: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }], value: TypeOf(DefId(5:204 ~ alloc[60ea]::boxed::{impl#0}::new), UserSubsts { substs: [^0], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(5:202 ~ alloc[60ea]::boxed::{impl#0}), self_ty: std::boxed::Box<^1, ^2> }) }) } at $DIR/uniform_array_move_out.rs:3:14: 3:22
+ | 1: Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }, CanonicalVarInfo { kind: Ty(General(U0)) }], value: TypeOf(DefId(5:204 ~ alloc[60ea]::boxed::{impl#0}::new), UserSubsts { substs: [^0], user_self_ty: Some(UserSelfTy { impl_def_id: DefId(5:202 ~ alloc[60ea]::boxed::{impl#0}), self_ty: std::boxed::Box<^1, ^2> }) }) } at $DIR/uniform_array_move_out.rs:3:27: 3:35
3 fn move_out_from_end() -> () {
3 fn move_out_from_end() -> () {
-     let mut _0: ();                      // return place in scope 0 at $DIR/uniform_array_move_out.rs:4:24: 4:24
-     let _1: [std::boxed::Box<i32>; 2];   // in scope 0 at $DIR/uniform_array_move_out.rs:5:9: 5:10
-     let mut _2: std::boxed::Box<i32>;    // in scope 0 at $DIR/uniform_array_move_out.rs:5:14: 5:19
-     let mut _3: std::boxed::Box<i32>;    // in scope 0 at $DIR/uniform_array_move_out.rs:5:14: 5:19
-     let mut _4: std::boxed::Box<i32>;    // in scope 0 at $DIR/uniform_array_move_out.rs:5:21: 5:26
-     let mut _5: std::boxed::Box<i32>;    // in scope 0 at $DIR/uniform_array_move_out.rs:5:21: 5:26
+     let mut _0: ();                      // return place in scope 0 at $DIR/uniform_array_move_out.rs:2:24: 2:24
+     let _1: [std::boxed::Box<i32>; 2];   // in scope 0 at $DIR/uniform_array_move_out.rs:3:9: 3:10
+     let mut _2: std::boxed::Box<i32>;    // in scope 0 at $DIR/uniform_array_move_out.rs:3:14: 3:25
+     let mut _3: std::boxed::Box<i32>;    // in scope 0 at $DIR/uniform_array_move_out.rs:3:27: 3:38
10     scope 1 {
-         debug a => _1;                   // in scope 1 at $DIR/uniform_array_move_out.rs:5:9: 5:10
-         let _6: std::boxed::Box<i32>;    // in scope 1 at $DIR/uniform_array_move_out.rs:6:14: 6:16
+         debug a => _1;                   // in scope 1 at $DIR/uniform_array_move_out.rs:3:9: 3:10
+         let _4: std::boxed::Box<i32>;    // in scope 1 at $DIR/uniform_array_move_out.rs:4:14: 4:16
13         scope 2 {
-             debug _y => _6;              // in scope 2 at $DIR/uniform_array_move_out.rs:6:14: 6:16
+             debug _y => _4;              // in scope 2 at $DIR/uniform_array_move_out.rs:4:14: 4:16
16     }
17 

18     bb0: {
18     bb0: {
-         StorageLive(_1);                 // scope 0 at $DIR/uniform_array_move_out.rs:5:9: 5:10
-         StorageLive(_2);                 // scope 0 at $DIR/uniform_array_move_out.rs:5:14: 5:19
-         StorageLive(_3);                 // scope 0 at $DIR/uniform_array_move_out.rs:5:14: 5:19
-         _3 = Box(i32);                   // scope 0 at $DIR/uniform_array_move_out.rs:5:14: 5:19
-         (*_3) = const 1_i32;             // scope 0 at $DIR/uniform_array_move_out.rs:5:18: 5:19
-         _2 = move _3;                    // scope 0 at $DIR/uniform_array_move_out.rs:5:14: 5:19
-         drop(_3) -> [return: bb1, unwind: bb9]; // scope 0 at $DIR/uniform_array_move_out.rs:5:18: 5:19
+         StorageLive(_1);                 // scope 0 at $DIR/uniform_array_move_out.rs:3:9: 3:10
+         StorageLive(_2);                 // scope 0 at $DIR/uniform_array_move_out.rs:3:14: 3:25
+         _2 = Box::<i32>::new(const 1_i32) -> [return: bb1, unwind: bb9]; // scope 0 at $DIR/uniform_array_move_out.rs:3:14: 3:25
+                                          // mir::Constant
+                                          // + span: $DIR/uniform_array_move_out.rs:3:14: 3:22
+                                          // + user_ty: UserType(0)
+                                          // + literal: Const { ty: fn(i32) -> std::boxed::Box<i32> {std::boxed::Box::<i32>::new}, val: Value(Scalar(<ZST>)) }
27 
28     bb1: {


-         StorageDead(_3);                 // scope 0 at $DIR/uniform_array_move_out.rs:5:18: 5:19
-         StorageLive(_4);                 // scope 0 at $DIR/uniform_array_move_out.rs:5:21: 5:26
-         StorageLive(_5);                 // scope 0 at $DIR/uniform_array_move_out.rs:5:21: 5:26
-         _5 = Box(i32);                   // scope 0 at $DIR/uniform_array_move_out.rs:5:21: 5:26
-         (*_5) = const 2_i32;             // scope 0 at $DIR/uniform_array_move_out.rs:5:25: 5:26
-         _4 = move _5;                    // scope 0 at $DIR/uniform_array_move_out.rs:5:21: 5:26
-         drop(_5) -> [return: bb2, unwind: bb8]; // scope 0 at $DIR/uniform_array_move_out.rs:5:25: 5:26
+         StorageLive(_3);                 // scope 0 at $DIR/uniform_array_move_out.rs:3:27: 3:38
+         _3 = Box::<i32>::new(const 2_i32) -> [return: bb2, unwind: bb8]; // scope 0 at $DIR/uniform_array_move_out.rs:3:27: 3:38
+                                          // mir::Constant
+                                          // + span: $DIR/uniform_array_move_out.rs:3:27: 3:35
+                                          // + user_ty: UserType(1)
+                                          // + literal: Const { ty: fn(i32) -> std::boxed::Box<i32> {std::boxed::Box::<i32>::new}, val: Value(Scalar(<ZST>)) }
37 
38     bb2: {


-         StorageDead(_5);                 // scope 0 at $DIR/uniform_array_move_out.rs:5:25: 5:26
-         _1 = [move _2, move _4];         // scope 0 at $DIR/uniform_array_move_out.rs:5:13: 5:27
-         drop(_4) -> [return: bb3, unwind: bb9]; // scope 0 at $DIR/uniform_array_move_out.rs:5:26: 5:27
+         _1 = [move _2, move _3];         // scope 0 at $DIR/uniform_array_move_out.rs:3:13: 3:39
+         drop(_3) -> [return: bb3, unwind: bb8]; // scope 0 at $DIR/uniform_array_move_out.rs:3:38: 3:39
43 
44     bb3: {


-         StorageDead(_4);                 // scope 0 at $DIR/uniform_array_move_out.rs:5:26: 5:27
-         drop(_2) -> [return: bb4, unwind: bb10]; // scope 0 at $DIR/uniform_array_move_out.rs:5:26: 5:27
+         StorageDead(_3);                 // scope 0 at $DIR/uniform_array_move_out.rs:3:38: 3:39
+         drop(_2) -> [return: bb4, unwind: bb9]; // scope 0 at $DIR/uniform_array_move_out.rs:3:38: 3:39
48 
49     bb4: {


-         StorageDead(_2);                 // scope 0 at $DIR/uniform_array_move_out.rs:5:26: 5:27
-         FakeRead(ForLet(None), _1);      // scope 0 at $DIR/uniform_array_move_out.rs:5:9: 5:10
-         StorageLive(_6);                 // scope 1 at $DIR/uniform_array_move_out.rs:6:14: 6:16
-         _6 = move _1[1 of 2];            // scope 1 at $DIR/uniform_array_move_out.rs:6:14: 6:16
-         _0 = const ();                   // scope 0 at $DIR/uniform_array_move_out.rs:4:24: 7:2
-         drop(_6) -> [return: bb5, unwind: bb7]; // scope 1 at $DIR/uniform_array_move_out.rs:7:1: 7:2
+         StorageDead(_2);                 // scope 0 at $DIR/uniform_array_move_out.rs:3:38: 3:39
+         FakeRead(ForLet(None), _1);      // scope 0 at $DIR/uniform_array_move_out.rs:3:9: 3:10
+         StorageLive(_4);                 // scope 1 at $DIR/uniform_array_move_out.rs:4:14: 4:16
+         _4 = move _1[1 of 2];            // scope 1 at $DIR/uniform_array_move_out.rs:4:14: 4:16
+         _0 = const ();                   // scope 0 at $DIR/uniform_array_move_out.rs:2:24: 5:2
+         drop(_4) -> [return: bb5, unwind: bb7]; // scope 1 at $DIR/uniform_array_move_out.rs:5:1: 5:2
57 
58     bb5: {


-         StorageDead(_6);                 // scope 1 at $DIR/uniform_array_move_out.rs:7:1: 7:2
-         drop(_1) -> [return: bb6, unwind: bb10]; // scope 0 at $DIR/uniform_array_move_out.rs:7:1: 7:2
+         StorageDead(_4);                 // scope 1 at $DIR/uniform_array_move_out.rs:5:1: 5:2
+         drop(_1) -> [return: bb6, unwind: bb9]; // scope 0 at $DIR/uniform_array_move_out.rs:5:1: 5:2
62 
63     bb6: {


-         StorageDead(_1);                 // scope 0 at $DIR/uniform_array_move_out.rs:7:1: 7:2
-         return;                          // scope 0 at $DIR/uniform_array_move_out.rs:7:2: 7:2
+         StorageDead(_1);                 // scope 0 at $DIR/uniform_array_move_out.rs:5:1: 5:2
+         return;                          // scope 0 at $DIR/uniform_array_move_out.rs:5:2: 5:2
67 
67 
68     bb7 (cleanup): {

-         drop(_1) -> bb10;                // scope 0 at $DIR/uniform_array_move_out.rs:7:1: 7:2
+         drop(_1) -> bb9;                 // scope 0 at $DIR/uniform_array_move_out.rs:5:1: 5:2
71 
71 
72     bb8 (cleanup): {

-         drop(_4) -> bb9;                 // scope 0 at $DIR/uniform_array_move_out.rs:5:26: 5:27
+         drop(_2) -> bb9;                 // scope 0 at $DIR/uniform_array_move_out.rs:3:38: 3:39
75 
75 
76     bb9 (cleanup): {

-         drop(_2) -> bb10;                // scope 0 at $DIR/uniform_array_move_out.rs:5:26: 5:27
- 
- 
-     bb10 (cleanup): {
-         resume;                          // scope 0 at $DIR/uniform_array_move_out.rs:4:1: 7:2
+         resume;                          // scope 0 at $DIR/uniform_array_move_out.rs:2:1: 5:2
83 }
84 


thread '[mir-opt] mir-opt/uniform_array_move_out.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/uniform_array_move_out.move_out_from_end.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3588:25

failures:
    [mir-opt] mir-opt/issue-62289.rs
    [mir-opt] mir-opt/simplify-locals.rs
    [mir-opt] mir-opt/simplify-locals.rs
    [mir-opt] mir-opt/uniform_array_move_out.rs

test result: FAILED. 158 passed; 3 failed; 3 ignored; 0 measured; 0 filtered out; finished in 2.29s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:15
