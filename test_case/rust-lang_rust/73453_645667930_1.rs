ll
define { { [0 x i8], i8, [3 x i8], [0 x i32] }*, i64 } @_ZN4test8coercion17h76ca7bfe6ae8dcf2E({ [0 x i32], [2 x i32], [0 x i8], i8, [3 x i8] }* noalias readonly align 4 dereferenceable(12) %x) unnamed_addr #0 {
start:
  %0 = bitcast { [0 x i32], [2 x i32], [0 x i8], i8, [3 x i8] }* %x to { [0 x i8], i8, [3 x i8], [0 x i32] }*
  %1 = insertvalue { { [0 x i8], i8, [3 x i8], [0 x i32] }*, i64 } undef, { [0 x i8], i8, [3 x i8], [0 x i32] }* %0, 0
  %2 = insertvalue { { [0 x i8], i8, [3 x i8], [0 x i32] }*, i64 } %1, i64 2, 1
  ret { { [0 x i8], i8, [3 x i8], [0 x i32] }*, i64 } %2
}
