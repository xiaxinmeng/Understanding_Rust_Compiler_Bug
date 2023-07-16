
  %.sroa.4.0.extract.shift = lshr i24 %0, 8
  %trunc = trunc i24 %0 to i8
  switch i8 %trunc, label %bb2 [
    i8 0, label %bb3
    i8 1, label %bb5
    i8 2, label %bb6
    i8 3, label %bb7
    i8 4, label %bb8
    i8 5, label %bb26
    i8 6, label %bb16
    i8 7, label %bb17
    i8 8, label %bb18
    i8 9, label %bb19
    i8 10, label %bb1
  ]
