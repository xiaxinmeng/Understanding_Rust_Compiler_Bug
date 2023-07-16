
; RUN: opt -passes='loop(indvars),loop-vectorize' < %s
target triple = "x86_64-unknown-linux-gnu"

define void @test(i64* %ptr.base) {
entry:
  br label %loop1

loop1:
  br i1 true, label %loop1.latch, label %loop2.preheader

loop2.preheader:
  br label %loop2

loop2:
  %iv = phi i64 [ 0, %loop2.preheader ], [ %iv.next, %loop2 ]
  %ptr = phi i64* [ %ptr.base, %loop2.preheader ], [ %ptr.next, %loop2 ]
  %iv.next = add nuw nsw i64 %iv, 1
  %ptr.next = getelementptr inbounds i64, i64* %ptr, i64 1
  %cmp = icmp eq i64 %iv, 1024
  br i1 %cmp, label %loop2.exit, label %loop2

loop2.exit:
  %ptr.next.lcssa = phi i64* [ %ptr.next, %loop2 ]
  br label %loop1.latch

loop1.latch:
  br label %loop1
}
