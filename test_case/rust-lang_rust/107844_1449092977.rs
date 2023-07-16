plain
failures:

---- [mir-opt] tests/mir-opt/basic_assignment.rs stdout ----
58   
59       bb4: {
60           StorageDead(_5);                 // scope 3 at $DIR/basic_assignment.rs:+14:1: +14:2
- -         drop(_4) -> bb5;                 // scope 2 at $DIR/basic_assignment.rs:+14:1: +14:2
+ -         drop(_4) -> [return: bb5, unwind: bb8]; // scope 2 at $DIR/basic_assignment.rs:+14:1: +14:2
62 +         goto -> bb5;                     // scope 2 at $DIR/basic_assignment.rs:+14:1: +14:2
64   


thread '[mir-opt] tests/mir-opt/basic_assignment.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/basic_assignment.main.ElaborateDrops.diff', src/tools/compiletest/src/runtest.rs:3481:21


failures:
    [mir-opt] tests/mir-opt/basic_assignment.rs
