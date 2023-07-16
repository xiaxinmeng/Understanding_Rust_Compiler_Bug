llvm
define i64 @parameters_by_value(i64 %v.0, i64 %v.1) unnamed_addr #0 {
start:
  %0 = add i64 %v.1, %v.0
  ret i64 %0
}

define { i64, i64 } @return_by_value() unnamed_addr #0 {
start:
  ret { i64, i64 } { i64 3, i64 4 }
}
