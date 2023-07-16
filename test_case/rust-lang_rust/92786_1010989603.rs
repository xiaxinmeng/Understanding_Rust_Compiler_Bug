
target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"
target triple = "aarch64-unknown-linux-gnu"

define void @test(i64* %p1, float* %p2, i32* %p3) {
  br label %loop

loop:
  %val = load i64, i64* %p1
  %test = uitofp i64 %val to float
  store float %test, float* %p2
  %trunc = trunc i64 %val to i32
  store i32 %trunc, i32* %p3
  br label %loop
}
