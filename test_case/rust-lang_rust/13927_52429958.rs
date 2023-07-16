 llvm
; Function Attrs: nounwind readnone uwtable
define zeroext i1 @_ZN3foo20h8ac12d4c0db8e3ccdaaE({ i8, i8 }*) unnamed_addr #0 {
match_case5.i:
  %p1 = getelementptr inbounds { i8, i8 }* %0, i64 0, i32 0
  %p2 = getelementptr inbounds { i8, i8 }* %0, i64 0, i32 1
  %v1 = load i8* %p1, align 2 ; Just for demonstration purposes, it's not actually aligned like that
  %v2 = load i8* %p2
  %cond.i = icmp eq i8 %v1, 1
  %1 = icmp eq i8 %v2, 2
  %sret_slot.0.i = and i1 %cond.i, %1
  ret i1 %sret_slot.0.i
}
