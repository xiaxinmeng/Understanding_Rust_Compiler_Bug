plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..................................i.........F......................
failures:

---- [mir-opt] mir-opt/separate_const_switch.rs stdout ----
17           scope 2 {
18               scope 8 (inlined #[track_caller] <Result<i32, i32> as FromResidual<Result<Infallible, i32>>>::from_residual) { // at $DIR/separate_const_switch.rs:29:8: 29:10
19                   debug residual => _8;    // in scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
-                   let _16: i32;            // in scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
-                   let mut _17: i32;        // in scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
+                   let mut _16: isize;      // in scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
+                   let _17: i32;            // in scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
22                   let mut _18: i32;        // in scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
+                   let mut _19: i32;        // in scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
+                   let _20: std::convert::Infallible; // in scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
23                   scope 9 {
-                       debug e => _16;      // in scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
-                       scope 10 (inlined <i32 as From<i32>>::from) { // at $DIR/separate_const_switch.rs:29:8: 29:10
-                           debug t => _18;  // in scope 10 at $DIR/separate_const_switch.rs:29:8: 29:10
+                       debug e => _17;      // in scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
+                       scope 11 (inlined <i32 as From<i32>>::from) { // at $DIR/separate_const_switch.rs:29:8: 29:10
+                           debug t => _19;  // in scope 11 at $DIR/separate_const_switch.rs:29:8: 29:10
28                   }
+                   scope 10 {
+                   scope 10 {
+                       debug never => _20;  // in scope 10 at $DIR/separate_const_switch.rs:29:8: 29:10
29               }
30           }
31       }


86           _6 = ((_3 as Break).0: std::result::Result<std::convert::Infallible, i32>); // scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
87           StorageLive(_8);                 // scope 2 at $DIR/separate_const_switch.rs:29:9: 29:10
88           _8 = _6;                         // scope 2 at $DIR/separate_const_switch.rs:29:9: 29:10
-           StorageLive(_16);                // scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
-           _16 = move ((_8 as Err).0: i32); // scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
-           StorageLive(_17);                // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
-           StorageLive(_18);                // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
-           _18 = move _16;                  // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
-           _17 = move _18;                  // scope 10 at $DIR/separate_const_switch.rs:29:8: 29:10
-           StorageDead(_18);                // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
-           ((_0 as Err).0: i32) = move _17; // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
-           discriminant(_0) = 1;            // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
-           StorageDead(_17);                // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
-           StorageDead(_16);                // scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
-           StorageDead(_8);                 // scope 2 at $DIR/separate_const_switch.rs:29:9: 29:10
-           StorageDead(_6);                 // scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
-           StorageDead(_2);                 // scope 0 at $DIR/separate_const_switch.rs:29:10: 29:11
-           StorageDead(_3);                 // scope 0 at $DIR/separate_const_switch.rs:30:1: 30:2
-           return;                          // scope 0 at $DIR/separate_const_switch.rs:30:2: 30:2
+           StorageLive(_16);                // scope 2 at $DIR/separate_const_switch.rs:29:8: 29:10
+           _16 = discriminant(_8);          // scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
+ -         switchInt(move _16) -> [0_isize: bb7, 1_isize: bb9, otherwise: bb8]; // scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
+ +         switchInt(move _16) -> [0_isize: bb6, 1_isize: bb8, otherwise: bb7]; // scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
106   
107 -     bb4: {


145 +         StorageDead(_4);                 // scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
146 +         _5 = discriminant(_3);           // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
147 +         switchInt(move _5) -> [0_isize: bb1, otherwise: bb2]; // scope 0 at $DIR/separate_const_switch.rs:29:8: 29:10
+   
+ -     bb7: {
+ +     bb6: {
+ +     bb6: {
+           StorageLive(_20);                // scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
+           unreachable;                     // scope 10 at $DIR/separate_const_switch.rs:29:8: 29:10
+   
+ -     bb8: {
+ +     bb7: {
+ +     bb7: {
+           unreachable;                     // scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
+   
+ -     bb9: {
+ +     bb8: {
+ +     bb8: {
+           StorageLive(_17);                // scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
+           _17 = move ((_8 as Err).0: i32); // scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
+           StorageLive(_18);                // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
+           StorageLive(_19);                // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
+           _19 = move _17;                  // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
+           _18 = move _19;                  // scope 11 at $DIR/separate_const_switch.rs:29:8: 29:10
+           StorageDead(_19);                // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
+           ((_0 as Err).0: i32) = move _18; // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
+           discriminant(_0) = 1;            // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
+           StorageDead(_18);                // scope 9 at $DIR/separate_const_switch.rs:29:8: 29:10
+           StorageDead(_17);                // scope 8 at $DIR/separate_const_switch.rs:29:8: 29:10
+           StorageDead(_16);                // scope 2 at $DIR/separate_const_switch.rs:29:8: 29:10
+           StorageDead(_8);                 // scope 2 at $DIR/separate_const_switch.rs:29:9: 29:10
+           StorageDead(_6);                 // scope 0 at $DIR/separate_const_switch.rs:29:9: 29:10
+           StorageDead(_2);                 // scope 0 at $DIR/separate_const_switch.rs:29:10: 29:11
+           StorageDead(_3);                 // scope 0 at $DIR/separate_const_switch.rs:30:1: 30:2
+           return;                          // scope 0 at $DIR/separate_const_switch.rs:30:2: 30:2
149   }
150   


thread '[mir-opt] mir-opt/separate_const_switch.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/separate_const_switch.identity.SeparateConstSwitch.diff', src/tools/compiletest/src/runtest.rs:3386:25


failures:
    [mir-opt] mir-opt/separate_const_switch.rs
