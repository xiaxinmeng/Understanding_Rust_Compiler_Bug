llvm
; Function Attrs: naked nounwind uwtable
define void @naked(i64) unnamed_addr #0 {
start:
  call void asm inteldialect "ret", "~{dirflag},~{fpsr},~{flags}"(), !srcloc !0
  ret void
}

; Function Attrs: nounwind uwtable
define void @non_naked(i64) unnamed_addr #1 {
start:
  ret void
}
