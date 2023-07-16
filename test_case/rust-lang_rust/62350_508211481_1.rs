llvm
define i32 @main() #0 {
  %1 = alloca i32, align 4
  %2 = alloca %struct.WrState, align 8
  %3 = alloca %struct.LayoutGeometry, align 4
  %4 = alloca %struct.LayoutGeometry, align 4
  %5 = alloca i8, align 1
  %6 = alloca %struct.WrSpaceAndClipChain, align 8
  %7 = alloca %struct.LayoutGeometry, align 4
  %8 = alloca i64, align 8
  %9 = alloca i32, align 4
  %10 = alloca i32, align 4
  %11 = alloca i8, align 1
  %12 = alloca %struct.SideOffsets, align 4
  %13 = alloca %struct.LayoutGeometry, align 4
  %14 = alloca i8, align 1
  %15 = alloca i8, align 1
  %16 = alloca %struct.LayoutGeometry, align 4
  %17 = alloca %struct.LayoutGeometry, align 4
  %18 = alloca %struct.LayoutGeometry, align 4
  %19 = alloca %struct.SideOffsets, align 8
  %20 = alloca %struct.LayoutGeometry, align 4
  store i32 0, i32* %1, align 4
  %21 = bitcast %struct.WrState* %2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %21, i8* align 8 bitcast (%struct.WrState* @__const.main.state to i8*), i64 8, i1 false)
  %22 = bitcast %struct.LayoutGeometry* %3 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %22, i8* align 4 bitcast (%struct.LayoutGeometry* @__const.main.rect to i8*), i64 16, i1 false)
  %23 = bitcast %struct.LayoutGeometry* %4 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %23, i8* align 4 bitcast (%struct.LayoutGeometry* @__const.main.clip to i8*), i64 16, i1 false)
  store i8 0, i8* %5, align 1
  %24 = bitcast %struct.WrSpaceAndClipChain* %6 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %24, i8* align 8 bitcast (%struct.WrSpaceAndClipChain* @__const.main.parent to i8*), i64 8, i1 false)
  %25 = bitcast %struct.LayoutGeometry* %7 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %25, i8* align 4 bitcast (%struct.LayoutGeometry* @__const.main.widths to i8*), i64 16, i1 false)
  store i64 6, i64* %8, align 8
  store i32 7, i32* %9, align 4
  store i32 8, i32* %10, align 4
  store i8 0, i8* %11, align 1
  %26 = bitcast %struct.SideOffsets* %12 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %26, i8* align 4 bitcast (%struct.SideOffsets* @__const.main.slice to i8*), i64 16, i1 false)
  %27 = bitcast %struct.LayoutGeometry* %13 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %27, i8* align 4 bitcast (%struct.LayoutGeometry* @__const.main.outset to i8*), i64 16, i1 false)
  store i8 0, i8* %14, align 1
  store i8 0, i8* %15, align 1
  %28 = load i32, i32* %9, align 4
  %29 = load i32, i32* %10, align 4
  %30 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([31 x i8], [31 x i8]* @.str, i32 0, i32 0), i32 %28, i32 %29)
  %31 = getelementptr inbounds %struct.SideOffsets, %struct.SideOffsets* %12, i32 0, i32 0
  %32 = load i32, i32* %31, align 4
  %33 = getelementptr inbounds %struct.SideOffsets, %struct.SideOffsets* %12, i32 0, i32 1
  %34 = load i32, i32* %33, align 4
  %35 = getelementptr inbounds %struct.SideOffsets, %struct.SideOffsets* %12, i32 0, i32 2
  %36 = load i32, i32* %35, align 4
  %37 = getelementptr inbounds %struct.SideOffsets, %struct.SideOffsets* %12, i32 0, i32 3
  %38 = load i32, i32* %37, align 4
  %39 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([30 x i8], [30 x i8]* @.str.1, i32 0, i32 0), i32 %32, i32 %34, i32 %36, i32 %38)
  %40 = getelementptr inbounds %struct.LayoutGeometry, %struct.LayoutGeometry* %13, i32 0, i32 0
  %41 = load float, float* %40, align 4
  %42 = fpext float %41 to double
  %43 = getelementptr inbounds %struct.LayoutGeometry, %struct.LayoutGeometry* %13, i32 0, i32 1
  %44 = load float, float* %43, align 4
  %45 = fpext float %44 to double
  %46 = getelementptr inbounds %struct.LayoutGeometry, %struct.LayoutGeometry* %13, i32 0, i32 2
  %47 = load float, float* %46, align 4
  %48 = fpext float %47 to double
  %49 = getelementptr inbounds %struct.LayoutGeometry, %struct.LayoutGeometry* %13, i32 0, i32 3
  %50 = load float, float* %49, align 4
  %51 = fpext float %50 to double
  %52 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([31 x i8], [31 x i8]* @.str.2, i32 0, i32 0), double %42, double %45, double %48, double %51)
  %53 = load i8, i8* %14, align 1
  %54 = zext i8 %53 to i32
  %55 = load i8, i8* %15, align 1
  %56 = zext i8 %55 to i32
  %57 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([29 x i8], [29 x i8]* @.str.3, i32 0, i32 0), i32 %54, i32 %56)
  %58 = bitcast %struct.LayoutGeometry* %16 to i8*
  %59 = bitcast %struct.LayoutGeometry* %3 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %58, i8* align 4 %59, i64 16, i1 false)
  %60 = bitcast %struct.LayoutGeometry* %17 to i8*
  %61 = bitcast %struct.LayoutGeometry* %4 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %60, i8* align 4 %61, i64 16, i1 false)
  %62 = load i8, i8* %5, align 1
  %63 = trunc i8 %62 to i1
  %64 = bitcast %struct.LayoutGeometry* %18 to i8*
  %65 = bitcast %struct.LayoutGeometry* %7 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %64, i8* align 4 %65, i64 16, i1 false)
  %66 = load i64, i64* %8, align 8
  %67 = load i32, i32* %9, align 4
  %68 = load i32, i32* %10, align 4
  %69 = load i8, i8* %11, align 1
  %70 = trunc i8 %69 to i1
  %71 = bitcast %struct.SideOffsets* %19 to i8*
  %72 = bitcast %struct.SideOffsets* %12 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %71, i8* align 4 %72, i64 16, i1 false)
  %73 = bitcast %struct.LayoutGeometry* %20 to i8*
  %74 = bitcast %struct.LayoutGeometry* %13 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %73, i8* align 4 %74, i64 16, i1 false)
  %75 = load i8, i8* %14, align 1
  %76 = load i8, i8* %15, align 1
  %77 = bitcast %struct.LayoutGeometry* %16 to { <2 x float>, <2 x float> }*
  %78 = getelementptr inbounds { <2 x float>, <2 x float> }, { <2 x float>, <2 x float> }* %77, i32 0, i32 0
  %79 = load <2 x float>, <2 x float>* %78, align 4
  %80 = getelementptr inbounds { <2 x float>, <2 x float> }, { <2 x float>, <2 x float> }* %77, i32 0, i32 1
  %81 = load <2 x float>, <2 x float>* %80, align 4
  %82 = bitcast %struct.LayoutGeometry* %17 to { <2 x float>, <2 x float> }*
  %83 = getelementptr inbounds { <2 x float>, <2 x float> }, { <2 x float>, <2 x float> }* %82, i32 0, i32 0
  %84 = load <2 x float>, <2 x float>* %83, align 4
  %85 = getelementptr inbounds { <2 x float>, <2 x float> }, { <2 x float>, <2 x float> }* %82, i32 0, i32 1
  %86 = load <2 x float>, <2 x float>* %85, align 4
  %87 = bitcast %struct.LayoutGeometry* %18 to { <2 x float>, <2 x float> }*
  %88 = getelementptr inbounds { <2 x float>, <2 x float> }, { <2 x float>, <2 x float> }* %87, i32 0, i32 0
  %89 = load <2 x float>, <2 x float>* %88, align 4
  %90 = getelementptr inbounds { <2 x float>, <2 x float> }, { <2 x float>, <2 x float> }* %87, i32 0, i32 1
  %91 = load <2 x float>, <2 x float>* %90, align 4
  %92 = bitcast %struct.LayoutGeometry* %20 to { <2 x float>, <2 x float> }*
  %93 = getelementptr inbounds { <2 x float>, <2 x float> }, { <2 x float>, <2 x float> }* %92, i32 0, i32 0
  %94 = load <2 x float>, <2 x float>* %93, align 4
  %95 = getelementptr inbounds { <2 x float>, <2 x float> }, { <2 x float>, <2 x float> }* %92, i32 0, i32 1
  %96 = load <2 x float>, <2 x float>* %95, align 4
  call void @wr_dp_push_border_image(%struct.WrState* %2, <2 x float> %79, <2 x float> %81, <2 x float> %84, <2 x float> %86, i1 zeroext %63, %struct.WrSpaceAndClipChain* %6, <2 x float> %89, <2 x float> %91, i64 %66, i32 %67, i32 %68, i1 zeroext %70, %struct.SideOffsets* byval align 8 %19, <2 x float> %94, <2 x float> %96, i8 zeroext %75, i8 zeroext %76)
  ret i32 0
}
