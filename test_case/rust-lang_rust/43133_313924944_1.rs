llvm
; _1::reverse2
; Function Attrs: norecurse nounwind readnone uwtable
define i8 @_1::reverse2::hbb951fc559ba0b9c(i8) unnamed_addr #0 {
start:
  %not.switch.selectcmp = icmp ne i8 %0, 0
  %switch.select = sext i1 %not.switch.selectcmp to i8
  %switch.selectcmp1 = icmp eq i8 %0, -1
  %switch.select2 = select i1 %switch.selectcmp1, i8 1, i8 %switch.select
  ret i8 %switch.select2
}

; _1::reverse3
; Function Attrs: norecurse nounwind readnone uwtable
define i8 @_1::reverse3::hf325c37bc61a71c6(i8) unnamed_addr #0 {
start:
  %1 = zext i8 %0 to i64
  %switch.tableidx = add nuw nsw i64 %1, 1
  %switch.idx.cast = trunc i64 %switch.tableidx to i8
  %switch.offset = sub i8 1, %switch.idx.cast
  ret i8 %switch.offset
}
