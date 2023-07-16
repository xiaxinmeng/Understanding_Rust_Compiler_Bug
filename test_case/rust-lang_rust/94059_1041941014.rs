plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.................................i...............................
failures:

---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
47 -     bb2: {
48 +                                          // + span: $DIR/inline-into-box-place.rs:8:33: 8:43
49 +                                          // + user_ty: UserType(0)
- +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(5:77 ~ alloc[25db]::raw_vec::{impl#0}::NEW), const_param_did: None }, substs: [u32], promoted: None }) }
+ +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(5:77 ~ alloc[bd09]::raw_vec::{impl#0}::NEW), const_param_did: None }, substs: [u32], promoted: None }) }
51 +         ((*_7).1: usize) = const 0_usize; // scope 3 at $DIR/inline-into-box-place.rs:8:33: 8:43
52 +         StorageDead(_7);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
53           _1 = move _5;                    // scope 0 at $DIR/inline-into-box-place.rs:8:29: 8:43

thread '[mir-opt] mir-opt/inline/inline-into-box-place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_into_box_place.main.Inline.64bit.diff', src/tools/compiletest/src/runtest.rs:3375:25


failures:
    [mir-opt] mir-opt/inline/inline-into-box-place.rs
