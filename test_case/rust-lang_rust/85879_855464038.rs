
; testlib::actual_lz
; Function Attrs: nounwind readnone willreturn
define dso_local i32 @_ZN7testlib9actual_lz17hff1ee67f9ad9af94E(i32 %x) unnamed_addr #0 {
start:
  %0 = tail call i32 @llvm.ctlz.i32(i32 %x, i1 false) #3, !range !1
  ret i32 %0
}
