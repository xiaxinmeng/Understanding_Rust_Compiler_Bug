plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
..
failures:

---- [mir-opt] src/test/mir-opt/const_prop/discriminant.rs stdout ----
10       scope 1 {
11           debug x => _1;                   // in scope 1 at $DIR/discriminant.rs:11:9: 11:10
+       scope 2 {
+       }
13   
14       bb0: {
14       bb0: {
15           StorageLive(_1);                 // scope 0 at $DIR/discriminant.rs:11:9: 11:10

16           StorageLive(_2);                 // scope 0 at $DIR/discriminant.rs:11:13: 11:64
-           StorageLive(_3);                 // scope 0 at $DIR/discriminant.rs:11:34: 11:44
-           Deinit(_3);                      // scope 0 at $DIR/discriminant.rs:11:34: 11:44
-           ((_3 as Some).0: bool) = const true; // scope 0 at $DIR/discriminant.rs:11:34: 11:44
-           discriminant(_3) = 1;            // scope 0 at $DIR/discriminant.rs:11:34: 11:44
- -         _4 = discriminant(_3);           // scope 0 at $DIR/discriminant.rs:11:21: 11:31
- -         switchInt(move _4) -> [1_isize: bb1, otherwise: bb3]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
- +         _4 = const 1_isize;              // scope 0 at $DIR/discriminant.rs:11:21: 11:31
- +         switchInt(const 1_isize) -> [1_isize: bb1, otherwise: bb3]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
+           StorageLive(_3);                 // scope 2 at $DIR/discriminant.rs:11:34: 11:44
+           Deinit(_3);                      // scope 2 at $DIR/discriminant.rs:11:34: 11:44
+           ((_3 as Some).0: bool) = const true; // scope 2 at $DIR/discriminant.rs:11:34: 11:44
+           discriminant(_3) = 1;            // scope 2 at $DIR/discriminant.rs:11:34: 11:44
+ -         _4 = discriminant(_3);           // scope 2 at $DIR/discriminant.rs:11:21: 11:31
+ -         switchInt(move _4) -> [1_isize: bb1, otherwise: bb3]; // scope 2 at $DIR/discriminant.rs:11:21: 11:31
+ +         _4 = const 1_isize;              // scope 2 at $DIR/discriminant.rs:11:21: 11:31
+ +         switchInt(const 1_isize) -> [1_isize: bb1, otherwise: bb3]; // scope 2 at $DIR/discriminant.rs:11:21: 11:31
26   
27       bb1: {


-           switchInt(((_3 as Some).0: bool)) -> [false: bb3, otherwise: bb2]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
+           switchInt(((_3 as Some).0: bool)) -> [false: bb3, otherwise: bb2]; // scope 2 at $DIR/discriminant.rs:11:21: 11:31
30   
31       bb2: {


-           _2 = const 42_i32;               // scope 0 at $DIR/discriminant.rs:11:47: 11:49
+           _2 = const 42_i32;               // scope 2 at $DIR/discriminant.rs:11:47: 11:49
33           goto -> bb4;                     // scope 0 at $DIR/discriminant.rs:11:13: 11:64
35   


thread '[mir-opt] src/test/mir-opt/const_prop/discriminant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/discriminant.main.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3437:25

---- [mir-opt] src/test/mir-opt/while_let_loops.rs stdout ----
---- [mir-opt] src/test/mir-opt/while_let_loops.rs stdout ----
13       let mut _8: !;                       // in scope 0 at $DIR/while_let_loops.rs:7:5: 10:6
14       scope 1 {
15           debug _x => _1;                  // in scope 1 at $DIR/while_let_loops.rs:6:9: 6:15
+           scope 2 {
16       }
17   
18       bb0: {


19           StorageLive(_1);                 // scope 0 at $DIR/while_let_loops.rs:6:9: 6:15
20           _1 = const 0_i32;                // scope 0 at $DIR/while_let_loops.rs:6:18: 6:19
-           StorageLive(_3);                 // scope 1 at $DIR/while_let_loops.rs:7:28: 7:32
-           Deinit(_3);                      // scope 1 at $DIR/while_let_loops.rs:7:28: 7:32
-           discriminant(_3) = 0;            // scope 1 at $DIR/while_let_loops.rs:7:28: 7:32
- -         _4 = discriminant(_3);           // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
- -         switchInt(move _4) -> [1_isize: bb1, otherwise: bb3]; // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
- +         _4 = const 0_isize;              // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
- +         switchInt(const 0_isize) -> [1_isize: bb1, otherwise: bb3]; // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
+           StorageLive(_3);                 // scope 2 at $DIR/while_let_loops.rs:7:28: 7:32
+           Deinit(_3);                      // scope 2 at $DIR/while_let_loops.rs:7:28: 7:32
+           discriminant(_3) = 0;            // scope 2 at $DIR/while_let_loops.rs:7:28: 7:32
+ -         _4 = discriminant(_3);           // scope 2 at $DIR/while_let_loops.rs:7:15: 7:25
+ -         switchInt(move _4) -> [1_isize: bb1, otherwise: bb3]; // scope 2 at $DIR/while_let_loops.rs:7:15: 7:25
+ +         _4 = const 0_isize;              // scope 2 at $DIR/while_let_loops.rs:7:15: 7:25
+ +         switchInt(const 0_isize) -> [1_isize: bb1, otherwise: bb3]; // scope 2 at $DIR/while_let_loops.rs:7:15: 7:25
29   
30       bb1: {


-           switchInt(((_3 as Some).0: u32)) -> [0_u32: bb2, otherwise: bb3]; // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
+           switchInt(((_3 as Some).0: u32)) -> [0_u32: bb2, otherwise: bb3]; // scope 2 at $DIR/while_let_loops.rs:7:15: 7:25
33   
34       bb2: {


-           _1 = const 1_i32;                // scope 1 at $DIR/while_let_loops.rs:8:9: 8:15
-           nop;                             // scope 1 at $DIR/while_let_loops.rs:9:9: 9:14
-           goto -> bb4;                     // scope 1 at $DIR/while_let_loops.rs:9:9: 9:14
+           _1 = const 1_i32;                // scope 2 at $DIR/while_let_loops.rs:8:9: 8:15
+           nop;                             // scope 2 at $DIR/while_let_loops.rs:9:9: 9:14
+           goto -> bb4;                     // scope 2 at $DIR/while_let_loops.rs:9:9: 9:14
39   
40       bb3: {


thread '[mir-opt] src/test/mir-opt/while_let_loops.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/while_let_loops.change_loop_body.ConstProp.32bit.diff', src/tools/compiletest/src/runtest.rs:3437:25

failures:
    [mir-opt] src/test/mir-opt/const_prop/discriminant.rs
    [mir-opt] src/test/mir-opt/while_let_loops.rs
