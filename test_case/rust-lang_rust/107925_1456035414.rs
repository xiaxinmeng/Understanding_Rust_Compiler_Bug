plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:46007752205b5430f5cabe1357251ea7621a9e98)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
failures:

---- [codegen] tests/codegen/consts.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll" "/checkout/tests/codegen/consts.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/consts.rs:12:11: error: CHECK: expected string not found in input
// CHECK: @alloc_76bfe2f13a3e3b01074971d122eac57e = {{.*}}, align 2
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:10:100: note: scanning from here
@STATIC = constant <{ [4 x i8], [4 x i8] }> <{ [4 x i8] zeroinitializer, [4 x i8] undef }>, align 4
                                                                                                   ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:11:118: note: possible intended match here
@alloc_7fd9a0a0621fe7a79fbbfbf31d29602f = private unnamed_addr constant <{ [3 x i8], [1 x i8] }> <{ [3 x i8] zeroinitializer, [1 x i8] undef }>, align 2
                                                                                                                     ^
/checkout/tests/codegen/consts.rs:45:89: error: undefined variable: LOW_HIGH
 // CHECK: memcpy.{{.+}}({{i8\*|ptr}} align 2 %{{[0-9]+}}, {{i8\*|ptr}} align 2 {{.*}}[[LOW_HIGH]]{{.*}}, i{{(32|64)}} 8, i1 false)
                                                                                        ^
/checkout/tests/codegen/consts.rs:53:89: error: undefined variable: LOW_HIGH
 // CHECK: memcpy.{{.+}}({{i8\*|ptr}} align 4 %{{[0-9]+}}, {{i8\*|ptr}} align 4 {{.*}}[[LOW_HIGH]]{{.*}}, i{{(32|64)}} 8, i1 false)

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll
Check file: /checkout/tests/codegen/consts.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'consts.c5919e7f-cgu.0' 
            2: source_filename = "consts.c5919e7f-cgu.0" 
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
            4: target triple = "x86_64-unknown-linux-gnu" 
            5:  
            6: %"E<i16, i32>" = type { i16, [3 x i16] } 
            7: %"E<i8, i16>" = type { i16, [1 x i16] } 
            8: %"E<i16, [i16; 3]>" = type { i16, [3 x i16] } 
            9:  
           10: @STATIC = constant <{ [4 x i8], [4 x i8] }> <{ [4 x i8] zeroinitializer, [4 x i8] undef }>, align 4 
check:12'0                                                                                                        X error: no match found
           11: @alloc_7fd9a0a0621fe7a79fbbfbf31d29602f = private unnamed_addr constant <{ [3 x i8], [1 x i8] }> <{ [3 x i8] zeroinitializer, [1 x i8] undef }>, align 2 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:12'1                                                                                                                          ?                                    possible intended match
           12: @alloc_200815826f51d420182c3b30d83fc06c = private unnamed_addr constant <{ [4 x i8], [4 x i8] }> <{ [4 x i8] zeroinitializer, [4 x i8] undef }>, align 4 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13:  
check:12'0     ~
           14: ; Function Attrs: nonlazybind uwtable 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           15: define i64 @static_enum_const() unnamed_addr #0 { 
check:12'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           16: start: 
           17:  %0 = alloca %"E<i16, i32>", align 4 
           18:  %1 = bitcast %"E<i16, i32>"* %0 to i8* 
           19:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %1, i8* align 4 getelementptr inbounds (<{ [4 x i8], [4 x i8] }>, <{ [4 x i8], [4 x i8] }>* @STATIC, i32 0, i32 0, i32 0), i64 8, i1 false) 
           20:  %2 = bitcast %"E<i16, i32>"* %0 to i64* 
           21:  %3 = load i64, i64* %2, align 4 
           22:  ret i64 %3 
           23: } 
           24:  
           25: ; Function Attrs: nonlazybind uwtable 
           26: define i32 @inline_enum_const() unnamed_addr #0 { 
           27: start: 
           28:  %0 = alloca %"E<i8, i16>", align 2 
           29:  %1 = bitcast %"E<i8, i16>"* %0 to i8* 
           30:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 2 %1, i8* align 2 getelementptr inbounds (<{ [3 x i8], [1 x i8] }>, <{ [3 x i8], [1 x i8] }>* @alloc_7fd9a0a0621fe7a79fbbfbf31d29602f, i32 0, i32 0, i32 0), i64 4, i1 false) 
           31:  %2 = bitcast %"E<i8, i16>"* %0 to i32* 
           32:  %3 = load i32, i32* %2, align 2 
           33:  ret i32 %3 
           34: } 
           35:  
           36: ; Function Attrs: nonlazybind uwtable 
           37: define i64 @low_align_const() unnamed_addr #0 { 
check:45'0                                X~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:45'1                                                      undefined variable: LOW_HIGH
           38: start: 
check:45'0     ~~~~~~~
           39:  %0 = alloca %"E<i16, [i16; 3]>", align 2 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           40:  %1 = bitcast %"E<i16, [i16; 3]>"* %0 to i8* 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           41:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 2 %1, i8* align 2 getelementptr inbounds (<{ [4 x i8], [4 x i8] }>, <{ [4 x i8], [4 x i8] }>* @alloc_200815826f51d420182c3b30d83fc06c, i32 0, i32 0, i32 0), i64 8, i1 false) 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           42:  %2 = bitcast %"E<i16, [i16; 3]>"* %0 to i64* 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           43:  %3 = load i64, i64* %2, align 2 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           44:  ret i64 %3 
check:45'0     ~~~~~~~~~~~~
           45: } 
check:45'0     ~~
           46:  
check:45'0     ~
           47: ; Function Attrs: nonlazybind uwtable 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           48: define i64 @high_align_const() unnamed_addr #0 { 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:53'0                                 X~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:53'1                                                       undefined variable: LOW_HIGH
           49: start: 
check:53'0     ~~~~~~~
           50:  %0 = alloca %"E<i16, i32>", align 4 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           51:  %1 = bitcast %"E<i16, i32>"* %0 to i8* 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           52:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %1, i8* align 4 getelementptr inbounds (<{ [4 x i8], [4 x i8] }>, <{ [4 x i8], [4 x i8] }>* @alloc_200815826f51d420182c3b30d83fc06c, i32 0, i32 0, i32 0), i64 8, i1 false) 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           53:  %2 = bitcast %"E<i16, i32>"* %0 to i64* 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           54:  %3 = load i64, i64* %2, align 4 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           55:  ret i64 %3 
check:53'0     ~~~~~~~~~~~~
           56: } 
check:53'0     ~~
           57:  
check:53'0     ~
           58: ; Function Attrs: argmemonly nofree nounwind willreturn 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           59: declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #1 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           60:  
check:53'0     ~
           61: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           62: attributes #1 = { argmemonly nofree nounwind willreturn } 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           63:  
check:53'0     ~
           64: !llvm.module.flags = !{!0, !1} 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           65:  
check:53'0     ~
           66: !0 = !{i32 7, !"PIC Level", i32 2} 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           67: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------


---- [codegen] tests/codegen/remap_path_prefix/main.rs stdout ----
---- [codegen] tests/codegen/remap_path_prefix/main.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll" "/checkout/tests/codegen/remap_path_prefix/main.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/remap_path_prefix/main.rs:15:11: error: CHECK: expected string not found in input
// CHECK: @alloc_92a59126a55aa3c0019b6c8a007fe001 = private unnamed_addr constant <{ [34 x i8] }> <{ [34 x i8] c"/the/src/remap_path_prefix/main.rs" }>
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll:1:1: note: scanning from here
; ModuleID = 'main.6225cf88-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll:10:1: note: possible intended match here
@alloc_da8614e7e4c17ddf0fa9f0a1720d07fe = private unnamed_addr constant <{ [34 x i8] }> <{ [34 x i8] c"/the/src/remap_path_prefix/main.rs" }>, align 1

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll
Check file: /checkout/tests/codegen/remap_path_prefix/main.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'main.6225cf88-cgu.0' 
check:15'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "main.6225cf88-cgu.0" 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "x86_64-unknown-linux-gnu" 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:15'0     ~
            6: %"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [2 x i64] } 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            7: %"unwind::libunwind::_Unwind_Context" = type { [0 x i8] } 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            8:  
check:15'0     ~
            9: @vtable.0 = private constant <{ i8*, [16 x i8], i8*, i8*, i8* }> <{ i8* bitcast (void (i64**)* @"_ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17h968111d5a3bb4704E" to i8*), [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", i8* bitcast (i32 (i64**)* @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h7504621ffdd45bc7E" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h37397ad3234a5916E" to i8*), i8* bitcast (i32 (i64**)* @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h37397ad3234a5916E" to i8*) }>, align 8, !dbg !0 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           10: @alloc_da8614e7e4c17ddf0fa9f0a1720d07fe = private unnamed_addr constant <{ [34 x i8] }> <{ [34 x i8] c"/the/src/remap_path_prefix/main.rs" }>, align 1 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:15'1     ?                                                                                                                                                       possible intended match
           11: @_ZN4main9FILE_PATH17hd0c2f73415230fa3E = internal constant <{ i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [34 x i8] }>, <{ [34 x i8] }>* @alloc_da8614e7e4c17ddf0fa9f0a1720d07fe, i32 0, i32 0, i32 0), [8 x i8] c"\22\00\00\00\00\00\00\00" }>, align 8, !dbg !24 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           12: @__rustc_debug_gdb_scripts_section__ = linkonce_odr unnamed_addr constant [34 x i8] c"\01gdb_load_rust_pretty_printers.py\00", section ".debug_gdb_scripts", align 1 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13:  
check:15'0     ~
           14: ; remap_path_prefix_aux::some_aux_function 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           15: ; Function Attrs: inlinehint nonlazybind uwtable 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           16: define internal noundef i32 @_ZN21remap_path_prefix_aux17some_aux_function17haf4b30d88d597ce2E() unnamed_addr #0 !dbg !42 { 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           17: start: 
check:15'0     ~~~~~~~
           18:  ret i32 1234, !dbg !48 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           19: } 
check:15'0     ~~
           20:  
check:15'0     ~
           21: ; std::sys_common::backtrace::__rust_begin_short_backtrace 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           22: ; Function Attrs: noinline nonlazybind uwtable 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           23: define internal void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hc6edc4e068aeda67E(void ()* noundef nonnull %f) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality !dbg !49 { 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           24: start: 
check:15'0     ~~~~~~~
           25:  %0 = alloca { i8*, i32 }, align 8 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           26:  %dummy.dbg.spill = alloca {}, align 1 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           27:  %f.dbg.spill = alloca void ()*, align 8 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           28:  %result.dbg.spill = alloca {}, align 1 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           29:  call void @llvm.dbg.declare(metadata {}* %result.dbg.spill, metadata !57, metadata !DIExpression()), !dbg !62 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           30:  store void ()* %f, void ()** %f.dbg.spill, align 8 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           31:  call void @llvm.dbg.declare(metadata void ()** %f.dbg.spill, metadata !56, metadata !DIExpression()), !dbg !63 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           32:  call void @llvm.dbg.declare(metadata {}* %dummy.dbg.spill, metadata !64, metadata !DIExpression()), !dbg !73 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           33: ; call core::ops::function::FnOnce::call_once 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           34:  call void @_ZN4core3ops8function6FnOnce9call_once17h08cf6b42c56c8bb1E(void ()* noundef nonnull %f), !dbg !75 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           35:  call void asm sideeffect "", "~{memory}"(), !dbg !76, !srcloc !77 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           36:  br label %bb4, !dbg !76 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
           37:  
check:15'0     ~
           38: bb4: ; preds = %start 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~
           39:  ret void, !dbg !78 
check:15'0     ~~~~~~~~~~~~~~~~~~~~
           40:  
check:15'0     ~
           41: bb2: ; No predecessors! 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           42:  %1 = bitcast { i8*, i32 }* %0 to i8**, !dbg !79 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           43:  %2 = load i8*, i8** %1, align 8, !dbg !79, !noundef !23 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           44:  %3 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1, !dbg !79 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           45:  %4 = load i32, i32* %3, align 8, !dbg !79, !noundef !23 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           46:  %5 = bitcast { i8*, i32 }* %0 to i8*, !dbg !79 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           47:  call void @llvm.lifetime.end.p0i8(i64 16, i8* %5), !dbg !79 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           48:  %6 = insertvalue { i8*, i32 } undef, i8* %2, 0, !dbg !79 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           49:  %7 = insertvalue { i8*, i32 } %6, i32 %4, 1, !dbg !79 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           50:  resume { i8*, i32 } %7, !dbg !79 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           51: } 
check:15'0     ~~
           52:  
check:15'0     ~
           53: ; std::rt::lang_start 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~
           54: ; Function Attrs: nonlazybind uwtable 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           55: define hidden noundef i64 @_ZN3std2rt10lang_start17h60e8cc21ef30f346E(void ()* noundef nonnull %main, i64 noundef %argc, i8** noundef %argv, i8 noundef %sigpipe) unnamed_addr #2 !dbg !80 { 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           56: start: 
check:15'0     ~~~~~~~
           57:  %v.dbg.spill = alloca i64, align 8 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           58:  %sigpipe.dbg.spill = alloca i8, align 1 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           59:  %argv.dbg.spill = alloca i8**, align 8 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           60:  %argc.dbg.spill = alloca i64, align 8 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           61:  %main.dbg.spill = alloca void ()*, align 8 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           62:  %_8 = alloca i64*, align 8 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           63:  %_5 = alloca i64, align 8 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
           64:  store void ()* %main, void ()** %main.dbg.spill, align 8 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           65:  call void @llvm.dbg.declare(metadata void ()** %main.dbg.spill, metadata !88, metadata !DIExpression()), !dbg !94 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           66:  store i64 %argc, i64* %argc.dbg.spill, align 8 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           67:  call void @llvm.dbg.declare(metadata i64* %argc.dbg.spill, metadata !89, metadata !DIExpression()), !dbg !95 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           68:  store i8** %argv, i8*** %argv.dbg.spill, align 8 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           69:  call void @llvm.dbg.declare(metadata i8*** %argv.dbg.spill, metadata !90, metadata !DIExpression()), !dbg !96 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           70:  store i8 %sigpipe, i8* %sigpipe.dbg.spill, align 1 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           71:  call void @llvm.dbg.declare(metadata i8* %sigpipe.dbg.spill, metadata !91, metadata !DIExpression()), !dbg !97 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           72:  %0 = bitcast i64* %_5 to i8*, !dbg !98 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           73:  call void @llvm.lifetime.start.p0i8(i64 8, i8* %0), !dbg !98 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           74:  %1 = bitcast i64** %_8 to i8*, !dbg !99 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           75:  call void @llvm.lifetime.start.p0i8(i64 8, i8* %1), !dbg !99 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           76:  %2 = bitcast i64** %_8 to void ()**, !dbg !99 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           77:  store void ()* %main, void ()** %2, align 8, !dbg !99 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           78:  %_6.0 = bitcast i64** %_8 to {}*, !dbg !100 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           79: ; call std::rt::lang_start_internal 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           80:  %3 = call noundef i64 @_ZN3std2rt19lang_start_internal17h242c631dc675c919E({}* noundef nonnull align 1 %_6.0, [3 x i64]* noalias noundef readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8], i8*, i8*, i8* }>* @vtable.0 to [3 x i64]*), i64 noundef %argc, i8** noundef %argv, i8 noundef %sigpipe), !dbg !98 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           81:  store i64 %3, i64* %_5, align 8, !dbg !98 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           82:  %v = load i64, i64* %_5, align 8, !dbg !101, !noundef !23 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           83:  store i64 %v, i64* %v.dbg.spill, align 8, !dbg !101 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           84:  call void @llvm.dbg.declare(metadata i64* %v.dbg.spill, metadata !92, metadata !DIExpression()), !dbg !102 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           85:  %4 = bitcast i64** %_8 to i8*, !dbg !103 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           86:  call void @llvm.lifetime.end.p0i8(i64 8, i8* %4), !dbg !103 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           87:  %5 = bitcast i64* %_5 to i8*, !dbg !103 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           88:  call void @llvm.lifetime.end.p0i8(i64 8, i8* %5), !dbg !103 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           89:  ret i64 %v, !dbg !104 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~
           90: } 
check:15'0     ~~
           91:  
check:15'0     ~
           92: ; std::rt::lang_start::{{closure}} 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           93: ; Function Attrs: inlinehint nonlazybind uwtable 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           94: define internal noundef i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h37397ad3234a5916E"(i64** noalias noundef readonly align 8 dereferenceable(8) %_1) unnamed_addr #0 !dbg !105 { 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           95: start: 
check:15'0     ~~~~~~~
           96:  %self.dbg.spill = alloca i8*, align 8 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           97:  %_1.dbg.spill = alloca i64**, align 8 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           98:  %self = alloca i8, align 1 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           99:  store i64** %_1, i64*** %_1.dbg.spill, align 8 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          100:  call void @llvm.dbg.declare(metadata i64*** %_1.dbg.spill, metadata !110, metadata !DIExpression(DW_OP_deref)), !dbg !111 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          101:  call void @llvm.dbg.declare(metadata i8* %self, metadata !112, metadata !DIExpression()), !dbg !129 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          102:  call void @llvm.lifetime.start.p0i8(i64 1, i8* %self), !dbg !131 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          103:  %0 = bitcast i64** %_1 to void ()**, !dbg !132 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          104:  %_4 = load void ()*, void ()** %0, align 8, !dbg !132, !nonnull !23, !noundef !23 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          105: ; call std::sys_common::backtrace::__rust_begin_short_backtrace 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          106:  call void @_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hc6edc4e068aeda67E(void ()* noundef nonnull %_4), !dbg !131 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          107: ; call <() as std::process::Termination>::report 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          108:  %1 = call noundef i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h233b8c733916e975E"(), !dbg !131 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          109:  store i8 %1, i8* %self, align 1, !dbg !131 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          110:  store i8* %self, i8** %self.dbg.spill, align 8, !dbg !133 
check:15'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
