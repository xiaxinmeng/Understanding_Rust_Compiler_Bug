 rust
$ rustc foo.rs -O --emit=ir -o - 
; ModuleID = '-.rs'
target datalayout = "e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: nounwind readnone uwtable
define internal void @_ZN4main20hbfcbec594e62aee2eaa4v0.0E() unnamed_addr #0 {
entry-block:
  ret void
}

define i64 @main(i64, i8**) unnamed_addr #1 {
top:
  %2 = tail call i64 @_ZN10lang_start20h53a1e3fe8c23b585vme11v0.11.0.preE(i8* bitcast (void ()* @_ZN4main20hbfcbec594e62aee2eaa4v0.0E to i8*), i64 %0, i8** %1)
  ret i64 %2
}

declare i64 @_ZN10lang_start20h53a1e3fe8c23b585vme11v0.11.0.preE(i8*, i64, i8**) unnamed_addr #1

attributes #0 = { nounwind readnone uwtable "split-stack" }
attributes #1 = { "split-stack" }
