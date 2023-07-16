
target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

define i32 @test(i8* %p, i8* %p2) {
  %v = load i8, i8* %p
  %v.ext = zext i8 %v to i32
  %cond = icmp eq i32 %v.ext, 0
  %shl = shl i8 %v, 0
  store i8 %shl, i8* %p2
  br label %bb2

bb2:
  br i1 %cond, label %bb3, label %bb4

bb4:
  ret i32 0

bb3:
  ret i32 1
}
