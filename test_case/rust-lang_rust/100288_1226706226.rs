plain

---- [mir-opt] src/test/mir-opt/const_prop/invalid_constant.rs stdout ----
33       }
34   
35       bb0: {
-           StorageLive(_2);                 // scope 0 at $DIR/invalid_constant.rs:+6:9: +6:22
-           StorageLive(_3);                 // scope 2 at $DIR/invalid_constant.rs:+6:34: +6:63
-           Deinit(_3);                      // scope 2 at $DIR/invalid_constant.rs:+6:34: +6:63
-           (_3.0: u32) = const 1114113_u32; // scope 2 at $DIR/invalid_constant.rs:+6:34: +6:63
- -         _2 = (_3.1: char);               // scope 2 at $DIR/invalid_constant.rs:+6:34: +6:67
- +         _2 = const {transmute(0x00110001): char}; // scope 2 at $DIR/invalid_constant.rs:+6:34: +6:67
-           StorageDead(_3);                 // scope 0 at $DIR/invalid_constant.rs:+6:69: +6:70
-           StorageLive(_4);                 // scope 1 at $DIR/invalid_constant.rs:+13:9: +13:21
-           StorageLive(_5);                 // scope 1 at $DIR/invalid_constant.rs:+13:25: +13:59
-           StorageLive(_6);                 // scope 4 at $DIR/invalid_constant.rs:+13:34: +13:55
-           Deinit(_6);                      // scope 4 at $DIR/invalid_constant.rs:+13:34: +13:55
-           (_6.0: u32) = const 4_u32;       // scope 4 at $DIR/invalid_constant.rs:+13:34: +13:55
- -         _5 = (_6.1: E);                  // scope 4 at $DIR/invalid_constant.rs:+13:34: +13:57
- -         _4 = [move _5];                  // scope 1 at $DIR/invalid_constant.rs:+13:24: +13:60
- +         _5 = const Scalar(0x00000004): E; // scope 4 at $DIR/invalid_constant.rs:+13:34: +13:57
- +                                          // mir::Constant
- +                                          // + span: $DIR/invalid_constant.rs:28:34: 28:57
- +                                          // + literal: Const { ty: E, val: Value(Scalar(0x00000004)) }
- +         _4 = [const Scalar(0x00000004): E]; // scope 1 at $DIR/invalid_constant.rs:+13:24: +13:60
- +                                          // mir::Constant
- +                                          // + span: $DIR/invalid_constant.rs:28:24: 28:60
- +                                          // + literal: Const { ty: E, val: Value(Scalar(0x00000004)) }
-           StorageDead(_5);                 // scope 1 at $DIR/invalid_constant.rs:+13:59: +13:60
-           StorageDead(_6);                 // scope 1 at $DIR/invalid_constant.rs:+13:60: +13:61
-           StorageLive(_7);                 // scope 3 at $DIR/invalid_constant.rs:+20:9: +20:31
-           StorageLive(_8);                 // scope 3 at $DIR/invalid_constant.rs:+20:35: +20:73
-           StorageLive(_9);                 // scope 6 at $DIR/invalid_constant.rs:+20:44: +20:65
-           Deinit(_9);                      // scope 6 at $DIR/invalid_constant.rs:+20:44: +20:65
-           (_9.0: u32) = const 0_u32;       // scope 6 at $DIR/invalid_constant.rs:+20:44: +20:65
-           nop;                             // scope 6 at $DIR/invalid_constant.rs:+20:44: +20:71
-           nop;                             // scope 3 at $DIR/invalid_constant.rs:+20:34: +20:74
-           StorageDead(_8);                 // scope 3 at $DIR/invalid_constant.rs:+20:73: +20:74
-           StorageDead(_9);                 // scope 3 at $DIR/invalid_constant.rs:+20:74: +20:75
-           StorageLive(_10);                // scope 5 at $DIR/invalid_constant.rs:+24:9: +24:22
-           StorageDead(_10);                // scope 5 at $DIR/invalid_constant.rs:+27:1: +27:2
-           StorageDead(_7);                 // scope 3 at $DIR/invalid_constant.rs:+27:1: +27:2
-           StorageDead(_4);                 // scope 1 at $DIR/invalid_constant.rs:+27:1: +27:2
-           StorageDead(_2);                 // scope 0 at $DIR/invalid_constant.rs:+27:1: +27:2
74           unreachable;                     // scope 0 at $DIR/invalid_constant.rs:+0:11: +27:2
76   }


thread '[mir-opt] src/test/mir-opt/const_prop/invalid_constant.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/const_prop/invalid_constant.main.ConstProp.diff', src/tools/compiletest/src/runtest.rs:3512:25

---- [mir-opt] src/test/mir-opt/uninhabited-enum.rs stdout ----
11     }
12 
12 
13     bb0: {
-         StorageLive(_2);                 // scope 0 at $DIR/uninhabited-enum.rs:+1:8: +1:14
-         StorageDead(_2);                 // scope 0 at $DIR/uninhabited-enum.rs:+4:1: +4:2
16         unreachable;                     // scope 0 at $DIR/uninhabited-enum.rs:+0:41: +4:2
18 }


thread '[mir-opt] src/test/mir-opt/uninhabited-enum.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/uninhabited_enum.process_void.SimplifyLocals.after.mir', src/tools/compiletest/src/runtest.rs:3512:25

failures:
    [mir-opt] src/test/mir-opt/const_prop/invalid_constant.rs
    [mir-opt] src/test/mir-opt/uninhabited-enum.rs
