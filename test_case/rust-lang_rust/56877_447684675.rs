
target datalayout = "e-m:w-p:64:64-i32:32-i64:64-i128:128-n32:64-S128"
target triple = "aarch64-pc-windows-msvc"

%Bar = type { [0 x i32], float, [0 x i32], float, [0 x i8], %"core::marker::PhantomData<()>", [0 x i8] }
%"core::marker::PhantomData<()>" = type {}

; Function Attrs: nounwind uwtable
define zeroext i1 @foo([2 x float]) unnamed_addr #0 {
start:
  %abi_cast = alloca [2 x float], align 4
  %f = alloca { float, float }, align 4
  store [2 x float] %0, [2 x float]* %abi_cast, align 4
  %1 = bitcast { float, float }* %f to i8* 
  %2 = bitcast [2 x float]* %abi_cast to i8* 
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %1, i8* align 4 %2, i64 8, i1 false)
  %3 = bitcast { float, float }* %f to float*
  %4 = load float, float* %3, align 4
  %5 = getelementptr inbounds { float, float }, { float, float }* %f, i32 0, i32 1
  %6 = load float, float* %5, align 4
  %7 = fcmp une float %4, %6
  ret i1 %7
}

; Function Attrs: nounwind uwtable
define zeroext i1 @bar(i64) unnamed_addr #0 {
start:
  %abi_cast = alloca i64, align 8
  %f = alloca %Bar, align 4
  store i64 %0, i64* %abi_cast, align 8
  %1 = bitcast %Bar* %f to i8* 
  %2 = bitcast i64* %abi_cast to i8* 
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %1, i8* align 8 %2, i64 8, i1 false)
  %3 = bitcast %Bar* %f to float*
  %4 = load float, float* %3, align 4
  %5 = getelementptr inbounds %Bar, %Bar* %f, i32 0, i32 3
  %6 = load float, float* %5, align 4
  %7 = fcmp une float %4, %6
  ret i1 %7
}
