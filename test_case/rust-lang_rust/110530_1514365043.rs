plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 244 tests
.............................................................................i..........  88/244
.....................................................i.F.........ii.i................... 176/244

failures:

---- [mir-opt] tests/mir-opt/inline/inline_instruction_set.rs stdout ----
---- [mir-opt] tests/mir-opt/inline/inline_instruction_set.rs stdout ----
11 +     }
12 +     scope 2 (inlined instruction_set_default) { // at $DIR/inline_instruction_set.rs:51:5: 51:30
13 +     }
+ +     scope 3 (inlined inline_always_and_using_inline_asm) { // at $DIR/inline_instruction_set.rs:52:5: 52:41
+ +         scope 4 {
+ +         }
+ +     }
15       bb0: {
15       bb0: {
16           StorageLive(_1);                 // scope 0 at $DIR/inline_instruction_set.rs:+1:5: +1:26

42           StorageDead(_3);                 // scope 0 at $DIR/inline_instruction_set.rs:+3:30: +3:31
43           StorageLive(_4);                 // scope 0 at $DIR/inline_instruction_set.rs:+4:5: +4:41
44 -         _4 = inline_always_and_using_inline_asm() -> [return: bb4, unwind unreachable]; // scope 0 at $DIR/inline_instruction_set.rs:+4:5: +4:41
- +         _4 = inline_always_and_using_inline_asm() -> [return: bb2, unwind unreachable]; // scope 0 at $DIR/inline_instruction_set.rs:+4:5: +4:41
-                                            // mir::Constant
-                                            // + span: $DIR/inline_instruction_set.rs:52:5: 52:39
-                                            // + literal: Const { ty: fn() {inline_always_and_using_inline_asm}, val: Value(<ZST>) }
+ -                                          // mir::Constant
+ -                                          // + span: $DIR/inline_instruction_set.rs:52:5: 52:39
+ -                                          // + literal: Const { ty: fn() {inline_always_and_using_inline_asm}, val: Value(<ZST>) }
+ +         asm!("/* do nothing */", options((empty))) -> [return: bb2, unwind unreachable]; // scope 4 at $DIR/inline_instruction_set.rs:43:14: 43:38
50   
51 -     bb4: {


thread '[mir-opt] tests/mir-opt/inline/inline_instruction_set.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/inline/inline_instruction_set.t32.Inline.diff', src/tools/compiletest/src/runtest.rs:3527:21


failures:
    [mir-opt] tests/mir-opt/inline/inline_instruction_set.rs
