
; testlib::actual_lz
; Function Attrs: nounwind readnone uwtable willreturn
define i64 @_ZN7testlib9actual_lz17h9c7db4bb56dbe73dE(i64 %x) unnamed_addr #0 {
start:
  %0 = tail call i64 @llvm.ctlz.i64(i64 %x, i1 false) #3, !range !2
  ret i64 %0
}
