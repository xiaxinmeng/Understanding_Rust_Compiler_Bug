
nnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %data = alloca [2 x i8], align 2
  store i16 %0, ptr %data, align 2
  %1 = getelementptr inbounds i8, ptr %data, i64 1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19) #6
  tail call void @llvm.experimental.noalias.scope.decl(metadata !22) #6
  br label %bb6.i.i

bb6.i.i:                                          ; preds = %start
  %iter.sroa.0.07.i.i = phi i64 [ 0, %start ]
  %_40.i.i = sub nsw i64 0, %iter.sroa.0.07.i.i
  %2 = add nuw nsw i64 %iter.sroa.0.07.i.i, 1
  %_34.i.i = getelementptr inbounds [0 x i8], ptr %data, i64 0, i64 %iter.sroa.0.07.i.i
  %_39.i.i = getelementptr inbounds [0 x i8], ptr %1, i64 0, i64 %_40.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24) #6
  tail call void @llvm.experimental.noalias.scope.decl(metadata !27) #6
  tail call void @llvm.experimental.noalias.scope.decl(metadata !29) #6
  tail call void @llvm.experimental.noalias.scope.decl(metadata !32) #6
  %tmp.0.copyload.i.i.i.i = load i8, ptr %_34.i.i, align 1, !alias.scope !34, !noalias !37
  %tmp2.0.copyload.i.i.i.i = load i8, ptr %_39.i.i, align 1, !alias.scope !38, !noalias !39
  store i8 %tmp2.0.copyload.i.i.i.i, ptr %_34.i.i, align 1, !alias.scope !34, !noalias !37
  store i8 %tmp.0.copyload.i.i.i.i, ptr %_39.i.i, align 1, !alias.scope !38, !noalias !39
  br label %"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$7reverse17h78a536b0d0fa8d9eE.exit"

"_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$7reverse17h78a536b0d0fa8d9eE.exit": ; preds = %bb6.i.i
  %.sroa.0.0.copyload = load i16, ptr %data, align 2
  ret i16 %.sroa.0.0.copyload
}
