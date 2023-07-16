llvm
define i32 @bar(i32 %i) unnamed_addr #0 {
start:
  %0 = icmp slt i32 %i, 1000
  br i1 %0, label %bb4, label %bb6

bb4:                                              ; preds = %bb3
  %1 = srem i32 %i, 2
  %2 = call i32 @bar(i32 %1)
  br label %bb6

bb6:                                              ; preds = %bb4, %bb2
  %_0.0 = phi i32 [ %2, %bb4 ], [ 1234567, %start ]
  ret i32 %_0.0
}

attributes #0 = { uwtable }
