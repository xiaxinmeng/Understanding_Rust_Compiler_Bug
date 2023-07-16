diff
--- before/lib.core_arch-x86-avx-_mm256_insertf128_pd.005-022.PreCodegen.before.mir	2021-03-02 00:03:20.903932968 +0100
+++ after/lib.core_arch-x86-avx-_mm256_insertf128_pd.005-022.PreCodegen.before.mir	2021-03-01 22:19:20.951121662 +0100
@@ -21,80 +21,56 @@
         let mut _12: core_arch::x86::__m128d; // in scope 2 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1550:31: 1550:56
         let mut _13: core_arch::x86::__m128d; // in scope 2 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1550:31: 1550:56
     }
 
     bb0: {
-        StorageLive(_4);                 // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1549:11: 1549:19
-        StorageLive(_5);                 // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1549:11: 1549:15
         _5 = _3;                         // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1549:11: 1549:15
         _4 = BitAnd(move _5, const 1_i32); // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1549:11: 1549:19
-        StorageDead(_5);                 // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1549:18: 1549:19
         switchInt(_4) -> [0_i32: bb2, otherwise: bb1]; // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1550:9: 1550:10
     }
 
     bb1: {
-        StorageLive(_8);                 // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1551:28: 1551:29
         _8 = _1;                         // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1551:28: 1551:29
-        StorageLive(_9);                 // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1551:31: 1551:56
         _11 = _2;                        // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1551:54: 1551:55
-        StorageLive(_10);                // scope 1 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1551:31: 1551:56
         _10 = _11;                       // scope 1 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1551:31: 1551:56
-        _9 = simd_llvm::simd_shuffle4::<x86::__m128d, x86::__m256d>(move _10, move _11, const [0_u32, 1_u32, 0_u32, 0_u32]) -> bb6; // scope 1 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1551:31: 1551:56
+        _9 = simd_llvm::simd_shuffle4::<x86::__m128d, x86::__m256d>(move _10, move _11, const [0_u32, 1_u32, 0_u32, 0_u32]) -> bb4; // scope 1 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1551:31: 1551:56
                                          // mir::Constant
                                          // + span: ./library/stdarch/crates/core_arch/src/x86/avx.rs:1551:31: 1551:56
                                          // + literal: Const { ty: unsafe extern "platform-intrinsic" fn(core_arch::x86::__m128d, core_arch::x86::__m128d, [u32; 4]) -> core_arch::x86::__m256d {core_arch::simd_llvm::simd_shuffle4::<core_arch::x86::__m128d, core_arch::x86::__m256d>}, val: Value(Scalar(<ZST>)) }
                                          // mir::Constant
                                          // + span: ./library/stdarch/crates/core_arch/src/x86/avx.rs:1551:31: 1551:56
                                          // + literal: Const { ty: [u32; 4], val: Value(ByRef { alloc: Allocation { bytes: [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [65535], len: Size { raw: 16 } }, size: Size { raw: 16 }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
     }
 
     bb2: {
-        StorageLive(_6);                 // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1550:28: 1550:29
         _6 = _1;                         // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1550:28: 1550:29
-        StorageLive(_7);                 // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1550:31: 1550:56
         _13 = _2;                        // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1550:54: 1550:55
-        StorageLive(_12);                // scope 2 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1550:31: 1550:56
         _12 = _13;                       // scope 2 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1550:31: 1550:56
-        _7 = simd_llvm::simd_shuffle4::<x86::__m128d, x86::__m256d>(move _12, move _13, const [0_u32, 1_u32, 0_u32, 0_u32]) -> bb7; // scope 2 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1550:31: 1550:56
+        _7 = simd_llvm::simd_shuffle4::<x86::__m128d, x86::__m256d>(move _12, move _13, const [0_u32, 1_u32, 0_u32, 0_u32]) -> bb5; // scope 2 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1550:31: 1550:56
                                          // mir::Constant
                                          // + span: ./library/stdarch/crates/core_arch/src/x86/avx.rs:1550:31: 1550:56
                                          // + literal: Const { ty: unsafe extern "platform-intrinsic" fn(core_arch::x86::__m128d, core_arch::x86::__m128d, [u32; 4]) -> core_arch::x86::__m256d {core_arch::simd_llvm::simd_shuffle4::<core_arch::x86::__m128d, core_arch::x86::__m256d>}, val: Value(Scalar(<ZST>)) }
                                          // mir::Constant
                                          // + span: ./library/stdarch/crates/core_arch/src/x86/avx.rs:1550:31: 1550:56
                                          // + literal: Const { ty: [u32; 4], val: Value(ByRef { alloc: Allocation { bytes: [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [65535], len: Size { raw: 16 } }, size: Size { raw: 16 }, align: Align { pow2: 2 }, mutability: Not, extra: () }, offset: Size { raw: 0 } }) }
     }
 
     bb3: {
-        StorageDead(_7);                 // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1550:70: 1550:71
-        StorageDead(_6);                 // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1550:70: 1550:71
-        goto -> bb5;                     // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1549:5: 1552:6
+        return;                          // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1549:5: 1552:6
     }
 
     bb4: {
-        StorageDead(_9);                 // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1551:70: 1551:71
-        StorageDead(_8);                 // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1551:70: 1551:71
-        goto -> bb5;                     // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1549:5: 1552:6
-    }
-
-    bb5: {
-        StorageDead(_4);                 // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1553:1: 1553:2
-        return;                          // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1553:2: 1553:2
-    }
-
-    bb6: {
-        StorageDead(_10);                // scope 1 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1551:31: 1551:56
-        _0 = simd_llvm::simd_shuffle4::<x86::__m256d, x86::__m256d>(move _8, move _9, const x86::avx::_mm256_insertf128_pd::promoted[1]) -> bb4; // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1551:14: 1551:71
+        _0 = simd_llvm::simd_shuffle4::<x86::__m256d, x86::__m256d>(move _8, move _9, const x86::avx::_mm256_insertf128_pd::promoted[1]) -> bb3; // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1551:14: 1551:71
                                          // mir::Constant
                                          // + span: ./library/stdarch/crates/core_arch/src/x86/avx.rs:1551:14: 1551:27
                                          // + literal: Const { ty: unsafe extern "platform-intrinsic" fn(core_arch::x86::__m256d, core_arch::x86::__m256d, [u32; 4]) -> core_arch::x86::__m256d {core_arch::simd_llvm::simd_shuffle4::<core_arch::x86::__m256d, core_arch::x86::__m256d>}, val: Value(Scalar(<ZST>)) }
                                          // mir::Constant
                                          // + span: ./library/stdarch/crates/core_arch/src/x86/avx.rs:1551:14: 1551:71
                                          // + literal: Const { ty: [u32; 4], val: Unevaluated(WithOptConstParam { did: DefId(0:1247 ~ lib[8787]::core_arch::x86::avx::_mm256_insertf128_pd), const_param_did: None }, [], Some(promoted[1])) }
     }
 
-    bb7: {
-        StorageDead(_12);                // scope 2 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1550:31: 1550:56
+    bb5: {
         _0 = simd_llvm::simd_shuffle4::<x86::__m256d, x86::__m256d>(move _6, move _7, const x86::avx::_mm256_insertf128_pd::promoted[0]) -> bb3; // scope 0 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:1550:14: 1550:71
                                          // mir::Constant
                                          // + span: ./library/stdarch/crates/core_arch/src/x86/avx.rs:1550:14: 1550:27
                                          // + literal: Const { ty: unsafe extern "platform-intrinsic" fn(core_arch::x86::__m256d, core_arch::x86::__m256d, [u32; 4]) -> core_arch::x86::__m256d {core_arch::simd_llvm::simd_shuffle4::<core_arch::x86::__m256d, core_arch::x86::__m256d>}, val: Value(Scalar(<ZST>)) }
                                          // mir::Constant
