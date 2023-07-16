plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
......F..
failures:

---- [mir-opt] src/test/mir-opt/exponential-or.rs stdout ----
8     let mut _4: bool;                    // in scope 0 at $DIR/exponential-or.rs:+2:70: +2:77
9     let mut _5: bool;                    // in scope 0 at $DIR/exponential-or.rs:+2:62: +2:67
10     let mut _6: bool;                    // in scope 0 at $DIR/exponential-or.rs:+2:62: +2:67
-     let _7: u32;                         // in scope 0 at $DIR/exponential-or.rs:+2:10: +2:21
-     let _8: u32;                         // in scope 0 at $DIR/exponential-or.rs:+2:57: +2:78
+     let _7: u32;                         // in scope 0 at $DIR/exponential-or.rs:+2:10: +2:11
+     let _8: u32;                         // in scope 0 at $DIR/exponential-or.rs:+2:57: +2:58
13     let mut _9: u32;                     // in scope 0 at $DIR/exponential-or.rs:+2:83: +2:84
14     let mut _10: u32;                    // in scope 0 at $DIR/exponential-or.rs:+2:87: +2:88
15     scope 1 {

-         debug y => _7;                   // in scope 1 at $DIR/exponential-or.rs:+2:10: +2:21
-         debug z => _8;                   // in scope 1 at $DIR/exponential-or.rs:+2:57: +2:78
+         debug y => _7;                   // in scope 1 at $DIR/exponential-or.rs:+2:10: +2:11
+         debug z => _8;                   // in scope 1 at $DIR/exponential-or.rs:+2:57: +2:58
19 
20     bb0: {

61     }
61     }
62 
63     bb9: {
-         StorageLive(_7);                 // scope 0 at $DIR/exponential-or.rs:+2:10: +2:21
-         _7 = (_1.0: u32);                // scope 0 at $DIR/exponential-or.rs:+2:10: +2:21
-         StorageLive(_8);                 // scope 0 at $DIR/exponential-or.rs:+2:57: +2:78
-         _8 = (_1.3: u32);                // scope 0 at $DIR/exponential-or.rs:+2:57: +2:78
+         StorageLive(_7);                 // scope 0 at $DIR/exponential-or.rs:+2:10: +2:11
+         _7 = (_1.0: u32);                // scope 0 at $DIR/exponential-or.rs:+2:10: +2:11
+         StorageLive(_8);                 // scope 0 at $DIR/exponential-or.rs:+2:57: +2:58
+         _8 = (_1.3: u32);                // scope 0 at $DIR/exponential-or.rs:+2:57: +2:58
68         StorageLive(_9);                 // scope 1 at $DIR/exponential-or.rs:+2:83: +2:84
69         _9 = _7;                         // scope 1 at $DIR/exponential-or.rs:+2:83: +2:84
70         StorageLive(_10);                // scope 1 at $DIR/exponential-or.rs:+2:87: +2:88

thread '[mir-opt] src/test/mir-opt/exponential-or.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/exponential_or.match_tuple.SimplifyCfg-initial.after.mir', src/tools/compiletest/src/runtest.rs:3516:25

---- [mir-opt] src/test/mir-opt/uniform_array_move_out.rs stdout ----
---- [mir-opt] src/test/mir-opt/uniform_array_move_out.rs stdout ----
15     let mut _11: std::boxed::Box<i32>;   // in scope 0 at $DIR/uniform_array_move_out.rs:+1:21: +1:26
16     scope 1 {
17         debug a => _1;                   // in scope 1 at $DIR/uniform_array_move_out.rs:+1:9: +1:10
-         let _12: [std::boxed::Box<i32>; 2]; // in scope 1 at $DIR/uniform_array_move_out.rs:+2:10: +2:17
+         let _12: [std::boxed::Box<i32>; 2]; // in scope 1 at $DIR/uniform_array_move_out.rs:+2:10: +2:12
19         scope 4 {
-             debug _y => _12;             // in scope 4 at $DIR/uniform_array_move_out.rs:+2:10: +2:17
+             debug _y => _12;             // in scope 4 at $DIR/uniform_array_move_out.rs:+2:10: +2:12
22     }
23     scope 2 {

77     bb6: {
77     bb6: {
78         StorageDead(_2);                 // scope 0 at $DIR/uniform_array_move_out.rs:+1:26: +1:27
79         FakeRead(ForLet(None), _1);      // scope 0 at $DIR/uniform_array_move_out.rs:+1:9: +1:10
-         StorageLive(_12);                // scope 1 at $DIR/uniform_array_move_out.rs:+2:10: +2:17
-         _12 = move _1[0..2];             // scope 1 at $DIR/uniform_array_move_out.rs:+2:10: +2:17
+         StorageLive(_12);                // scope 1 at $DIR/uniform_array_move_out.rs:+2:10: +2:12
+         _12 = move _1[0..2];             // scope 1 at $DIR/uniform_array_move_out.rs:+2:10: +2:12
82         _0 = const ();                   // scope 0 at $DIR/uniform_array_move_out.rs:+0:27: +3:2
83         drop(_12) -> [return: bb7, unwind: bb9]; // scope 1 at $DIR/uniform_array_move_out.rs:+3:1: +3:2


thread '[mir-opt] src/test/mir-opt/uniform_array_move_out.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/uniform_array_move_out.move_out_by_subslice.mir_map.0.mir', src/tools/compiletest/src/runtest.rs:3516:25

failures:
    [mir-opt] src/test/mir-opt/exponential-or.rs
    [mir-opt] src/test/mir-opt/uniform_array_move_out.rs
