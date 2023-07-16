llvm
  %_3 = alloca %S
  %s = alloca %S
  %0 = getelementptr inbounds %S, %S* %s, i32 0, i32 0
  store i32 1, i32* %0
  %1 = getelementptr inbounds %S, %S* %s, i32 0, i32 2
  store i32 2, i32* %1
  %2 = getelementptr inbounds %S, %S* %s, i32 0, i32 4
  store i32 3, i32* %2
  %3 = bitcast %S* %s to i8*
  %4 = bitcast %S* %_3 to i8*
  ; Copy 12 bytes with alignment of 4
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %4, i8* %3, i64 12, i32 4, i1 false)
  %5 = bitcast %S* %_3 to { i64, i32 }*
  ; Load 12 bytes -- this is now fine, not loading 16 bytes
  %6 = load { i64, i32 }, { i64, i32 }* %5, align 4
  call void @foo({ i64, i32 } %6)
  br label %bb1
