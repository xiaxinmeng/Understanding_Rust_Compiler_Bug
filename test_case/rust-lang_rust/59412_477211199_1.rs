 llvm
define void @main() unnamed_addr #3 !dbg !1240, !fn !0 {
; ..
  %4 = tail call i32 %3() #9, !dbg !1254, !fn !1
; ..                                      ^^^^^^
}

!0 = "fn()"
!1 = "fn() -> u32" ; tool needs to compare this string against the output of `-Z print-reified-functions`
