
define internal void @_ZN3foo20h33c0f4c7a8c48249eaaE(%"3.collections::string::String"* noalias nocapture sret dereferenceable(24), %"3.collections::vec::Vec<u8>"* noalias nocapture dereferenceable(24) %x) unnamed_addr #0 {
entry-block:
  %dropflag_hint_6 = alloca i8
  store i8 61, i8* %dropflag_hint_6
  %1 = bitcast %"3.collections::string::String"* %0 to %"3.collections::vec::Vec<u8>"*
  %2 = bitcast %"3.collections::vec::Vec<u8>"* %x to i8*
  %3 = bitcast %"3.collections::vec::Vec<u8>"* %1 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %3, i8* %2, i64 24, i32 8, i1 false)
  %4 = bitcast %"3.collections::vec::Vec<u8>"* %x to i8*
  call void @llvm.memset.p0i8.i64(i8* %4, i8 29, i64 24, i32 8, i1 false)
  call void @"_ZN31collections..vec..Vec$LT$u8$GT$9drop.359317hfad10bb9521885b9E"(%"3.collections::vec::Vec<u8>"* %x)
  ret void
}
