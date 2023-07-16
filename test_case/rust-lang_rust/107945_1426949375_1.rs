llvm
; playground::make_undef_intrin
; Function Attrs: mustprogress nofree nosync nounwind nonlazybind willreturn uwtable
define void @_ZN10playground17make_undef_intrin17h301d7039825564c2E(ptr noalias nocapture noundef writeonly sret(<2 x i64>) dereferenceable(16) %0) unnamed_addr #1 {
start:
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %0, i8 0, i64 16, i1 false), !alias.scope !2
  ret void
}
