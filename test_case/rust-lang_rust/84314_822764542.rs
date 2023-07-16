
  %0 = icmp ult i64 %data.1, 2
  br i1 %0, label %bb6, label %bb3

bb3:                                              ; preds = %start
  %_4 = lshr i64 %data.1, 1
  %_12 = add nsw i64 %_4, -1
  %_16 = icmp ult i64 %_12, %data.1
  br i1 %_16, label %bb5, label %panic1, !prof !2
