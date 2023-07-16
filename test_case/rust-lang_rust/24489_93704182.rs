 llvm
match_case:                                       ; preds = %match_case, %match_case.lr.ph
  %.1417 = phi i32 [ %2, %match_case.lr.ph ], [ %.14, %match_case ]
; black_box omitted here
  %5 = icmp eq i32 %4, 0
  %6 = add i32 %4, -1
  %.14 = select i1 %5, i32 0, i32 %6
  br i1 %5, label %clean_ast_57_.loopexit, label %match_case
