 llvm
; Function Attrs: nounwind readnone uwtable
define zeroext i1 @_ZN3foo20h8ac12d4c0db8e3ccdaaE({ i8, i8 }* nocapture readonly) unnamed_addr #0 {
match_case5.i:
  %1 = bitcast { i8, i8 }* %0 to i16*
  %v1 = load i16* %1, align 2
  %2 = icmp eq i16 %v1, 513
  ret i1 %2
}
