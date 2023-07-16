llvm
; ModuleID = 'a.ll'
source_filename = "fls.4e5c6cbe-cgu.3"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: nounwind nonlazybind
define hidden i32 @f() unnamed_addr #0 {
  %1 = call { [0 x i32]*, i64 } bitcast ({ [0 x i64]*, i64 } ()* @g to { [0 x i32]*, i64 } ()*)() #0
  ret i32 0
}

; Function Attrs: nounwind nonlazybind
define hidden { [0 x i64]*, i64 } @g() unnamed_addr #0 {
  ret { [0 x i64]*, i64 } undef
}

attributes #0 = { nounwind nonlazybind "target-cpu"="x86-64" }
