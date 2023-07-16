 llvm
define i64 @_ZN3foo20h043091a9d9208cb0saa4v0.0E(i8* nocapture readonly) unnamed_addr #0 {
entry-block:
  %1 = load i8* %0, align 1, !range !0
  %2 = zext i8 %1 to i64
  %3 = getelementptr inbounds [4 x i64]* @_ZN1X20hcca41332b11ffec7iaa4v0.0E, i64 0, i64 %2
  %4 = load i64* %3, align 8
  ret i64 %4
}

!0 = metadata !{i8 0, i8 4}
