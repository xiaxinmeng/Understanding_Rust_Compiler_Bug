llvm
define void @_ZN7example11scalar_test17h9448becacb3cda6dE() unnamed_addr #0 {
  br label %bb1

bb1:                                              ; preds = %start
  br i1 false, label %bb2, label %bb3

bb2:                                              ; preds = %bb1
  call void @_ZN3std9panicking11begin_panic17h0260f1c8a33dd8a1E([0 x i8]* noalias nonnull readonly bitcast (<{ [14 x i8] }>* @byte_str.6 to [0 x i8]*), i64 14, { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @byte_str.5 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable

bb3:                                              ; preds = %bb1
  ret void
}
