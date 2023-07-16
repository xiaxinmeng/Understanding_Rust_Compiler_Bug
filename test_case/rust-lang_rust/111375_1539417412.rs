plain
failures:

---- [codegen] tests/codegen/sanitizer-cfi-emit-type-metadata-trait-objects.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-cfi-emit-type-metadata-trait-objects/sanitizer-cfi-emit-type-metadata-trait-objects.ll" "/checkout/tests/codegen/sanitizer-cfi-emit-type-metadata-trait-objects.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
/checkout/tests/codegen/sanitizer-cfi-emit-type-metadata-trait-objects.rs:31:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/sanitizer-cfi-emit-type-metadata-trait-objects.rs:31:12: error: CHECK: expected string not found in input
 // CHECK: call i1 @llvm.type.test({{i8\*|ptr}} {{%f|%0}}, metadata !"[[TYPE1:[[:print:]]+]]")
Build completed unsuccessfully in 0:13:32
Build completed unsuccessfully in 0:13:32
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-cfi-emit-type-metadata-trait-objects/sanitizer-cfi-emit-type-metadata-trait-objects.ll:36:147: note: scanning from here
define void @_ZN46sanitizer_cfi_emit_type_metadata_trait_objects3bar17h2d74a46ff83af3a4E() unnamed_addr #1 !type !12 !type !13 !type !14 !type !15 {
                                                                                                                                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-cfi-emit-type-metadata-trait-objects/sanitizer-cfi-emit-type-metadata-trait-objects.ll:42:7: note: possible intended match here
 %2 = call i1 @llvm.type.test(i8* %1, metadata !"_ZTSFvu3refIu3dynIu71NtCsjllV26HSHoZ_46sanitizer_cfi_emit_type_metadata_trait_objects6Trait1u6regionEEE")
/checkout/tests/codegen/sanitizer-cfi-emit-type-metadata-trait-objects.rs:41:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/sanitizer-cfi-emit-type-metadata-trait-objects.rs:41:12: error: CHECK: expected string not found in input
 // CHECK: call i1 @llvm.type.test({{i8\*|ptr}} {{%f|%0}}, metadata !"[[TYPE1:[[:print:]]+]]")
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-cfi-emit-type-metadata-trait-objects/sanitizer-cfi-emit-type-metadata-trait-objects.ll:60:126: note: scanning from here
; call <sanitizer_cfi_emit_type_metadata_trait_objects::Type1 as sanitizer_cfi_emit_type_metadata_trait_objects::Trait1>::foo
                                                                                                                             ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-cfi-emit-type-metadata-trait-objects/sanitizer-cfi-emit-type-metadata-trait-objects.ll:77:3: note: possible intended match here
declare i1 @llvm.type.test(i8*, metadata) #2

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-cfi-emit-type-metadata-trait-objects/sanitizer-cfi-emit-type-metadata-trait-objects.ll
Check file: /checkout/tests/codegen/sanitizer-cfi-emit-type-metadata-trait-objects.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'sanitizer_cfi_emit_type_metadata_trait_objects.e150fb1e01a078fa-cgu.0' 
            2: source_filename = "sanitizer_cfi_emit_type_metadata_trait_objects.e150fb1e01a078fa-cgu.0" 
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
            4: target triple = "x86_64-unknown-linux-gnu" 
            5:  
            6: %Type1 = type {} 
            7:  
            8: @vtable.0 = private unnamed_addr constant <{ i8*, [16 x i8], i8* }> <{ i8* bitcast (void (%Type1*)* @"_ZN4core3ptr74drop_in_place$LT$sanitizer_cfi_emit_type_metadata_trait_objects..Type1$GT$17hc1d37d0dc2496dd9E" to i8*), [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", i8* bitcast (void (%Type1*)* @"_ZN128_$LT$sanitizer_cfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_cfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h45b0be674ea95681E" to i8*) }>, align 8 
            9:  
           10: ; core::ptr::drop_in_place<sanitizer_cfi_emit_type_metadata_trait_objects::Type1> 
           11: ; Function Attrs: inlinehint nonlazybind uwtable 
           12: define internal void @"_ZN4core3ptr74drop_in_place$LT$sanitizer_cfi_emit_type_metadata_trait_objects..Type1$GT$17hc1d37d0dc2496dd9E"(%Type1* noundef %_1) unnamed_addr #0 !type !4 !type !5 !type !6 !type !7 { 
           14:  ret void 
           15: } 
           16:  
           16:  
           17: ; <sanitizer_cfi_emit_type_metadata_trait_objects::Type1 as sanitizer_cfi_emit_type_metadata_trait_objects::Trait1>::foo 
           18: ; Function Attrs: nonlazybind uwtable 
           19: define void @"_ZN128_$LT$sanitizer_cfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_cfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h45b0be674ea95681E"(%Type1* noalias noundef nonnull readonly align 1 %self) unnamed_addr #1 !type !8 !type !9 !type !10 !type !11 { 
           21:  ret void 
           22: } 
           23:  
           24: ; sanitizer_cfi_emit_type_metadata_trait_objects::foo 
           24: ; sanitizer_cfi_emit_type_metadata_trait_objects::foo 
           25: ; Function Attrs: nonlazybind uwtable 
           26: define void @_ZN46sanitizer_cfi_emit_type_metadata_trait_objects3foo17h5e89258861d944cfE() unnamed_addr #1 !type !12 !type !13 !type !14 !type !15 { 
           27: start: 
           28:  %_1 = alloca %Type1, align 1 
           29: ; call <sanitizer_cfi_emit_type_metadata_trait_objects::Type1 as sanitizer_cfi_emit_type_metadata_trait_objects::Trait1>::foo 
           30:  call void @"_ZN128_$LT$sanitizer_cfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_cfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h45b0be674ea95681E"(%Type1* noalias noundef nonnull readonly align 1 %_1) 
           31:  ret void 
           32: } 
           33:  
           34: ; sanitizer_cfi_emit_type_metadata_trait_objects::bar 
           35: ; Function Attrs: nonlazybind uwtable 
           36: define void @_ZN46sanitizer_cfi_emit_type_metadata_trait_objects3bar17h2d74a46ff83af3a4E() unnamed_addr #1 !type !12 !type !13 !type !14 !type !15 { 
check:31'0                                                                                                                                                       X~~ error: no match found
           37: start: 
check:31'0     ~~~~~~~
           38:  %_1 = alloca %Type1, align 1 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           39:  %b.0 = bitcast %Type1* %_1 to {}* 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           40:  %0 = load void ({}*)*, void ({}*)** getelementptr inbounds (void ({}*)*, void ({}*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.0 to void ({}*)**), i64 3), align 8, !invariant.load !16, !nonnull !16 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           41:  %1 = bitcast void ({}*)* %0 to i8* 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           42:  %2 = call i1 @llvm.type.test(i8* %1, metadata !"_ZTSFvu3refIu3dynIu71NtCsjllV26HSHoZ_46sanitizer_cfi_emit_type_metadata_trait_objects6Trait1u6regionEEE") 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:31'1           ?                                                                                                                                                     possible intended match
           43:  br i1 %2, label %type_test.pass, label %type_test.fail 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           44:  
check:31'0     ~
           45: type_test.pass: ; preds = %start 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           46:  call void %0({}* noundef align 1 %b.0) 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           47:  ret void 
check:31'0     ~~~~~~~~~~
           48:  
check:31'0     ~
           49: type_test.fail: ; preds = %start 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           50:  call void @llvm.trap() 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           51:  unreachable 
check:31'0     ~~~~~~~~~~~~~
           52: } 
check:31'0     ~~
           53:  
check:31'0     ~
           54: ; sanitizer_cfi_emit_type_metadata_trait_objects::baz 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           55: ; Function Attrs: nonlazybind uwtable 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           56: define void @_ZN46sanitizer_cfi_emit_type_metadata_trait_objects3baz17h92d5c2b1292137eeE() unnamed_addr #1 !type !12 !type !13 !type !14 !type !15 { 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           57: start: 
           58:  %_1 = alloca %Type1, align 1 
           59:  %b.0 = bitcast %Type1* %_1 to {}* 
           60: ; call <sanitizer_cfi_emit_type_metadata_trait_objects::Type1 as sanitizer_cfi_emit_type_metadata_trait_objects::Trait1>::foo 
check:41'0                                                                                                                                  X error: no match found
           61:  call void @"_ZN128_$LT$sanitizer_cfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_cfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h45b0be674ea95681E"(%Type1* noalias noundef nonnull readonly align 1 %_1) 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           62:  %0 = load void ({}*)*, void ({}*)** getelementptr inbounds (void ({}*)*, void ({}*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.0 to void ({}*)**), i64 3), align 8, !invariant.load !16, !nonnull !16 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           63:  %1 = bitcast void ({}*)* %0 to i8* 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           64:  %2 = call i1 @llvm.type.test(i8* %1, metadata !"_ZTSFvu3refIu3dynIu71NtCsjllV26HSHoZ_46sanitizer_cfi_emit_type_metadata_trait_objects6Trait1u6regionEEE") 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           65:  br i1 %2, label %type_test.pass, label %type_test.fail 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           66:  
check:41'0     ~
           67: type_test.pass: ; preds = %start 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           68:  call void %0({}* noundef align 1 %b.0) 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           69:  ret void 
check:41'0     ~~~~~~~~~~
           70:  
check:41'0     ~
           71: type_test.fail: ; preds = %start 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           72:  call void @llvm.trap() 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           73:  unreachable 
check:41'0     ~~~~~~~~~~~~~
           74: } 
check:41'0     ~~
           75:  
check:41'0     ~
           76: ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           77: declare i1 @llvm.type.test(i8*, metadata) #2 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:41'1       ?                                           possible intended match
           78:  
check:41'0     ~
           79: ; Function Attrs: cold noreturn nounwind 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           80: declare void @llvm.trap() #3 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           81:  
check:41'0     ~
           82: attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           83: attributes #1 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           84: attributes #2 = { nofree nosync nounwind readnone speculatable willreturn } 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           85: attributes #3 = { cold noreturn nounwind } 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           86:  
check:41'0     ~
           87: !llvm.module.flags = !{!0, !1, !2, !3} 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           88:  
check:41'0     ~
           89: !0 = !{i32 7, !"PIC Level", i32 2} 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           90: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           91: !2 = !{i32 4, !"CFI Canonical Jump Tables", i32 1} 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           92: !3 = !{i32 4, !"EnableSplitLTOUnit", i32 1} 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           93: !4 = !{i64 0, !"_ZTSFvPu70NtCsjllV26HSHoZ_46sanitizer_cfi_emit_type_metadata_trait_objects5Type1E"} 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           94: !5 = !{i64 0, !"_ZTSFvPvE.generalized"} 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           95: !6 = !{i64 0, !"_ZTSFvPu70NtCsjllV26HSHoZ_46sanitizer_cfi_emit_type_metadata_trait_objects5Type1E.normalized"} 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           96: !7 = !{i64 0, !"_ZTSFvPvE.normalized.generalized"} 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           97: !8 = !{i64 0, !"_ZTSFvu3refIu3dynIu71NtCsjllV26HSHoZ_46sanitizer_cfi_emit_type_metadata_trait_objects6Trait1u6regionEEE"} 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           98: !9 = !{i64 0, !"_ZTSFvu3refIvEE.generalized"} 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           99: !10 = !{i64 0, !"_ZTSFvu3refIu3dynIu71NtCsjllV26HSHoZ_46sanitizer_cfi_emit_type_metadata_trait_objects6Trait1u6regionEEE.normalized"} 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          100: !11 = !{i64 0, !"_ZTSFvu3refIvEE.normalized.generalized"} 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          101: !12 = !{i64 0, !"_ZTSFvvE"} 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          102: !13 = !{i64 0, !"_ZTSFvvE.generalized"} 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          103: !14 = !{i64 0, !"_ZTSFvvE.normalized"} 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          104: !15 = !{i64 0, !"_ZTSFvvE.normalized.generalized"} 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          105: !16 = !{} 
check:41'0     ~~~~~~~~~~
------------------------------------------



