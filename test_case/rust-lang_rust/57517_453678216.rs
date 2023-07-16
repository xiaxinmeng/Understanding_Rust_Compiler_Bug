
bb4: ; preds = %bb8, %start
  %iter.sroa.0.0 = phi i64 [ %1, %start ], [ %iter.sroa.0.1, %bb8 ]
  %iter.sroa.13.0 = phi i1 [ false, %start ], [ true, %bb8 ]
  br i1 %iter.sroa.13.0, label %bb2.i, label %bb1.i
