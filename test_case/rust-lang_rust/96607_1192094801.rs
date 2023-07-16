plain

---- [mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs stdout ----
18       scope 2 {
19       }
20 +     scope 3 (inlined Vec::<u32>::new) {  // at $DIR/inline-into-box-place.rs:8:33: 8:43
- +         let mut _10: alloc::raw_vec::RawVec<u32>; // in scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +         let mut _10: std::boxed::Box<[std::mem::MaybeUninit<u32>]>; // in scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
23   
24       bb0: {


40 +         StorageLive(_9);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
41 +         _9 = &mut (*_7);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
42 +         StorageLive(_10);                // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
- +         _10 = const alloc::raw_vec::RawVec::<u32>::NEW; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +         _10 = const Box::<[MaybeUninit<u32>]>::EMPTY; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
44                                            // mir::Constant
45 -                                          // + span: $DIR/inline-into-box-place.rs:8:33: 8:41
46 -                                          // + user_ty: UserType(1)
50 -     bb2: {
50 -     bb2: {
51 +                                          // + span: $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
52 +                                          // + user_ty: UserType(0)
- +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Unevaluated(alloc::raw_vec::RawVec::<T>::NEW, [u32], None) }
+ +                                          // + literal: Const { ty: Box<[MaybeUninit<u32>]>, val: Unevaluated(Box::<[T]>::EMPTY, [std::mem::MaybeUninit<u32>], None) }
54 +         Deinit((*_9));                   // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
- +         ((*_9).0: alloc::raw_vec::RawVec<u32>) = move _10; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
- +         ((*_9).1: usize) = const 0_usize; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +         ((*_9).0: std::boxed::Box<[std::mem::MaybeUninit<u32>]>) = move _10; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +         ((*_9).2: usize) = const 0_usize; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
57 +         StorageDead(_10);                // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
58 +         StorageDead(_9);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
59           StorageDead(_7);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43

thread '[mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_into_box_place.main.Inline.32bit.diff', src/tools/compiletest/src/runtest.rs:3512:25


failures:
    [mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs
