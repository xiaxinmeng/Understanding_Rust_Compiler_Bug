 llvm
; Function Attrs: nounwind uwtable
define void @_ZN8rust_out9test_func17h4ab794041fab200cE() unnamed_addr #0 {
entry-block:
  %arg = alloca %Big, align 8
  %0 = bitcast %Big* %arg to i8*
  call void @llvm.lifetime.start(i64 800000, i8* %0)
  call void @llvm.memset.p0i8.i64(i8* %0, i8 0, i64 800000, i32 8, i1 false)
  call void asm "", "r,~{dirflag},~{fpsr},~{flags}"(%Big* nonnull %arg) #2, !noalias !0, !srcloc !3
  call void @llvm.lifetime.end(i64 800000, i8* %0) #2, !alias.scope !4, !noalias !0
  call void @llvm.lifetime.end(i64 800000, i8* %0)
  ret void
}
