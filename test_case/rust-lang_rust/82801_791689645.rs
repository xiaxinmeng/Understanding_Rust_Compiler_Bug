
  %10 = bitcast i64* %4 to <2 x i64>*
  store <2 x i64> <i64 3, i64 2>, <2 x i64>* %10, align 8
  %_6.idx.val.i.2 = load i64, i64* %4, align 8
  %_3.i.2 = icmp eq i64 %_6.idx.val.i.2, 2
