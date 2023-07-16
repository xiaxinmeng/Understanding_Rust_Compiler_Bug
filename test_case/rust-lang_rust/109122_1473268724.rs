plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

running 235 tests
........................................................................i............... 88/235
.............................................................i.......................... 176/235
...............ii.......i..........F.F..............F......
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] tests/mir-opt/retag.rs stdout ----
1 // MIR for `std::ptr::drop_in_place` after SimplifyCfg-make_shim
2 
2 
3 fn std::ptr::drop_in_place(_1: *mut Test) -> () {
-     let mut _0: ();                      // return place in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-     let mut _2: &mut Test;               // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-     let mut _3: &mut Test;               // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-     let mut _4: ();                      // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+     let mut _0: ();                      // return place in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+     let mut _2: &mut Test;               // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+     let mut _3: &mut Test;               // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+     let mut _4: ();                      // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
9     bb0: {
9     bb0: {
-         _2 = &mut (*_1);                 // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         Retag([fn entry] _2);            // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         _3 = &mut (*_2);                 // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         _4 = <Test as Drop>::drop(move _3) -> bb1; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         _2 = &mut (*_1);                 // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         Retag([fn entry] _2);            // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         _3 = &mut (*_2);                 // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         _4 = <Test as Drop>::drop(move _3) -> bb1; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
14                                          // mir::Constant
15                                          // + span: $SRC_DIR/core/src/ptr/mod.rs:LL:COL
16                                          // + literal: Const { ty: for<'a> fn(&'a mut Test) {<Test as Drop>::drop}, val: Value(<ZST>) }
17     }
18 
19     bb1: {
19     bb1: {
-         return;                          // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         return;                          // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
22 }
23 


thread '[mir-opt] tests/mir-opt/retag.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/retag.core.ptr-drop_in_place.Test.SimplifyCfg-make_shim.after.mir', src/tools/compiletest/src/runtest.rs:3492:21

---- [mir-opt] tests/mir-opt/slice_drop_shim.rs stdout ----
1 // MIR for `std::ptr::drop_in_place` before AddMovesForPackedDrops
2 
2 
3 fn std::ptr::drop_in_place(_1: *mut [String]) -> () {
-     let mut _0: ();                      // return place in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-     let mut _2: usize;                   // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-     let mut _3: usize;                   // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-     let mut _4: usize;                   // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-     let mut _5: *mut std::string::String; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-     let mut _6: bool;                    // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-     let mut _7: *mut std::string::String; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-     let mut _8: bool;                    // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-     let mut _9: *mut std::string::String; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-     let mut _10: *mut std::string::String; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-     let mut _11: *mut std::string::String; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-     let mut _12: bool;                   // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-     let mut _13: *mut std::string::String; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-     let mut _14: bool;                   // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-     let mut _15: *mut [std::string::String]; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+     let mut _0: ();                      // return place in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+     let mut _2: usize;                   // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+     let mut _3: usize;                   // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+     let mut _4: usize;                   // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+     let mut _5: *mut std::string::String; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+     let mut _6: bool;                    // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+     let mut _7: *mut std::string::String; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+     let mut _8: bool;                    // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+     let mut _9: *mut std::string::String; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+     let mut _10: *mut std::string::String; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+     let mut _11: *mut std::string::String; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+     let mut _12: bool;                   // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+     let mut _13: *mut std::string::String; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+     let mut _14: bool;                   // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+     let mut _15: *mut [std::string::String]; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
20     bb0: {
20     bb0: {
-         goto -> bb15;                    // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         goto -> bb15;                    // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
23 
24     bb1: {


-         return;                          // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         return;                          // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
27 
27 
28     bb2 (cleanup): {

-         resume;                          // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         resume;                          // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
31 
31 
32     bb3 (cleanup): {

-         _5 = &raw mut (*_1)[_4];         // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         _4 = Add(move _4, const 1_usize); // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         drop((*_5)) -> bb4;              // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         _5 = &raw mut (*_1)[_4];         // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         _4 = Add(move _4, const 1_usize); // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         drop((*_5)) -> bb4;              // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
37 
37 
38     bb4 (cleanup): {

-         _6 = Eq(_4, _3);                 // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         switchInt(move _6) -> [0: bb3, otherwise: bb2]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         _6 = Eq(_4, _3);                 // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         switchInt(move _6) -> [0: bb3, otherwise: bb2]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
42 
43     bb5: {


-         _7 = &raw mut (*_1)[_4];         // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         _4 = Add(move _4, const 1_usize); // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         drop((*_7)) -> [return: bb6, unwind: bb4]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         _7 = &raw mut (*_1)[_4];         // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         _4 = Add(move _4, const 1_usize); // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         drop((*_7)) -> [return: bb6, unwind: bb4]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
48 
49     bb6: {


-         _8 = Eq(_4, _3);                 // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         switchInt(move _8) -> [0: bb5, otherwise: bb1]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         _8 = Eq(_4, _3);                 // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         switchInt(move _8) -> [0: bb5, otherwise: bb1]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
53 
54     bb7: {


-         _4 = const 0_usize;              // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         goto -> bb6;                     // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         _4 = const 0_usize;              // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         goto -> bb6;                     // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
58 
59     bb8: {


-         goto -> bb7;                     // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         goto -> bb7;                     // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
62 
62 
63     bb9 (cleanup): {

-         _11 = _9;                        // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         _9 = Offset(move _9, const 1_usize); // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         drop((*_11)) -> bb10;            // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         _11 = _9;                        // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         _9 = Offset(move _9, const 1_usize); // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         drop((*_11)) -> bb10;            // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
68 
68 
69     bb10 (cleanup): {

-         _12 = Eq(_9, _10);               // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         switchInt(move _12) -> [0: bb9, otherwise: bb2]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         _12 = Eq(_9, _10);               // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         switchInt(move _12) -> [0: bb9, otherwise: bb2]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
73 
74     bb11: {


-         _13 = _9;                        // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         _9 = Offset(move _9, const 1_usize); // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         drop((*_13)) -> [return: bb12, unwind: bb10]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         _13 = _9;                        // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         _9 = Offset(move _9, const 1_usize); // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         drop((*_13)) -> [return: bb12, unwind: bb10]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
79 
80     bb12: {


-         _14 = Eq(_9, _10);               // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         switchInt(move _14) -> [0: bb11, otherwise: bb1]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         _14 = Eq(_9, _10);               // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         switchInt(move _14) -> [0: bb11, otherwise: bb1]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
84 
85     bb13: {


-         _15 = &raw mut (*_1);            // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         _9 = move _15 as *mut std::string::String (PtrToPtr); // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         _10 = Offset(_9, move _3);       // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         goto -> bb12;                    // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         _15 = &raw mut (*_1);            // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         _9 = move _15 as *mut std::string::String (PtrToPtr); // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         _10 = Offset(_9, move _3);       // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         goto -> bb12;                    // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
91 
92     bb14: {


-         goto -> bb13;                    // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         goto -> bb13;                    // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
95 
96     bb15: {


-         _2 = SizeOf(std::string::String); // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         _3 = Len((*_1));                 // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         switchInt(move _2) -> [0: bb8, otherwise: bb14]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         _2 = SizeOf(std::string::String); // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         _3 = Len((*_1));                 // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         switchInt(move _2) -> [0: bb8, otherwise: bb14]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
101 }
102 


thread '[mir-opt] tests/mir-opt/slice_drop_shim.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/slice_drop_shim.core.ptr-drop_in_place.[String].AddMovesForPackedDrops.before.mir', src/tools/compiletest/src/runtest.rs:3492:21
---- [mir-opt] tests/mir-opt/unusual_item_types.rs stdout ----
1 // MIR for `std::ptr::drop_in_place` before AddMovesForPackedDrops
2 
2 
3 fn std::ptr::drop_in_place(_1: *mut Vec<i32>) -> () {
-     let mut _0: ();                      // return place in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-     let mut _2: &mut std::vec::Vec<i32>; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-     let mut _3: ();                      // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+     let mut _0: ();                      // return place in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+     let mut _2: &mut std::vec::Vec<i32>; // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+     let mut _3: ();                      // in scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
8     bb0: {
8     bb0: {
-         goto -> bb6;                     // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         goto -> bb6;                     // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
11 
12     bb1: {


-         return;                          // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         return;                          // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
15 
15 
16     bb2 (cleanup): {

-         resume;                          // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         resume;                          // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
19 
20     bb3: {


-         goto -> bb1;                     // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         goto -> bb1;                     // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
23 
23 
24     bb4 (cleanup): {

-         drop(((*_1).0: alloc::raw_vec::RawVec<i32>)) -> bb2; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         drop(((*_1).0: alloc::raw_vec::RawVec<i32>)) -> bb2; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
27 
28     bb5: {


-         drop(((*_1).0: alloc::raw_vec::RawVec<i32>)) -> [return: bb3, unwind: bb2]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         drop(((*_1).0: alloc::raw_vec::RawVec<i32>)) -> [return: bb3, unwind: bb2]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
31 
32     bb6: {


-         _2 = &mut (*_1);                 // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
-         _3 = <Vec<i32> as Drop>::drop(move _2) -> [return: bb5, unwind: bb4]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:56
+         _2 = &mut (*_1);                 // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
+         _3 = <Vec<i32> as Drop>::drop(move _2) -> [return: bb5, unwind: bb4]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:+0:1: +0:80
35                                          // mir::Constant
36                                          // + span: $SRC_DIR/core/src/ptr/mod.rs:LL:COL
37                                          // + literal: Const { ty: for<'a> fn(&'a mut Vec<i32>) {<Vec<i32> as Drop>::drop}, val: Value(<ZST>) }

thread '[mir-opt] tests/mir-opt/unusual_item_types.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/unusual_item_types.core.ptr-drop_in_place.Vec_i32_.AddMovesForPackedDrops.before.mir', src/tools/compiletest/src/runtest.rs:3492:21

failures:
    [mir-opt] tests/mir-opt/retag.rs
    [mir-opt] tests/mir-opt/slice_drop_shim.rs
