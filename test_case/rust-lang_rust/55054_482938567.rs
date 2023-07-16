
declare i64 @llvm.cttz.i64(i64, i1)

define i32 @test(i64 %x) {
start:
  %c = icmp eq i64 %x, 0
  br i1 %c, label %exit, label %non_zero

non_zero:
  %ctz = call i64 @llvm.cttz.i64(i64 %x, i1 false)
  %ctz32 = trunc i64 %ctz to i32
  br label %exit

exit:
  %res = phi i32 [ %ctz32, %non_zero ], [ 0, %start ]
  ret i32 %res
}
