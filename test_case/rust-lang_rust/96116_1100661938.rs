plain
failures:

---- [mir-opt] src/test/mir-opt/early_otherwise_branch_68867.rs stdout ----
95   
96       bb2: {
97           StorageLive(_33);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:26:25: 26:27
+           nop;                             // scope 0 at $DIR/early_otherwise_branch_68867.rs:26:25: 26:27
+           Deinit(_0);                      // scope 0 at $DIR/early_otherwise_branch_68867.rs:26:21: 26:28
98           nop;                             // scope 0 at $DIR/early_otherwise_branch_68867.rs:26:21: 26:28
99           discriminant(_0) = 1;            // scope 0 at $DIR/early_otherwise_branch_68867.rs:26:21: 26:28
100           StorageDead(_33);                // scope 0 at $DIR/early_otherwise_branch_68867.rs:26:27: 26:28
240       }
241   
242       bb10: {
242       bb10: {
+           Deinit(_0);                      // scope 0 at $DIR/early_otherwise_branch_68867.rs:21:5: 27:7
243           ((_0 as Ok).0: ViewportPercentageLength) = move _3; // scope 0 at $DIR/early_otherwise_branch_68867.rs:21:5: 27:7
244           discriminant(_0) = 0;            // scope 0 at $DIR/early_otherwise_branch_68867.rs:21:5: 27:7
245           StorageDead(_3);                 // scope 0 at $DIR/early_otherwise_branch_68867.rs:27:6: 27:7

thread '[mir-opt] src/test/mir-opt/early_otherwise_branch_68867.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/early_otherwise_branch_68867.try_sum.EarlyOtherwiseBranch.diff', src/tools/compiletest/src/runtest.rs:3406:25

---- [mir-opt] src/test/mir-opt/inline/inline-closure-captures.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/inline-closure-captures.rs stdout ----
54         _13 = move ((*_6).1: &T);        // scope 2 at $DIR/inline-closure-captures.rs:11:22: 11:23
55         _11 = (*_13);                    // scope 2 at $DIR/inline-closure-captures.rs:11:22: 11:23
56         StorageDead(_13);                // scope 2 at $DIR/inline-closure-captures.rs:11:18: 11:24
+         Deinit(_0);                      // scope 2 at $DIR/inline-closure-captures.rs:11:18: 11:24
57         (_0.0: i32) = move _10;          // scope 2 at $DIR/inline-closure-captures.rs:11:18: 11:24
58         (_0.1: T) = move _11;            // scope 2 at $DIR/inline-closure-captures.rs:11:18: 11:24
59         StorageDead(_11);                // scope 2 at $DIR/inline-closure-captures.rs:11:23: 11:24

thread '[mir-opt] src/test/mir-opt/inline/inline-closure-captures.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_closure_captures.foo.Inline.after.mir', src/tools/compiletest/src/runtest.rs:3406:25

failures:
    [mir-opt] src/test/mir-opt/early_otherwise_branch_68867.rs
    [mir-opt] src/test/mir-opt/inline/inline-closure-captures.rs
