
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%Bar = type { [0 x i32], float, [0 x i32], float, [0 x i8], %"core::marker::PhantomData<()>", [0 x i8] }
%"core::marker::PhantomData<()>" = type {}

; Function Attrs: nounwind nonlazybind uwtable
define zeroext i1 @foo(double) unnamed_addr #0 {
start:
  %abi_cast = alloca double, align 8
  %f = alloca { float, float }, align 4
  store double %0, double* %abi_cast, align 8
  %1 = bitcast { float, float }* %f to i8* 
  %2 = bitcast double* %abi_cast to i8* 
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %1, i8* align 8 %2, i64 8, i1 false)
  %3 = bitcast { float, float }* %f to float*
  %4 = load float, float* %3, align 4
  %5 = getelementptr inbounds { float, float }, { float, float }* %f, i32 0, i32 1
  %6 = load float, float* %5, align 4
  %7 = fcmp une float %4, %6
  ret i1 %7
}

; Function Attrs: nounwind nonlazybind uwtable
define zeroext i1 @bar(double) unnamed_addr #0 {
start:
  %abi_cast = alloca double, align 8
  %f = alloca %Bar, align 4
  store double %0, double* %abi_cast, align 8
  %1 = bitcast %Bar* %f to i8* 
  %2 = bitcast double* %abi_cast to i8* 
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %1, i8* align 8 %2, i64 8, i1 false)
  %3 = bitcast %Bar* %f to float*
  %4 = load float, float* %3, align 4
  %5 = getelementptr inbounds %Bar, %Bar* %f, i32 0, i32 3
  %6 = load float, float* %5, align 4
  %7 = fcmp une float %4, %6
  ret i1 %7
}
