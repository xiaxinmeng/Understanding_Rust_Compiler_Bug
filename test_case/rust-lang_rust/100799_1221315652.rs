plain
test [ui] src/test/ui/abi/lib-defaults.rs ... ok
test [ui] src/test/ui/abi/issues/issue-62350-sysv-neg-reg-counts.rs ... ok
test [ui] src/test/ui/abi/mir/mir_codegen_calls_variadic.rs ... ok
test [ui] src/test/ui/abi/foreign/invoke-external-foreign.rs ... ok
test [ui] src/test/ui/abi/issues/issue-97463-broken-abi-leaked-uninit-data.rs ... ok
test [ui] src/test/ui/abi/foreign/foreign-dupe.rs ... ok
test [ui] src/test/ui/abi/numbers-arithmetic/i128-ffi.rs ... ok
test [ui] src/test/ui/abi/extern/extern-call-deep2.rs ... ok
test [ui] src/test/ui/abi/unsupported.rs#aarch64 ... ok
---
test [ui] src/test/ui/feature-gates/issue-43106-gating-of-unstable.rs ... ok
test [ui] src/test/ui/feature-gates/issue-49983-see-issue-0.rs ... ok
test [ui] src/test/ui/feature-gates/rustc-private.rs ... ok
test [ui] src/test/ui/feature-gates/stability-attribute-consistency.rs ... ok
test [ui] src/test/ui/feature-gates/soft-syntax-gates-without-errors.rs ... ok
test [ui] src/test/ui/feature-gates/soft-syntax-gates-with-errors.rs ... ok
test [ui] src/test/ui/feature-gates/stable-features.rs ... ok
test [ui] src/test/ui/ffi_const.rs ... ok
test [ui] src/test/ui/feature-gates/unknown-feature.rs ... ok
test [ui] src/test/ui/ffi_const2.rs ... ok
---
test [codegen] src/test/codegen/simd-intrinsic/simd-intrinsic-float-pow.rs ... ok
test [codegen] src/test/codegen/slice-iter-len-eq-zero.rs ... ok
test [codegen] src/test/codegen/simd-intrinsic/simd-intrinsic-float-sin.rs ... ok
test [codegen] src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs ... ok
test [codegen] src/test/codegen/some-abis-do-extend-params-to-32-bits.rs#aarch64-apple ... ok
test [codegen] src/test/codegen/some-abis-do-extend-params-to-32-bits.rs#x86_64 ... ok
test [codegen] src/test/codegen/some-abis-do-extend-params-to-32-bits.rs#i686 ... ok
test [codegen] src/test/codegen/some-abis-do-extend-params-to-32-bits.rs#arm ... ok
test [codegen] src/test/codegen/slice-ref-equality.rs ... ok
test [codegen] src/test/codegen/some-abis-do-extend-params-to-32-bits.rs#aarch64-linux ... ok
test [codegen] src/test/codegen/some-abis-do-extend-params-to-32-bits.rs#aarch64-windows ... ok
test [codegen] src/test/codegen/sparc-struct-abi.rs ... ok
test [codegen] src/test/codegen/sparc-struct-abi.rs ... ok
test [codegen] src/test/codegen/some-abis-do-extend-params-to-32-bits.rs#riscv ... ok
test [codegen] src/test/codegen/slice-position-bounds-check.rs ... ok
test [codegen] src/test/codegen/swap-large-types.rs ... ignored
test [codegen] src/test/codegen/swap-simd-types.rs ... ignored
test [codegen] src/test/codegen/swap-small-types.rs ... ignored
---
failures:

---- [codegen] src/test/codegen/intrinsics/mask.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/intrinsics/mask/mask.ll" "/checkout/src/test/codegen/intrinsics/mask.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/intrinsics/mask.rs:9:17: error: CHECK-SAME: expected string not found in input
 // CHECK-SAME: @llvm.ptrmask.p0i8.[[WORD]](
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/intrinsics/mask/mask.ll:9:16: note: scanning from here
 %0 = tail call ptr @llvm.ptrmask.p0.i64(ptr %ptr, i64 %mask)
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/intrinsics/mask/mask.ll:9:16: note: with "WORD" equal to "i64"
 %0 = tail call ptr @llvm.ptrmask.p0.i64(ptr %ptr, i64 %mask)
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/intrinsics/mask/mask.ll:9:21: note: possible intended match here
 %0 = tail call ptr @llvm.ptrmask.p0.i64(ptr %ptr, i64 %mask)

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/intrinsics/mask/mask.ll
Check file: /checkout/src/test/codegen/intrinsics/mask.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
          1: ; ModuleID = 'mask.1ff5246e-cgu.0' 
          2: source_filename = "mask.1ff5246e-cgu.0" 
          3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
          4: target triple = "x86_64-unknown-linux-gnu" 
          5:  
          6: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind readnone willreturn uwtable 
          7: define ptr @mask_ptr(ptr readnone %ptr, i64 %mask) unnamed_addr #0 { 
          8: start: 
          9:  %0 = tail call ptr @llvm.ptrmask.p0.i64(ptr %ptr, i64 %mask) 
same:9'0                    X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
same:9'1                                                                    with "WORD" equal to "i64"
same:9'2                         ?                                          possible intended match
         10:  ret ptr %0 
same:9'0     ~~~~~~~~~~~~
         11: } 
same:9'0     ~~
         12:  
same:9'0     ~
         13: ; Function Attrs: mustprogress nocallback nofree nosync nounwind readnone speculatable willreturn 
same:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         14: declare ptr @llvm.ptrmask.p0.i64(ptr, i64) #1 
same:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          .
          .
>>>>>>
------------------------------------------
