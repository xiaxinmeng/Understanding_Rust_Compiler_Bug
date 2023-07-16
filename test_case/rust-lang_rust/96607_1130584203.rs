plain

---- [mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs stdout ----
16       scope 2 {
17       }
18 +     scope 3 (inlined Vec::<u32>::new) {  // at $DIR/inline-into-box-place.rs:8:33: 8:43
- +         let mut _8: alloc::raw_vec::RawVec<u32>; // in scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +         let mut _8: std::boxed::Box<[std::mem::MaybeUninit<u32>]>; // in scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
21   
22       bb0: {


36 +         StorageLive(_7);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
37 +         _7 = &mut (*_5);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
38 +         StorageLive(_8);                 // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
- +         _8 = const alloc::raw_vec::RawVec::<u32> { ptr: Unique::<u32> { pointer: NonNull::<u32> { pointer: {0x4 as *const u32} }, _marker: PhantomData::<u32> }, cap: 0_usize, alloc: std::alloc::Global }; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +         _8 = const Box::<[MaybeUninit<u32>]>(Unique::<[MaybeUninit<u32>]> { pointer: NonNull::<[MaybeUninit<u32>]> { pointer: ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [65535], len: Size(16 bytes) }, align: Align(8 bytes), mutability: Not, extra: () }, offset: Size(0 bytes) }: *const [MaybeUninit::<u32>] }, _marker: PhantomData::<[MaybeUninit<u32>]> }, std::alloc::Global); // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
40                                            // mir::Constant
41 -                                          // + span: $DIR/inline-into-box-place.rs:8:33: 8:41
42 -                                          // + user_ty: UserType(1)
46 -     bb2: {
46 -     bb2: {
47 +                                          // + span: $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
48 +                                          // + user_ty: UserType(0)
- +                                          // + literal: Const { ty: alloc::raw_vec::RawVec<u32>, val: Value(ByRef(..)) }
+ +                                          // + literal: Const { ty: Box<[MaybeUninit<u32>]>, val: Value(ByRef(..)) }
50 +         Deinit((*_7));                   // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
- +         ((*_7).0: alloc::raw_vec::RawVec<u32>) = move _8; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
- +         ((*_7).1: usize) = const 0_usize; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +         ((*_7).0: std::boxed::Box<[std::mem::MaybeUninit<u32>]>) = move _8; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +         ((*_7).2: usize) = const 0_usize; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
53 +         StorageDead(_8);                 // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
54 +         StorageDead(_7);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
55           _1 = move _5;                    // scope 0 at $DIR/inline-into-box-place.rs:8:29: 8:43

thread '[mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_into_box_place.main.Inline.64bit.diff', src/tools/compiletest/src/runtest.rs:3410:25


failures:
    [mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs
