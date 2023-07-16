 llvm
match_else:                                       ; preds = %entry-block
  %8 = bitcast %"enum.Enum<[]>"* %match to { i8, i8 }*
  %9 = getelementptr inbounds { i8, i8 }* %8, i32 0, i32 1
  %10 = load i8* %9, !range !1
  switch i8 %10, label %match_else7 [
    i8 1, label %match_case8
    i8 2, label %match_case9
  ]
