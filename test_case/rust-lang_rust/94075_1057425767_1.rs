
  %.sroa.0.0.extract.trunc = trunc i16 %0 to i8
  %.sroa.4.0.extract.shift = lshr i16 %0, 8
  %1 = add i8 %.sroa.0.0.extract.trunc, -2
  %2 = icmp ult i8 %1, 10
  %3 = zext i8 %1 to i64
  %4 = add nuw nsw i64 %3, 1
  %_2 = select i1 %2, i64 %4, i64 0
  switch i64 %_2, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb5
    i64 2, label %bb6
    i64 3, label %bb7
    i64 4, label %bb8
    i64 5, label %bb26
    i64 6, label %bb16
    i64 7, label %bb17
    i64 8, label %bb18                            
    i64 9, label %bb19
    i64 10, label %bb1
  ]
