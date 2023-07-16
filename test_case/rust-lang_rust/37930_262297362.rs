
$ rustc +nightly src/main.rs --emit llvm-ir -C panic=abort -Copt-level=2 && grep 'call .* @_ZN4core9panicking5panic' main.ll
$ rustc +nightly src/main.rs --emit llvm-ir -C panic=abort -Copt-level=3 && grep 'call .* @_ZN4core9panicking5panic' main.ll
  tail call void @_ZN4core9panicking5panic17h194ce5d68a8f28a1E({ %str_slice, %str_slice, i32 }* noalias readonly dereferenceable(40) bitcast ({ %str_slice, %str_slice, i32, [4 x i8] }* @"_ZN47_$LT$main..Chain$LT$A$C$$u20$B$C$$u20$C$GT$$GT$4poll14_MSG_FILE_LINE17hf0dd0d6c8e2c1d26E" to { %str_slice, %str_slice, i32 }*))
