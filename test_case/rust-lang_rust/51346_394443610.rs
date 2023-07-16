llvm
define i64 @foo(i64 %c) {
bb1:
  %mul = tail call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %c, i64 8)
  %overflow = extractvalue { i64, i1 } %mul, 1
  %select = select i1 %overflow, i64 0, i64 8
  br i1 %overflow, label %abort, label %bb2

bb2:
  call void @dummy(i64 %select)
  ret i64 %select

abort:
  call void @abort()
  unreachable
}
declare void @abort()
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64)
declare void @dummy(i64)

