plain
failures:

---- [codegen] tests/codegen/thread-local.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll" "/checkout/tests/codegen/thread-local.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
/checkout/tests/codegen/thread-local.rs:17:11: error: CHECK: expected string not found in input
/checkout/tests/codegen/thread-local.rs:17:11: error: CHECK: expected string not found in input
// CHECK: [[TLS_AUX:@.+]] = external thread_local local_unnamed_addr global i64
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll:1:1: note: scanning from here
; ModuleID = 'thread_local.214c776032439a7d-cgu.0'
^
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll:10:52: note: possible intended match here
@_ZN16thread_local_aux1A7__getit3VAL17h5f4ac2b045f34f55E = external thread_local global i64
/checkout/tests/codegen/thread-local.rs:23:46: error: undefined variable: TLS
/checkout/tests/codegen/thread-local.rs:23:46: error: undefined variable: TLS
 // CHECK: [[RET_0:%.+]] = load i32, {{.*}}[[TLS]]{{.*}}
                                             ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll:45:2: note: possible intended match here
 %1 = load i32, i32* bitcast (<{ [4 x i8] }>* @_ZN12thread_local1A7__getit3VAL17h9876ec948e5452caE to i32*), align 4, !noundef !3
/checkout/tests/codegen/thread-local.rs:31:34: error: undefined variable: TLS
/checkout/tests/codegen/thread-local.rs:31:34: error: undefined variable: TLS
 // CHECK: store i32 %0, {{.*}}[[TLS]]{{.*}}
                                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll:70:2: note: possible intended match here
 store i32 %0, i32* bitcast (<{ [4 x i8] }>* @_ZN12thread_local1A7__getit3VAL17h9876ec948e5452caE to i32*), align 4, !alias.scope !7, !noalias !10
 ^
/checkout/tests/codegen/thread-local.rs:39:46: error: undefined variable: TLS_AUX
 // CHECK: [[RET_1:%.+]] = load i64, {{.*}}[[TLS_AUX]]
                                             ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll:95:2: note: possible intended match here
 %1 = load i64, i64* @_ZN16thread_local_aux1A7__getit3VAL17h5f4ac2b045f34f55E, align 8, !noundef !3
 ^
/checkout/tests/codegen/thread-local.rs:47:34: error: undefined variable: TLS_AUX
 // CHECK: store i64 %0, {{.*}}[[TLS_AUX]]
                                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll:120:2: note: possible intended match here
 store i64 %0, i64* @_ZN16thread_local_aux1A7__getit3VAL17h5f4ac2b045f34f55E, align 8, !alias.scope !18, !noalias !21

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/thread-local/thread-local.ll
Check file: /checkout/tests/codegen/thread-local.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'thread_local.214c776032439a7d-cgu.0' 
check:17'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "thread_local.214c776032439a7d-cgu.0" 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "x86_64-unknown-linux-gnu" 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:17'0     ~
            6: %"core::panic::location::Location<'_>" = type { { [0 x i8]*, i64 }, i32, i32 } 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            7: %"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [2 x i64] } 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            8: %"unwind::libunwind::_Unwind_Context" = type { [0 x i8] } 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            9:  
check:17'0     ~
           10: @_ZN16thread_local_aux1A7__getit3VAL17h5f4ac2b045f34f55E = external thread_local global i64 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:17'1                                                        ?                                         possible intended match
           11: @alloc_ba00a30ebb2e7122ca26eeaf18c6f93c = private unnamed_addr constant <{ [41 x i8] }> <{ [41 x i8] c"/checkout/library/std/src/thread/local.rs" }>, align 1 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           12: @alloc_7260c71e1a6505bba99fc9cc66893e69 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [41 x i8] }>, <{ [41 x i8] }>* @alloc_ba00a30ebb2e7122ca26eeaf18c6f93c, i32 0, i32 0, i32 0), [16 x i8] c")\00\00\00\00\00\00\00\F6\00\00\00\09\00\00\00" }>, align 8 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13: @alloc_87c007504da6cf6c9c62f16c0e3b77cf = private unnamed_addr constant <{ i8* }> <{ i8* bitcast (i32* (i32*)* @_ZN12thread_local1A7__getit17h82f4a4c0c4ba1d92E to i8*) }>, align 8 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14: @alloc_d58f8a1f1a4262cdd8764da216a004cb = private unnamed_addr constant <{ i8* }> <{ i8* bitcast (i64* (i64*)* @_ZN16thread_local_aux1A7__getit17hd4226b24e99e81adE to i8*) }>, align 8 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           15: @_ZN12thread_local1A7__getit3VAL17h9876ec948e5452caE = internal thread_local global <{ [4 x i8] }> <{ [4 x i8] c"\01\00\00\00" }>, align 4 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           16:  
check:17'0     ~
           17: ; thread_local_aux::A::__getit 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18: ; Function Attrs: inlinehint mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           19: define internal noundef nonnull align 8 i64* @_ZN16thread_local_aux1A7__getit17hd4226b24e99e81adE(i64* noalias nocapture noundef readnone align 8 dereferenceable_or_null(16) %_init) unnamed_addr #0 { 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           20: start: 
check:17'0     ~~~~~~~
           21:  ret i64* @_ZN16thread_local_aux1A7__getit3VAL17h5f4ac2b045f34f55E 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           22: } 
check:17'0     ~~
           23:  
check:17'0     ~
           24: ; Function Attrs: nounwind nonlazybind uwtable 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           25: define noundef i32 @get() unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
check:17'0     ~~~~~~~~~~~~~~~~~~~~~~~
check:23'0                            X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:23'1                                                                                                                                                                                          undefined variable: TLS
           26: start: 
check:23'0     ~~~~~~~
           27:  br i1 icmp ule (i64 sub (i64 ptrtoint (<{ i8* }>* @alloc_87c007504da6cf6c9c62f16c0e3b77cf to i64), i64 1), i64 -2), label %"_ZN3std6thread5local17LocalKey$LT$T$GT$4with17h15311ab163678eb0E.exit", label %panic.i, !prof !2 
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           28:  
check:23'0     ~
           29: panic.i: ; preds = %start 
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           30: ; invoke core::panicking::panic_occupied_niche 
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           31:  invoke void @_ZN4core9panicking20panic_occupied_niche17h7c1389c3af8370d9E(i128 noundef zext (i64 ptrtoint (<{ i8* }>* @alloc_87c007504da6cf6c9c62f16c0e3b77cf to i64) to i128), i128 noundef 1, i128 noundef 18446744073709551615, %"core::panic::location::Location<'_>"* noalias noundef readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc_7260c71e1a6505bba99fc9cc66893e69 to %"core::panic::location::Location<'_>"*)) #5 
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           32:  to label %unreachable.i unwind label %terminate.i 
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           33:  
check:23'0     ~
           34: terminate.i: ; preds = %panic.i 
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           35:  %0 = landingpad { i8*, i32 } 
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           36:  cleanup 
check:23'0     ~~~~~~~~~
           37: ; call core::panicking::panic_cannot_unwind 
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           38:  tail call void @_ZN4core9panicking19panic_cannot_unwind17h04fbafe6331ec042E() #6 
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           39:  unreachable 
check:23'0     ~~~~~~~~~~~~~
           40:  
check:23'0     ~
           41: unreachable.i: ; preds = %panic.i 
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           42:  unreachable 
check:23'0     ~~~~~~~~~~~~~
           43:  
check:23'0     ~
           44: "_ZN3std6thread5local17LocalKey$LT$T$GT$4with17h15311ab163678eb0E.exit": ; preds = %start 
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           45:  %1 = load i32, i32* bitcast (<{ [4 x i8] }>* @_ZN12thread_local1A7__getit3VAL17h9876ec948e5452caE to i32*), align 4, !noundef !3 
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:23'2      ?                                                                                                                                 possible intended match
           46:  ret i32 %1 
check:23'0     ~~~~~~~~~~~~
           47: } 
check:23'0     ~~
           48:  
check:23'0     ~
           49: ; Function Attrs: nounwind nonlazybind uwtable 
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           50: define void @set(i32 noundef %0) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
check:23'0     ~~~~~~~~~~~~~~~~
check:31'0                     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:31'1                                                                                                                                                                                                 undefined variable: TLS
           51: start: 
check:31'0     ~~~~~~~
           52:  br i1 icmp ule (i64 sub (i64 ptrtoint (<{ i8* }>* @alloc_87c007504da6cf6c9c62f16c0e3b77cf to i64), i64 1), i64 -2), label %"_ZN3std6thread5local17LocalKey$LT$T$GT$4with17hc7c80afd66b42fefE.exit", label %panic.i, !prof !2 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           53:  
check:31'0     ~
           54: panic.i: ; preds = %start 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           55: ; invoke core::panicking::panic_occupied_niche 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           56:  invoke void @_ZN4core9panicking20panic_occupied_niche17h7c1389c3af8370d9E(i128 noundef zext (i64 ptrtoint (<{ i8* }>* @alloc_87c007504da6cf6c9c62f16c0e3b77cf to i64) to i128), i128 noundef 1, i128 noundef 18446744073709551615, %"core::panic::location::Location<'_>"* noalias noundef readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc_7260c71e1a6505bba99fc9cc66893e69 to %"core::panic::location::Location<'_>"*)) #5 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           57:  to label %unreachable.i unwind label %terminate.i, !noalias !4 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           58:  
check:31'0     ~
           59: terminate.i: ; preds = %panic.i 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           60:  %1 = landingpad { i8*, i32 } 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           61:  cleanup 
check:31'0     ~~~~~~~~~
           62: ; call core::panicking::panic_cannot_unwind 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           63:  tail call void @_ZN4core9panicking19panic_cannot_unwind17h04fbafe6331ec042E() #6, !noalias !4 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           64:  unreachable 
check:31'0     ~~~~~~~~~~~~~
           65:  
check:31'0     ~
           66: unreachable.i: ; preds = %panic.i 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           67:  unreachable 
check:31'0     ~~~~~~~~~~~~~
           68:  
check:31'0     ~
           69: "_ZN3std6thread5local17LocalKey$LT$T$GT$4with17hc7c80afd66b42fefE.exit": ; preds = %start 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           70:  store i32 %0, i32* bitcast (<{ [4 x i8] }>* @_ZN12thread_local1A7__getit3VAL17h9876ec948e5452caE to i32*), align 4, !alias.scope !7, !noalias !10 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:31'2      ?                                                                                                                                                  possible intended match
           71:  ret void 
check:31'0     ~~~~~~~~~~
           72: } 
check:31'0     ~~
           73:  
check:31'0     ~
           74: ; Function Attrs: nounwind nonlazybind uwtable 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           75: define noundef i64 @get_aux() unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:39'0                                X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:39'1                                                                                                                                                                                              undefined variable: TLS_AUX
           76: start: 
check:39'0     ~~~~~~~
           77:  br i1 icmp ule (i64 sub (i64 ptrtoint (<{ i8* }>* @alloc_d58f8a1f1a4262cdd8764da216a004cb to i64), i64 1), i64 -2), label %"_ZN3std6thread5local17LocalKey$LT$T$GT$4with17h2ccbbbfe58144cb4E.exit", label %panic.i, !prof !2 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           78:  
check:39'0     ~
           79: panic.i: ; preds = %start 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           80: ; invoke core::panicking::panic_occupied_niche 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           81:  invoke void @_ZN4core9panicking20panic_occupied_niche17h7c1389c3af8370d9E(i128 noundef zext (i64 ptrtoint (<{ i8* }>* @alloc_d58f8a1f1a4262cdd8764da216a004cb to i64) to i128), i128 noundef 1, i128 noundef 18446744073709551615, %"core::panic::location::Location<'_>"* noalias noundef readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc_7260c71e1a6505bba99fc9cc66893e69 to %"core::panic::location::Location<'_>"*)) #5 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           82:  to label %unreachable.i unwind label %terminate.i 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           83:  
check:39'0     ~
           84: terminate.i: ; preds = %panic.i 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           85:  %0 = landingpad { i8*, i32 } 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           86:  cleanup 
check:39'0     ~~~~~~~~~
           87: ; call core::panicking::panic_cannot_unwind 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           88:  tail call void @_ZN4core9panicking19panic_cannot_unwind17h04fbafe6331ec042E() #6 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           89:  unreachable 
check:39'0     ~~~~~~~~~~~~~
           90:  
check:39'0     ~
           91: unreachable.i: ; preds = %panic.i 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           92:  unreachable 
check:39'0     ~~~~~~~~~~~~~
           93:  
check:39'0     ~
           94: "_ZN3std6thread5local17LocalKey$LT$T$GT$4with17h2ccbbbfe58144cb4E.exit": ; preds = %start 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           95:  %1 = load i64, i64* @_ZN16thread_local_aux1A7__getit3VAL17h5f4ac2b045f34f55E, align 8, !noundef !3 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:39'2      ?                                                                                                   possible intended match
           96:  ret i64 %1 
check:39'0     ~~~~~~~~~~~~
           97: } 
check:39'0     ~~
           98:  
check:39'0     ~
           99: ; Function Attrs: nounwind nonlazybind uwtable 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          100: define void @set_aux(i64 noundef %0) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality { 
check:39'0     ~~~~~~~~~~~~~~~~~~~~
check:47'0                         X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:47'1                                                                                                                                                                                                     undefined variable: TLS_AUX
          101: start: 
check:47'0     ~~~~~~~
          102:  br i1 icmp ule (i64 sub (i64 ptrtoint (<{ i8* }>* @alloc_d58f8a1f1a4262cdd8764da216a004cb to i64), i64 1), i64 -2), label %"_ZN3std6thread5local17LocalKey$LT$T$GT$4with17hc142a6650cac28c2E.exit", label %panic.i, !prof !2 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          103:  
check:47'0     ~
          104: panic.i: ; preds = %start 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
          105: ; invoke core::panicking::panic_occupied_niche 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          106:  invoke void @_ZN4core9panicking20panic_occupied_niche17h7c1389c3af8370d9E(i128 noundef zext (i64 ptrtoint (<{ i8* }>* @alloc_d58f8a1f1a4262cdd8764da216a004cb to i64) to i128), i128 noundef 1, i128 noundef 18446744073709551615, %"core::panic::location::Location<'_>"* noalias noundef readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc_7260c71e1a6505bba99fc9cc66893e69 to %"core::panic::location::Location<'_>"*)) #5 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          107:  to label %unreachable.i unwind label %terminate.i, !noalias !15 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          108:  
check:47'0     ~
          109: terminate.i: ; preds = %panic.i 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          110:  %1 = landingpad { i8*, i32 } 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          111:  cleanup 
check:47'0     ~~~~~~~~~
          112: ; call core::panicking::panic_cannot_unwind 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          113:  tail call void @_ZN4core9panicking19panic_cannot_unwind17h04fbafe6331ec042E() #6, !noalias !15 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          114:  unreachable 
check:47'0     ~~~~~~~~~~~~~
          115:  
check:47'0     ~
          116: unreachable.i: ; preds = %panic.i 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          117:  unreachable 
check:47'0     ~~~~~~~~~~~~~
          118:  
check:47'0     ~
          119: "_ZN3std6thread5local17LocalKey$LT$T$GT$4with17hc142a6650cac28c2E.exit": ; preds = %start 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          120:  store i64 %0, i64* @_ZN16thread_local_aux1A7__getit3VAL17h5f4ac2b045f34f55E, align 8, !alias.scope !18, !noalias !21 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:47'2      ?                                                                                                                     possible intended match
          121:  ret void 
check:47'0     ~~~~~~~~~~
          122: } 
check:47'0     ~~
          123:  
check:47'0     ~
          124: ; thread_local::A::__getit 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
          125: ; Function Attrs: inlinehint mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          126: define internal noundef align 4 i32* @_ZN12thread_local1A7__getit17h82f4a4c0c4ba1d92E(i32* noalias nocapture noundef readnone align 4 dereferenceable_or_null(8) %_init) unnamed_addr #0 { 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          127: start: 
check:47'0     ~~~~~~~
          128:  ret i32* bitcast (<{ [4 x i8] }>* @_ZN12thread_local1A7__getit3VAL17h9876ec948e5452caE to i32*) 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          129: } 
check:47'0     ~~
          130:  
check:47'0     ~
          131: ; Function Attrs: nonlazybind uwtable 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          132: declare noundef i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, %"unwind::libunwind::_Unwind_Exception"* noundef, %"unwind::libunwind::_Unwind_Context"* noundef) unnamed_addr #2 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          133:  
check:47'0     ~
          134: ; core::panicking::panic_occupied_niche 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          135: ; Function Attrs: cold noinline noreturn nonlazybind uwtable 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          136: declare void @_ZN4core9panicking20panic_occupied_niche17h7c1389c3af8370d9E(i128 noundef, i128 noundef, i128 noundef, %"core::panic::location::Location<'_>"* noalias noundef readonly align 8 dereferenceable(24)) unnamed_addr #3 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          137:  
check:47'0     ~
          138: ; core::panicking::panic_cannot_unwind 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          139: ; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          140: declare void @_ZN4core9panicking19panic_cannot_unwind17h04fbafe6331ec042E() unnamed_addr #4 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          141:  
check:47'0     ~
          142: attributes #0 = { inlinehint mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          143: attributes #1 = { nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          144: attributes #2 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          145: attributes #3 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          146: attributes #4 = { cold noinline noreturn nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          147: attributes #5 = { noreturn } 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          148: attributes #6 = { noinline noreturn nounwind } 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          149:  
check:47'0     ~
          150: !llvm.module.flags = !{!0, !1} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          151:  
check:47'0     ~
          152: !0 = !{i32 7, !"PIC Level", i32 2} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          153: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          154: !2 = !{!"branch_weights", i32 2000, i32 1} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          155: !3 = !{} 
check:47'0     ~~~~~~~~~
          156: !4 = !{!5} 
check:47'0     ~~~~~~~~~~~
          157: !5 = distinct !{!5, !6, !"_ZN3std6thread5local17LocalKey$LT$T$GT$4with17hc7c80afd66b42fefE: %f"} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          158: !6 = distinct !{!6, !"_ZN3std6thread5local17LocalKey$LT$T$GT$4with17hc7c80afd66b42fefE"} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          159: !7 = !{!8} 
check:47'0     ~~~~~~~~~~~
          160: !8 = distinct !{!8, !9, !"_ZN4core3mem7replace17hc9a10968ea40b0afE: %dest"} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          161: !9 = distinct !{!9, !"_ZN4core3mem7replace17hc9a10968ea40b0afE"} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          162: !10 = !{!11, !13, !5} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~
          163: !11 = distinct !{!11, !12, !"_ZN12thread_local3set28_$u7b$$u7b$closure$u7d$$u7d$17h291f0e108c0742cdE: argument 0"} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          164: !12 = distinct !{!12, !"_ZN12thread_local3set28_$u7b$$u7b$closure$u7d$$u7d$17h291f0e108c0742cdE"} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          165: !13 = distinct !{!13, !14, !"_ZN3std6thread5local17LocalKey$LT$T$GT$8try_with17hec3fc3dd95623c67E: %f"} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          166: !14 = distinct !{!14, !"_ZN3std6thread5local17LocalKey$LT$T$GT$8try_with17hec3fc3dd95623c67E"} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          167: !15 = !{!16} 
check:47'0     ~~~~~~~~~~~~~
          168: !16 = distinct !{!16, !17, !"_ZN3std6thread5local17LocalKey$LT$T$GT$4with17hc142a6650cac28c2E: %f"} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          169: !17 = distinct !{!17, !"_ZN3std6thread5local17LocalKey$LT$T$GT$4with17hc142a6650cac28c2E"} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          170: !18 = !{!19} 
check:47'0     ~~~~~~~~~~~~~
          171: !19 = distinct !{!19, !20, !"_ZN4core3mem7replace17h4ca94562aa488285E: %dest"} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          172: !20 = distinct !{!20, !"_ZN4core3mem7replace17h4ca94562aa488285E"} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          173: !21 = !{!22, !24, !16} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~
          174: !22 = distinct !{!22, !23, !"_ZN12thread_local7set_aux28_$u7b$$u7b$closure$u7d$$u7d$17h0b7f68cce97fa09fE: argument 0"} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          175: !23 = distinct !{!23, !"_ZN12thread_local7set_aux28_$u7b$$u7b$closure$u7d$$u7d$17h0b7f68cce97fa09fE"} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          176: !24 = distinct !{!24, !25, !"_ZN3std6thread5local17LocalKey$LT$T$GT$8try_with17hadd11754ad641af7E: %f"} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          177: !25 = distinct !{!25, !"_ZN3std6thread5local17LocalKey$LT$T$GT$8try_with17hadd11754ad641af7E"} 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------



