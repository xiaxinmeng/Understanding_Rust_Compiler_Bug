plain
 finished in 0.529 seconds
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 176 tests
i......................................i............................................F.F. 88/176
....i......F...........................................i................................ 176/176
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] src/test/mir-opt/inline/dyn-trait.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/dyn-trait.rs stdout ----
32 +         StorageDead(_4);                 // scope 1 at $DIR/dyn-trait.rs:21:21: 21:22
33           StorageDead(_2);                 // scope 0 at $DIR/dyn-trait.rs:27:15: 27:16
34           return;                          // scope 0 at $DIR/dyn-trait.rs:28:2: 28:2
+ +     }
+ + 
+ +     bb2 (cleanup): {
+ +         resume;                          // scope 0 at $DIR/dyn-trait.rs:26:1: 28:2
36   }
37   


thread '[mir-opt] src/test/mir-opt/inline/dyn-trait.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/dyn_trait.try_execute_query.Inline.diff', src/tools/compiletest/src/runtest.rs:3410:25

---- [mir-opt] src/test/mir-opt/inline/inline-cycle-generic.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/inline-cycle-generic.rs stdout ----
17           StorageDead(_1);                 // scope 0 at $DIR/inline-cycle-generic.rs:9:24: 9:25
18           _0 = const ();                   // scope 0 at $DIR/inline-cycle-generic.rs:8:11: 10:2
19           return;                          // scope 0 at $DIR/inline-cycle-generic.rs:10:2: 10:2
- +     }
- + 
- +     bb2 (cleanup): {
- +         resume;                          // scope 0 at $DIR/inline-cycle-generic.rs:8:1: 10:2
25   }
26   


thread '[mir-opt] src/test/mir-opt/inline/inline-cycle-generic.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_cycle_generic.main.Inline.diff', src/tools/compiletest/src/runtest.rs:3410:25
---- [mir-opt] src/test/mir-opt/inline/inline-compatibility.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/inline-compatibility.rs stdout ----
19           StorageDead(_1);                 // scope 0 at $DIR/inline-compatibility.rs:13:21: 13:22
20           _0 = const ();                   // scope 0 at $DIR/inline-compatibility.rs:12:40: 14:2
21           return;                          // scope 0 at $DIR/inline-compatibility.rs:14:2: 14:2
+ +     }
+ + 
+ +     bb1 (cleanup): {
+ +         resume;                          // scope 0 at $DIR/inline-compatibility.rs:12:1: 14:2
23   }
24   


thread '[mir-opt] src/test/mir-opt/inline/inline-compatibility.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_compatibility.inlined_target_feature.Inline.diff', src/tools/compiletest/src/runtest.rs:3410:25

failures:
    [mir-opt] src/test/mir-opt/inline/dyn-trait.rs
    [mir-opt] src/test/mir-opt/inline/inline-compatibility.rs
