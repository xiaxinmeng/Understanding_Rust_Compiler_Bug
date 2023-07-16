plain
test [mir-opt] tests/mir-opt/inline/polymorphic_recursion.rs ... ok

failures:

---- [mir-opt] tests/mir-opt/combine_transmutes.rs stdout ----
34 +         _2 = move _3;                    // scope 1 at $DIR/combine_transmutes.rs:+3:14: +3:57
35           StorageDead(_3);                 // scope 1 at $DIR/combine_transmutes.rs:+3:56: +3:57
36           nop;                             // scope 0 at $DIR/combine_transmutes.rs:+0:37: +4:2
-           drop(_2) -> [return: bb1, unwind: bb2]; // scope 1 at $DIR/combine_transmutes.rs:+4:1: +4:2
+           drop(_2) -> bb1;                 // scope 1 at $DIR/combine_transmutes.rs:+4:1: +4:2
39   
40       bb1: {


41           StorageDead(_2);                 // scope 1 at $DIR/combine_transmutes.rs:+4:1: +4:2
42           StorageDead(_1);                 // scope 0 at $DIR/combine_transmutes.rs:+4:1: +4:2
43           return;                          // scope 0 at $DIR/combine_transmutes.rs:+4:2: +4:2
-   
-   
-       bb2 (cleanup): {
-           resume;                          // scope 0 at $DIR/combine_transmutes.rs:+0:1: +4:2
49   }
50   


thread '[mir-opt] tests/mir-opt/combine_transmutes.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/combine_transmutes.identity_transmutes.InstCombine.diff', src/tools/compiletest/src/runtest.rs:3515:21


failures:
    [mir-opt] tests/mir-opt/combine_transmutes.rs
