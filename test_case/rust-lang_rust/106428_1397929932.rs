plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 221 tests
.......................................................i................................ 88/221
................................................i.....F....F............................ 176/221
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:


---- [mir-opt] tests/mir-opt/inline/unwrap_unchecked.rs stdout ----
4     debug slf => _1;                     // in scope 0 at $DIR/unwrap_unchecked.rs:+0:35: +0:38
5     let mut _0: T;                       // return place in scope 0 at $DIR/unwrap_unchecked.rs:+0:54: +0:55
6     let mut _2: std::option::Option<T>;  // in scope 0 at $DIR/unwrap_unchecked.rs:+1:5: +1:8
-     scope 1 (inlined #[track_caller] Option::<T>::unwrap_unchecked) { // at $DIR/unwrap_unchecked.rs:7:9: 7:27
-         debug self => _2;                // in scope 1 at $SRC_DIR/core/src/option.rs:LL:COL
-         let mut _3: &std::option::Option<T>; // in scope 1 at $SRC_DIR/core/src/option.rs:LL:COL
-         let mut _4: isize;               // in scope 1 at $SRC_DIR/core/src/option.rs:LL:COL
-         scope 2 {
-             debug val => _0;             // in scope 2 at $SRC_DIR/core/src/option.rs:LL:COL
-         scope 3 {
-         scope 3 {
-             scope 5 (inlined unreachable_unchecked) { // at $SRC_DIR/core/src/option.rs:LL:COL
-                 scope 6 {
-                     scope 7 (inlined unreachable_unchecked::runtime) { // at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                 }
-             }
-         }
-         }
-         scope 4 (inlined Option::<T>::is_some) { // at $SRC_DIR/core/src/option.rs:LL:COL
-             debug self => _3;            // in scope 4 at $SRC_DIR/core/src/option.rs:LL:COL
-     }
26 
27     bb0: {
27     bb0: {
28         StorageLive(_2);                 // scope 0 at $DIR/unwrap_unchecked.rs:+1:5: +1:8

29         _2 = move _1;                    // scope 0 at $DIR/unwrap_unchecked.rs:+1:5: +1:8
-         StorageLive(_3);                 // scope 0 at $DIR/unwrap_unchecked.rs:+1:9: +1:27
-         _4 = discriminant(_2);           // scope 1 at $SRC_DIR/core/src/option.rs:LL:COL
-         switchInt(move _4) -> [0: bb1, 1: bb3, otherwise: bb2]; // scope 1 at $SRC_DIR/core/src/option.rs:LL:COL
+         _0 = Option::<T>::unwrap_unchecked(move _2) -> bb1; // scope 0 at $DIR/unwrap_unchecked.rs:+1:5: +1:27
+                                          // mir::Constant
+                                          // + span: $DIR/unwrap_unchecked.rs:7:9: 7:25
+                                          // + literal: Const { ty: unsafe fn(Option<T>) -> T {Option::<T>::unwrap_unchecked}, val: Value(<ZST>) }
34 
35     bb1: {


-         unreachable;                     // scope 6 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
- 
-     bb2: {
-     bb2: {
-         unreachable;                     // scope 1 at $SRC_DIR/core/src/option.rs:LL:COL
- 
-     bb3: {
-     bb3: {
-         _0 = move ((_2 as Some).0: T);   // scope 1 at $SRC_DIR/core/src/option.rs:LL:COL
-         StorageDead(_3);                 // scope 0 at $DIR/unwrap_unchecked.rs:+1:9: +1:27
46         StorageDead(_2);                 // scope 0 at $DIR/unwrap_unchecked.rs:+1:26: +1:27
47         return;                          // scope 0 at $DIR/unwrap_unchecked.rs:+2:2: +2:2


thread '[mir-opt] tests/mir-opt/inline/unwrap_unchecked.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/unwrap_unchecked.unwrap_unchecked.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3463:21


---- [mir-opt] tests/mir-opt/inline/unchecked_shifts.rs stdout ----
31                 }
32             }
32             }
-             scope 7 (inlined #[track_caller] Option::<u16>::unwrap_unchecked) { // at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-                 debug self => _7;        // in scope 7 at $SRC_DIR/core/src/option.rs:LL:COL
-                 let mut _14: &std::option::Option<u16>; // in scope 7 at $SRC_DIR/core/src/option.rs:LL:COL
-                 let mut _15: isize;      // in scope 7 at $SRC_DIR/core/src/option.rs:LL:COL
-                 scope 8 {
-                     debug val => _6;     // in scope 8 at $SRC_DIR/core/src/option.rs:LL:COL
-                 scope 9 {
-                 scope 9 {
-                     scope 11 (inlined unreachable_unchecked) { // at $SRC_DIR/core/src/option.rs:LL:COL
-                         scope 12 {
-                             scope 13 (inlined unreachable_unchecked::runtime) { // at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-                         }
-                     }
-                 }
-                 }
-                 scope 10 (inlined Option::<u16>::is_some) { // at $SRC_DIR/core/src/option.rs:LL:COL
-                     debug self => _14;   // in scope 10 at $SRC_DIR/core/src/option.rs:LL:COL
-             }
52         }
53     }
54 
54 

73     bb1: {
74         StorageDead(_9);                 // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
75         _10 = discriminant(_8);          // scope 3 at $SRC_DIR/core/src/result.rs:LL:COL
-         switchInt(move _10) -> [0: bb6, 1: bb4, otherwise: bb5]; // scope 3 at $SRC_DIR/core/src/result.rs:LL:COL
+         switchInt(move _10) -> [0: bb7, 1: bb5, otherwise: bb6]; // scope 3 at $SRC_DIR/core/src/result.rs:LL:COL
78 
79     bb2: {


80         StorageDead(_8);                 // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-         StorageLive(_14);                // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-         _15 = discriminant(_7);          // scope 7 at $SRC_DIR/core/src/option.rs:LL:COL
-         switchInt(move _15) -> [0: bb7, 1: bb9, otherwise: bb8]; // scope 7 at $SRC_DIR/core/src/option.rs:LL:COL
+         _6 = Option::<u16>::unwrap_unchecked(move _7) -> bb3; // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+                                          // mir::Constant
+                                          // + span: $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+                                          // + literal: Const { ty: unsafe fn(Option<u16>) -> u16 {Option::<u16>::unwrap_unchecked}, val: Value(<ZST>) }
85 
86     bb3: {


+         StorageDead(_7);                 // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+         _0 = unchecked_shl::<u16>(move _5, move _6) -> bb4; // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+                                          // mir::Constant
+                                          // + span: $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
+                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(u16, u16) -> u16 {unchecked_shl::<u16>}, val: Value(<ZST>) }
+ 
+     bb4: {
+     bb4: {
87         StorageDead(_6);                 // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
88         StorageDead(_5);                 // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
89         StorageDead(_4);                 // scope 0 at $DIR/unchecked_shifts.rs:+1:22: +1:23

91         return;                          // scope 0 at $DIR/unchecked_shifts.rs:+2:2: +2:2
93 
-     bb4: {
+     bb5: {
+     bb5: {
95         StorageLive(_13);                // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
96         Deinit(_7);                      // scope 6 at $SRC_DIR/core/src/result.rs:LL:COL
97         discriminant(_7) = 0;            // scope 6 at $SRC_DIR/core/src/result.rs:LL:COL

99         goto -> bb2;                     // scope 5 at $SRC_DIR/core/src/result.rs:LL:COL
101 
-     bb5: {
+     bb6: {
+     bb6: {
103         unreachable;                     // scope 3 at $SRC_DIR/core/src/result.rs:LL:COL
105 

-     bb6: {
+     bb7: {
+     bb7: {
107         StorageLive(_11);                // scope 3 at $SRC_DIR/core/src/result.rs:LL:COL
108         _11 = move ((_8 as Ok).0: u16);  // scope 3 at $SRC_DIR/core/src/result.rs:LL:COL
109         StorageLive(_12);                // scope 4 at $SRC_DIR/core/src/result.rs:LL:COL

114         StorageDead(_12);                // scope 4 at $SRC_DIR/core/src/result.rs:LL:COL
115         StorageDead(_11);                // scope 3 at $SRC_DIR/core/src/result.rs:LL:COL
116         goto -> bb2;                     // scope 3 at $SRC_DIR/core/src/result.rs:LL:COL
- 
-     bb7: {
-     bb7: {
-         unreachable;                     // scope 12 at $SRC_DIR/core/src/intrinsics.rs:LL:COL
- 
-     bb8: {
-     bb8: {
-         unreachable;                     // scope 7 at $SRC_DIR/core/src/option.rs:LL:COL
- 
-     bb9: {
-     bb9: {
-         _6 = move ((_7 as Some).0: u16); // scope 7 at $SRC_DIR/core/src/option.rs:LL:COL
-         StorageDead(_14);                // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-         StorageDead(_7);                 // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-         _0 = unchecked_shl::<u16>(move _5, move _6) -> bb3; // scope 2 at $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-                                          // mir::Constant
-                                          // + span: $SRC_DIR/core/src/num/uint_macros.rs:LL:COL
-                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(u16, u16) -> u16 {unchecked_shl::<u16>}, val: Value(<ZST>) }
136 }
137 


thread '[mir-opt] tests/mir-opt/inline/unchecked_shifts.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/unchecked_shifts.unchecked_shl_unsigned_smaller.PreCodegen.after.mir', src/tools/compiletest/src/runtest.rs:3463:21

failures:
    [mir-opt] tests/mir-opt/inline/unchecked_shifts.rs
    [mir-opt] tests/mir-opt/inline/unwrap_unchecked.rs
