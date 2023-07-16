 llvm
"_ZN17proc.Send$LP$$RP$9glue_drop19hd34fc795812c1ef2akE.exit5": ; preds = %join, %cond.i4
  %cond.not = xor i1 %cond, true
  %18 = icmp eq { i64, %tydesc*, i8*, i8*, i8 }* %tmp.sroa.616.0.copyload19.i.i, null
  %or.cond = or i1 %18, %cond.not
  br i1 %or.cond, label %"_ZN17proc.Send$LP$$RP$9glue_drop19hd34fc795812c1ef2akE.exit", label %cond.i.i8
