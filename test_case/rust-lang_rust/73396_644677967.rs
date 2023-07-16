
  %_4.i.i.i = icmp ult i64 %.sroa.3.0.i.i.i, %s.1, !dbg !66
  tail call void @llvm.assume(i1 %_4.i.i.i) #2, !dbg !74, !noalias !75
  %_8.i.i.i = icmp ugt i64 %.sroa.3.0.i.i.i, %s.1, !dbg !80
  br i1 %_8.i.i.i, label %bb5.i.i.i, label %bb6, !dbg !87
