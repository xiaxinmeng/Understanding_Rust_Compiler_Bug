
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-linux-gnu"

declare void @dummy(i8 zeroext)

define void @test() {
  br label %1

1:
  %2 = phi i8 [ 0, %0 ], [ %4, %1 ]
  call void @dummy(i8 %2)

  %3 = icmp eq i8 %2, -1
  %4 = add nuw i8 %2, 1
  br i1 %3, label %5, label %1

5:
  ret void
}
