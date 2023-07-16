 llvm
define internal void @main::hd4b0664c300788adah::v0.0({ i64, %tydesc*, i8*, i8*, i8 }* nocapture readnone) unnamed_addr #0 {
"function top level":
  %1 = tail call %"enum.std::libc::types::common::c95::c_void[#1]"* @malloc(i64 8)
  %2 = icmp eq %"enum.std::libc::types::common::c95::c_void[#1]"* %1, null
  br i1 %2, label %_ZN2rt11global_heap10malloc_raw19h000735e5ce40b7f3aR4v0.9E.exit.thread, label %cond.i

_ZN2rt11global_heap10malloc_raw19h000735e5ce40b7f3aR4v0.9E.exit.thread: ; preds = %"function top level"
  tail call void @abort()
  call void @llvm.trap()
  unreachable

cond.i:                                           ; preds = %"function top level"
  %3 = bitcast %"enum.std::libc::types::common::c95::c_void[#1]"* %1 to i64*
  store i64 3, i64* %3, align 8
  tail call void @free(%"enum.std::libc::types::common::c95::c_void[#1]"* %1) #1
  ret void
}
