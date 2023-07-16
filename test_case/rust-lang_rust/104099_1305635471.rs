
; RUN: llc < %s
target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"
target triple = "aarch64-unknown-linux-gnu"

define void @test(i8 %arg) {
  %arg.ext = zext i8 %arg to i64
  %trunc = trunc i64 %arg.ext to i3
  %switch.tableidx = xor i3 %trunc, 1
  %switch.maskindex = zext i3 %trunc to i8
  %switch.lobit = icmp ne i8 %switch.maskindex, 0
  call void @llvm.assume(i1 %switch.lobit)
  %ext = zext i3 %switch.tableidx to i64
  ret void
}

declare void @llvm.assume(i1)
