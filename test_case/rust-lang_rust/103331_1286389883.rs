
  %3 = load i32, ptr %x, align 4, !range !2, !noundef !3
  %_2 = zext i32 %3 to i64
  switch i64 %_2, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb1
  ]

bb2:                                              ; preds = %start
  unreachable
