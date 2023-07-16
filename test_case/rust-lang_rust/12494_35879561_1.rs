 llvm
  %283 = bitcast [4 x i64]* %282 to i8*
  %284 = load i8* %283, align 8, !range !1
  switch i8 %284, label %match_else42 [
  ;...
  %285 = bitcast [4 x i64]* %282 to { i8, %"struct.syntax::ast::DefId[#9]", i8, { i64, void (i8*)*, i8*, i8*, %"struct.syntax::ast::Method[#9]" }* }*
  ;...
  %impl_did.sroa.0.0.idx = getelementptr inbounds { i8, %"struct.syntax::ast::DefId[#9]", i8, { i64, void (i8*)*, i8*, i8*, %"struct.syntax::ast::Method[#9]" }* }* %285, i64 0, i32 1, i32 0
  %impl_did.sroa.0.0.copyload = load i32* %impl_did.sroa.0.0.idx, align 4
