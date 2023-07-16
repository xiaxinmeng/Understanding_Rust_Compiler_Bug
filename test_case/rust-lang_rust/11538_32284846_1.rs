 llvm
; before
define internal void @main::h56d0d172826fda0bah::v0.0({ i64, %tydesc*, i8*, i8*, i8 }* nocapture readnone) unnamed_addr #2 {
"function top level":
  %__self.i = alloca { %"enum.std::fmt::rt::Piece[#1]"*, i64 }, align 8
  %__self2.i = alloca { %"struct.std::fmt::Argument[#1]"*, i64 }, align 8
  %1 = alloca %str_slice, align 8
  %2 = call %"enum.std::libc::types::common::c95::c_void[#1]"* @malloc(i64 8)
  %3 = icmp eq %"enum.std::libc::types::common::c95::c_void[#1]"* %2, null
  br i1 %3, label %next.thread, label %cond.i.i

next.thread:                                      ; preds = %"function top level"
  call void @abort()
  %4 = bitcast %"enum.std::libc::types::common::c95::c_void[#1]"* %2 to i64*
  store i64 3, i64* %4, align 8
  br label %"_ZN8_$UP$u329glue_drop19hb6a3c7b062d25f8faFE.exit"

cond.i.i:                                         ; preds = %"function top level"
  %5 = bitcast %"enum.std::libc::types::common::c95::c_void[#1]"* %2 to i64*
  store i64 3, i64* %5, align 8
  call void @free(%"enum.std::libc::types::common::c95::c_void[#1]"* %2) #0
  br label %"_ZN8_$UP$u329glue_drop19hb6a3c7b062d25f8faFE.exit"

"_ZN8_$UP$u329glue_drop19hb6a3c7b062d25f8faFE.exit": ; preds = %next.thread, %cond.i.i
  ret void
}
