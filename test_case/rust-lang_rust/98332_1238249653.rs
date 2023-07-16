plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.........
failures:

---- [mir-opt] src/test/mir-opt/lower_intrinsics.rs stdout ----
49           StorageDead(_9);                 // scope 3 at $DIR/lower_intrinsics.rs:+4:90: +4:91
50 -         _3 = copy_nonoverlapping::<i32>(move _4, move _8, const 0_usize) -> bb1; // scope 3 at $DIR/lower_intrinsics.rs:+4:9: +4:95
51 -                                          // mir::Constant
- -                                          // + span: $DIR/lower_intrinsics.rs:90:9: 90:28
+ -                                          // + span: $DIR/lower_intrinsics.rs:65:9: 65:28
53 -                                          // + literal: Const { ty: unsafe extern "rust-intrinsic" fn(*const i32, *mut i32, usize) {copy_nonoverlapping::<i32>}, val: Value(<ZST>) }
54 +         copy_nonoverlapping(dst = move _8, src = move _4, count = const 0_usize); // scope 3 at $DIR/lower_intrinsics.rs:+4:9: +4:95
55 +         goto -> bb1;                     // scope 3 at $DIR/lower_intrinsics.rs:+4:9: +4:95

thread '[mir-opt] src/test/mir-opt/lower_intrinsics.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/lower_intrinsics.f_copy_nonoverlapping.LowerIntrinsics.diff', src/tools/compiletest/src/runtest.rs:3516:25


failures:
    [mir-opt] src/test/mir-opt/lower_intrinsics.rs
