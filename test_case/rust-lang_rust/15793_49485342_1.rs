 llvm
match_else:                                       ; preds = %entry-block
  %cond10 = icmp eq i8 %.fca.2.0.extract12, 1
  %. = select i1 %cond10, i64 3, i64 5
  br label %join
