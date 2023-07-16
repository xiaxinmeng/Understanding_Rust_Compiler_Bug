plain
.........................ii........i..................................

failures:

---- [mir-opt] tests/mir-opt/const_prop/optimizes_into_variable.rs stdout ----
4     let mut _0: ();                      // return place in scope 0 at $DIR/optimizes_into_variable.rs:+0:11: +0:11
5     scope 1 {
6         debug x => const 4_i32;          // in scope 1 at $DIR/optimizes_into_variable.rs:+1:9: +1:10
+         let _1: i32;                     // in scope 1 at $DIR/optimizes_into_variable.rs:+2:9: +2:10
7         scope 2 {
-             debug y => const 3_i32;      // in scope 2 at $DIR/optimizes_into_variable.rs:+2:9: +2:10
+             debug y => _1;               // in scope 2 at $DIR/optimizes_into_variable.rs:+2:9: +2:10
9             scope 3 {
10                 debug z => const 42_u32; // in scope 3 at $DIR/optimizes_into_variable.rs:+3:9: +3:10

13     }
14 
15     bb0: {
15     bb0: {
+         StorageLive(_1);                 // scope 1 at $DIR/optimizes_into_variable.rs:+2:9: +2:10
+         StorageDead(_1);                 // scope 1 at $DIR/optimizes_into_variable.rs:+4:1: +4:2
16         return;                          // scope 0 at $DIR/optimizes_into_variable.rs:+4:2: +4:2
18 }


thread '[mir-opt] tests/mir-opt/const_prop/optimizes_into_variable.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/optimizes_into_variable.main.SimplifyLocals-final.after.64bit.mir', src/tools/compiletest/src/runtest.rs:3553:21


failures:
    [mir-opt] tests/mir-opt/const_prop/optimizes_into_variable.rs
