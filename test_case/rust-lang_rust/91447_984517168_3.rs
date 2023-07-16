llvm
%Stats = type { float, float, float }

define i96 @_ZN7example3sum17h2dbae3ec3349ec5dE(%Stats* align 4 dereferenceable(12) %a, %Stats* align 4 dereferenceable(12) %b) unnamed_addr #0 !dbg !6 {
  %0 = alloca %Stats, align 4
  %1 = bitcast %Stats* %a to float*, !dbg !10
  %_4 = load float, float* %1, align 4, !dbg !10
  %2 = bitcast %Stats* %b to float*, !dbg !11
  %_5 = load float, float* %2, align 4, !dbg !11
  %_3 = fadd float %_4, %_5, !dbg !10
  %3 = getelementptr inbounds %Stats, %Stats* %a, i32 0, i32 1, !dbg !12
  %_7 = load float, float* %3, align 4, !dbg !12
  %4 = getelementptr inbounds %Stats, %Stats* %b, i32 0, i32 1, !dbg !13
  %_8 = load float, float* %4, align 4, !dbg !13
  %_6 = fadd float %_7, %_8, !dbg !12
  %5 = getelementptr inbounds %Stats, %Stats* %a, i32 0, i32 2, !dbg !14
  %_10 = load float, float* %5, align 4, !dbg !14
  %6 = getelementptr inbounds %Stats, %Stats* %b, i32 0, i32 2, !dbg !15
  %_11 = load float, float* %6, align 4, !dbg !15
  %_9 = fadd float %_10, %_11, !dbg !14
  %7 = bitcast %Stats* %0 to float*, !dbg !16
  store float %_3, float* %7, align 4, !dbg !16
  %8 = getelementptr inbounds %Stats, %Stats* %0, i32 0, i32 1, !dbg !16
  store float %_6, float* %8, align 4, !dbg !16
  %9 = getelementptr inbounds %Stats, %Stats* %0, i32 0, i32 2, !dbg !16
  store float %_9, float* %9, align 4, !dbg !16
  %10 = bitcast %Stats* %0 to i96*, !dbg !17
  %11 = load i96, i96* %10, align 4, !dbg !17
  ret i96 %11, !dbg !17
}
