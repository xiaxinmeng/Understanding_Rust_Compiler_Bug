diff
--- okay.ir     2020-06-11 10:05:02.000000000 +0200
+++ panic.ir    2020-06-11 10:05:07.000000000 +0200
@@ -2,16 +2,15 @@
 ; Function Attrs: nonlazybind uwtable
 define internal void @_ZN10playground4main17he15b3beb12586d44E() unnamed_addr #1 {
 start:
-  %split.dbg.spill = alloca i64, align 8
-  %v.dbg.spill = alloca i64, align 8
-  %prev = alloca { i64, i64 }, align 8
-  %_14 = alloca i64, align 8
+  %prev.dbg.spill = alloca { i64, i64 }, align 8
+  %_15 = alloca i64, align 8
   %_5 = alloca { i64, i64 }, align 8
   %_3 = alloca { i64, i64 }, align 8
+  %split = alloca i64, align 8
   %ls = alloca [2 x i32], align 4
   %0 = alloca {}, align 1
   call void @llvm.dbg.declare(metadata [2 x i32]* %ls, metadata !679, metadata !DIExpression()),
-  call void @llvm.dbg.declare(metadata { i64, i64 }* %prev, metadata !688, metadata !DIExpression()),
+  call void @llvm.dbg.declare(metadata i64* %split, metadata !684, metadata !DIExpression()),
   %1 = bitcast [2 x i32]* %ls to i32*,
   store i32 0, i32* %1, align 4,
   %2 = getelementptr inbounds [2 x i32], [2 x i32]* %ls, i32 0, i32 1,
@@ -41,33 +40,31 @@
   unreachable,
 
 bb4:                                              ; preds = %bb1
-  %8 = bitcast { i64, i64 }* %_3 to %"core::option::Option<usize>::Some"*,
-  %9 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %8, i32 0, i32 1,
-  %v = load i64, i64* %9, align 8,
-  store i64 %v, i64* %v.dbg.spill, align 8,
-  call void @llvm.dbg.declare(metadata i64* %v.dbg.spill, metadata !686, metadata !DIExpression()),
-  store i64 %v, i64* %split.dbg.spill, align 8,
-  call void @llvm.dbg.declare(metadata i64* %split.dbg.spill, metadata !684, metadata !DIExpression()),
-  %_12.0 = bitcast [2 x i32]* %ls to [0 x i32]*,
-  store i64 %v, i64* %_14, align 8,
-  %10 = load i64, i64* %_14, align 8,
+  %8 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_3, i32 0, i32 0,
+  %prev.0 = load i64, i64* %8, align 8,, !range !192
+  %9 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %_3, i32 0, i32 1,
+  %prev.1 = load i64, i64* %9, align 8,
+  %10 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %prev.dbg.spill, i32 0, i32 0,
+  store i64 %prev.0, i64* %10, align 8,
+  %11 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %prev.dbg.spill, i32 0, i32 1,
+  store i64 %prev.1, i64* %11, align 8,
+  call void @llvm.dbg.declare(metadata { i64, i64 }* %prev.dbg.spill, metadata !688, metadata !DIExpression()),
+  %_13.0 = bitcast [2 x i32]* %ls to [0 x i32]*,
+  %_16 = load i64, i64* %split, align 8,
+  store i64 %_16, i64* %_15, align 8,
+  %12 = load i64, i64* %_15, align 8,
 ; call core::slice::<impl core::ops::index::Index<I> for [T]>::index
-  %11 = call { [0 x i32]*, i64 } @"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17h58cba0627d7192e9E"([0 x i32]* noalias nonnull readonly align 4 %_12.0, i64 2, i64 %10, %"core::panic::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc23 to %"core::panic::Location"*)),
-  %_11.0 = extractvalue { [0 x i32]*, i64 } %11, 0,
-  %_11.1 = extractvalue { [0 x i32]*, i64 } %11, 1,
+  %13 = call { [0 x i32]*, i64 } @"_ZN4core5slice74_$LT$impl$u20$core..ops..index..Index$LT$I$GT$$u20$for$u20$$u5b$T$u5d$$GT$5index17h58cba0627d7192e9E"([0 x i32]* noalias nonnull readonly align 4 %_13.0, i64 2, i64 %12, %"core::panic::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc23 to %"core::panic::Location"*)),
+  %_12.0 = extractvalue { [0 x i32]*, i64 } %13, 0,
+  %_12.1 = extractvalue { [0 x i32]*, i64 } %13, 1,
   br label %bb5,
 
 bb5:                                              ; preds = %bb4
 ; call playground::nothing
-  call void @_ZN10playground7nothing17h8446c2fa6397d61fE([0 x i32]* noalias nonnull readonly align 4 %_11.0, i64 %_11.1),
+  call void @_ZN10playground7nothing17h8446c2fa6397d61fE([0 x i32]* noalias nonnull readonly align 4 %_12.0, i64 %_12.1),
   br label %bb6,
 
 bb6:                                              ; preds = %bb5
-  %12 = bitcast { i64, i64 }* %prev to %"core::option::Option<usize>::Some"*,
-  %13 = getelementptr inbounds %"core::option::Option<usize>::Some", %"core::option::Option<usize>::Some"* %12, i32 0, i32 1,
-  store i64 %v, i64* %13, align 8,
-  %14 = bitcast { i64, i64 }* %prev to i64*,
-  store i64 1, i64* %14, align 8,
   br label %bb7,
 
 bb7:                                              ; preds = %bb2, %bb6
