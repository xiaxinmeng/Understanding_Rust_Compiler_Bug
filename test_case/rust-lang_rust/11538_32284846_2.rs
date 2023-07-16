 llvm
;after
define internal void @main::h56d0d172826fda0bah::v0.0({ i64, %tydesc*, i8*, i8*, i8 }* nocapture readnone) unnamed_addr #2 {
"function top level":
  %__self.i = alloca { %"enum.std::fmt::rt::Piece[#1]"*, i64 }, align 8
  %__self2.i = alloca { %"struct.std::fmt::Argument[#1]"*, i64 }, align 8
  %1 = alloca %str_slice, align 8
  %2 = call %"enum.std::libc::types::common::c95::c_void[#1]"* @malloc(i64 8)
  %3 = icmp eq %"enum.std::libc::types::common::c95::c_void[#1]"* %2, null
  br i1 %3, label %then.i, label %"_ZN13_$UP$$LP$$RP$9glue_drop19hbc5ddb2494333fa7asE.exit"

then.i:                                           ; preds = %"function top level"
  call void @abort()
  unreachable

"_ZN13_$UP$$LP$$RP$9glue_drop19hbc5ddb2494333fa7asE.exit": ; preds = %"function top level"
  %4 = bitcast %"enum.std::libc::types::common::c95::c_void[#1]"* %2 to i64*
  store i64 3, i64* %4, align 8
  call void @free(%"enum.std::libc::types::common::c95::c_void[#1]"* %2) #0
  ret void
}
