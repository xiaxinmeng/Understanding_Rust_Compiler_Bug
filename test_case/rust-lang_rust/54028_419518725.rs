llvm
; playground::load_unaligned
; Function Attrs: uwtable
define double @_ZN10playground14load_unaligned17hb6493993a7b635a6E(%Compound* noalias nocapture dereferenceable(16) %cmp) unnamed_addr #0 !dbg !4 {
start:
  %imag = alloca double, align 8, !dbg !17
  %real = alloca double, align 8, !dbg !17
  call void @llvm.dbg.declare(metadata %Compound* %cmp, metadata !18, metadata !DIExpression()), !dbg !17
  call void @llvm.dbg.declare(metadata double* %real, metadata !19, metadata !DIExpression()), !dbg !21
  call void @llvm.dbg.declare(metadata double* %imag, metadata !22, metadata !DIExpression()), !dbg !21
  %0 = bitcast %Compound* %cmp to %Complex*, !dbg !23
  %1 = bitcast %Complex* %0 to double*, !dbg !23
  %2 = load double, double* %1, align 8, !dbg !23
  store double %2, double* %real, align 8, !dbg !23
  %3 = bitcast %Compound* %cmp to %Complex*, !dbg !23
  %4 = getelementptr inbounds %Complex, %Complex* %3, i32 0, i32 3, !dbg !23
  %5 = load double, double* %4, align 8, !dbg !23
  store double %5, double* %imag, align 8, !dbg !23
  %6 = load double, double* %real, align 8, !dbg !24
  %7 = load double, double* %real, align 8, !dbg !24
  %8 = fmul double %6, %7, !dbg !24
  %9 = load double, double* %imag, align 8, !dbg !24
  %10 = load double, double* %imag, align 8, !dbg !24
  %11 = fmul double %9, %10, !dbg !24
  %12 = fadd double %8, %11, !dbg !24
  ret double %12, !dbg !25
}

; playground::load_aligned
; Function Attrs: uwtable
define double @_ZN10playground12load_aligned17h2a5b875154d5e906E(%Compound* noalias nocapture dereferenceable(16) %cmp) unnamed_addr #0 !dbg !26 {
start:
  %imag = alloca double, align 8, !dbg !27
  %real = alloca double, align 8, !dbg !27
  %_3 = alloca %Compound, align 16, !dbg !27
  %cmp1 = alloca %Compound, align 16, !dbg !27
  call void @llvm.dbg.declare(metadata %Compound* %cmp, metadata !28, metadata !DIExpression()), !dbg !27
  call void @llvm.dbg.declare(metadata %Compound* %cmp1, metadata !29, metadata !DIExpression()), !dbg !31
  call void @llvm.dbg.declare(metadata double* %real, metadata !32, metadata !DIExpression()), !dbg !34
  call void @llvm.dbg.declare(metadata double* %imag, metadata !35, metadata !DIExpression()), !dbg !34
  %0 = bitcast %Compound* %cmp to i8*, !dbg !36
  %1 = bitcast %Compound* %_3 to i8*, !dbg !36
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %1, i8* %0, i64 16, i32 16, i1 false), !dbg !36
  %2 = bitcast %Compound* %_3 to i8*, !dbg !36
  %3 = bitcast %Compound* %cmp1 to i8*, !dbg !36
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %3, i8* %2, i64 16, i32 16, i1 false), !dbg !36
  %4 = bitcast %Compound* %cmp1 to %Complex*, !dbg !37
  %5 = bitcast %Complex* %4 to double*, !dbg !37
  %6 = load double, double* %5, align 8, !dbg !37
  store double %6, double* %real, align 8, !dbg !37
  %7 = bitcast %Compound* %cmp1 to %Complex*, !dbg !37
  %8 = getelementptr inbounds %Complex, %Complex* %7, i32 0, i32 3, !dbg !37
  %9 = load double, double* %8, align 8, !dbg !37
  store double %9, double* %imag, align 8, !dbg !37
  %10 = load double, double* %real, align 8, !dbg !38
  %11 = load double, double* %real, align 8, !dbg !38
  %12 = fmul double %10, %11, !dbg !38
  %13 = load double, double* %imag, align 8, !dbg !38
  %14 = load double, double* %imag, align 8, !dbg !38
  %15 = fmul double %13, %14, !dbg !38
  %16 = fadd double %12, %15, !dbg !38
  ret double %16, !dbg !39
}
