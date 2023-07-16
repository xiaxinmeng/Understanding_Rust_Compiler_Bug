plain
.......................ii........i..................................

failures:

---- [mir-opt] tests/mir-opt/const_promotion_extern_static.rs stdout ----
18 -         StorageLive(_3);                 // scope 0 at $DIR/const_promotion_extern_static.rs:+0:31: +0:46
19 -         StorageLive(_4);                 // scope 0 at $DIR/const_promotion_extern_static.rs:+0:32: +0:45
20 -         StorageLive(_5);                 // scope 1 at $DIR/const_promotion_extern_static.rs:+0:42: +0:43
- -         _5 = const {alloc2: *const i32}; // scope 1 at $DIR/const_promotion_extern_static.rs:+0:42: +0:43
+ -         _5 = const {alloc3: *const i32}; // scope 1 at $DIR/const_promotion_extern_static.rs:+0:42: +0:43
22 +         _6 = const _;                    // scope 0 at $DIR/const_promotion_extern_static.rs:+0:31: +0:55
23                                            // mir::Constant
24 -                                          // + span: $DIR/const_promotion_extern_static.rs:13:42: 13:43

- -                                          // + literal: Const { ty: *const i32, val: Value(Scalar(alloc2)) }
+ -                                          // + literal: Const { ty: *const i32, val: Value(Scalar(alloc3)) }
26 -         _4 = &(*_5);                     // scope 1 at $DIR/const_promotion_extern_static.rs:+0:41: +0:43
27 -         _3 = [move _4];                  // scope 0 at $DIR/const_promotion_extern_static.rs:+0:31: +0:46
28 -         _2 = &_3;                        // scope 0 at $DIR/const_promotion_extern_static.rs:+0:31: +0:55
50       }
51   }
52 - 
52 - 
- - alloc2 (extern static: X)
+ - alloc3 (extern static: X)
55 


thread '[mir-opt] tests/mir-opt/const_promotion_extern_static.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/const_promotion_extern_static.FOO.PromoteTemps.diff', src/tools/compiletest/src/runtest.rs:3527:21


failures:
    [mir-opt] tests/mir-opt/const_promotion_extern_static.rs
