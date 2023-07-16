llvm
; ModuleID = 'cast2.cgu-0.rs'
source_filename = "cast2.cgu-0.rs"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%U24 = type { i8, [0 x i8], i8, [0 x i8], i8, [0 x i8] }

; cast2::foo
; Function Attrs: nounwind uwtable
define i32 @_ZN5cast23foo17h1f3d5ac3e0c9cc02E(i32) unnamed_addr #0 {
start:
  %abi_cast1 = alloca i32
  %_3 = alloca %U24
  %x = alloca %U24
  %_0 = alloca %U24
  %abi_cast = alloca i32
  %arg0 = alloca %U24
  store i32 %0, i32* %abi_cast
  %1 = bitcast %U24* %arg0 to i8*
  %2 = bitcast i32* %abi_cast to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %1, i8* %2, i64 3, i32 1, i1 false)
  %3 = bitcast %U24* %arg0 to i8*
  %4 = bitcast %U24* %x to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %4, i8* %3, i64 3, i32 1, i1 false)
  %5 = bitcast %U24* %x to i8*
  %6 = bitcast %U24* %_3 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %6, i8* %5, i64 3, i32 1, i1 false)
  %7 = bitcast %U24* %_3 to i32*
  %8 = load i32, i32* %7, align 1 ; bad load #1
  %9 = call i32 @bar(i32 %8)
  store i32 %9, i32* %abi_cast1
  %10 = bitcast %U24* %_0 to i8*
  %11 = bitcast i32* %abi_cast1 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %10, i8* %11, i64 3, i32 1, i1 false)
  br label %bb1

bb1:                                              ; preds = %start
  %12 = bitcast %U24* %_0 to i32*
  %13 = load i32, i32* %12, align 1 ; bad load #2
  ret i32 %13
}

; Function Attrs: argmemonly nounwind
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* nocapture writeonly, i8* nocapture readonly, i64, i32, i1) #1

; Function Attrs: nounwind
declare i32 @bar(i32) unnamed_addr #2

attributes #0 = { nounwind uwtable "probe-stack"="__rust_probestack" }
attributes #1 = { argmemonly nounwind }
attributes #2 = { nounwind "probe-stack"="__rust_probestack" }
