diff
--- lib.core_arch-x86-avx-_mm256_loadu2_m128d.005-012.DestinationPropagation.before.mir	2021-03-01 22:19:22.215157300 +0100
+++ lib.core_arch-x86-avx-_mm256_loadu2_m128d.005-012.DestinationPropagation.after.mir	2021-03-01 22:19:22.215157300 +0100
@@ -179,10 +179,10 @@
     }
 
     bb7: {
-        _32 = _6;                        // scope 19 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:3104:5: 3104:53
-        _39 = _7;                        // scope 19 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:3104:5: 3104:53
-        _38 = _39;                       // scope 21 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:3104:5: 3104:53
-        _33 = simd_llvm::simd_shuffle4::<x86::__m128d, x86::__m256d>(move _38, move _39, const [0_u32, 1_u32, 0_u32, 0_u32]) -> bb10; // scope 21 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:3104:5: 3104:53
+        _32 = _34;                       // scope 19 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:3104:5: 3104:53
+        _38 = _7;                        // scope 19 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:3104:5: 3104:53
+        nop;                             // scope 21 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:3104:5: 3104:53
+        _33 = simd_llvm::simd_shuffle4::<x86::__m128d, x86::__m256d>(move _38, move _38, const [0_u32, 1_u32, 0_u32, 0_u32]) -> bb10; // scope 21 at ./library/stdarch/crates/core_arch/src/x86/avx.rs:3104:5: 3104:53
                                          // mir::Constant
                                          // + span: ./library/stdarch/crates/core_arch/src/x86/avx.rs:3104:5: 3104:53
                                          // + literal: Const { ty: unsafe extern "platform-intrinsic" fn(core_arch::x86::__m128d, core_arch::x86::__m128d, [u32; 4]) -> core_arch::x86::__m256d {core_arch::simd_llvm::simd_shuffle4::<core_arch::x86::__m128d, core_arch::x86::__m256d>}, val: Value(Scalar(<ZST>)) }
