
define void @test(ptr %arr.base, i64 %arr.size, i32 %arg) {
start:
  %arg.min = tail call i32 @llvm.umin.i32(i32 %arg, i32 16777216)
  %end = getelementptr inbounds i8, ptr %arr.base, i64 %arr.size
  br label %loop

loop:
  %ptr = phi ptr [ %arr.base, %start ], [ %ptr.next, %loop.latch ]
  %cmp = icmp eq ptr %ptr, %end
  br i1 %cmp, label %exit, label %loop.latch

loop.latch:
  %ptr.next = getelementptr inbounds i8, ptr %ptr, i64 1
  %v = load i8, ptr %ptr, align 1
  %v.ext = zext i8 %v to i32
  %mulo = tail call { i32, i1 } @llvm.umul.with.overflow.i32(i32 %v.ext, i32 %arg.min)
  %ov = extractvalue { i32, i1 } %mulo, 1
  br i1 %ov, label %panic, label %loop

exit:
  ret void

panic:
  call void @panic()
  unreachable
}

declare void @panic()
declare { i32, i1 } @llvm.umul.with.overflow.i32(i32, i32)
declare i32 @llvm.umin.i32(i32, i32)
