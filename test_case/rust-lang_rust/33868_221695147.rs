
  %1 = alloca { i64, i64 }
  store { i64, i64 } %0, { i64, i64 }* %1
  %2 = bitcast { i64, i64 }* %1 to %S*
  %3 = call i32 @_ZN4test10__rust_abiE(%S* noalias nocapture dereferenceable(12) %2)
  ret i32 %3
