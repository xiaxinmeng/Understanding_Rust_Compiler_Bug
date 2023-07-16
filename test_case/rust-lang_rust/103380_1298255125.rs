
target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"
target triple = "aarch64-unknown-linux-gnu"

define void @test(ptr %start, ptr %end) {
entry:
  br label %loop

loop:
  %ptr = phi ptr [ %start, %entry ], [ %ptr.next, %loop.latch ]
  %cmp = icmp eq ptr %ptr, %end
  br i1 %cmp, label %exit, label %loop.latch

loop.latch:
  %ptr.next = getelementptr inbounds i64, ptr %ptr, i64 1
  %v1 = load i32, ptr %ptr, align 4
  %gep = getelementptr inbounds i8, ptr %ptr, i64 4
  %v2 = load i24, ptr %gep, align 4
  br label %loop

exit:
  ret void
}
