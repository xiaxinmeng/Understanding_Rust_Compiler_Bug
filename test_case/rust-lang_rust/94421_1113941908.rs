plain
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
.i........F.........................................i................................
failures:

---- [mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs stdout ----
35 +         StorageLive(_7);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
36 +         _7 = &mut (*_5);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
37 +         Deinit((*_7));                   // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
- +         ((*_7).0: std::boxed::Box<[std::mem::MaybeUninit<u32>]>) = const Box::<[MaybeUninit<u32>]>(Unique::<[MaybeUninit<u32>]> { pointer: NonNull::<[MaybeUninit<u32>]> { pointer: ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }: *const [MaybeUninit::<u32>] }, _marker: PhantomData::<[MaybeUninit<u32>]> }, std::alloc::Global); // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COLalloc/src/vec/mod.rs:LL:COL
+ +         ((*_7).0: std::boxed::Box<[std::mem::MaybeUninit<u32>]>) = const Box::<[MaybeUninit<u32>]>(Unique::<[MaybeUninit<u32>]> { pointer: NonNull::<[MaybeUninit<u32>]> { pointer: ByRef { alloc: Allocation { bytes: [4, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [255], len: Size { raw: 8 } }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }: *const [MaybeUninit::<u32>] }, _marker: PhantomData::<[MaybeUninit<u32>]> }, std::alloc::Global); // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
39                                            // mir::Constant
40 -                                          // + span: $DIR/inline-into-box-place.rs:8:33: 8:41
41 -                                          // + user_ty: UserType(1)

thread '[mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_into_box_place.main.Inline.32bit.diff', src/tools/compiletest/src/runtest.rs:3410:25


failures:
    [mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs
