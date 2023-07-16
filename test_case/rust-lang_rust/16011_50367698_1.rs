 llvm
; ModuleID = 'test.no-opt.bc'
target datalayout = "e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

define void @test(i1 %c) {
entry-block:
  %x = alloca i8
  br label %loop_body

loop_body:                                        ; preds = %loop_body, %entry-block
  call void @llvm.lifetime.start(i64 8, i8* %x)
  br label %loop_body
}

; Function Attrs: nounwind
declare void @llvm.lifetime.start(i64, i8* nocapture) unnamed_addr #0

; Function Attrs: nounwind
declare void @llvm.lifetime.end(i64, i8* nocapture) unnamed_addr #0

attributes #0 = { nounwind }
