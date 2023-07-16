plain
Successfully built 02224fb5ad9f
Successfully tagged rust-ci:latest
Built container sha256:02224fb5ad9f582c07078d008ded7c1bde9c2868f112344837f7529607772a93
Uploading finished image to https://ci-caches.rust-lang.org/docker/7a933025cb171c4b5f5b6011a3583795b8dcc545c75914dd6be8b89a1706a0990abf8b940dcd0d758184ddd671b7cd225927f49feaa72b37c79d66fed681b775
upload failed: - to s3://rust-lang-ci-sccache2/docker/7a933025cb171c4b5f5b6011a3583795b8dcc545c75914dd6be8b89a1706a0990abf8b940dcd0d758184ddd671b7cd225927f49feaa72b37c79d66fed681b775 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..................................................i................................
failures:

---- [mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs stdout ----
35 +         StorageLive(_7);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
36 +         _7 = &mut (*_5);                 // scope 0 at $DIR/inline-into-box-place.rs:8:33: 8:43
37 +         Deinit((*_7));                   // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
- +         ((*_7).0: alloc::raw_vec::RawVec<u32>) = const alloc::raw_vec::RawVec::<u32> { ptr: Unique::<u32> { pointer: {0x4 as *const u32}, _marker: PhantomData::<u32> }, cap: 0_usize, alloc: std::alloc::Global }; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
+ +         ((*_7).0: alloc::raw_vec::RawVec<u32>) = const alloc::raw_vec::RawVec::<u32> { ptr: Unique::<u32> { pointer: NonNull::<u32> { pointer: {0x4 as *const u32} }, _marker: PhantomData::<u32> }, cap: 0_usize, alloc: std::alloc::Global }; // scope 3 at $SRC_DIR/alloc/src/vec/mod.rs:LL:COL
39                                            // mir::Constant
40 -                                          // + span: $DIR/inline-into-box-place.rs:8:33: 8:41
41 -                                          // + user_ty: UserType(1)

thread '[mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs' panicked at 'Actual MIR output differs from expected MIR output /checkout/src/test/mir-opt/inline/inline_into_box_place.main.Inline.64bit.diff', src/tools/compiletest/src/runtest.rs:3406:25


failures:
    [mir-opt] src/test/mir-opt/inline/inline-into-box-place.rs
