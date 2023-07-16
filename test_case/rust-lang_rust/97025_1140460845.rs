plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 178 tests
i......................................i................................................ 88/178
......i...F..............................................i.............................. 176/178
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

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


thread '[mir-opt] src/test/mir-opt/inline/inline-compatibility.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_compatibility.inlined_target_feature.Inline.diff', src/tools/compiletest/src/runtest.rs:3420:25


failures:
    [mir-opt] src/test/mir-opt/inline/inline-compatibility.rs
