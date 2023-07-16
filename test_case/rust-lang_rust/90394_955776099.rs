llvm
target datalayout = "e-m:e-p:64:64-i64:64-i128:128-n64-S128"
target triple = "riscv64-unknown-linux-gnu"

define zeroext i1 @_ZN7example4test17h68457115120fac10E([0 x i32]* noalias nonnull readonly align 4 %0, i64 %1) unnamed_addr #0 !dbg !7 {
start:
  %_3.not.i.i.i = icmp eq i64 %1, 1, !dbg !11
  br i1 %_3.not.i.i.i, label %bb2.i.i.i, label %"_ZN4core5array8equality96_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$A$u3b$$u20$N$u5d$$GT$$u20$for$u20$$RF$$u5b$B$u5d$$GT$2eq17h5788217a92cf04faE.exit", !dbg !11

bb2.i.i.i:                                        ; preds = %start
  %2 = bitcast [0 x i32]* %0 to i8*, !dbg !28
  %bcmp.i.i.i = tail call i32 @bcmp(i8* noundef nonnull dereferenceable(4) %2, i8* noundef nonnull dereferenceable(4) getelementptr inbounds (<{ [4 x i8] }>, <{ [4 x i8] }>* @alloc3, i64 0, i32 0, i64 0), i64 4) #2, !dbg !29, !alias.scope !30, !noalias !37
  %3 = icmp eq i32 %bcmp.i.i.i, 0, !dbg !29
  br label %"_ZN4core5array8equality96_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$A$u3b$$u20$N$u5d$$GT$$u20$for$u20$$RF$$u5b$B$u5d$$GT$2eq17h5788217a92cf04faE.exit", !dbg !40

"_ZN4core5array8equality96_$LT$impl$u20$core..cmp..PartialEq$LT$$u5b$A$u3b$$u20$N$u5d$$GT$$u20$for$u20$$RF$$u5b$B$u5d$$GT$2eq17h5788217a92cf04faE.exit": ; preds = %start, %bb2.i.i.i
  %.0.off0.i.i.i = phi i1 [ %3, %bb2.i.i.i ], [ false, %start ]
  ret i1 %.0.off0.i.i.i, !dbg !41
}

declare i32 @bcmp(i8* nocapture, i8* nocapture, i64) local_unnamed_addr #1

[...]
