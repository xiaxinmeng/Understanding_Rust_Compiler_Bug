plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 181 tests
i......................................i................................................ 88/181
........i...F...............................................i........................... 176/181
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] src/test/mir-opt/inline/inline-compatibility.rs stdout ----
---- [mir-opt] src/test/mir-opt/inline/inline-compatibility.rs stdout ----
4   fn inlined_target_feature() -> () {
5       let mut _0: ();                      // return place in scope 0 at $DIR/inline-compatibility.rs:+0:40: +0:40
6       let _1: ();                          // in scope 0 at $DIR/inline-compatibility.rs:+1:5: +1:21
- +     scope 1 (inlined target_feature) {   // at $DIR/inline-compatibility.rs:13:5: 13:21
+ +     scope 1 (inlined target_feature) {   // at $DIR/inline-compatibility.rs:12:5: 12:21
9   
10       bb0: {


11           StorageLive(_1);                 // scope 0 at $DIR/inline-compatibility.rs:+1:5: +1:21
12 -         _1 = target_feature() -> bb1;    // scope 0 at $DIR/inline-compatibility.rs:+1:5: +1:21
13 -                                          // mir::Constant
- -                                          // + span: $DIR/inline-compatibility.rs:13:5: 13:19
+ -                                          // + span: $DIR/inline-compatibility.rs:12:5: 12:19
15 -                                          // + literal: Const { ty: unsafe fn() {target_feature}, val: Value(<ZST>) }
17 - 


thread '[mir-opt] src/test/mir-opt/inline/inline-compatibility.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_compatibility.inlined_target_feature.Inline.diff', src/tools/compiletest/src/runtest.rs:3513:25


failures:
    [mir-opt] src/test/mir-opt/inline/inline-compatibility.rs
