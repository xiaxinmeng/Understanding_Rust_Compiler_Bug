
target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"
target triple = "wasm32-unknown-unknown"

declare i128 @foo(i128)

define void @test(i1 %b) {
start:
  %zext = zext i1 %b to i128
  br label %next

next:                                             ; preds = %start
  %ret = call i128 @foo(i128 %zext)
  ret void 
} 
