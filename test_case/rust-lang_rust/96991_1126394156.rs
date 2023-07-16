plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..i..................................................i.............................F..
failures:

---- [mir-opt] src/test/mir-opt/const_prop/discriminant.rs stdout ----
18           Deinit(_3);                      // scope 0 at $DIR/discriminant.rs:11:34: 11:44
19           ((_3 as Some).0: bool) = const true; // scope 0 at $DIR/discriminant.rs:11:34: 11:44
20           discriminant(_3) = 1;            // scope 0 at $DIR/discriminant.rs:11:34: 11:44
- -         _4 = discriminant(_3);           // scope 0 at $DIR/discriminant.rs:11:21: 11:31
+           _4 = const 1_isize;              // scope 0 at $DIR/discriminant.rs:11:21: 11:31
22 -         switchInt(move _4) -> [1_isize: bb1, otherwise: bb3]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
- +         _4 = const 1_isize;              // scope 0 at $DIR/discriminant.rs:11:21: 11:31
24 +         switchInt(const 1_isize) -> [1_isize: bb1, otherwise: bb3]; // scope 0 at $DIR/discriminant.rs:11:21: 11:31
26   


thread '[mir-opt] src/test/mir-opt/const_prop/discriminant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/discriminant.main.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3410:25

---- [mir-opt] src/test/mir-opt/while_let_loops.rs stdout ----
---- [mir-opt] src/test/mir-opt/while_let_loops.rs stdout ----
21           StorageLive(_3);                 // scope 1 at $DIR/while_let_loops.rs:7:28: 7:32
22           Deinit(_3);                      // scope 1 at $DIR/while_let_loops.rs:7:28: 7:32
23           discriminant(_3) = 0;            // scope 1 at $DIR/while_let_loops.rs:7:28: 7:32
- -         _4 = discriminant(_3);           // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
+           _4 = const 0_isize;              // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
25 -         switchInt(move _4) -> [1_isize: bb1, otherwise: bb3]; // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
- +         _4 = const 0_isize;              // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
27 +         switchInt(const 0_isize) -> [1_isize: bb1, otherwise: bb3]; // scope 1 at $DIR/while_let_loops.rs:7:15: 7:25
29   


thread '[mir-opt] src/test/mir-opt/while_let_loops.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/while_let_loops.change_loop_body.ConstProp.64bit.diff', src/tools/compiletest/src/runtest.rs:3410:25

failures:
    [mir-opt] src/test/mir-opt/const_prop/discriminant.rs
    [mir-opt] src/test/mir-opt/while_let_loops.rs
