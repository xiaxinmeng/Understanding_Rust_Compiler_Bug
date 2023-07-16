
  %1 = icmp eq i64 %0, 0
  br i1 %1, label %bb4, label %bb1

bb1:
  %2 = icmp ult i64 %0, 4
