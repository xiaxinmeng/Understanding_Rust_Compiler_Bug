plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
....
failures:

---- [mir-opt] src/test/mir-opt/lower_intrinsics.rs stdout ----
50 -         _3 = copy_nonoverlapping::<i32>(move _4, move _8, const 0_usize) -> bb1; // scope 3 at $DIR/lower_intrinsics.rs:90:9: 90:95
51 -                                          // mir::Constant
52 -                                          // + span: $DIR/lower_intrinsics.rs:90:9: 90:28
- -                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(*const i32, *mut i32, usize) {copy_nonoverlapping::<i32>}, val: Value(Scalar(<ZST>)) }
+ -                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(*const i32, *mut i32, usize) {copy_nonoverlapping::<i32>}, val: Value(<ZST>) }
54 +         copy_nonoverlapping(move _4, move _8, const 0_usize); // scope 3 at $DIR/lower_intrinsics.rs:90:9: 90:95
55 +         goto -> bb1;                     // scope 3 at $DIR/lower_intrinsics.rs:90:9: 90:95


thread '[mir-opt] src/test/mir-opt/lower_intrinsics.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/lower_intrinsics.f_copy_nonoverlapping.LowerIntrinsics.diff', src/tools/compiletest/src/runtest.rs:3503:25


failures:
    [mir-opt] src/test/mir-opt/lower_intrinsics.rs
