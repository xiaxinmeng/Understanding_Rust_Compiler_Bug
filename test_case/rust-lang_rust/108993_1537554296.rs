plain
................iii.iiiii........ii........i................................

failures:

---- [mir-opt] tests/mir-opt/const_prop/invalid_constant.rs stdout ----
7       let mut _2: main::InvalidChar;       // in scope 0 at $DIR/invalid_constant.rs:+6:34: +6:63
8       let mut _4: E;                       // in scope 0 at $DIR/invalid_constant.rs:+13:25: +13:59
9       let mut _5: main::InvalidTag;        // in scope 0 at $DIR/invalid_constant.rs:+13:34: +13:55
-       let mut _7: Empty;                   // in scope 0 at $DIR/invalid_constant.rs:+20:35: +20:73
-       let mut _8: main::NoVariants;        // in scope 0 at $DIR/invalid_constant.rs:+20:44: +20:65
12       scope 1 {
Build completed unsuccessfully in 0:12:53
13           debug _invalid_char => _1;       // in scope 1 at $DIR/invalid_constant.rs:+6:9: +6:22
14           let _3: [E; 1];                  // in scope 1 at $DIR/invalid_constant.rs:+13:9: +13:21
15           scope 3 {
15           scope 3 {
16               debug _invalid_tag => _3;    // in scope 3 at $DIR/invalid_constant.rs:+13:9: +13:21
-               let _6: [Empty; 1];          // in scope 3 at $DIR/invalid_constant.rs:+20:9: +20:31
18               scope 5 {
19                   debug _enum_without_variants => const [ZeroSized: Empty]; // in scope 5 at $DIR/invalid_constant.rs:+20:9: +20:31
-                   let _9: main::Str<"���">; // in scope 5 at $DIR/invalid_constant.rs:+24:9: +24:22
21                   scope 7 {
22                       debug _non_utf8_str => const Str::<"���">; // in scope 7 at $DIR/invalid_constant.rs:+24:9: +24:22


43           StorageLive(_5);                 // scope 4 at $DIR/invalid_constant.rs:+13:34: +13:55
44           _5 = InvalidTag { int: const 4_u32 }; // scope 4 at $DIR/invalid_constant.rs:+13:34: +13:55
45 -         _4 = (_5.1: E);                  // scope 4 at $DIR/invalid_constant.rs:+13:34: +13:57
+ -         _3 = [move _4];                  // scope 1 at $DIR/invalid_constant.rs:+13:24: +13:60
46 +         _4 = const Scalar(0x00000004): E; // scope 4 at $DIR/invalid_constant.rs:+13:34: +13:57
47 +                                          // mir::Constant
48 +                                          // + span: no-location

49 +                                          // + literal: Const { ty: E, val: Value(Scalar(0x00000004)) }
-           _3 = [move _4];                  // scope 1 at $DIR/invalid_constant.rs:+13:24: +13:60
+ +         _3 = [const Scalar(0x00000004): E]; // scope 1 at $DIR/invalid_constant.rs:+13:24: +13:60
+ +                                          // mir::Constant
+ +                                          // + span: no-location
+ +                                          // + literal: Const { ty: E, val: Value(Scalar(0x00000004)) }
51           StorageDead(_4);                 // scope 1 at $DIR/invalid_constant.rs:+13:59: +13:60
52           StorageDead(_5);                 // scope 1 at $DIR/invalid_constant.rs:+13:60: +13:61
-           nop;                             // scope 3 at $DIR/invalid_constant.rs:+20:9: +20:31
-           nop;                             // scope 3 at $DIR/invalid_constant.rs:+20:35: +20:73
-           StorageLive(_8);                 // scope 6 at $DIR/invalid_constant.rs:+20:44: +20:65
-           _8 = NoVariants { int: const 0_u32 }; // scope 6 at $DIR/invalid_constant.rs:+20:44: +20:65
-           nop;                             // scope 6 at $DIR/invalid_constant.rs:+20:44: +20:71
-           nop;                             // scope 3 at $DIR/invalid_constant.rs:+20:34: +20:74
-           nop;                             // scope 3 at $DIR/invalid_constant.rs:+20:73: +20:74
-           StorageDead(_8);                 // scope 3 at $DIR/invalid_constant.rs:+20:74: +20:75
-           nop;                             // scope 5 at $DIR/invalid_constant.rs:+24:9: +24:22
-           nop;                             // scope 0 at $DIR/invalid_constant.rs:+0:11: +27:2
-           nop;                             // scope 5 at $DIR/invalid_constant.rs:+27:1: +27:2
-           nop;                             // scope 3 at $DIR/invalid_constant.rs:+27:1: +27:2
65           StorageDead(_3);                 // scope 1 at $DIR/invalid_constant.rs:+27:1: +27:2
66           StorageDead(_1);                 // scope 0 at $DIR/invalid_constant.rs:+27:1: +27:2
67           unreachable;                     // scope 0 at $DIR/invalid_constant.rs:+0:11: +27:2

thread '[mir-opt] tests/mir-opt/const_prop/invalid_constant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_prop/invalid_constant.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3634:21


failures:
    [mir-opt] tests/mir-opt/const_prop/invalid_constant.rs
