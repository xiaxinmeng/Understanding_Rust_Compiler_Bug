 llvm
define void @_ZN3ffi3bla17h4acc14686285caf0E() unnamed_addr #0 {
entry-block:
  %temp0 = alloca {}
  %temp1 = alloca %S
  %arg = alloca %S
  br label %start

start:                                            ; preds = %entry-block
  %0 = getelementptr inbounds %S, %S* %temp1, i32 0, i32 0
  store i8 4, i8* %0
  %1 = getelementptr inbounds %S, %S* %temp1, i32 0, i32 1
  store i8 8, i8* %1
  %2 = load %S, %S* %temp1
  store %S %2, %S* %arg
  %3 = bitcast %S* %arg to { i64 }*
  %4 = load { i64 }, { i64 }* %3, align 1
  call void @foo({ i64 } %4)
  br label %bb2

end:                                              ; preds = %bb3
  ret void

bb2:                                              ; preds = %start
  br label %bb3

bb3:                                              ; preds = %bb2
  br label %end
}
