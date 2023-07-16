plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.ii.......i................................
failures:

---- [mir-opt] tests/mir-opt/const_prop/invalid_constant.rs stdout ----
46 -         _3 = [move _4];                  // scope 1 at $DIR/invalid_constant.rs:+13:24: +13:60
47 +         _4 = const Scalar(0x00000004): E; // scope 4 at $DIR/invalid_constant.rs:+13:34: +13:57
48 +                                          // mir::Constant
- +                                          // + span: $DIR/invalid_constant.rs:28:34: 28:57
+ +                                          // + span: $DIR/invalid_constant.rs:26:34: 26:57
50 +                                          // + literal: Const { ty: E, val: Value(Scalar(0x00000004)) }
51 +         _3 = [const Scalar(0x00000004): E]; // scope 1 at $DIR/invalid_constant.rs:+13:24: +13:60
52 +                                          // mir::Constant

- +                                          // + span: $DIR/invalid_constant.rs:28:24: 28:60
+ +                                          // + span: $DIR/invalid_constant.rs:26:24: 26:60
54 +                                          // + literal: Const { ty: E, val: Value(Scalar(0x00000004)) }
55           StorageDead(_4);                 // scope 1 at $DIR/invalid_constant.rs:+13:59: +13:60
56           StorageDead(_5);                 // scope 1 at $DIR/invalid_constant.rs:+13:60: +13:61

thread '[mir-opt] tests/mir-opt/const_prop/invalid_constant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/invalid_constant.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3483:21


failures:
    [mir-opt] tests/mir-opt/const_prop/invalid_constant.rs
