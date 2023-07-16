 llvm
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

%struct.S = type { i64, i64 }

@take.s = internal constant %struct.S { i64 1, i64 0 }

define internal void @swap(%struct.S* noalias %x, %struct.S* noalias %y) {
  %tmp = alloca %struct.S
  %1 = bitcast %struct.S* %tmp to i8*
  %2 = bitcast %struct.S* %x to i8*
  %3 = bitcast %struct.S* %y to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %1, i8* %2, i64 16, i32 8, i1 false)
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %2, i8* %3, i64 16, i32 8, i1 false)
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %3, i8* %1, i64 16, i32 8, i1 false)
  ret void
}

; Function Attrs: nounwind
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* nocapture, i8* nocapture readonly, i64, i32, i1) #0

define internal void @replace(%struct.S* noalias sret %agg.result, %struct.S* noalias %x, %struct.S* noalias %y) {
  call void @swap(%struct.S* %x, %struct.S* %y)
  %1 = bitcast %struct.S* %agg.result to i8*
  %2 = bitcast %struct.S* %y to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %1, i8* %2, i64 16, i32 8, i1 false)
  ret void
}

define internal void @take(%struct.S* noalias sret %agg.result, %struct.S* noalias %x) {
  %arg = alloca %struct.S
  %1 = bitcast %struct.S* %arg to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %1, i8* bitcast (%struct.S* @take.s to i8*), i64 16, i32 8, i1 false)
  call void @replace(%struct.S* sret %agg.result, %struct.S* %x, %struct.S* %arg)
  ret void
}

define void @take2(%struct.S* noalias sret %agg.result, %struct.S* noalias %x) {
  call void @take(%struct.S* sret %agg.result, %struct.S* %x)
  ret void
}

attributes #0 = { nounwind }
