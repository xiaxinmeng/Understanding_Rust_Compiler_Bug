
define i64 @_ZN4test12min_selector17hce2ef68927145e7fE(i64 %i) unnamed_addr #0 {
start:
  %0 = tail call i64 @llvm.ctlz.i64(i64 %i, i1 false) #2, !range !2
  %1 = getelementptr inbounds [65 x i64], [65 x i64]* bitcast (<{ [520 x i8] }>* @0 to [65 x i64]*), i64 0, i64 %0
  %2 = load i64, i64* %1, align 8
  ret i64 %2
}
