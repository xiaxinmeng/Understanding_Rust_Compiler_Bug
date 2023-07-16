plain
.................iiiFiiiii.........ii........i................................

failures:

---- [mir-opt] tests/mir-opt/lower_intrinsics.rs stdout ----
30           _4 = _1;                         // scope 0 at $DIR/lower_intrinsics.rs:+1:45: +1:46
31           StorageLive(_5);                 // scope 0 at $DIR/lower_intrinsics.rs:+1:48: +1:49
32           _5 = _2;                         // scope 0 at $DIR/lower_intrinsics.rs:+1:48: +1:49
- -         _3 = wrapping_add::<i32>(move _4, move _5) -> [return: bb1, unwind unreachable]; // scope 0 at $DIR/lower_intrinsics.rs:+1:14: +1:50
+ -         _3 = std::intrinsics::wrapping_add::<i32>(move _4, move _5) -> [return: bb1, unwind unreachable]; // scope 0 at $DIR/lower_intrinsics.rs:+1:14: +1:50
34 -                                          // mir::Constant
35 -                                          // + span: $DIR/lower_intrinsics.rs:9:14: 9:44
- -                                          // + literal: Const { ty: extern "rust-intrinsic" fn(i32, i32) -> i32 {wrapping_add::<i32>}, val: Value(<ZST>) }
+ -                                          // + literal: Const { ty: extern "rust-intrinsic" fn(i32, i32) -> i32 {std::intrinsics::wrapping_add::<i32>}, val: Value(<ZST>) }
37 +         _3 = Add(move _4, move _5);      // scope 0 at $DIR/lower_intrinsics.rs:+1:14: +1:50
38 +         goto -> bb1;                     // scope 0 at $DIR/lower_intrinsics.rs:+1:14: +1:50


46           _7 = _1;                         // scope 1 at $DIR/lower_intrinsics.rs:+2:45: +2:46
47           StorageLive(_8);                 // scope 1 at $DIR/lower_intrinsics.rs:+2:48: +2:49
48           _8 = _2;                         // scope 1 at $DIR/lower_intrinsics.rs:+2:48: +2:49
- -         _6 = wrapping_sub::<i32>(move _7, move _8) -> [return: bb2, unwind unreachable]; // scope 1 at $DIR/lower_intrinsics.rs:+2:14: +2:50
Build completed unsuccessfully in 0:13:19
+ -         _6 = std::intrinsics::wrapping_sub::<i32>(move _7, move _8) -> [return: bb2, unwind unreachable]; // scope 1 at $DIR/lower_intrinsics.rs:+2:14: +2:50
50 -                                          // mir::Constant
51 -                                          // + span: $DIR/lower_intrinsics.rs:10:14: 10:44
- -                                          // + literal: Const { ty: extern "rust-intrinsic" fn(i32, i32) -> i32 {wrapping_sub::<i32>}, val: Value(<ZST>) }
+ -                                          // + literal: Const { ty: extern "rust-intrinsic" fn(i32, i32) -> i32 {std::intrinsics::wrapping_sub::<i32>}, val: Value(<ZST>) }
53 +         _6 = Sub(move _7, move _8);      // scope 1 at $DIR/lower_intrinsics.rs:+2:14: +2:50
54 +         goto -> bb2;                     // scope 1 at $DIR/lower_intrinsics.rs:+2:14: +2:50


thread '[mir-opt] tests/mir-opt/lower_intrinsics.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/tests/mir-opt/lower_intrinsics.wrapping.LowerIntrinsics.diff', src/tools/compiletest/src/runtest.rs:3634:21


failures:
    [mir-opt] tests/mir-opt/lower_intrinsics.rs
