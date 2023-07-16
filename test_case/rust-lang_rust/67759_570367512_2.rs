
bb10:                                             ; preds = %bb3.i.i, %bb6
  %iter1.sroa.5.0 = phi i64 [ 100000, %bb6 ], [ %spec.select, %bb3.i.i ]
  %iter1.sroa.9.0 = phi i8 [ 2, %bb6 ], [ %3, %bb3.i.i ]
  %count.1 = phi i64 [ %count.0, %bb6 ], [ %4, %bb3.i.i ]
  switch i8 %iter1.sroa.9.0, label %bb2.loopexit [
    i8 2, label %bb3.i.i
    i8 0, label %bb3.i.i
  ]

bb3.i.i:                                          ; preds = %bb10, %bb10
  %2 = icmp ne i64 %iter1.sroa.5.0, 0
  %_10.i.i = xor i1 %2, true
  %3 = zext i1 %_10.i.i to i8
  %_5.0.i.i.i.i = add i64 %iter1.sroa.5.0, -1
  %spec.select = select i1 %2, i64 %_5.0.i.i.i.i, i64 0
  %4 = add i64 %count.1, %iter1.sroa.5.0
  br label %bb10
