
define i64 @test(i64* %p, i64 %n) nounwind {
  %x = load i64, i64* %p
  %y = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %x, i64 %n)
  %z = extractvalue { i64, i1 } %y, 1
  br i1 %z, label %bb3, label %bb1

bb1:
  %a = extractvalue { i64, i1 } %y, 0
  %b = icmp ult i64 %a, %x
  br i1 %b, label %bb3, label %bb2

bb2:
  store i64 %a, i64* %p
  ret i64 0

bb3:
  ret i64 1
}

declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64)
