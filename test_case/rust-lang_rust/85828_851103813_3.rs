llvm
  %x = alloca i128, align 8
  store i128 %0, i128* %x, align 8
  %_11.i.i.i = bitcast i128* %x to i8*
  %bcmp.i.i.i = call i32 @bcmp(i8* nonnull dereferenceable(16) %_11.i.i.i, i8* nonnull dereferenceable(16) getelementptr inbounds (<{ [16 x i8] }>, <{ [16 x i8] }>* @alloc2, i64 0, i32 0, i64 0), i64 16) #2, !alias.scope !2
  %1 = icmp eq i32 %bcmp.i.i.i, 0
  ret i1 %1
