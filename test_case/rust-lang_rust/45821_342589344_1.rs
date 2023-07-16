llvm-ir
define i8 @no_op(i8) {
  switch i8 %0, label %one [
    i8 0, label %zero
  ]

zero:                                             ; preds = %1
  br label %end

one:                                              ; preds = %1
  br label %end

end:                                              ; preds = %one, %zero
  %ret = phi i8 [ 0, %zero ], [ 1, %one ]
  ret i8 %ret
}
