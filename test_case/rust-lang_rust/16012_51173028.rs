 llvm
else-block.i.i.i:                                 ; preds = %else-block.i.i.i.preheader, %else-block.i.i.i
  %.tr710.i.i.i = phi i64 [ %107, %else-block.i.i.i ], [ %104, %else-block.i.i.i.preheader ]
  %.tr69.i.i.i = phi i64 [ %106, %else-block.i.i.i ], [ 1, %else-block.i.i.i.preheader ]
  %.tr8.i.i.i = phi i64 [ %.tr69.i.i.i, %else-block.i.i.i ], [ 0, %else-block.i.i.i.preheader ]
  %106 = add i64 %.tr8.i.i.i, %.tr69.i.i.i
  %107 = add i64 %.tr710.i.i.i, -1
  %108 = icmp eq i64 %107, 0
  br i1 %108, label %_ZN3fib20hcf5cee5c8487747eOaaE.exit.i.loopexit, label %else-block.i.i.i
