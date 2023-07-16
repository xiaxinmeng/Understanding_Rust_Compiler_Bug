plain
test [mir-opt] src/test/mir-opt/inline/polymorphic-recursion.rs ... ok

failures:

---- [mir-opt] src/test/mir-opt/lower_intrinsics.rs stdout ----
105           StorageDead(_15);                // scope 0 at $DIR/lower_intrinsics.rs:+4:48: +4:49
106           StorageDead(_13);                // scope 0 at $DIR/lower_intrinsics.rs:+4:48: +4:49
107           _0 = const ();                   // scope 0 at $DIR/lower_intrinsics.rs:+0:30: +5:2
-           drop(_1) -> [return: bb5, unwind: bb6]; // scope 0 at $DIR/lower_intrinsics.rs:+5:1: +5:2
+           drop(_1) -> bb5;                 // scope 0 at $DIR/lower_intrinsics.rs:+5:1: +5:2
110   
111       bb5: {


112           return;                          // scope 0 at $DIR/lower_intrinsics.rs:+5:2: +5:2
-   
-   
-       bb6 (cleanup): {
-           resume;                          // scope 0 at $DIR/lower_intrinsics.rs:+0:1: +5:2
118   }
119   


thread '[mir-opt] src/test/mir-opt/lower_intrinsics.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/lower_intrinsics.discriminant.LowerIntrinsics.diff', src/tools/compiletest/src/runtest.rs:3516:25


failures:
    [mir-opt] src/test/mir-opt/lower_intrinsics.rs
