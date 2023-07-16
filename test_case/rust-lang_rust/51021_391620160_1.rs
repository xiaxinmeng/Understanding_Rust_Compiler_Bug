
; main::main
; Function Attrs: norecurse noreturn nounwind readnone uwtable
define internal void @_ZN4main4main17hba75c185e3039e42E() unnamed_addr #2 {
start:
  unreachable
}

define i32 @main(i32, i8**) unnamed_addr #3 {
top:
  %_7.i = alloca i8*, align 8
  %2 = sext i32 %0 to i64
  %3 = bitcast i8** %_7.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %3)
  %4 = bitcast i8** %_7.i to void ()**
  store void ()* @_ZN4main4main17hba75c185e3039e42E, void ()** %4, align 8
  %5 = bitcast i8** %_7.i to {}*
; call std::rt::lang_start_internal
  %6 = call i64 @_ZN3std2rt19lang_start_internal17h666beac78d364b11E({}* nonnull %5, {}* noalias nonnull readonly bitcast ({ void (i8**)*, i64, i64, i32 (i8**)*, i32 (i8**)*, i32 (i8*)* }* @vtable.0 to {}*), i64 %2, i8** %1)
  call void @llvm.lifetime.end.p0i8(i64 8, i8* nonnull %3)
  %7 = trunc i64 %6 to i32
  ret i32 %7
}
