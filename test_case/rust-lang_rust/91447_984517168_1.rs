llvm
%struct.Stats = type { float, float, float }

define dso_local { <2 x float>, float } @_Z3sumRK5StatsS1_(%struct.Stats* nonnull align 4 dereferenceable(12) %0, %struct.Stats* nonnull align 4 dereferenceable(12) %1) #0 !dbg !105 {
  %3 = alloca %struct.Stats, align 4
  %4 = alloca %struct.Stats*, align 8
  %5 = alloca %struct.Stats*, align 8
  %6 = alloca { <2 x float>, float }, align 8
  store %struct.Stats* %0, %struct.Stats** %4, align 8
  call void @llvm.dbg.declare(metadata %struct.Stats** %4, metadata !111, metadata !DIExpression()), !dbg !112
  store %struct.Stats* %1, %struct.Stats** %5, align 8
  call void @llvm.dbg.declare(metadata %struct.Stats** %5, metadata !113, metadata !DIExpression()), !dbg !114
  %7 = getelementptr inbounds %struct.Stats, %struct.Stats* %3, i32 0, i32 0, !dbg !115
  %8 = load %struct.Stats*, %struct.Stats** %4, align 8, !dbg !116
  %9 = getelementptr inbounds %struct.Stats, %struct.Stats* %8, i32 0, i32 0, !dbg !117
  %10 = load float, float* %9, align 4, !dbg !117
  %11 = load %struct.Stats*, %struct.Stats** %5, align 8, !dbg !118
  %12 = getelementptr inbounds %struct.Stats, %struct.Stats* %11, i32 0, i32 0, !dbg !119
  %13 = load float, float* %12, align 4, !dbg !119
  %14 = fadd float %10, %13, !dbg !120
  store float %14, float* %7, align 4, !dbg !115
  %15 = getelementptr inbounds %struct.Stats, %struct.Stats* %3, i32 0, i32 1, !dbg !115
  %16 = load %struct.Stats*, %struct.Stats** %4, align 8, !dbg !121
  %17 = getelementptr inbounds %struct.Stats, %struct.Stats* %16, i32 0, i32 1, !dbg !122
  %18 = load float, float* %17, align 4, !dbg !122
  %19 = load %struct.Stats*, %struct.Stats** %5, align 8, !dbg !123
  %20 = getelementptr inbounds %struct.Stats, %struct.Stats* %19, i32 0, i32 1, !dbg !124
  %21 = load float, float* %20, align 4, !dbg !124
  %22 = fadd float %18, %21, !dbg !125
  store float %22, float* %15, align 4, !dbg !115
  %23 = getelementptr inbounds %struct.Stats, %struct.Stats* %3, i32 0, i32 2, !dbg !115
  %24 = load %struct.Stats*, %struct.Stats** %4, align 8, !dbg !126
  %25 = getelementptr inbounds %struct.Stats, %struct.Stats* %24, i32 0, i32 2, !dbg !127
  %26 = load float, float* %25, align 4, !dbg !127
  %27 = load %struct.Stats*, %struct.Stats** %5, align 8, !dbg !128
  %28 = getelementptr inbounds %struct.Stats, %struct.Stats* %27, i32 0, i32 2, !dbg !129
  %29 = load float, float* %28, align 4, !dbg !129
  %30 = fadd float %26, %29, !dbg !130
  store float %30, float* %23, align 4, !dbg !115
  %31 = bitcast { <2 x float>, float }* %6 to i8*, !dbg !131
  %32 = bitcast %struct.Stats* %3 to i8*, !dbg !131
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %31, i8* align 4 %32, i64 12, i1 false), !dbg !131
  %33 = load { <2 x float>, float }, { <2 x float>, float }* %6, align 8, !dbg !131
  ret { <2 x float>, float } %33, !dbg !131
}
