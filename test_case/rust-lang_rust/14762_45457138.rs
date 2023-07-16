 llvm
*** IR Dump After Remove unused exception handling info ***
; Function Attrs: inlinehint nounwind uwtable
define internal fastcc i8* @"_ZN5slice60_$BP$$x27a$x20$x5bT$x5d.ImmutableVector$LT$$x27a$C$$x20T$GT$6as_ptr21h152019210325997247454v0.0E"({ i8*, i32 }* nocapture nonnull) unnamed_addr #2 {
entry-block:
  %1 = alloca %"struct.core::raw::Slice<u8>[#1]", align 4
  call fastcc void @_ZN3raw4Repr4repr19h7328813308158244434v0.0E(%"struct.core::raw::Slice<u8>[#1]"* noalias nocapture nonnull sret %1, { i8*, i32 }* nonnull %0)
  %2 = getelementptr inbounds %"struct.core::raw::Slice<u8>[#1]"* %1, i32 0, i32 0
  %3 = load i8** %2, align 4
  ret i8* %3
}
*** IR Dump After Function Integration/Inlining ***
; Function Attrs: inlinehint nounwind uwtable
define internal fastcc i8* @"_ZN5slice60_$BP$$x27a$x20$x5bT$x5d.ImmutableVector$LT$$x27a$C$$x20T$GT$6as_ptr21h152019210325997247454v0.0E"({ i8*, i32 }* nocapture nonnull) unnamed_addr #2 {
entry-block:
  %1 = alloca %"struct.core::raw::Slice<u8>[#1]", align 4
  %2 = bitcast { i8*, i32 }* %0 to i64*
  %3 = load i64* %2, align 4
  %4 = bitcast %"struct.core::raw::Slice<u8>[#1]"* %1 to i64*
  store i64 %3, i64* %4, align 4
  %5 = getelementptr inbounds %"struct.core::raw::Slice<u8>[#1]"* %1, i32 0, i32 0
  %6 = load i8** %5, align 4
  ret i8* %6
}
[snip]
*** IR Dump After SROA ***
; Function Attrs: inlinehint nounwind readonly uwtable
define internal fastcc i8* @"_ZN5slice60_$BP$$x27a$x20$x5bT$x5d.ImmutableVector$LT$$x27a$C$$x20T$GT$6as_ptr21h152019210325997247454v0.0E"({ i8*, i32 }* nocapture nonnull readonly) unnamed_addr #2 {
entry-block:
  %1 = bitcast { i8*, i32 }* %0 to i64*
  %2 = load i64* %1, align 4
  %.sroa.0.0.extract.trunc = trunc i64 %2 to i32
  %3 = inttoptr i32 %.sroa.0.0.extract.trunc to i8*
  %.sroa.2.0.extract.shift = lshr i64 %2, 32
  %.sroa.2.0.extract.trunc = trunc i64 %.sroa.2.0.extract.shift to i32
  ret i8* %3
}
