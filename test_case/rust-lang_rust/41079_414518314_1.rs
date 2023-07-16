llvm
; playground::demo
; Function Attrs: norecurse nounwind readnone uwtable
define i64 @_ZN10playground4demo17h3547ff97a5dad82dE(i8* %x, i8* %y) unnamed_addr #0 {
start:
  %0 = ptrtoint i8* %x to i64
  %1 = ptrtoint i8* %y to i64
  %2 = sub i64 %0, %1
  ret i64 %2
}
