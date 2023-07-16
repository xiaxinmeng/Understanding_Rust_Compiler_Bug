ll
; ModuleID = '13wuyk23ot48ix3i'
source_filename = "13wuyk23ot48ix3i"
target datalayout = "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"
target triple = "thumbv6m-none-unknown-eabi"

%"core::fmt::Formatter" = type { [0 x i32], i32, [0 x i32], i32, [0 x i32], { i32, i32 }, [0 x i32], { i32, i32 }, [0 x i32], { {}*, [3 x i32]* }, [0 x i8], i8, [3 x i8] }
%"core::fmt::Arguments" = type { [0 x i32], { [0 x { [0 x i8]*, i32 }]*, i32 }, [0 x i32], { i32*, i32 }, [0 x i32], { [0 x { i8*, i32* }]*, i32 }, [0 x i32] }
%"core::panic::Location" = type { [0 x i32], { [0 x i8]*, i32 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }

@alloc42 = private unnamed_addr constant <{ [2 x i8] }> <{ [2 x i8] c"()" }>, align 1
@alloc43 = private unnamed_addr constant <{ [43 x i8] }> <{ [43 x i8] c"called `Result::unwrap()` on an `Err` value" }>, align 1
@vtable.0 = private unnamed_addr constant { void ({}*)*, i32, i32, i1 ({}*, %"core::fmt::Formatter"*)* } { void ({}*)* @"_ZN4core3ptr29drop_in_place$LT$$LP$$RP$$GT$17hd540dc9d5e454804E", i32 0, i32 1, i1 ({}*, %"core::fmt::Formatter"*)* @"_ZN45_$LT$$LP$$RP$$u20$as$u20$core..fmt..Debug$GT$3fmt17h145b8a12344daa9aE" }, align 4, !dbg !0
@alloc1 = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c"[" }>, align 1
@alloc3 = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c":" }>, align 1
@alloc4 = private unnamed_addr constant <{ [2 x i8] }> <{ [2 x i8] c"] " }>, align 1
@alloc5 = private unnamed_addr constant <{ [3 x i8] }> <{ [3 x i8] c" = " }>, align 1
@alloc6 = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c"\0A" }>, align 1
@alloc2 = private unnamed_addr constant <{ i8*, [4 x i8], i8*, [4 x i8], i8*, [4 x i8], i8*, [4 x i8], i8*, [4 x i8] }> <{ i8* getelementptr inbounds (<{ [1 x i8] }>, <{ [1 x i8] }>* @alloc1, i32 0, i32 0, i32 0), [4 x i8] c"\01\00\00\00", i8* getelementptr inbounds (<{ [1 x i8] }>, <{ [1 x i8] }>* @alloc3, i32 0, i32 0, i32 0), [4 x i8] c"\01\00\00\00", i8* getelementptr inbounds (<{ [2 x i8] }>, <{ [2 x i8] }>* @alloc4, i32 0, i32 0, i32 0), [4 x i8] c"\02\00\00\00", i8* getelementptr inbounds (<{ [3 x i8] }>, <{ [3 x i8] }>* @alloc5, i32 0, i32 0, i32 0), [4 x i8] c"\03\00\00\00", i8* getelementptr inbounds (<{ [1 x i8] }>, <{ [1 x i8] }>* @alloc6, i32 0, i32 0, i32 0), [4 x i8] c"\01\00\00\00" }>, align 4
@alloc44 = private unnamed_addr constant <{ [11 x i8] }> <{ [11 x i8] c"src/main.rs" }>, align 1
@alloc9 = private unnamed_addr constant <{ i8*, [4 x i8] }> <{ i8* getelementptr inbounds (<{ [11 x i8] }>, <{ [11 x i8] }>* @alloc44, i32 0, i32 0, i32 0), [4 x i8] c"\0B\00\00\00" }>, align 4
@alloc11 = private unnamed_addr constant <{ [4 x i8] }> <{ [4 x i8] c"\0B\00\00\00" }>, align 4
@alloc13 = private unnamed_addr constant <{ [5 x i8] }> <{ [5 x i8] c"1i128" }>, align 1
@alloc14 = private unnamed_addr constant <{ i8*, [4 x i8] }> <{ i8* getelementptr inbounds (<{ [5 x i8] }>, <{ [5 x i8] }>* @alloc13, i32 0, i32 0, i32 0), [4 x i8] c"\05\00\00\00" }>, align 4
@alloc41 = private unnamed_addr constant <{ [128 x i8] }> <{ [128 x i8] c"\00\00\00\00 \00\00\00\00\00\00\00\02\00\00\00\00\00\00\00\02\00\00\00\00\00\00\00\03\00\00\00\01\00\00\00 \00\00\00\00\00\00\00\02\00\00\00\00\00\00\00\02\00\00\00\00\00\00\00\03\00\00\00\02\00\00\00 \00\00\00\00\00\00\00\02\00\00\00\00\00\00\00\02\00\00\00\00\00\00\00\03\00\00\00\03\00\00\00 \00\00\00\04\00\00\00\02\00\00\00\00\00\00\00\02\00\00\00\00\00\00\00\03\00\00\00" }>, align 4
@alloc45 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [11 x i8] }>, <{ [11 x i8] }>* @alloc44, i32 0, i32 0, i32 0), [12 x i8] c"\0B\00\00\00\0B\00\00\00\05\00\00\00" }>, align 4

; <&T as core::fmt::Debug>::fmt
; Function Attrs: nounwind
define internal zeroext i1 @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf2e383e57d7cd2f0E"(i128** noalias nocapture readonly align 4 dereferenceable(4) %self, %"core::fmt::Formatter"* noalias align 4 dereferenceable(36) %f) unnamed_addr #0 !dbg !27 {
start:
  call void @llvm.dbg.value(metadata i128** %self, metadata !71, metadata !DIExpression()), !dbg !75
  call void @llvm.dbg.value(metadata %"core::fmt::Formatter"* %f, metadata !72, metadata !DIExpression()), !dbg !75
  %_4 = load i128*, i128** %self, align 4, !dbg !76, !nonnull !4
; call core::fmt::num::<impl core::fmt::Debug for i128>::fmt
  %0 = call fastcc zeroext i1 @"_ZN4core3fmt3num51_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i128$GT$3fmt17h98aecc833f509da7E"(i128* noalias nonnull readonly align 8 dereferenceable(16) %_4, %"core::fmt::Formatter"* noalias nonnull align 4 dereferenceable(36) %f), !dbg !77
  ret i1 %0, !dbg !78
}

; <() as core::fmt::Debug>::fmt
; Function Attrs: inlinehint nounwind
define internal zeroext i1 @"_ZN45_$LT$$LP$$RP$$u20$as$u20$core..fmt..Debug$GT$3fmt17h145b8a12344daa9aE"({}* noalias nocapture nonnull readonly align 1 %self, %"core::fmt::Formatter"* noalias align 4 dereferenceable(36) %f) unnamed_addr #1 !dbg !79 {
start:
  call void @llvm.dbg.value(metadata {}* %self, metadata !84, metadata !DIExpression()), !dbg !86
  call void @llvm.dbg.value(metadata %"core::fmt::Formatter"* %f, metadata !85, metadata !DIExpression()), !dbg !86
; call core::fmt::Formatter::pad
  %0 = call zeroext i1 @_ZN4core3fmt9Formatter3pad17hcbeeb976699479d7E(%"core::fmt::Formatter"* noalias nonnull align 4 dereferenceable(36) %f, [0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [2 x i8] }>* @alloc42 to [0 x i8]*), i32 2), !dbg !87
  ret i1 %0, !dbg !88
}

; core::fmt::ArgumentV1::new
; Function Attrs: norecurse nounwind readnone willreturn
define internal fastcc nonnull i8* @_ZN4core3fmt10ArgumentV13new17h91b00cee34599045E(i128** noalias readonly align 4 dereferenceable(4) %x) unnamed_addr #2 !dbg !89 {
start:
  call void @llvm.dbg.value(metadata i128** %x, metadata !103, metadata !DIExpression()), !dbg !107
  call void @llvm.dbg.value(metadata i1 (i128**, %"core::fmt::Formatter"*)* @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf2e383e57d7cd2f0E", metadata !104, metadata !DIExpression()), !dbg !107
  %0 = bitcast i128** %x to i8*, !dbg !108
  ret i8* %0, !dbg !109
}

; core::fmt::num::<impl core::fmt::Debug for i128>::fmt
; Function Attrs: inlinehint nounwind
define internal fastcc zeroext i1 @"_ZN4core3fmt3num51_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i128$GT$3fmt17h98aecc833f509da7E"(i128* noalias readonly align 8 dereferenceable(16) %self, %"core::fmt::Formatter"* noalias align 4 dereferenceable(36) %f) unnamed_addr #1 !dbg !110 {
start:
  call void @llvm.dbg.value(metadata i128* %self, metadata !117, metadata !DIExpression()), !dbg !119
  call void @llvm.dbg.value(metadata %"core::fmt::Formatter"* %f, metadata !118, metadata !DIExpression()), !dbg !119
; call core::fmt::Formatter::debug_lower_hex
  %_3 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_lower_hex17h8d0023f0b6bcb4e2E(%"core::fmt::Formatter"* noalias nonnull readonly align 4 dereferenceable(36) %f), !dbg !120
  br i1 %_3, label %bb2, label %bb3, !dbg !121

bb3:                                              ; preds = %start
; call core::fmt::Formatter::debug_upper_hex
  %_7 = call zeroext i1 @_ZN4core3fmt9Formatter15debug_upper_hex17h55d5da0528680253E(%"core::fmt::Formatter"* noalias nonnull readonly align 4 dereferenceable(36) %f), !dbg !122
  br i1 %_7, label %bb6, label %bb7, !dbg !123

bb2:                                              ; preds = %start
; call core::fmt::num::<impl core::fmt::LowerHex for i128>::fmt
  %0 = call zeroext i1 @"_ZN4core3fmt3num54_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i128$GT$3fmt17hfad7c627e744b3baE"(i128* noalias nonnull readonly align 8 dereferenceable(16) %self, %"core::fmt::Formatter"* noalias nonnull align 4 dereferenceable(36) %f), !dbg !124
  br label %bb11, !dbg !121

bb11:                                             ; preds = %bb6, %bb7, %bb2
  %.0.in = phi i1 [ %0, %bb2 ], [ %2, %bb6 ], [ %1, %bb7 ]
  ret i1 %.0.in, !dbg !125

bb7:                                              ; preds = %bb3
; call core::fmt::num::<impl core::fmt::Display for i128>::fmt
  %1 = call zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..Display$u20$for$u20$i128$GT$3fmt17hdfd3005ee42c0625E"(i128* noalias nonnull readonly align 8 dereferenceable(16) %self, %"core::fmt::Formatter"* noalias nonnull align 4 dereferenceable(36) %f), !dbg !126
  br label %bb11, !dbg !123

bb6:                                              ; preds = %bb3
; call core::fmt::num::<impl core::fmt::UpperHex for i128>::fmt
  %2 = call zeroext i1 @"_ZN4core3fmt3num54_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i128$GT$3fmt17h8d5974fe4f2a5e2aE"(i128* noalias nonnull readonly align 8 dereferenceable(16) %self, %"core::fmt::Formatter"* noalias nonnull align 4 dereferenceable(36) %f), !dbg !127
  br label %bb11, !dbg !123
}

; core::fmt::Arguments::new_v1_formatted
; Function Attrs: inlinehint nofree norecurse nounwind willreturn writeonly
define internal fastcc void @_ZN4core3fmt9Arguments16new_v1_formatted17h3eb2b1d0a534bdb2E(%"core::fmt::Arguments"* noalias nocapture sret(%"core::fmt::Arguments") dereferenceable(24) %0, [0 x { i8*, i32* }]* noalias nonnull readonly align 4 %args.0) unnamed_addr #3 !dbg !128 {
start:
  call void @llvm.dbg.value(metadata [0 x { [0 x i8]*, i32 }]* bitcast (<{ i8*, [4 x i8], i8*, [4 x i8], i8*, [4 x i8], i8*, [4 x i8], i8*, [4 x i8] }>* @alloc2 to [0 x { [0 x i8]*, i32 }]*), metadata !195, metadata !DIExpression(DW_OP_LLVM_fragment, 0, 32)), !dbg !198
  call void @llvm.dbg.value(metadata i32 5, metadata !195, metadata !DIExpression(DW_OP_LLVM_fragment, 32, 32)), !dbg !198
  call void @llvm.dbg.value(metadata [0 x { i8*, i32* }]* %args.0, metadata !196, metadata !DIExpression(DW_OP_LLVM_fragment, 0, 32)), !dbg !198
  call void @llvm.dbg.value(metadata i32 4, metadata !196, metadata !DIExpression(DW_OP_LLVM_fragment, 32, 32)), !dbg !198
  call void @llvm.dbg.value(metadata i32 4, metadata !197, metadata !DIExpression(DW_OP_LLVM_fragment, 32, 32)), !dbg !198
  %1 = bitcast %"core::fmt::Arguments"* %0 to [0 x { [0 x i8]*, i32 }]**, !dbg !199
  store [0 x { [0 x i8]*, i32 }]* bitcast (<{ i8*, [4 x i8], i8*, [4 x i8], i8*, [4 x i8], i8*, [4 x i8], i8*, [4 x i8] }>* @alloc2 to [0 x { [0 x i8]*, i32 }]*), [0 x { [0 x i8]*, i32 }]** %1, align 4, !dbg !199
  %2 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 1, i32 1, !dbg !199
  store i32 5, i32* %2, align 4, !dbg !199
  %3 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 3, i32 0, !dbg !199
  store i32* bitcast (<{ [128 x i8] }>* @alloc41 to i32*), i32** %3, align 4, !dbg !199
  %4 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 3, i32 1, !dbg !199
  store i32 4, i32* %4, align 4, !dbg !199
  %5 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 5, i32 0, !dbg !199
  store [0 x { i8*, i32* }]* %args.0, [0 x { i8*, i32* }]** %5, align 4, !dbg !199
  %6 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %0, i32 0, i32 5, i32 1, !dbg !199
  store i32 4, i32* %6, align 4, !dbg !199
  ret void, !dbg !200
}

; core::ptr::drop_in_place<()>
; Function Attrs: inlinehint norecurse nounwind readnone willreturn
define internal void @"_ZN4core3ptr29drop_in_place$LT$$LP$$RP$$GT$17hd540dc9d5e454804E"({}* nocapture %_1) unnamed_addr #4 !dbg !201 {
start:
  call void @llvm.dbg.value(metadata {}* %_1, metadata !208, metadata !DIExpression()), !dbg !211
  ret void, !dbg !212
}

; core::result::Result<T,E>::unwrap
; Function Attrs: inlinehint nounwind
define internal fastcc void @"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h49828c914be60d72E"(i1 zeroext %0) unnamed_addr #1 !dbg !213 {
start:
  %e = alloca {}, align 1
  call void @llvm.dbg.value(metadata i1 %0, metadata !225, metadata !DIExpression(DW_OP_LLVM_convert, 1, DW_ATE_unsigned, DW_OP_LLVM_convert, 8, DW_ATE_unsigned, DW_OP_stack_value)), !dbg !232
  call void @llvm.dbg.declare(metadata {}* %e, metadata !228, metadata !DIExpression()), !dbg !233
  br i1 %0, label %bb1, label %bb3, !dbg !234

bb3:                                              ; preds = %start
  ret void, !dbg !235

bb1:                                              ; preds = %start
; call core::result::unwrap_failed
  call void @_ZN4core6result13unwrap_failed17h65bd3230b49c6117E([0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [43 x i8] }>* @alloc43 to [0 x i8]*), i32 43, {}* nonnull align 1 %e, [3 x i32]* noalias readonly align 4 dereferenceable(12) bitcast ({ void ({}*)*, i32, i32, i1 ({}*, %"core::fmt::Formatter"*)* }* @vtable.0 to [3 x i32]*), %"core::panic::Location"* noalias readonly align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc45 to %"core::panic::Location"*)), !dbg !236
  unreachable, !dbg !236
}

; Function Attrs: noreturn nounwind
define dso_local void @main() unnamed_addr #5 !dbg !237 {
start:
; call bug_example::__cortex_m_rt_main
  call fastcc void @_ZN11bug_example18__cortex_m_rt_main17h2f141ec759b6b8e2E(), !dbg !242
  unreachable, !dbg !242
}

; bug_example::__cortex_m_rt_main
; Function Attrs: noreturn nounwind
define internal fastcc void @_ZN11bug_example18__cortex_m_rt_main17h2f141ec759b6b8e2E() unnamed_addr #5 !dbg !243 {
start:
  %_18 = alloca i128*, align 4
  %_12 = alloca [4 x { i8*, i32* }], align 4
  %_5 = alloca %"core::fmt::Arguments", align 4
  %tmp = alloca i128, align 8
  %0 = bitcast i128* %tmp to i8*, !dbg !257
  call void @llvm.lifetime.start.p0i8(i64 16, i8* nonnull %0), !dbg !257
  call void @llvm.dbg.value(metadata i128 1, metadata !245, metadata !DIExpression()), !dbg !258
  store i128 1, i128* %tmp, align 8, !dbg !257
  %1 = bitcast %"core::fmt::Arguments"* %_5 to i8*, !dbg !259
  call void @llvm.lifetime.start.p0i8(i64 24, i8* nonnull %1), !dbg !259
  %2 = bitcast [4 x { i8*, i32* }]* %_12 to i8*, !dbg !259
  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %2), !dbg !259
  %3 = bitcast i128** %_18 to i8*, !dbg !259
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %3), !dbg !259
  store i128* %tmp, i128** %_18, align 4, !dbg !259
  call void @llvm.dbg.value(metadata { [0 x i8]*, i32 }* bitcast (<{ i8*, [4 x i8] }>* @alloc9 to { [0 x i8]*, i32 }*), metadata !249, metadata !DIExpression()), !dbg !260
  call void @llvm.dbg.value(metadata i32* bitcast (<{ [4 x i8] }>* @alloc11 to i32*), metadata !253, metadata !DIExpression()), !dbg !260
  call void @llvm.dbg.value(metadata { [0 x i8]*, i32 }* bitcast (<{ i8*, [4 x i8] }>* @alloc14 to { [0 x i8]*, i32 }*), metadata !255, metadata !DIExpression()), !dbg !260
  call void @llvm.dbg.value(metadata i128** %_18, metadata !256, metadata !DIExpression()), !dbg !260
; call core::fmt::ArgumentV1::new
  %4 = call { i8*, i32* } @_ZN4core3fmt10ArgumentV13new17h3b48c65fd8d4eabdE({ [0 x i8]*, i32 }* noalias readonly align 4 dereferenceable(8) bitcast (<{ i8*, [4 x i8] }>* @alloc9 to { [0 x i8]*, i32 }*), i1 ({ [0 x i8]*, i32 }*, %"core::fmt::Formatter"*)* nonnull @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h105553cee7bcc434E"), !dbg !261
  %_23.0 = extractvalue { i8*, i32* } %4, 0, !dbg !261
  %_23.1 = extractvalue { i8*, i32* } %4, 1, !dbg !261
; call core::fmt::ArgumentV1::new
  %5 = call { i8*, i32* } @_ZN4core3fmt10ArgumentV13new17hdb6e74f41fa81aecE(i32* noalias readonly align 4 dereferenceable(4) bitcast (<{ [4 x i8] }>* @alloc11 to i32*), i1 (i32*, %"core::fmt::Formatter"*)* nonnull @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h4006b789e2d038b0E"), !dbg !261
  %_26.0 = extractvalue { i8*, i32* } %5, 0, !dbg !261
  %_26.1 = extractvalue { i8*, i32* } %5, 1, !dbg !261
; call core::fmt::ArgumentV1::new
  %6 = call { i8*, i32* } @_ZN4core3fmt10ArgumentV13new17h3b48c65fd8d4eabdE({ [0 x i8]*, i32 }* noalias readonly align 4 dereferenceable(8) bitcast (<{ i8*, [4 x i8] }>* @alloc14 to { [0 x i8]*, i32 }*), i1 ({ [0 x i8]*, i32 }*, %"core::fmt::Formatter"*)* nonnull @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h105553cee7bcc434E"), !dbg !261
  %_29.0 = extractvalue { i8*, i32* } %6, 0, !dbg !261
  %_29.1 = extractvalue { i8*, i32* } %6, 1, !dbg !261
; call core::fmt::ArgumentV1::new
  %7 = call fastcc i8* @_ZN4core3fmt10ArgumentV13new17h91b00cee34599045E(i128** noalias nonnull readonly align 4 dereferenceable(4) %_18), !dbg !261
  %8 = getelementptr inbounds [4 x { i8*, i32* }], [4 x { i8*, i32* }]* %_12, i32 0, i32 0, i32 0, !dbg !261
  store i8* %_23.0, i8** %8, align 4, !dbg !261
  %9 = getelementptr inbounds [4 x { i8*, i32* }], [4 x { i8*, i32* }]* %_12, i32 0, i32 0, i32 1, !dbg !261
  store i32* %_23.1, i32** %9, align 4, !dbg !261
  %10 = getelementptr inbounds [4 x { i8*, i32* }], [4 x { i8*, i32* }]* %_12, i32 0, i32 1, i32 0, !dbg !261
  store i8* %_26.0, i8** %10, align 4, !dbg !261
  %11 = getelementptr inbounds [4 x { i8*, i32* }], [4 x { i8*, i32* }]* %_12, i32 0, i32 1, i32 1, !dbg !261
  store i32* %_26.1, i32** %11, align 4, !dbg !261
  %12 = getelementptr inbounds [4 x { i8*, i32* }], [4 x { i8*, i32* }]* %_12, i32 0, i32 2, i32 0, !dbg !261
  store i8* %_29.0, i8** %12, align 4, !dbg !261
  %13 = getelementptr inbounds [4 x { i8*, i32* }], [4 x { i8*, i32* }]* %_12, i32 0, i32 2, i32 1, !dbg !261
  store i32* %_29.1, i32** %13, align 4, !dbg !261
  %14 = getelementptr inbounds [4 x { i8*, i32* }], [4 x { i8*, i32* }]* %_12, i32 0, i32 3, i32 0, !dbg !261
  store i8* %7, i8** %14, align 4, !dbg !261
  %15 = getelementptr inbounds [4 x { i8*, i32* }], [4 x { i8*, i32* }]* %_12, i32 0, i32 3, i32 1, !dbg !261
  store i32* bitcast (i1 (i128**, %"core::fmt::Formatter"*)* @"_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf2e383e57d7cd2f0E" to i32*), i32** %15, align 4, !dbg !261
  %_9.0 = bitcast [4 x { i8*, i32* }]* %_12 to [0 x { i8*, i32* }]*, !dbg !259
; call core::fmt::Arguments::new_v1_formatted
  call fastcc void @_ZN4core3fmt9Arguments16new_v1_formatted17h3eb2b1d0a534bdb2E(%"core::fmt::Arguments"* noalias nocapture nonnull sret(%"core::fmt::Arguments") dereferenceable(24) %_5, [0 x { i8*, i32* }]* noalias nonnull readonly align 4 %_9.0), !dbg !259
; call cortex_m_semihosting::export::hstderr_fmt
  %_4 = call zeroext i1 @_ZN20cortex_m_semihosting6export11hstderr_fmt17h086ab563a8dc96fbE(%"core::fmt::Arguments"* noalias nocapture nonnull dereferenceable(24) %_5), !dbg !259
  call void @llvm.lifetime.end.p0i8(i64 24, i8* nonnull %1), !dbg !259
; call core::result::Result<T,E>::unwrap
  call fastcc void @"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h49828c914be60d72E"(i1 zeroext %_4), !dbg !259
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %3), !dbg !259
  call void @llvm.lifetime.end.p0i8(i64 32, i8* nonnull %2), !dbg !259
  call void @llvm.lifetime.end.p0i8(i64 16, i8* nonnull %0), !dbg !257
  br label %bb8, !dbg !262

bb8:                                              ; preds = %bb8, %start
  br label %bb8, !dbg !262
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #6

; core::fmt::Formatter::pad
; Function Attrs: nounwind
declare dso_local zeroext i1 @_ZN4core3fmt9Formatter3pad17hcbeeb976699479d7E(%"core::fmt::Formatter"* noalias align 4 dereferenceable(36), [0 x i8]* noalias nonnull readonly align 1, i32) unnamed_addr #0

; Function Attrs: argmemonly nofree nosync nounwind willreturn
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #7

; Function Attrs: argmemonly nofree nosync nounwind willreturn
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #7

; core::fmt::Formatter::debug_lower_hex
; Function Attrs: nounwind
declare dso_local zeroext i1 @_ZN4core3fmt9Formatter15debug_lower_hex17h8d0023f0b6bcb4e2E(%"core::fmt::Formatter"* noalias readonly align 4 dereferenceable(36)) unnamed_addr #0

; core::fmt::num::<impl core::fmt::LowerHex for i128>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN4core3fmt3num54_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i128$GT$3fmt17hfad7c627e744b3baE"(i128* noalias readonly align 8 dereferenceable(16), %"core::fmt::Formatter"* noalias align 4 dereferenceable(36)) unnamed_addr #0

; core::fmt::Formatter::debug_upper_hex
; Function Attrs: nounwind
declare dso_local zeroext i1 @_ZN4core3fmt9Formatter15debug_upper_hex17h55d5da0528680253E(%"core::fmt::Formatter"* noalias readonly align 4 dereferenceable(36)) unnamed_addr #0

; core::fmt::num::<impl core::fmt::UpperHex for i128>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN4core3fmt3num54_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i128$GT$3fmt17h8d5974fe4f2a5e2aE"(i128* noalias readonly align 8 dereferenceable(16), %"core::fmt::Formatter"* noalias align 4 dereferenceable(36)) unnamed_addr #0

; core::fmt::num::<impl core::fmt::Display for i128>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..Display$u20$for$u20$i128$GT$3fmt17hdfd3005ee42c0625E"(i128* noalias readonly align 8 dereferenceable(16), %"core::fmt::Formatter"* noalias align 4 dereferenceable(36)) unnamed_addr #0

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn nounwind
declare dso_local void @_ZN4core6result13unwrap_failed17h65bd3230b49c6117E([0 x i8]* noalias nonnull readonly align 1, i32, {}* nonnull align 1, [3 x i32]* noalias readonly align 4 dereferenceable(12), %"core::panic::Location"* noalias readonly align 4 dereferenceable(16)) unnamed_addr #8

; <&T as core::fmt::Display>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h105553cee7bcc434E"({ [0 x i8]*, i32 }* noalias readonly align 4 dereferenceable(8), %"core::fmt::Formatter"* noalias align 4 dereferenceable(36)) unnamed_addr #0

; core::fmt::ArgumentV1::new
; Function Attrs: nounwind
declare dso_local { i8*, i32* } @_ZN4core3fmt10ArgumentV13new17h3b48c65fd8d4eabdE({ [0 x i8]*, i32 }* noalias readonly align 4 dereferenceable(8), i1 ({ [0 x i8]*, i32 }*, %"core::fmt::Formatter"*)* nonnull) unnamed_addr #0

; core::fmt::num::imp::<impl core::fmt::Display for u32>::fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h4006b789e2d038b0E"(i32* noalias readonly align 4 dereferenceable(4), %"core::fmt::Formatter"* noalias align 4 dereferenceable(36)) unnamed_addr #0

; core::fmt::ArgumentV1::new
; Function Attrs: nounwind
declare dso_local { i8*, i32* } @_ZN4core3fmt10ArgumentV13new17hdb6e74f41fa81aecE(i32* noalias readonly align 4 dereferenceable(4), i1 (i32*, %"core::fmt::Formatter"*)* nonnull) unnamed_addr #0

; cortex_m_semihosting::export::hstderr_fmt
; Function Attrs: nounwind
declare dso_local zeroext i1 @_ZN20cortex_m_semihosting6export11hstderr_fmt17h086ab563a8dc96fbE(%"core::fmt::Arguments"* noalias nocapture dereferenceable(24)) unnamed_addr #0

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.value(metadata, metadata, metadata) #6

attributes #0 = { nounwind "frame-pointer"="all" "target-cpu"="generic" }
attributes #1 = { inlinehint nounwind "frame-pointer"="all" "target-cpu"="generic" }
attributes #2 = { norecurse nounwind readnone willreturn "frame-pointer"="all" "target-cpu"="generic" }
attributes #3 = { inlinehint nofree norecurse nounwind willreturn writeonly "frame-pointer"="all" "target-cpu"="generic" }
attributes #4 = { inlinehint norecurse nounwind readnone willreturn "frame-pointer"="all" "target-cpu"="generic" }
attributes #5 = { noreturn nounwind "frame-pointer"="all" "target-cpu"="generic" }
attributes #6 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #7 = { argmemonly nofree nosync nounwind willreturn }
attributes #8 = { cold noinline noreturn nounwind "frame-pointer"="all" "target-cpu"="generic" }

!llvm.dbg.cu = !{!6}
!llvm.module.flags = !{!26}

!0 = !DIGlobalVariableExpression(var: !1, expr: !DIExpression())
!1 = distinct !DIGlobalVariable(name: "vtable", scope: null, file: !2, type: !3, isLocal: true, isDefinition: true)
!2 = !DIFile(filename: "<unknown>", directory: "")
!3 = !DICompositeType(tag: DW_TAG_structure_type, name: "vtable", file: !2, align: 32, flags: DIFlagArtificial, elements: !4, vtableHolder: !5, identifier: "vtable")
!4 = !{}
!5 = !DIBasicType(name: "()", encoding: DW_ATE_unsigned)
!6 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !7, producer: "clang LLVM (rustc version 1.54.0-nightly (c79419af0 2021-06-04))", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, enums: !8, globals: !25)
!7 = !DIFile(filename: "src/main.rs", directory: "/data/Documents/EarlStTech/Code/bug-example")
!8 = !{!9, !16}
!9 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Result", scope: !10, file: !2, baseType: !12, size: 8, align: 8, flags: DIFlagEnumClass, elements: !13)
!10 = !DINamespace(name: "result", scope: !11)
!11 = !DINamespace(name: "core", scope: null)
!12 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
!13 = !{!14, !15}
!14 = !DIEnumerator(name: "Ok", value: 0)
!15 = !DIEnumerator(name: "Err", value: 1)
!16 = !DICompositeType(tag: DW_TAG_enumeration_type, name: "Alignment", scope: !17, file: !2, baseType: !12, size: 8, align: 8, flags: DIFlagEnumClass, elements: !20)
!17 = !DINamespace(name: "v1", scope: !18)
!18 = !DINamespace(name: "rt", scope: !19)
!19 = !DINamespace(name: "fmt", scope: !11)
!20 = !{!21, !22, !23, !24}
!21 = !DIEnumerator(name: "Left", value: 0)
!22 = !DIEnumerator(name: "Right", value: 1)
!23 = !DIEnumerator(name: "Center", value: 2)
!24 = !DIEnumerator(name: "Unknown", value: 3)
!25 = !{!0}
!26 = !{i32 2, !"Debug Info Version", i32 3}
!27 = distinct !DISubprogram(name: "fmt<i128>", linkageName: "_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf2e383e57d7cd2f0E", scope: !29, file: !28, line: 2031, type: !30, scopeLine: 2031, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !6, templateParams: !73, retainedNodes: !70)
!28 = !DIFile(filename: "/home/nmertin/.rustup/toolchains/nightly-2021-06-05-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "4b6e9db3564a56632b2a6241d583bc4b")
!29 = !DINamespace(name: "{{impl}}", scope: !19)
!30 = !DISubroutineType(types: !31)
!31 = !{!9, !32, !35}
!32 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&&i128", baseType: !33, size: 32, align: 32, dwarfAddressSpace: 0)
!33 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&i128", baseType: !34, size: 32, align: 32, dwarfAddressSpace: 0)
!34 = !DIBasicType(name: "i128", size: 128, encoding: DW_ATE_signed)
!35 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut core::fmt::Formatter", baseType: !36, size: 32, align: 32, dwarfAddressSpace: 0)
!36 = !DICompositeType(tag: DW_TAG_structure_type, name: "Formatter", scope: !19, file: !2, size: 288, align: 32, elements: !37, templateParams: !4, identifier: "37f9022a79d5a7d13b2f1e3a603cebf")
!37 = !{!38, !40, !42, !43, !59, !60}
!38 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !36, file: !2, baseType: !39, size: 32, align: 32)
!39 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!40 = !DIDerivedType(tag: DW_TAG_member, name: "fill", scope: !36, file: !2, baseType: !41, size: 32, align: 32, offset: 32)
!41 = !DIBasicType(name: "char", size: 32, encoding: DW_ATE_unsigned_char)
!42 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !36, file: !2, baseType: !16, size: 8, align: 8, offset: 256)
!43 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !36, file: !2, baseType: !44, size: 64, align: 32, offset: 64)
!44 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<usize>", scope: !45, file: !2, size: 64, align: 32, elements: !46, identifier: "d06364e5c039a40e6e3ed7866786c11")
!45 = !DINamespace(name: "option", scope: !11)
!46 = !{!47}
!47 = !DICompositeType(tag: DW_TAG_variant_part, scope: !45, file: !2, size: 64, align: 32, elements: !48, templateParams: !51, identifier: "d06364e5c039a40e6e3ed7866786c11_variant_part", discriminator: !58)
!48 = !{!49, !54}
!49 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !47, file: !2, baseType: !50, size: 64, align: 32, extraData: i64 0)
!50 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !44, file: !2, size: 64, align: 32, elements: !4, templateParams: !51, identifier: "d06364e5c039a40e6e3ed7866786c11::None")
!51 = !{!52}
!52 = !DITemplateTypeParameter(name: "T", type: !53)
!53 = !DIBasicType(name: "usize", size: 32, encoding: DW_ATE_unsigned)
!54 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !47, file: !2, baseType: !55, size: 64, align: 32, extraData: i64 1)
!55 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !44, file: !2, size: 64, align: 32, elements: !56, templateParams: !51, identifier: "d06364e5c039a40e6e3ed7866786c11::Some")
!56 = !{!57}
!57 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !55, file: !2, baseType: !53, size: 32, align: 32, offset: 32)
!58 = !DIDerivedType(tag: DW_TAG_member, scope: !45, file: !2, baseType: !39, size: 32, align: 32, flags: DIFlagArtificial)
!59 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !36, file: !2, baseType: !44, size: 64, align: 32, offset: 128)
!60 = !DIDerivedType(tag: DW_TAG_member, name: "buf", scope: !36, file: !2, baseType: !61, size: 64, align: 32, offset: 192)
!61 = !DICompositeType(tag: DW_TAG_structure_type, name: "&mut Write", scope: !19, file: !2, size: 64, align: 32, elements: !62, templateParams: !4, identifier: "a3a63fe2288d321a9a96975e15f987dc")
!62 = !{!63, !65}
!63 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", scope: !61, file: !2, baseType: !64, size: 32, align: 32, flags: DIFlagArtificial)
!64 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut u8", baseType: !12, size: 32, align: 32, dwarfAddressSpace: 0)
!65 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", scope: !61, file: !2, baseType: !66, size: 32, align: 32, offset: 32, flags: DIFlagArtificial)
!66 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 3]", baseType: !67, size: 32, align: 32, dwarfAddressSpace: 0)
!67 = !DICompositeType(tag: DW_TAG_array_type, baseType: !53, size: 96, align: 32, elements: !68)
!68 = !{!69}
!69 = !DISubrange(count: 3, lowerBound: 0)
!70 = !{!71, !72}
!71 = !DILocalVariable(name: "self", arg: 1, scope: !27, file: !28, line: 2031, type: !32)
!72 = !DILocalVariable(name: "f", arg: 2, scope: !27, file: !28, line: 2031, type: !35)
!73 = !{!74}
!74 = !DITemplateTypeParameter(name: "T", type: !34)
!75 = !DILocation(line: 0, scope: !27)
!76 = !DILocation(line: 2031, column: 71, scope: !27)
!77 = !DILocation(line: 2031, column: 62, scope: !27)
!78 = !DILocation(line: 2031, column: 84, scope: !27)
!79 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN45_$LT$$LP$$RP$$u20$as$u20$core..fmt..Debug$GT$3fmt17h145b8a12344daa9aE", scope: !29, file: !28, line: 2235, type: !80, scopeLine: 2235, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !6, templateParams: !4, retainedNodes: !83)
!80 = !DISubroutineType(types: !81)
!81 = !{!9, !82, !35}
!82 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&()", baseType: !5, size: 32, align: 32, dwarfAddressSpace: 0)
!83 = !{!84, !85}
!84 = !DILocalVariable(name: "self", arg: 1, scope: !79, file: !28, line: 2235, type: !82)
!85 = !DILocalVariable(name: "f", arg: 2, scope: !79, file: !28, line: 2235, type: !35)
!86 = !DILocation(line: 0, scope: !79)
!87 = !DILocation(line: 2236, column: 9, scope: !79)
!88 = !DILocation(line: 2237, column: 6, scope: !79)
!89 = distinct !DISubprogram(name: "new<&i128>", linkageName: "_ZN4core3fmt10ArgumentV13new17h91b00cee34599045E", scope: !90, file: !28, line: 291, type: !99, scopeLine: 291, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !6, templateParams: !105, retainedNodes: !102)
!90 = !DICompositeType(tag: DW_TAG_structure_type, name: "ArgumentV1", scope: !19, file: !2, size: 64, align: 32, elements: !91, templateParams: !4, identifier: "8adc6ba7da683cfc96f55696b44147e4")
!91 = !{!92, !95}
!92 = !DIDerivedType(tag: DW_TAG_member, name: "value", scope: !90, file: !2, baseType: !93, size: 32, align: 32)
!93 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::fmt::::Opaque", baseType: !94, size: 32, align: 32, dwarfAddressSpace: 0)
!94 = !DICompositeType(tag: DW_TAG_structure_type, name: "Opaque", file: !2, align: 8, elements: !4, identifier: "6d898fe2bd0cd930417838be4992b0a2")
!95 = !DIDerivedType(tag: DW_TAG_member, name: "formatter", scope: !90, file: !2, baseType: !96, size: 32, align: 32, offset: 32)
!96 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn(&core::fmt::::Opaque, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !97, size: 32, align: 32, dwarfAddressSpace: 0)
!97 = !DISubroutineType(types: !98)
!98 = !{!9, !93, !35}
!99 = !DISubroutineType(types: !100)
!100 = !{!90, !32, !101}
!101 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "fn(&&i128, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>", baseType: !30, size: 32, align: 32, dwarfAddressSpace: 0)
!102 = !{!103, !104}
!103 = !DILocalVariable(name: "x", arg: 1, scope: !89, file: !28, line: 291, type: !32)
!104 = !DILocalVariable(name: "f", arg: 2, scope: !89, file: !28, line: 291, type: !101)
!105 = !{!106}
!106 = !DITemplateTypeParameter(name: "T", type: !33)
!107 = !DILocation(line: 0, scope: !89)
!108 = !DILocation(line: 300, column: 18, scope: !89)
!109 = !DILocation(line: 301, column: 6, scope: !89)
!110 = distinct !DISubprogram(name: "fmt", linkageName: "_ZN4core3fmt3num51_$LT$impl$u20$core..fmt..Debug$u20$for$u20$i128$GT$3fmt17h98aecc833f509da7E", scope: !112, file: !111, line: 185, type: !114, scopeLine: 185, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !6, templateParams: !4, retainedNodes: !116)
!111 = !DIFile(filename: "/home/nmertin/.rustup/toolchains/nightly-2021-06-05-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/num.rs", directory: "", checksumkind: CSK_MD5, checksum: "9015781b6b0707b2f9013dc3bf8db592")
!112 = !DINamespace(name: "{{impl}}", scope: !113)
!113 = !DINamespace(name: "num", scope: !19)
!114 = !DISubroutineType(types: !115)
!115 = !{!9, !33, !35}
!116 = !{!117, !118}
!117 = !DILocalVariable(name: "self", arg: 1, scope: !110, file: !111, line: 185, type: !33)
!118 = !DILocalVariable(name: "f", arg: 2, scope: !110, file: !111, line: 185, type: !35)
!119 = !DILocation(line: 0, scope: !110)
!120 = !DILocation(line: 186, column: 20, scope: !110)
!121 = !DILocation(line: 186, column: 17, scope: !110)
!122 = !DILocation(line: 188, column: 27, scope: !110)
!123 = !DILocation(line: 188, column: 24, scope: !110)
!124 = !DILocation(line: 187, column: 21, scope: !110)
!125 = !DILocation(line: 193, column: 14, scope: !110)
!126 = !DILocation(line: 191, column: 21, scope: !110)
!127 = !DILocation(line: 189, column: 21, scope: !110)
!128 = distinct !DISubprogram(name: "new_v1_formatted", linkageName: "_ZN4core3fmt9Arguments16new_v1_formatted17h3eb2b1d0a534bdb2E", scope: !129, file: !28, line: 350, type: !192, scopeLine: 350, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !6, templateParams: !4, retainedNodes: !194)
!129 = !DICompositeType(tag: DW_TAG_structure_type, name: "Arguments", scope: !19, file: !2, size: 192, align: 32, elements: !130, templateParams: !4, identifier: "dbcc1f48a752220472fb4a91a8f64147")
!130 = !{!131, !142, !186}
!131 = !DIDerivedType(tag: DW_TAG_member, name: "pieces", scope: !129, file: !2, baseType: !132, size: 64, align: 32)
!132 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[&str]", file: !2, size: 64, align: 32, elements: !133, templateParams: !4, identifier: "e5181a2ba73cefd2b9372dc5646453a9")
!133 = !{!134, !141}
!134 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !132, file: !2, baseType: !135, size: 32, align: 32)
!135 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const &str", baseType: !136, size: 32, align: 32, dwarfAddressSpace: 0)
!136 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !2, size: 64, align: 32, elements: !137, templateParams: !4, identifier: "7ef2a91eecc7bcf4b4aaea2dbce79437")
!137 = !{!138, !140}
!138 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !136, file: !2, baseType: !139, size: 32, align: 32)
!139 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !12, size: 32, align: 32, dwarfAddressSpace: 0)
!140 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !136, file: !2, baseType: !53, size: 32, align: 32, offset: 32)
!141 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !132, file: !2, baseType: !53, size: 32, align: 32, offset: 32)
!142 = !DIDerivedType(tag: DW_TAG_member, name: "fmt", scope: !129, file: !2, baseType: !143, size: 64, align: 32, offset: 64)
!143 = !DICompositeType(tag: DW_TAG_structure_type, name: "Option<&[core::fmt::rt::v1::Argument]>", scope: !45, file: !2, size: 64, align: 32, elements: !144, identifier: "d5f08358c5ba236bccfec2cc4b044ca5")
!144 = !{!145}
!145 = !DICompositeType(tag: DW_TAG_variant_part, scope: !45, file: !2, size: 64, align: 32, elements: !146, templateParams: !149, identifier: "d5f08358c5ba236bccfec2cc4b044ca5_variant_part", discriminator: !58)
!146 = !{!147, !182}
!147 = !DIDerivedType(tag: DW_TAG_member, name: "None", scope: !145, file: !2, baseType: !148, size: 64, align: 32, extraData: i64 0)
!148 = !DICompositeType(tag: DW_TAG_structure_type, name: "None", scope: !143, file: !2, size: 64, align: 32, elements: !4, templateParams: !149, identifier: "d5f08358c5ba236bccfec2cc4b044ca5::None")
!149 = !{!150}
!150 = !DITemplateTypeParameter(name: "T", type: !151)
!151 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::rt::v1::Argument]", file: !2, size: 64, align: 32, elements: !152, templateParams: !4, identifier: "956793386a1d274354ae250d5d6022ab")
!152 = !{!153, !181}
!153 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !151, file: !2, baseType: !154, size: 32, align: 32)
!154 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const core::fmt::rt::v1::Argument", baseType: !155, size: 32, align: 32, dwarfAddressSpace: 0)
!155 = !DICompositeType(tag: DW_TAG_structure_type, name: "Argument", scope: !17, file: !2, size: 256, align: 32, elements: !156, templateParams: !4, identifier: "832b9612a647a5007562f02519bfe9e8")
!156 = !{!157, !158}
!157 = !DIDerivedType(tag: DW_TAG_member, name: "position", scope: !155, file: !2, baseType: !53, size: 32, align: 32)
!158 = !DIDerivedType(tag: DW_TAG_member, name: "format", scope: !155, file: !2, baseType: !159, size: 224, align: 32, offset: 32)
!159 = !DICompositeType(tag: DW_TAG_structure_type, name: "FormatSpec", scope: !17, file: !2, size: 224, align: 32, elements: !160, templateParams: !4, identifier: "fb182e15bc007ffb3b07228d20fef49d")
!160 = !{!161, !162, !163, !164, !180}
!161 = !DIDerivedType(tag: DW_TAG_member, name: "fill", scope: !159, file: !2, baseType: !41, size: 32, align: 32)
!162 = !DIDerivedType(tag: DW_TAG_member, name: "align", scope: !159, file: !2, baseType: !16, size: 8, align: 8, offset: 192)
!163 = !DIDerivedType(tag: DW_TAG_member, name: "flags", scope: !159, file: !2, baseType: !39, size: 32, align: 32, offset: 32)
!164 = !DIDerivedType(tag: DW_TAG_member, name: "precision", scope: !159, file: !2, baseType: !165, size: 64, align: 32, offset: 64)
!165 = !DICompositeType(tag: DW_TAG_structure_type, name: "Count", scope: !17, file: !2, size: 64, align: 32, elements: !166, identifier: "9a8cb257d3e48854eaf4f64dd202e170")
!166 = !{!167}
!167 = !DICompositeType(tag: DW_TAG_variant_part, scope: !17, file: !2, size: 64, align: 32, elements: !168, templateParams: !4, identifier: "9a8cb257d3e48854eaf4f64dd202e170_variant_part", discriminator: !179)
!168 = !{!169, !173, !177}
!169 = !DIDerivedType(tag: DW_TAG_member, name: "Is", scope: !167, file: !2, baseType: !170, size: 64, align: 32, extraData: i64 0)
!170 = !DICompositeType(tag: DW_TAG_structure_type, name: "Is", scope: !165, file: !2, size: 64, align: 32, elements: !171, templateParams: !4, identifier: "9a8cb257d3e48854eaf4f64dd202e170::Is")
!171 = !{!172}
!172 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !170, file: !2, baseType: !53, size: 32, align: 32, offset: 32)
!173 = !DIDerivedType(tag: DW_TAG_member, name: "Param", scope: !167, file: !2, baseType: !174, size: 64, align: 32, extraData: i64 1)
!174 = !DICompositeType(tag: DW_TAG_structure_type, name: "Param", scope: !165, file: !2, size: 64, align: 32, elements: !175, templateParams: !4, identifier: "9a8cb257d3e48854eaf4f64dd202e170::Param")
!175 = !{!176}
!176 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !174, file: !2, baseType: !53, size: 32, align: 32, offset: 32)
!177 = !DIDerivedType(tag: DW_TAG_member, name: "Implied", scope: !167, file: !2, baseType: !178, size: 64, align: 32, extraData: i64 2)
!178 = !DICompositeType(tag: DW_TAG_structure_type, name: "Implied", scope: !165, file: !2, size: 64, align: 32, elements: !4, templateParams: !4, identifier: "9a8cb257d3e48854eaf4f64dd202e170::Implied")
!179 = !DIDerivedType(tag: DW_TAG_member, scope: !17, file: !2, baseType: !39, size: 32, align: 32, flags: DIFlagArtificial)
!180 = !DIDerivedType(tag: DW_TAG_member, name: "width", scope: !159, file: !2, baseType: !165, size: 64, align: 32, offset: 128)
!181 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !151, file: !2, baseType: !53, size: 32, align: 32, offset: 32)
!182 = !DIDerivedType(tag: DW_TAG_member, name: "Some", scope: !145, file: !2, baseType: !183, size: 64, align: 32)
!183 = !DICompositeType(tag: DW_TAG_structure_type, name: "Some", scope: !143, file: !2, size: 64, align: 32, elements: !184, templateParams: !149, identifier: "d5f08358c5ba236bccfec2cc4b044ca5::Some")
!184 = !{!185}
!185 = !DIDerivedType(tag: DW_TAG_member, name: "__0", scope: !183, file: !2, baseType: !151, size: 64, align: 32)
!186 = !DIDerivedType(tag: DW_TAG_member, name: "args", scope: !129, file: !2, baseType: !187, size: 64, align: 32, offset: 128)
!187 = !DICompositeType(tag: DW_TAG_structure_type, name: "&[core::fmt::ArgumentV1]", file: !2, size: 64, align: 32, elements: !188, templateParams: !4, identifier: "313e6f6b43b29704375294558716cfe2")
!188 = !{!189, !191}
!189 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !187, file: !2, baseType: !190, size: 32, align: 32)
!190 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const core::fmt::ArgumentV1", baseType: !90, size: 32, align: 32, dwarfAddressSpace: 0)
!191 = !DIDerivedType(tag: DW_TAG_member, name: "length", scope: !187, file: !2, baseType: !53, size: 32, align: 32, offset: 32)
!192 = !DISubroutineType(types: !193)
!193 = !{!129, !132, !187, !151}
!194 = !{!195, !196, !197}
!195 = !DILocalVariable(name: "pieces", arg: 1, scope: !128, file: !28, line: 351, type: !132)
!196 = !DILocalVariable(name: "args", arg: 2, scope: !128, file: !28, line: 352, type: !187)
!197 = !DILocalVariable(name: "fmt", arg: 3, scope: !128, file: !28, line: 353, type: !151)
!198 = !DILocation(line: 0, scope: !128)
!199 = !DILocation(line: 355, column: 9, scope: !128)
!200 = !DILocation(line: 356, column: 6, scope: !128)
!201 = distinct !DISubprogram(name: "drop_in_place<()>", linkageName: "_ZN4core3ptr29drop_in_place$LT$$LP$$RP$$GT$17hd540dc9d5e454804E", scope: !203, file: !202, line: 192, type: !204, scopeLine: 192, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !6, templateParams: !209, retainedNodes: !207)
!202 = !DIFile(filename: "/home/nmertin/.rustup/toolchains/nightly-2021-06-05-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs", directory: "", checksumkind: CSK_MD5, checksum: "f3b49df57dda79a6c474c1ac500bad75")
!203 = !DINamespace(name: "ptr", scope: !11)
!204 = !DISubroutineType(types: !205)
!205 = !{null, !206}
!206 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*mut ()", baseType: !5, size: 32, align: 32, dwarfAddressSpace: 0)
!207 = !{!208}
!208 = !DILocalVariable(arg: 1, scope: !201, file: !202, line: 192, type: !206)
!209 = !{!210}
!210 = !DITemplateTypeParameter(name: "T", type: !5)
!211 = !DILocation(line: 0, scope: !201)
!212 = !DILocation(line: 192, column: 1, scope: !201)
!213 = distinct !DISubprogram(name: "unwrap<(),()>", linkageName: "_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17h49828c914be60d72E", scope: !9, file: !214, line: 1034, type: !215, scopeLine: 1034, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !6, templateParams: !230, retainedNodes: !224)
!214 = !DIFile(filename: "/home/nmertin/.rustup/toolchains/nightly-2021-06-05-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs", directory: "", checksumkind: CSK_MD5, checksum: "501da03ef6b59978ba5083fcb39515ae")
!215 = !DISubroutineType(types: !216)
!216 = !{null, !9, !217}
!217 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&core::panic::Location", baseType: !218, size: 32, align: 32, dwarfAddressSpace: 0)
!218 = !DICompositeType(tag: DW_TAG_structure_type, name: "Location", scope: !219, file: !2, size: 128, align: 32, elements: !220, templateParams: !4, identifier: "1a36cb34b9630a59702559ad85402148")
!219 = !DINamespace(name: "panic", scope: !11)
!220 = !{!221, !222, !223}
!221 = !DIDerivedType(tag: DW_TAG_member, name: "file", scope: !218, file: !2, baseType: !136, size: 64, align: 32)
!222 = !DIDerivedType(tag: DW_TAG_member, name: "line", scope: !218, file: !2, baseType: !39, size: 32, align: 32, offset: 64)
!223 = !DIDerivedType(tag: DW_TAG_member, name: "col", scope: !218, file: !2, baseType: !39, size: 32, align: 32, offset: 96)
!224 = !{!225, !226, !228}
!225 = !DILocalVariable(name: "self", arg: 1, scope: !213, file: !214, line: 1034, type: !9)
!226 = !DILocalVariable(name: "t", scope: !227, file: !214, line: 1036, type: !5, align: 1)
!227 = distinct !DILexicalBlock(scope: !213, file: !214, line: 1036, column: 13)
!228 = !DILocalVariable(name: "e", scope: !229, file: !214, line: 1037, type: !5, align: 1)
!229 = distinct !DILexicalBlock(scope: !213, file: !214, line: 1037, column: 13)
!230 = !{!210, !231}
!231 = !DITemplateTypeParameter(name: "E", type: !5)
!232 = !DILocation(line: 0, scope: !213)
!233 = !DILocation(line: 1037, column: 17, scope: !229)
!234 = !DILocation(line: 1036, column: 13, scope: !213)
!235 = !DILocation(line: 1039, column: 6, scope: !213)
!236 = !DILocation(line: 1037, column: 23, scope: !229)
!237 = distinct !DISubprogram(name: "__cortex_m_rt_main_trampoline", linkageName: "main", scope: !239, file: !238, line: 9, type: !240, scopeLine: 9, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !6, templateParams: !4, retainedNodes: !4)
!238 = !DIFile(filename: "src/main.rs", directory: "/data/Documents/EarlStTech/Code/bug-example", checksumkind: CSK_MD5, checksum: "b24cccb5d5d328f6638533cc44e3850c")
!239 = !DINamespace(name: "bug_example", scope: null)
!240 = !DISubroutineType(types: !241)
!241 = !{null}
!242 = !DILocation(line: 9, column: 1, scope: !237)
!243 = distinct !DISubprogram(name: "__cortex_m_rt_main", linkageName: "_ZN11bug_example18__cortex_m_rt_main17h2f141ec759b6b8e2E", scope: !239, file: !238, line: 10, type: !240, scopeLine: 10, flags: DIFlagPrototyped | DIFlagNoReturn, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !6, templateParams: !4, retainedNodes: !244)
!244 = !{!245, !249, !253, !255, !256}
!245 = !DILocalVariable(name: "tmp", scope: !246, file: !238, line: 11, type: !34, align: 8)
!246 = !DILexicalBlockFile(scope: !247, file: !238, discriminator: 0)
!247 = distinct !DILexicalBlock(scope: !243, file: !248, line: 102, column: 13)
!248 = !DIFile(filename: "/home/nmertin/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-semihosting-0.3.7/src/macros.rs", directory: "", checksumkind: CSK_MD5, checksum: "cc22f0eda8963bf086d6da07906ff946")
!249 = !DILocalVariable(name: "arg0", scope: !250, file: !238, line: 11, type: !252, align: 4)
!250 = !DILexicalBlockFile(scope: !251, file: !238, discriminator: 0)
!251 = distinct !DILexicalBlock(scope: !247, file: !248, line: 85, column: 37)
!252 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&&str", baseType: !136, size: 32, align: 32, dwarfAddressSpace: 0)
!253 = !DILocalVariable(name: "arg1", scope: !250, file: !238, line: 11, type: !254, align: 4)
!254 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&u32", baseType: !39, size: 32, align: 32, dwarfAddressSpace: 0)
!255 = !DILocalVariable(name: "arg2", scope: !250, file: !238, line: 11, type: !252, align: 4)
!256 = !DILocalVariable(name: "arg3", scope: !250, file: !238, line: 11, type: !32, align: 4)
!257 = !DILocation(line: 11, column: 5, scope: !243)
!258 = !DILocation(line: 0, scope: !246)
!259 = !DILocation(line: 11, column: 5, scope: !246)
!260 = !DILocation(line: 0, scope: !250)
!261 = !DILocation(line: 11, column: 5, scope: !250)
!262 = !DILocation(line: 12, column: 5, scope: !243)
