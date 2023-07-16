plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.................................................i...........................F....
failures:

---- [mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs stdout ----
34 -         (*_5) = Vec::<u32>::new() -> [return: bb2, unwind: bb5]; // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
35 +         StorageLive(_7);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
36 +         _7 = &mut (*_5);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
- +         ((*_7).0: alloc::raw_vec::RawVec<u32>) = const alloc::raw_vec::RawVec::<u32> { ptr: Unique::<u32> { pointer: {0x4 as *const u32}, _marker: PhantomData::<u32> }, cap: 0_usize, alloc: std::alloc::Global }; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +         ((*_7).0: std::boxed::Box<[std::mem::MaybeUninit<u32>]>) = const Box::<[MaybeUninit<u32>]>(Unique::<[MaybeUninit<u32>]> { pointer: ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [65535], len: Size { raw: 16 } }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }: *const [MaybeUninit::<u32>], _marker: PhantomData::<[MaybeUninit<u32>]> }, std::alloc::Global); // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
38                                            // mir::Constant
39 -                                          // + span: $DIR/inline-into-box-place.rs:8:33: 8:41
40 -                                          // + user_ty: UserType(1)
44 -     bb2: {
44 -     bb2: {
45 +                                          // + span: $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
46 +                                          // + user_ty: UserType(0)
- +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [65535], len: Size { raw: 16 } }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
- +         ((*_7).1: usize) = const 0_usize; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +                                          // + literal: Const { ty: Box<[MaybeUninit<u32>]>, val: Value(ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [65535], len: Size { raw: 16 } }, align: Align { pow2: 3 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
+ +         ((*_7).2: usize) = const 0_usize; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
49 +         StorageDead(_7);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
50           _1 = move _5;                    // scope 0 at $DIR/inline-into-box-place.rs:8:29: 8:43
51           StorageDead(_5);                 // scope 0 at $DIR/inline-into-box-place.rs:8:42: 8:43

thread '[mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_into_box_place.main.Inline.64bit.diff', src/tools/compiletest/src/runtest.rs:3406:25

---- [mir-opt] src/test/mir-opt/unusual-item-types.rs stdout ----
22     }
23 
23 
24     bb4 (cleanup): {
-         drop(((*_1).0: alloc::raw_vec::RawVec<i32>)) -> bb2; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+         drop(((*_1).0: std::boxed::Box<[std::mem::MaybeUninit<i32>]>)) -> bb2; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
27 
28     bb5: {


-         drop(((*_1).0: alloc::raw_vec::RawVec<i32>)) -> [return: bb3, unwind: bb2]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+         drop(((*_1).0: std::boxed::Box<[std::mem::MaybeUninit<i32>]>)) -> [return: bb3, unwind: bb2]; // scope 0 at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
31 
32     bb6: {


thread '[mir-opt] src/test/mir-opt/unusual-item-types.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/unusual_item_types.core.ptr-drop_in_place.Vec_i32_.AddMovesForPackedDrops.before.64bit.mir', src/tools/compiletest/src/runtest.rs:3406:25

failures:
    [mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs
    [mir-opt] src/test/mir-opt/unusual-item-types.rs
