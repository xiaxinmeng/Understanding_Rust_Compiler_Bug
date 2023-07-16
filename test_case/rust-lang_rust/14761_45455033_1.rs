 llvm
; ModuleID = '14761.rs'
target datalayout = "e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@const = private constant [5 x i8] c"TEXT!"

; Function Attrs: nounwind readnone
define i64 @main(i64, i8** nocapture readnone) unnamed_addr #0 {
top:
  ret i64 ptrtoint ([5 x i8]* @const to i64)
}

attributes #0 = { nounwind readnone "split-stack" }
