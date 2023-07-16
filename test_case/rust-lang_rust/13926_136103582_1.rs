 llvm
; Function Attrs: nounwind readnone uwtable
define i64 @_ZN3foo20h8ac558f51be88dd53aaE(i8) unnamed_addr #1 {
entry-block:
  %1 = zext i8 %0 to i64
  %2 = getelementptr inbounds [4 x i64], [4 x i64]* @_ZN1X20h5e497aaeb62c0700UaaE, i64 0, i64 %1
  %3 = load i64, i64* %2, align 8, !noalias !1
  ret i64 %3
}
