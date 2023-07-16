
  %.val.i.i.i.i.i.i.i = load i64, ptr %self.sroa.0.010.i.i.i.i.i.i.i, align 4, !dbg !242, !noalias !243
  %rhs.sroa.0.0.extract.trunc.i.i.i.i.i.i.i.i.i.i.i.i = trunc i64 %.val.i.i.i.i.i.i.i to i32
  %10 = bitcast i32 %rhs.sroa.0.0.extract.trunc.i.i.i.i.i.i.i.i.i.i.i.i to float
  %rhs.sroa.2.0.extract.shift.i.i.i.i.i.i.i.i.i.i.i.i = lshr i64 %.val.i.i.i.i.i.i.i, 32
  %rhs.sroa.2.0.extract.trunc.i.i.i.i.i.i.i.i.i.i.i.i = trunc i64 %rhs.sroa.2.0.extract.shift.i.i.i.i.i.i.i.i.i.i.i.i to i32
  %11 = bitcast i32 %rhs.sroa.2.0.extract.trunc.i.i.i.i.i.i.i.i.i.i.i.i to float
