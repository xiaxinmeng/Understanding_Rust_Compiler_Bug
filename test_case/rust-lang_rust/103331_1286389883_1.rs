
  %3 = load i32, ptr %x, align 4, !range !2, !noundef !3
  %_2 = zext i32 %3 to i64
  %4 = icmp eq i64 %_2, 0
  br i1 %4, label %bb3, label %bb1
