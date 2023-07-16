
bb3.i.i:                                          ; preds = %bb6, %bb3.i.i
  %count.159 = phi i64 [ %count.0, %bb6 ], [ %6, %bb3.i.i ]
  %iter1.sroa.5.058 = phi i64 [ 100000, %bb6 ], [ %spec.select, %bb3.i.i ]
  %2 = icmp ne i64 %iter1.sroa.5.058, 0
  %3 = xor i1 %2, true
  %4 = zext i1 %3 to i8
  %5 = add i64 %iter1.sroa.5.058, -1
  %spec.select = select i1 %2, i64 %5, i64 0
  %6 = add i64 %count.159, %iter1.sroa.5.058
  %7 = icmp eq i8 %4, 2
  %8 = and i8 %4, 1
  %phitmp.i.i.i = icmp eq i8 %8, 0
  %or.cond = or i1 %7, %phitmp.i.i.i
  br i1 %or.cond, label %bb3.i.i, label %bb2.loopexit
