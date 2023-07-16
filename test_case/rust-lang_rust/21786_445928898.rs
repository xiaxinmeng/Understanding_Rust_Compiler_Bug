text
; playground::test_func
; Function Attrs: nounwind nonlazybind uwtable
define void @_ZN10playground9test_func17h2edfd294766010a4E() unnamed_addr #1 {
start:
  %_3 = alloca %Big, align 8
  %x = alloca %Big, align 8
  %0 = bitcast %Big* %x to i8*
  call void @llvm.lifetime.start.p0i8(i64 800000, i8* nonnull %0)
; call playground::foo
  call fastcc void @_ZN10playground3foo17he93a54860fc3d9b4E(%Big* noalias nocapture nonnull dereferenceable(800000) %x)
  %1 = bitcast %Big* %_3 to i8*
  call void @llvm.lifetime.start.p0i8(i64 800000, i8* nonnull %1)
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 8 %1, i8* nonnull align 8 %0, i64 800000, i1 false)
  call void asm sideeffect "", "r,~{dirflag},~{fpsr},~{flags}"(%Big* nonnull %_3) #3, !noalias !3, !srcloc !6
  call void @llvm.lifetime.end.p0i8(i64 800000, i8* nonnull %1)
  call void @llvm.lifetime.end.p0i8(i64 800000, i8* nonnull %0)
  ret void
}
