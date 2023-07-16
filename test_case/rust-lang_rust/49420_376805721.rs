llvm
; bar::foo
; Function Attrs: norecurse nounwind readnone uwtable
define { i32, i32 } @_ZN3bar3foo17h361da50dc86c3b65E(i32, i32) unnamed_addr #0 {
start:
  %switch = icmp eq i32 %0, 0
  %. = zext i1 %switch to i32
  %2 = insertvalue { i32, i32 } undef, i32 %., 0
  %3 = insertvalue { i32, i32 } %2, i32 %1, 1
  ret { i32, i32 } %3
}

; bar::bar
; Function Attrs: norecurse nounwind readnone uwtable
define { i32, i32 } @_ZN3bar3bar17h01d34a35402b7c22E(i32 %x.0, i32 %x.1) unnamed_addr #0 {
start:
  %switch.i = icmp ne i32 %x.0, 0
  %. = zext i1 %switch.i to i32
  %0 = insertvalue { i32, i32 } undef, i32 %., 0
  %1 = insertvalue { i32, i32 } %0, i32 %x.1, 1
  ret { i32, i32 } %1
}
