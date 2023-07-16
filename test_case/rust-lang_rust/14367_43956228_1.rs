 llvm
  %1 = call i64 @_ZN15precise_time_ns20h53df8191405a33f50pa11v0.11.0.preE()
  %2 = call fastcc i64 @_ZN3fib20hc787ae08bc88b138iaa4v0.0E(i64 20) #3
  %3 = bitcast i64* %dummy.i.i to i8*
  br label %match_else.i

match_else.i:                                     ; preds = %match_else.i, %entry-block
  %4 = phi i64 [ 0, %entry-block ], [ %5, %match_else.i ]
  %5 = add i64 %4, 1
  call void @llvm.lifetime.start(i64 8, i8* %3) #3
  store i64 %2, i64* %dummy.i.i, align 8
  call void asm "", "r,~{dirflag},~{fpsr},~{flags}"(i64* %dummy.i.i) #3
  call void @llvm.lifetime.end(i64 8, i8* %3) #3
  %exitcond.i = icmp eq i64 %5, 1000
  br i1 %exitcond.i, label %_ZN5bench20hd83cb69b9731384fJaa4v0.0E.exit, label %match_else.i

_ZN5bench20hd83cb69b9731384fJaa4v0.0E.exit:       ; preds = %match_else.i
  %6 = call i64 @_ZN15precise_time_ns20h53df8191405a33f50pa11v0.11.0.preE()
