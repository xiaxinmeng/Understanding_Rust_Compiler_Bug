llvm
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

define i64 @test(ptr %arr) {
entry:
  br label %loop

loop:
  %accum = phi i64 [ %accum.next, %loop ], [ 0, %entry ]
  %iv = phi i64 [ %iv.next, %loop ], [ 0, %entry ]
  %iv.next = add nuw i64 %iv, 1
  %gep = getelementptr inbounds i64, ptr %arr, i64 %iv
  %value = load i64, ptr %gep, align 8
  %ctpop = tail call i64 @llvm.ctpop.i64(i64 %value)
  %accum.next = add i64 %accum, %ctpop
  %exitcond = icmp eq i64 %iv.next, 2
  br i1 %exitcond, label %exit, label %loop

exit:
  %lcssa = phi i64 [ %accum.next, %loop ]
  ret i64 %lcssa
}

declare i64 @llvm.ctpop.i64(i64)
