ll
define zeroext i1 @is_zero_slice([4 x i8]* noalias readonly align 1 dereferenceable(4) %data) unnamed_addr #0 {
start:
  %0 = icmp eq [4 x i8]* %data, getelementptr inbounds (<{ [4 x i8] }>, <{ [4 x i8] }>* @alloc2, i64 0, i32 0)
  br i1 %0, label %"_ZN4core5array103_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u3b$$u20$N$u5d$$GT$$u20$for$u20$$u5b$A$u3b$$u20$N$u5d$$GT$2eq17h702fd33f1073d678E.exit", label %bb9.i.i.i

bb9.i.i.i:                                        ; preds = %start
  %1 = getelementptr [4 x i8], [4 x i8]* %data, i64 0, i64 0
  %bcmp.i.i.i = tail call i32 @bcmp(i8* nonnull dereferenceable(4) %1, i8* nonnull dereferenceable(4) getelementptr inbounds (<{ [4 x i8] }>, <{ [4 x i8] }>* @alloc2, i64 0, i32 0, i64 0), i64 4) #2
  %2 = icmp eq i32 %bcmp.i.i.i, 0
  br label %"_ZN4core5array103_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u3b$$u20$N$u5d$$GT$$u20$for$u20$$u5b$A$u3b$$u20$N$u5d$$GT$2eq17h702fd33f1073d678E.exit"

"_ZN4core5array103_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$B$u3b$$u20$N$u5d$$GT$$u20$for$u20$$u5b$A$u3b$$u20$N$u5d$$GT$2eq17h702fd33f1073d678E.exit": ; preds = %start, %bb9.i.i.i
  %.0.i.i.i = phi i1 [ %2, %bb9.i.i.i ], [ true, %start ]
  ret i1 %.0.i.i.i
}
