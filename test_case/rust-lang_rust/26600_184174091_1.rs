
FastISel missed terminator:   switch i1 %29, label %match_else [
    i1 true, label %match_case
    i1 false, label %match_case3
  ]
FastISel missed terminator:   ret { i8*, i64 } %20
FastISel missed terminator:   ret { i8*, i64 } %7
FastISel miss:   %24 = icmp eq i1 %23, true
LLVM ERROR: FastISel didn't select the entire block
