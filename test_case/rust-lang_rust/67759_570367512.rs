
bb10:                                             ; preds = %bb3.i.i, %bb6
  %iter1.sroa.5.0 = phi i64 [ 100000, %bb6 ], [ %spec.select, %bb3.i.i ]
  %iter1.sroa.9.0 = phi i8 [ 2, %bb6 ], [ %6, %bb3.i.i ]
  %count.1 = phi i64 [ %count.0, %bb6 ], [ %8, %bb3.i.i ]
  %2 = icmp eq i8 %iter1.sroa.9.0, 2
  %3 = and i8 %iter1.sroa.9.0, 1
  %phitmp.i.i.i = icmp eq i8 %3, 0
  %or.cond = or i1 %2, %phitmp.i.i.i
  br i1 %or.cond, label %bb3.i.i, label %bb2.loopexit

bb3.i.i:                                          ; preds = %bb10
  %4 = icmp ne i64 %iter1.sroa.5.0, 0
  %5 = xor i1 %4, true
  %6 = zext i1 %5 to i8
  %7 = add i64 %iter1.sroa.5.0, -1
  %spec.select = select i1 %4, i64 %7, i64 0
  %8 = add i64 %count.1, %iter1.sroa.5.0
  br label %bb10
