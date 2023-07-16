 llvm
    %11 = icmp eq i64 %.sroa.3.0.copyload.i.i.i, %2
    br i1 %11, label %before_rhs.i, label %next-block.i.i
  
  next-block.i.i:                                   ; preds = %entry-block
    %12 = icmp ult i64 %.sroa.3.0.copyload.i.i.i, %2
    br i1 %12, label %then-block-54-.i, label %next-block1.i.i
  
  next-block1.i.i:                                  ; preds = %next-block.i.i
    %13 = icmp ugt i64 %.sroa.3.0.copyload.i.i.i, %2 ; impossible to fail!
  