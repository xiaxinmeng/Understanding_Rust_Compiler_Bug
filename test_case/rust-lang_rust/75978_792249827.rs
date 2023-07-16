
; Preheader:
start:
  %s = alloca [7 x i32], align 4
  %0 = bitcast [7 x i32]* %s to i8*
  call void @llvm.lifetime.start.p0i8(i64 28, i8* nonnull %0)
  %1 = getelementptr inbounds [7 x i32], [7 x i32]* %s, i64 0, i64 0
  store i32 1, i32* %1, align 4
  %2 = getelementptr inbounds [7 x i32], [7 x i32]* %s, i64 0, i64 1
  store i32 2, i32* %2, align 4 
  %3 = getelementptr inbounds [7 x i32], [7 x i32]* %s, i64 0, i64 2
  store i32 3, i32* %3, align 4
  %4 = getelementptr inbounds [7 x i32], [7 x i32]* %s, i64 0, i64 3
  store i32 4, i32* %4, align 4
  %5 = getelementptr inbounds [7 x i32], [7 x i32]* %s, i64 0, i64 4
  store i32 5, i32* %5, align 4
  %6 = getelementptr inbounds [7 x i32], [7 x i32]* %s, i64 0, i64 5
  store i32 6, i32* %6, align 4
  %7 = getelementptr inbounds [7 x i32], [7 x i32]* %s, i64 0, i64 6
  store i32 7, i32* %7, align 4
  %8 = getelementptr inbounds [7 x i32], [7 x i32]* %s, i64 0, i64 0
  %9 = getelementptr inbounds [7 x i32], [7 x i32]* %s, i64 0, i64 7
  %10 = getelementptr inbounds [7 x i32], [7 x i32]* %s, i64 0, i64 1
  br label %bb5

; Loop:
bb5:                                              ; preds = %start, %"_ZN4core6option15Option$LT$T$GT$6map_or17h8c48b1da3da3943fE.exit" 
  %11 = phi i32* [ %10, %start ], [ %14, %"_ZN4core6option15Option$LT$T$GT$6map_or17h8c48b1da3da3943fE.exit" ]
  %sum.020 = phi i32 [ 0, %start ], [ %13, %"_ZN4core6option15Option$LT$T$GT$6map_or17h8c48b1da3da3943fE.exit" ]
  %iter.sroa.0.019 = phi i32* [ %8, %start ], [ %iter.sroa.0.218, %"_ZN4core6option15Option$LT$T$GT$6map_or17h8c48b1da3da3943fE.exit" ]
  %_12.i = icmp eq i32* %11, %9
  br i1 %_12.i, label %"_ZN4core6option15Option$LT$T$GT$6map_or17h8c48b1da3da3943fE.exit", label %bb3.i

bb3.i:                                            ; preds = %bb5
  %12 = getelementptr inbounds i32, i32* %iter.sroa.0.019, i64 2
  %.val.i = load i32, i32* %11, align 4, !alias.scope !2
  br label %"_ZN4core6option15Option$LT$T$GT$6map_or17h8c48b1da3da3943fE.exit"

"_ZN4core6option15Option$LT$T$GT$6map_or17h8c48b1da3da3943fE.exit": ; preds = %bb5, %bb3.i
  %iter.sroa.0.218 = phi i32* [ %12, %bb3.i ], [ %11, %bb5 ]
  %.0.i = phi i32 [ %.val.i, %bb3.i ], [ 1, %bb5 ]
  %13 = add i32 %.0.i, %sum.020
  %_12.i7 = icmp eq i32* %iter.sroa.0.218, %9
  %14 = getelementptr inbounds i32, i32* %iter.sroa.0.218, i64 1
  br i1 %_12.i7, label %bb4, label %bb5

; Exit blocks
bb4:                                              ; preds = %"_ZN4core6option15Option$LT$T$GT$6map_or17h8c48b1da3da3943fE.exit"
  %.lcssa = phi i32 [ %13, %"_ZN4core6option15Option$LT$T$GT$6map_or17h8c48b1da3da3943fE.exit" ] 
  call void @llvm.lifetime.end.p0i8(i64 28, i8* nonnull %0)
  ret i32 %.lcssa
