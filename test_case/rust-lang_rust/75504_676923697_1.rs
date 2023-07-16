llvm
; Function Attrs: nounwind writeonly
define avr_signalcc  void @__vector_0() unnamed_addr addrspace(1) #0 {
start:
  %item = alloca i8, align 1
  call addrspace(1) void @llvm.lifetime.start.p0i8(i64 1, i8* nonnull %item)
  store i8 0, i8* %item, align 1
; call codegen_bug_2::use_var
  call fastcc addrspace(1) void @_ZN13codegen_bug_27use_var17h8e808e1926aec402E(i8* nonnull align 1 dereferenceable(1) %item)
  call addrspace(1) void @llvm.lifetime.end.p0i8(i64 1, i8* nonnull %item)
  ret void
}
