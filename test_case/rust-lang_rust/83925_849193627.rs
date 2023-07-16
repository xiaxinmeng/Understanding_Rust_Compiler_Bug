
; ModuleID = 'example.bxq6xejs-cgu.0'
source_filename = "example.bxq6xejs-cgu.0"
target datalayout = "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"
target triple = "thumbv7em-none-unknown-eabi"

; Function Attrs: minsize norecurse noreturn nounwind optsize readnone
define void @_start() unnamed_addr #0 {
bb1:
  br label %bb2

bb2:                                              ; preds = %bb2, %bb1
  br label %bb2
}

attributes #0 = { minsize norecurse noreturn nounwind optsize readnone "frame-pointer"="all" "target-cpu"="generic" }
