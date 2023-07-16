plain
...........F.....i.........................................ii.......i................... 176/188
............
failures:

Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---- [mir-opt] src/test/mir-opt/inline/inline_instruction_set.rs stdout ----
8       let _3: ();                          // in scope 0 at $DIR/inline_instruction_set.rs:+5:5: +5:30
9 +     scope 1 (inlined instruction_set_t32) { // at $DIR/inline_instruction_set.rs:43:5: 43:26
10 +     }
+ +     scope 2 (inlined instruction_set_default) { // at $DIR/inline_instruction_set.rs:46:5: 46:30
+ +     }
12       bb0: {
12       bb0: {
13           StorageLive(_1);                 // scope 0 at $DIR/inline_instruction_set.rs:+1:5: +1:26

30           StorageDead(_2);                 // scope 0 at $DIR/inline_instruction_set.rs:+2:26: +2:27
31           StorageLive(_3);                 // scope 0 at $DIR/inline_instruction_set.rs:+5:5: +5:30
32 -         _3 = instruction_set_default() -> bb3; // scope 0 at $DIR/inline_instruction_set.rs:+5:5: +5:30
- +         _3 = instruction_set_default() -> bb2; // scope 0 at $DIR/inline_instruction_set.rs:+5:5: +5:30
-                                            // mir::Constant
-                                            // + span: $DIR/inline_instruction_set.rs:46:5: 46:28
-                                            // + literal: Const { ty: fn() {instruction_set_default}, val: Value(<ZST>) }
-   
+ -                                          // mir::Constant
+ -                                          // mir::Constant
+ -                                          // + span: $DIR/inline_instruction_set.rs:46:5: 46:28
+ -                                          // + literal: Const { ty: fn() {instruction_set_default}, val: Value(<ZST>) }
+ -     }
39 -     bb3: {
- +     bb2: {
- +     bb2: {
41           StorageDead(_3);                 // scope 0 at $DIR/inline_instruction_set.rs:+5:30: +5:31
42           _0 = const ();                   // scope 0 at $DIR/inline_instruction_set.rs:+0:14: +6:2
43           return;                          // scope 0 at $DIR/inline_instruction_set.rs:+6:2: +6:2

thread '[mir-opt] src/test/mir-opt/inline/inline_instruction_set.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_instruction_set.t32.Inline.diff', src/tools/compiletest/src/runtest.rs:3447:21


failures:
    [mir-opt] src/test/mir-opt/inline/inline_instruction_set.rs
