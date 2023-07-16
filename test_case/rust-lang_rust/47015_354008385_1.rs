
$ opt output.ll -s -O2 
; ModuleID = '<stdin>'
source_filename = "a.ll"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: norecurse nounwind readnone
define i64 @foo() local_unnamed_addr #0 {
start:
  ret i64 9246255105
}

attributes #0 = { norecurse nounwind readnone "target-cpu"="haswell" }
