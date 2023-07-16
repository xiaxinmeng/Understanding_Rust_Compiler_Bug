llvm

%Stats = type { float, float, float }

define void @_ZN7example3sum17h258bf76ba01aa602E(%Stats* align 4 dereferenceable(12) %a, %Stats* align 4 dereferenceable(12) %b, %Stats* align 4 dereferenceable(12) %res) unnamed_addr #0 !dbg !6 {
  %0 = bitcast %Stats* %a to float*, !dbg !10
  %_4 = load float, float* %0, align 4, !dbg !10
  %1 = bitcast %Stats* %b to float*, !dbg !11
  %_5 = load float, float* %1, align 4, !dbg !11
  %2 = bitcast %Stats* %res to float*, !dbg !12
  %3 = fadd float %_4, %_5, !dbg !12
  store float %3, float* %2, align 4, !dbg !12
  %4 = getelementptr inbounds %Stats, %Stats* %a, i32 0, i32 1, !dbg !13
  %_6 = load float, float* %4, align 4, !dbg !13
  %5 = getelementptr inbounds %Stats, %Stats* %b, i32 0, i32 1, !dbg !14
  %_7 = load float, float* %5, align 4, !dbg !14
  %6 = getelementptr inbounds %Stats, %Stats* %res, i32 0, i32 1, !dbg !15
  %7 = fadd float %_6, %_7, !dbg !15
  store float %7, float* %6, align 4, !dbg !15
  %8 = getelementptr inbounds %Stats, %Stats* %a, i32 0, i32 2, !dbg !16
  %_8 = load float, float* %8, align 4, !dbg !16
  %9 = getelementptr inbounds %Stats, %Stats* %b, i32 0, i32 2, !dbg !17
  %_9 = load float, float* %9, align 4, !dbg !17
  %10 = getelementptr inbounds %Stats, %Stats* %res, i32 0, i32 2, !dbg !18
  %11 = fadd float %_8, %_9, !dbg !18
  store float %11, float* %10, align 4, !dbg !18
  ret void, !dbg !19
}
