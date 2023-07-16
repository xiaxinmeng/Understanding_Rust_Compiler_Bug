plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [mir-opt] mir-opt/const_prop/invalid_constant.rs stdout ----
24   
25       bb0: {
26           StorageLive(_1);                 // scope 0 at $DIR/invalid_constant.rs:21:9: 21:22
- -         _1 = const { InvalidChar { int: 0x110001 } }; // scope 0 at $DIR/invalid_constant.rs:21:25: 21:64
- +         _1 = const InvalidChar { int: 1114113_u32, chr: {transmute(0x00110001): char} }; // scope 0 at $DIR/invalid_constant.rs:21:25: 21:64
+           _1 = const { InvalidChar { int: 0x110001 } }; // scope 0 at $DIR/invalid_constant.rs:21:25: 21:64
29                                            // mir::Constant
30                                            // + span: $DIR/invalid_constant.rs:21:25: 21:64
- -                                          // + literal: Const { ty: InvalidChar, val: Unevaluated(main::{constant#0}, [main::InvalidChar], None) }
- +                                          // + literal: Const { ty: InvalidChar, val: Value(Scalar(0x00110001)) }
+                                            // + literal: Const { ty: InvalidChar, val: Unevaluated(main::{constant#0}, [main::InvalidChar], None) }
33           StorageLive(_2);                 // scope 1 at $DIR/invalid_constant.rs:28:9: 28:21
34           StorageLive(_3);                 // scope 1 at $DIR/invalid_constant.rs:28:25: 28:46
35           (_3.0: u32) = const 4_u32;       // scope 1 at $DIR/invalid_constant.rs:28:25: 28:46

- -         _2 = [move _3];                  // scope 1 at $DIR/invalid_constant.rs:28:24: 28:47
- +         _2 = [const InvalidTag { int: 4_u32, e: Scalar(0x00000004): E }]; // scope 1 at $DIR/invalid_constant.rs:28:24: 28:47
- +                                          // mir::Constant
- +                                          // + span: $DIR/invalid_constant.rs:28:24: 28:47
- +                                          // + literal: Const { ty: InvalidTag, val: Value(Scalar(0x00000004)) }
+           _2 = [move _3];                  // scope 1 at $DIR/invalid_constant.rs:28:24: 28:47
41           StorageDead(_3);                 // scope 1 at $DIR/invalid_constant.rs:28:46: 28:47
42           StorageLive(_4);                 // scope 2 at $DIR/invalid_constant.rs:35:9: 35:31
43           StorageLive(_5);                 // scope 2 at $DIR/invalid_constant.rs:35:35: 35:56

44           (_5.0: u32) = const 0_u32;       // scope 2 at $DIR/invalid_constant.rs:35:35: 35:56
- -         _4 = [move _5];                  // scope 2 at $DIR/invalid_constant.rs:35:34: 35:57
- +         _4 = [const NoVariants { int: 0_u32, empty: Scalar(<ZST>): Empty }]; // scope 2 at $DIR/invalid_constant.rs:35:34: 35:57
- +                                          // mir::Constant
- +                                          // + span: $DIR/invalid_constant.rs:35:34: 35:57
- +                                          // + literal: Const { ty: NoVariants, val: Value(Scalar(0x00000000)) }
+           _4 = [move _5];                  // scope 2 at $DIR/invalid_constant.rs:35:34: 35:57
50           StorageDead(_5);                 // scope 2 at $DIR/invalid_constant.rs:35:56: 35:57
51           StorageLive(_6);                 // scope 3 at $DIR/invalid_constant.rs:39:9: 39:22
52           nop;                             // scope 0 at $DIR/invalid_constant.rs:15:11: 42:2

thread '[mir-opt] mir-opt/const_prop/invalid_constant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/invalid_constant.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3386:25


failures:
    [mir-opt] mir-opt/const_prop/invalid_constant.rs
