llvm
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%VecU8 = type { [0 x i64], { i8*, i64 }, [0 x i64], i64, [0 x i64] }

define i64 @_ZN7example1a17haca38bae68eb7066E() {
  %v2 = alloca %VecU8, align 8
  %1 = bitcast %VecU8* %v2 to i8**
  %2 = getelementptr inbounds %VecU8, %VecU8* %v2, i64 0, i32 1, i32 1
  %3 = bitcast i64* %2 to i8*
  
  ; `buf` is a 16-bytes, "zero-initialized" by a memset, array
  %buf = alloca [16 x i8], align 8
  %buf_ptr = getelementptr inbounds [16 x i8], [16 x i8]* %buf, i64 0, i64 0
  call void @llvm.memset.p0i8.i64(i8* nonnull align 8 %buf_ptr, i8 0, i64 16, i1 false)
  
  store i8* inttoptr (i64 1 to i8*), i8** %1, align 8
  
  ; "zero-initializes" the two i64 (skipping the i8*) of %v2 through %2 by copying from %buf which is "known" to be zero
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 8 %3, i8* nonnull align 8 %buf_ptr, i64 16, i1 false)
  
  ; llvm can't prove that the i64 at %v2.idx has been zero-initialized by the memcpy
  %v2.idx = getelementptr inbounds %VecU8, %VecU8* %v2, i64 0, i32 3
  %v2.idx.val = load i64, i64* %v2.idx, align 8
  
  ret i64 %v2.idx.val
}

declare void @llvm.memcpy.p0i8.p0i8.i64(i8* nocapture writeonly, i8* nocapture readonly, i64, i1)
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1)
