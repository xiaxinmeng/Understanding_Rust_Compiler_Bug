
define i8 @test_offset(i8* %base) #0 {
entry:
  %z = alloca [128 x i8], align 16
  %gep0 = getelementptr inbounds i8, i8* %base, i64 7
  store volatile i8 0, i8* %gep0
  %gep1 = getelementptr inbounds i8, i8* %base, i64 5
  %bc1 = bitcast i8* %gep1 to i16*
  store volatile i16 0, i16* %bc1
  %gep2 = getelementptr inbounds i8, i8* %base, i64 1
  %bc2 = bitcast i8* %gep2 to i32*
  store volatile i32 0, i32* %bc2

  %y1 = getelementptr inbounds i8, i8* %base, i64 -4
  %y2 = bitcast [128 x i8]* %z to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %y2, i8* %y1, i64 16, i1 false)

  %gep4 = getelementptr inbounds [128 x i8], [128 x i8]* %z, i64 0, i64 4
  %ret = load i8, i8* %gep4
  ret i8 %ret
}

; Function Attrs: argmemonly nounwind
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* nocapture writeonly, i8* nocapture readonly, i64, i1) #1

attributes #0 = { "target-cpu"="core-avx2" }
