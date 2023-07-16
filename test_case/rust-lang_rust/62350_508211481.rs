llvm
; Function Attrs: nounwind uwtable
define void @wr_dp_push_border_image(%WrState* align 8 dereferenceable(8), { double, double }, { double, double }, i1 zeroext, %WrSpaceAndClipChain* noalias readonly align 8 dereferenceable(8), { double, double }, i64, i32, i32, i1 zeroext, %SideOffsets* byval noalias nocapture dereferenceable(16) %slice, %LayoutGeometry* byval noalias nocapture dereferenceable(16) %outset, i8 zeroext, i8 zeroext) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality !dbg !41 {
start:
  %personalityslot = alloca { i8*, i32 }, align 8
  %arg16 = alloca i8*, align 8
  %arg05 = alloca i8*, align 8
  %_84 = alloca i8, align 1
  %_81 = alloca i8, align 1
  %_79 = alloca { i8*, i8* }, align 8
  %_78 = alloca [2 x { i8*, i8* }], align 8
  %_71 = alloca %"core::fmt::Arguments", align 8
  %arg04 = alloca %LayoutGeometry*, align 8
  %_64 = alloca i32*, align 8
  %_63 = alloca [1 x { i8*, i8* }], align 8
  %_56 = alloca %"core::fmt::Arguments", align 8
  %arg03 = alloca %SideOffsets*, align 8
  %_49 = alloca i32*, align 8
  %_48 = alloca [1 x { i8*, i8* }], align 8
  %_41 = alloca %"core::fmt::Arguments", align 8
  %arg2 = alloca i32*, align 8
  %arg1 = alloca i32*, align 8
  %arg0 = alloca i64*, align 8
  %_24 = alloca { [0 x i64], i64*, [0 x i64], i32*, [0 x i64], i32*, [0 x i64] }, align 8
  %_23 = alloca [3 x { i8*, i8* }], align 8
  %_16 = alloca %"core::fmt::Arguments", align 8
  %repeat_vertical = alloca i8, align 1
  %repeat_horizontal = alloca i8, align 1
  %fill = alloca i8, align 1
  %height = alloca i32, align 4
  %width = alloca i32, align 4
  %image = alloca i64, align 8
  %abi_cast2 = alloca { double, double }, align 8
  %widths = alloca %LayoutGeometry, align 4
  %parent = alloca %WrSpaceAndClipChain*, align 8
  %is_backface_visible = alloca i8, align 1
  %abi_cast1 = alloca { double, double }, align 8
  %clip = alloca %LayoutGeometry, align 4
  %abi_cast = alloca { double, double }, align 8
  %rect = alloca %LayoutGeometry, align 4
  %state = alloca %WrState*, align 8
  store %WrState* %0, %WrState** %state, align 8
  call void @llvm.dbg.declare(metadata %WrState** %state, metadata !67, metadata !DIExpression()), !dbg !68
  store { double, double } %1, { double, double }* %abi_cast, align 8
  %12 = bitcast %LayoutGeometry* %rect to i8*
  %13 = bitcast { double, double }* %abi_cast to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %12, i8* align 8 %13, i64 16, i1 false)
  call void @llvm.dbg.declare(metadata %LayoutGeometry* %rect, metadata !69, metadata !DIExpression()), !dbg !68
  store { double, double } %2, { double, double }* %abi_cast1, align 8
  %14 = bitcast %LayoutGeometry* %clip to i8*
  %15 = bitcast { double, double }* %abi_cast1 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %14, i8* align 8 %15, i64 16, i1 false)
  call void @llvm.dbg.declare(metadata %LayoutGeometry* %clip, metadata !70, metadata !DIExpression()), !dbg !68
  %16 = zext i1 %3 to i8
  store i8 %16, i8* %is_backface_visible, align 1
  call void @llvm.dbg.declare(metadata i8* %is_backface_visible, metadata !71, metadata !DIExpression()), !dbg !68
  store %WrSpaceAndClipChain* %4, %WrSpaceAndClipChain** %parent, align 8
  call void @llvm.dbg.declare(metadata %WrSpaceAndClipChain** %parent, metadata !72, metadata !DIExpression()), !dbg !68
  store { double, double } %5, { double, double }* %abi_cast2, align 8
  %17 = bitcast %LayoutGeometry* %widths to i8*
  %18 = bitcast { double, double }* %abi_cast2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %17, i8* align 8 %18, i64 16, i1 false)
  call void @llvm.dbg.declare(metadata %LayoutGeometry* %widths, metadata !73, metadata !DIExpression()), !dbg !68
  store i64 %6, i64* %image, align 8
  call void @llvm.dbg.declare(metadata i64* %image, metadata !74, metadata !DIExpression()), !dbg !68
  store i32 %7, i32* %width, align 4
  call void @llvm.dbg.declare(metadata i32* %width, metadata !75, metadata !DIExpression()), !dbg !68
  store i32 %8, i32* %height, align 4
  call void @llvm.dbg.declare(metadata i32* %height, metadata !76, metadata !DIExpression()), !dbg !68
  %19 = zext i1 %9 to i8
  store i8 %19, i8* %fill, align 1
  call void @llvm.dbg.declare(metadata i8* %fill, metadata !77, metadata !DIExpression()), !dbg !68
  call void @llvm.dbg.declare(metadata %SideOffsets* %slice, metadata !78, metadata !DIExpression()), !dbg !68
  call void @llvm.dbg.declare(metadata %LayoutGeometry* %outset, metadata !79, metadata !DIExpression()), !dbg !68
  store i8 %10, i8* %repeat_horizontal, align 1
  call void @llvm.dbg.declare(metadata i8* %repeat_horizontal, metadata !80, metadata !DIExpression()), !dbg !68
  store i8 %11, i8* %repeat_vertical, align 1
  call void @llvm.dbg.declare(metadata i8* %repeat_vertical, metadata !81, metadata !DIExpression()), !dbg !68
  call void @llvm.dbg.declare(metadata i64** %arg0, metadata !82, metadata !DIExpression()), !dbg !85
  call void @llvm.dbg.declare(metadata i32** %arg1, metadata !86, metadata !DIExpression()), !dbg !85
  call void @llvm.dbg.declare(metadata i32** %arg2, metadata !87, metadata !DIExpression()), !dbg !85
  call void @llvm.dbg.declare(metadata %SideOffsets** %arg03, metadata !88, metadata !DIExpression()), !dbg !91
  call void @llvm.dbg.declare(metadata %LayoutGeometry** %arg04, metadata !92, metadata !DIExpression()), !dbg !95
  call void @llvm.dbg.declare(metadata i8** %arg05, metadata !96, metadata !DIExpression()), !dbg !99
  call void @llvm.dbg.declare(metadata i8** %arg16, metadata !100, metadata !DIExpression()), !dbg !99
  %20 = bitcast { [0 x i64], i64*, [0 x i64], i32*, [0 x i64], i32*, [0 x i64] }* %_24 to i64**, !dbg !101
  store i64* %image, i64** %20, align 8, !dbg !101
  %21 = getelementptr inbounds { [0 x i64], i64*, [0 x i64], i32*, [0 x i64], i32*, [0 x i64] }, { [0 x i64], i64*, [0 x i64], i32*, [0 x i64], i32*, [0 x i64] }* %_24, i32 0, i32 3, !dbg !101
  store i32* %width, i32** %21, align 8, !dbg !101
  %22 = getelementptr inbounds { [0 x i64], i64*, [0 x i64], i32*, [0 x i64], i32*, [0 x i64] }, { [0 x i64], i64*, [0 x i64], i32*, [0 x i64], i32*, [0 x i64] }* %_24, i32 0, i32 5, !dbg !101
  store i32* %height, i32** %22, align 8, !dbg !101
  %23 = bitcast { [0 x i64], i64*, [0 x i64], i32*, [0 x i64], i32*, [0 x i64] }* %_24 to i64**, !dbg !101
  %24 = load i64*, i64** %23, align 8, !dbg !101, !nonnull !4
  store i64* %24, i64** %arg0, align 8, !dbg !101
  %25 = getelementptr inbounds { [0 x i64], i64*, [0 x i64], i32*, [0 x i64], i32*, [0 x i64] }, { [0 x i64], i64*, [0 x i64], i32*, [0 x i64], i32*, [0 x i64] }* %_24, i32 0, i32 3, !dbg !101
  %26 = load i32*, i32** %25, align 8, !dbg !101, !nonnull !4
  store i32* %26, i32** %arg1, align 8, !dbg !101
  %27 = getelementptr inbounds { [0 x i64], i64*, [0 x i64], i32*, [0 x i64], i32*, [0 x i64] }, { [0 x i64], i64*, [0 x i64], i32*, [0 x i64], i32*, [0 x i64] }* %_24, i32 0, i32 5, !dbg !101
  %28 = load i32*, i32** %27, align 8, !dbg !101, !nonnull !4
  store i32* %28, i32** %arg2, align 8, !dbg !101
  %29 = load i64*, i64** %arg0, align 8, !dbg !85, !nonnull !4
; invoke core::fmt::ArgumentV1::new
  %30 = invoke { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17h2cd404e66e3cb619E(i64* noalias readonly align 8 dereferenceable(8) %29, i1 (i64*, %"core::fmt::Formatter"*)* nonnull @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h2ac5106ffdd97f42E")
          to label %bb2 unwind label %cleanup, !dbg !85

bb1:                                              ; preds = %cleanup
  call void @llvm.trap(), !dbg !102
  unreachable, !dbg !102

bb2:                                              ; preds = %start
  %31 = extractvalue { i8*, i8* } %30, 0, !dbg !85
  %32 = extractvalue { i8*, i8* } %30, 1, !dbg !85
  %33 = load i32*, i32** %arg1, align 8, !dbg !85, !nonnull !4
; invoke core::fmt::ArgumentV1::new
  %34 = invoke { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17h513aa92b17c4c1a0E(i32* noalias readonly align 4 dereferenceable(4) %33, i1 (i32*, %"core::fmt::Formatter"*)* nonnull @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h7f05f19ad67f84e1E")
          to label %bb3 unwind label %cleanup, !dbg !85

bb3:                                              ; preds = %bb2
  %35 = extractvalue { i8*, i8* } %34, 0, !dbg !85
  %36 = extractvalue { i8*, i8* } %34, 1, !dbg !85
  %37 = load i32*, i32** %arg2, align 8, !dbg !85, !nonnull !4
; invoke core::fmt::ArgumentV1::new
  %38 = invoke { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17h513aa92b17c4c1a0E(i32* noalias readonly align 4 dereferenceable(4) %37, i1 (i32*, %"core::fmt::Formatter"*)* nonnull @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17h7f05f19ad67f84e1E")
          to label %bb4 unwind label %cleanup, !dbg !85

bb4:                                              ; preds = %bb3
  %39 = extractvalue { i8*, i8* } %38, 0, !dbg !85
  %40 = extractvalue { i8*, i8* } %38, 1, !dbg !85
  %41 = bitcast [3 x { i8*, i8* }]* %_23 to { i8*, i8* }*, !dbg !85
  %42 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %41, i32 0, i32 0, !dbg !85
  store i8* %31, i8** %42, align 8, !dbg !85
  %43 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %41, i32 0, i32 1, !dbg !85
  store i8* %32, i8** %43, align 8, !dbg !85
  %44 = getelementptr inbounds [3 x { i8*, i8* }], [3 x { i8*, i8* }]* %_23, i32 0, i32 1, !dbg !85
  %45 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %44, i32 0, i32 0, !dbg !85
  store i8* %35, i8** %45, align 8, !dbg !85
  %46 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %44, i32 0, i32 1, !dbg !85
  store i8* %36, i8** %46, align 8, !dbg !85
  %47 = getelementptr inbounds [3 x { i8*, i8* }], [3 x { i8*, i8* }]* %_23, i32 0, i32 2, !dbg !85
  %48 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %47, i32 0, i32 0, !dbg !85
  store i8* %39, i8** %48, align 8, !dbg !85
  %49 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %47, i32 0, i32 1, !dbg !85
  store i8* %40, i8** %49, align 8, !dbg !85
  %50 = bitcast [3 x { i8*, i8* }]* %_23 to [0 x { i8*, i8* }]*, !dbg !101
; invoke core::fmt::Arguments::new_v1
  invoke void @_ZN4core3fmt9Arguments6new_v117h16ed99bba8168eb0E(%"core::fmt::Arguments"* noalias nocapture sret dereferenceable(48) %_16, [0 x { [0 x i8]*, i64 }]* noalias nonnull readonly align 8 bitcast (<{ i8*, [8 x i8], i8*, [8 x i8], i8*, [8 x i8], i8*, [8 x i8] }>* @4 to [0 x { [0 x i8]*, i64 }]*), i64 4, [0 x { i8*, i8* }]* noalias nonnull readonly align 8 %50, i64 3)
          to label %bb5 unwind label %cleanup, !dbg !101

bb5:                                              ; preds = %bb4
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17hab5ed5358a32b4caE(%"core::fmt::Arguments"* noalias nocapture dereferenceable(48) %_16)
          to label %bb6 unwind label %cleanup, !dbg !101

bb6:                                              ; preds = %bb5
  %51 = bitcast i32** %_49 to %SideOffsets**, !dbg !103
  store %SideOffsets* %slice, %SideOffsets** %51, align 8, !dbg !103
  %52 = bitcast i32** %_49 to %SideOffsets**, !dbg !103
  %53 = load %SideOffsets*, %SideOffsets** %52, align 8, !dbg !103, !nonnull !4
  store %SideOffsets* %53, %SideOffsets** %arg03, align 8, !dbg !103
  %54 = load %SideOffsets*, %SideOffsets** %arg03, align 8, !dbg !91, !nonnull !4
; invoke core::fmt::ArgumentV1::new
  %55 = invoke { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17he2347c9d9f5ae000E(%SideOffsets* noalias readonly align 4 dereferenceable(16) %54, i1 (%SideOffsets*, %"core::fmt::Formatter"*)* nonnull @"_ZN59_$LT$myrustffi..SideOffsets$u20$as$u20$core..fmt..Debug$GT$3fmt17h19d8085e4fafa4e8E")
          to label %bb7 unwind label %cleanup, !dbg !91

bb7:                                              ; preds = %bb6
  %56 = extractvalue { i8*, i8* } %55, 0, !dbg !91
  %57 = extractvalue { i8*, i8* } %55, 1, !dbg !91
  %58 = bitcast [1 x { i8*, i8* }]* %_48 to { i8*, i8* }*, !dbg !91
  %59 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %58, i32 0, i32 0, !dbg !91
  store i8* %56, i8** %59, align 8, !dbg !91
  %60 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %58, i32 0, i32 1, !dbg !91
  store i8* %57, i8** %60, align 8, !dbg !91
  %61 = bitcast [1 x { i8*, i8* }]* %_48 to [0 x { i8*, i8* }]*, !dbg !103
; invoke core::fmt::Arguments::new_v1
  invoke void @_ZN4core3fmt9Arguments6new_v117h16ed99bba8168eb0E(%"core::fmt::Arguments"* noalias nocapture sret dereferenceable(48) %_41, [0 x { [0 x i8]*, i64 }]* noalias nonnull readonly align 8 bitcast (<{ i8*, [8 x i8], i8*, [8 x i8] }>* @6 to [0 x { [0 x i8]*, i64 }]*), i64 2, [0 x { i8*, i8* }]* noalias nonnull readonly align 8 %61, i64 1)
          to label %bb8 unwind label %cleanup, !dbg !103

bb8:                                              ; preds = %bb7
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17hab5ed5358a32b4caE(%"core::fmt::Arguments"* noalias nocapture dereferenceable(48) %_41)
          to label %bb9 unwind label %cleanup, !dbg !103

bb9:                                              ; preds = %bb8
  %62 = bitcast i32** %_64 to %LayoutGeometry**, !dbg !104
  store %LayoutGeometry* %outset, %LayoutGeometry** %62, align 8, !dbg !104
  %63 = bitcast i32** %_64 to %LayoutGeometry**, !dbg !104
  %64 = load %LayoutGeometry*, %LayoutGeometry** %63, align 8, !dbg !104, !nonnull !4
  store %LayoutGeometry* %64, %LayoutGeometry** %arg04, align 8, !dbg !104
  %65 = load %LayoutGeometry*, %LayoutGeometry** %arg04, align 8, !dbg !95, !nonnull !4
; invoke core::fmt::ArgumentV1::new
  %66 = invoke { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17hd3c3355ce4b5eac4E(%LayoutGeometry* noalias readonly align 4 dereferenceable(16) %65, i1 (%LayoutGeometry*, %"core::fmt::Formatter"*)* nonnull @"_ZN62_$LT$myrustffi..LayoutGeometry$u20$as$u20$core..fmt..Debug$GT$3fmt17h1b041ecfbe32053cE")
          to label %bb10 unwind label %cleanup, !dbg !95

bb10:                                             ; preds = %bb9
  %67 = extractvalue { i8*, i8* } %66, 0, !dbg !95
  %68 = extractvalue { i8*, i8* } %66, 1, !dbg !95
  %69 = bitcast [1 x { i8*, i8* }]* %_63 to { i8*, i8* }*, !dbg !95
  %70 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %69, i32 0, i32 0, !dbg !95
  store i8* %67, i8** %70, align 8, !dbg !95
  %71 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %69, i32 0, i32 1, !dbg !95
  store i8* %68, i8** %71, align 8, !dbg !95
  %72 = bitcast [1 x { i8*, i8* }]* %_63 to [0 x { i8*, i8* }]*, !dbg !104
; invoke core::fmt::Arguments::new_v1
  invoke void @_ZN4core3fmt9Arguments6new_v117h16ed99bba8168eb0E(%"core::fmt::Arguments"* noalias nocapture sret dereferenceable(48) %_56, [0 x { [0 x i8]*, i64 }]* noalias nonnull readonly align 8 bitcast (<{ i8*, [8 x i8], i8*, [8 x i8] }>* @8 to [0 x { [0 x i8]*, i64 }]*), i64 2, [0 x { i8*, i8* }]* noalias nonnull readonly align 8 %72, i64 1)
          to label %bb11 unwind label %cleanup, !dbg !104

bb11:                                             ; preds = %bb10
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17hab5ed5358a32b4caE(%"core::fmt::Arguments"* noalias nocapture dereferenceable(48) %_56)
          to label %bb12 unwind label %cleanup, !dbg !104

bb12:                                             ; preds = %bb11
  %73 = load i8, i8* %repeat_horizontal, align 1, !dbg !105, !range !106
  %74 = icmp ule i8 %73, 3, !dbg !105
  call void @llvm.assume(i1 %74), !dbg !105
  store i8 %73, i8* %_81, align 1, !dbg !105
  %75 = load i8, i8* %repeat_vertical, align 1, !dbg !107, !range !106
  %76 = icmp ule i8 %75, 3, !dbg !107
  call void @llvm.assume(i1 %76), !dbg !107
  store i8 %75, i8* %_84, align 1, !dbg !107
  %77 = bitcast { i8*, i8* }* %_79 to i8**, !dbg !108
  store i8* %_81, i8** %77, align 8, !dbg !108
  %78 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %_79, i32 0, i32 1, !dbg !108
  store i8* %_84, i8** %78, align 8, !dbg !108
  %79 = bitcast { i8*, i8* }* %_79 to i8**, !dbg !108
  %80 = load i8*, i8** %79, align 8, !dbg !108, !nonnull !4
  store i8* %80, i8** %arg05, align 8, !dbg !108
  %81 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %_79, i32 0, i32 1, !dbg !108
  %82 = load i8*, i8** %81, align 8, !dbg !108, !nonnull !4
  store i8* %82, i8** %arg16, align 8, !dbg !108
  %83 = load i8*, i8** %arg05, align 8, !dbg !99, !nonnull !4
; invoke core::fmt::ArgumentV1::new
  %84 = invoke { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17h2b158ad86e1e2672E(i8* noalias readonly align 1 dereferenceable(1) %83, i1 (i8*, %"core::fmt::Formatter"*)* nonnull @"_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17hc04471a81f11ba5fE")
          to label %bb13 unwind label %cleanup, !dbg !99

bb13:                                             ; preds = %bb12
  %85 = extractvalue { i8*, i8* } %84, 0, !dbg !99
  %86 = extractvalue { i8*, i8* } %84, 1, !dbg !99
  %87 = load i8*, i8** %arg16, align 8, !dbg !99, !nonnull !4
; invoke core::fmt::ArgumentV1::new
  %88 = invoke { i8*, i8* } @_ZN4core3fmt10ArgumentV13new17h2b158ad86e1e2672E(i8* noalias readonly align 1 dereferenceable(1) %87, i1 (i8*, %"core::fmt::Formatter"*)* nonnull @"_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17hc04471a81f11ba5fE")
          to label %bb14 unwind label %cleanup, !dbg !99

bb14:                                             ; preds = %bb13
  %89 = extractvalue { i8*, i8* } %88, 0, !dbg !99
  %90 = extractvalue { i8*, i8* } %88, 1, !dbg !99
  %91 = bitcast [2 x { i8*, i8* }]* %_78 to { i8*, i8* }*, !dbg !99
  %92 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %91, i32 0, i32 0, !dbg !99
  store i8* %85, i8** %92, align 8, !dbg !99
  %93 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %91, i32 0, i32 1, !dbg !99
  store i8* %86, i8** %93, align 8, !dbg !99
  %94 = getelementptr inbounds [2 x { i8*, i8* }], [2 x { i8*, i8* }]* %_78, i32 0, i32 1, !dbg !99
  %95 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %94, i32 0, i32 0, !dbg !99
  store i8* %89, i8** %95, align 8, !dbg !99
  %96 = getelementptr inbounds { i8*, i8* }, { i8*, i8* }* %94, i32 0, i32 1, !dbg !99
  store i8* %90, i8** %96, align 8, !dbg !99
  %97 = bitcast [2 x { i8*, i8* }]* %_78 to [0 x { i8*, i8* }]*, !dbg !108
; invoke core::fmt::Arguments::new_v1
  invoke void @_ZN4core3fmt9Arguments6new_v117h16ed99bba8168eb0E(%"core::fmt::Arguments"* noalias nocapture sret dereferenceable(48) %_71, [0 x { [0 x i8]*, i64 }]* noalias nonnull readonly align 8 bitcast (<{ i8*, [8 x i8], i8*, [8 x i8], i8*, [8 x i8] }>* @11 to [0 x { [0 x i8]*, i64 }]*), i64 3, [0 x { i8*, i8* }]* noalias nonnull readonly align 8 %97, i64 2)
          to label %bb15 unwind label %cleanup, !dbg !108

bb15:                                             ; preds = %bb14
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17hab5ed5358a32b4caE(%"core::fmt::Arguments"* noalias nocapture dereferenceable(48) %_71)
          to label %bb16 unwind label %cleanup, !dbg !108

bb16:                                             ; preds = %bb15
  ret void, !dbg !109

cleanup:                                          ; preds = %bb15, %bb14, %bb13, %bb12, %bb11, %bb10, %bb9, %bb8, %bb7, %bb6, %bb5, %bb4, %bb3, %bb2, %start
  %98 = landingpad { i8*, i32 }
          cleanup
  %99 = extractvalue { i8*, i32 } %98, 0
  %100 = extractvalue { i8*, i32 } %98, 1
  %101 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 0
  store i8* %99, i8** %101, align 8
  %102 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %personalityslot, i32 0, i32 1
  store i32 %100, i32* %102, align 8
  br label %bb1
}
