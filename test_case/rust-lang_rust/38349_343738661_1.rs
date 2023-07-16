llvm
; test::id_result
; Function Attrs: norecurse nounwind uwtable
define void @_ZN4test9id_result17ha0e8372a885d01bfE(%"core::result::Result<u64, i64>"* noalias nocapture sret dereferenceable(16), %"core::result::Result<u64, i64>"* noalias nocapture readonly dereferenceable(16) %a) unnamed_addr #0 {
start:
  %1 = bitcast %"core::result::Result<u64, i64>"* %a to <2 x i64>*
  %2 = load <2 x i64>, <2 x i64>* %1, align 8
  %3 = bitcast %"core::result::Result<u64, i64>"* %0 to <2 x i64>*
  store <2 x i64> %2, <2 x i64>* %3, align 8
  ret void
}
