ll
define zeroext i1 @is_zero_slice([4 x i8]* noalias nocapture readonly align 1 dereferenceable(4) %data) unnamed_addr #0 {
start:
  %0 = getelementptr [4 x i8], [4 x i8]* %data, i64 0, i64 0
  %bcmp.i.i.i = tail call i32 @bcmp(i8* nonnull dereferenceable(4) %0, i8* nonnull dereferenceable(4) getelementptr inbounds (<{ [4 x i8] }>, <{ [4 x i8] }>* @alloc2, i64 0, i32 0, i64 0), i64 4) #2
  %1 = icmp eq i32 %bcmp.i.i.i, 0
  ret i1 %1
}
