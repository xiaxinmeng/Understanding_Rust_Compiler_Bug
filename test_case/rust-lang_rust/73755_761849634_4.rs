
; ModuleID = 'test.3a1fbbbh-cgu.0'
source_filename = "test.3a1fbbbh-cgu.0"
target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"
target triple = "wasm32-unknown-unknown"

; Function Attrs: norecurse nounwind readnone
define { i32, i32 } @greet(i32 %x, i32 %y) unnamed_addr #0 {
start:
  %_3 = add i32 %y, %x
  %_6 = sub i32 %x, %y
  %0 = insertvalue { i32, i32 } undef, i32 %_3, 0
  %1 = insertvalue { i32, i32 } %0, i32 %_6, 1
  ret { i32, i32 } %1
}

attributes #0 = { norecurse nounwind readnone "target-cpu"="generic" "target-features"="+multivalue" }
