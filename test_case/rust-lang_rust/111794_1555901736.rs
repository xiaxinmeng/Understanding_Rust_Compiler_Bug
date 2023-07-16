plain

running 255 tests
................................................................................i.......  88/255
.........................................................i...........iii................ 176/255
..................iii.iiiii.........ii........i....................F.F..F......
failures:

---- [mir-opt] tests/mir-opt/uninhabited_enum.rs stdout ----
---- [mir-opt] tests/mir-opt/uninhabited_enum.rs stdout ----
3 fn process_never(_1: *const !) -> () {
4     debug input => _1;                   // in scope 0 at $DIR/uninhabited_enum.rs:+0:22: +0:27
5     let mut _0: ();                      // return place in scope 0 at $DIR/uninhabited_enum.rs:+0:39: +0:39
-     let _2: &!;                          // in scope 0 at $DIR/uninhabited_enum.rs:+1:8: +1:14
7     scope 1 {
-         debug _input => _2;              // in scope 1 at $DIR/uninhabited_enum.rs:+1:8: +1:14
+         debug _input => _1;              // in scope 1 at $DIR/uninhabited_enum.rs:+1:8: +1:14
10     scope 2 {
11     }


thread '[mir-opt] tests/mir-opt/uninhabited_enum.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/uninhabited_enum.process_never.SimplifyLocals-final.after.mir', src/tools/compiletest/src/runtest.rs:3639:21

---- [mir-opt] tests/mir-opt/unreachable.rs stdout ----
---- [mir-opt] tests/mir-opt/unreachable.rs stdout ----
thread '[mir-opt] tests/mir-opt/unreachable.rs' panicked at 'the mir dump file for unreachable.main.UnreachablePropagation.before.mir does not exist (requested in /checkout/tests/mir-opt/unreachable.rs)', src/tools/compiletest/src/runtest.rs:3652:17
---- [mir-opt] tests/mir-opt/unreachable_diverging.rs stdout ----
---- [mir-opt] tests/mir-opt/unreachable_diverging.rs stdout ----
thread '[mir-opt] tests/mir-opt/unreachable_diverging.rs' panicked at 'the mir dump file for unreachable_diverging.main.UnreachablePropagation.before.mir does not exist (requested in /checkout/tests/mir-opt/unreachable_diverging.rs)', src/tools/compiletest/src/runtest.rs:3652:17
Build completed unsuccessfully in 0:13:06

failures:
    [mir-opt] tests/mir-opt/uninhabited_enum.rs
