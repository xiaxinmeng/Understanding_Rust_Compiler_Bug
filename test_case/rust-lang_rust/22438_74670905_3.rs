 llvm
define internal void @_ZN4main20hed4aaa978de9fce6eaaE() unnamed_addr #0 {
entry-block:
  %x = alloca [1000 x i32]
  %0 = getelementptr inbounds [1000 x i32]* %x, i32 0, i32 0
  br label %expr_repeat

expr_repeat:                                      ; preds = %expr_repeat, %entry-block
  %1 = phi i64 [ 0, %entry-block ], [ %3, %expr_repeat ]
  %2 = getelementptr inbounds i32* %0, i64 %1
  store i32 0, i32* %2
  %3 = add i64 %1, 1
  %4 = icmp ult i64 %3, 1000
  br i1 %4, label %expr_repeat, label %"expr_repeat: next"

"expr_repeat: next":                              ; preds = %expr_repeat
  ret void
}
