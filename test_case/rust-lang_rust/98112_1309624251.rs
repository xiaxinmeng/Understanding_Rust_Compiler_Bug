plain
failures:

---- [codegen] src/test/codegen/issue-37945.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll" "/checkout/src/test/codegen/issue-37945.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/issue-37945.rs:23:16: error: CHECK-NEXT: is not on the line after the previous match
// CHECK-NEXT: ret i1 [[B:%.*]]
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll:34:2: note: 'next' match was here
 ret i1 %.not
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll:17:36: note: previous match ended here
 %_10.i = icmp eq i32* %xs.1, %xs.0
                                   ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll:18:1: note: non-matching line after previous match is here
 br i1 %_10.i, label %"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h964d62a539f1e4f7E.exit", label %bb6.i
^
/checkout/src/test/codegen/issue-37945.rs:36:16: error: CHECK-NEXT: is not on the line after the previous match
// CHECK-NEXT: ret i1 [[D:%.*]]
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll:59:2: note: 'next' match was here
 ret i1 %.not
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll:42:36: note: previous match ended here
 %_10.i = icmp eq i32* %xs.1, %xs.0
                                   ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll:43:1: note: non-matching line after previous match is here
 br i1 %_10.i, label %"_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h964d62a539f1e4f7E.exit", label %bb6.i

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll
Check file: /checkout/src/test/codegen/issue-37945.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
         .
         .
         .
         .
        29:  unreachable 
        30:  
        31: "_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h964d62a539f1e4f7E.exit": ; preds = %start, %bb6.i 
        32:  %.0.i = phi i32* [ null, %start ], [ %xs.0, %bb6.i ] 
        33:  %.not = icmp eq i32* %.0.i, null 
        34:  ret i1 %.not 
next:23      !~~~~~~~~~~~  error: match on wrong line
        35: } 
        36:  
        37: ; Function Attrs: nonlazybind uwtable 
        38: define noundef zeroext i1 @is_empty_2(i32* noundef nonnull %xs.0, i32* %xs.1) unnamed_addr #0 { 
         .
         .
         .
        54:  unreachable 
        54:  unreachable 
        55:  
        56: "_ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h964d62a539f1e4f7E.exit": ; preds = %start, %bb6.i 
        57:  %.0.i = phi i32* [ null, %start ], [ %xs.0, %bb6.i ] 
        58:  %.not = icmp eq i32* %.0.i, null 
        59:  ret i1 %.not 
next:36      !~~~~~~~~~~~  error: match on wrong line
        60: } 
        61:  
        62: ; Function Attrs: inaccessiblememonly mustprogress nofree nosync nounwind willreturn 
        63: declare void @llvm.assume(i1 noundef) #1 
         .
         .
         .
>>>>>>
>>>>>>
------------------------------------------


---- [codegen] src/test/codegen/virtual-function-elimination.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/virtual-function-elimination/virtual-function-elimination.ll" "/checkout/src/test/codegen/virtual-function-elimination.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/virtual-function-elimination.rs:4:11: error: CHECK: expected string not found in input
// CHECK: @vtable.0 = {{.*}}, !type ![[TYPE0:[0-9]+]], !vcall_visibility ![[VCALL_VIS0:[0-9]+]]
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/virtual-function-elimination/virtual-function-elimination.ll:1:1: note: scanning from here
; ModuleID = 'virtual_function_elimination.c870555b-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/virtual-function-elimination/virtual-function-elimination.ll:15:638: note: possible intended match here
@vtable.1 = private unnamed_addr constant <{ i8*, [16 x i8], i8*, i8*, i8*, i8*, i8* }> <{ i8* bitcast (void (%S*)* @_RINvNtCs1KUvY9FIIPZ_4core3ptr13drop_in_placeNtCsfRpWlKdQPZn_28virtual_function_elimination1SEBI_ to i8*), [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", i8* bitcast (i32 (%S*)* @_RNvYNtCsfRpWlKdQPZn_28virtual_function_elimination1SNtB4_1T4usedB4_ to i8*), i8* bitcast (i32 (%S*)* @_RNvYNtCsfRpWlKdQPZn_28virtual_function_elimination1SNtB4_1T22used_through_sub_traitB4_ to i8*), i8* bitcast (i32 (i64*)* @_RNvYNtCsfRpWlKdQPZn_28virtual_function_elimination1SNtB4_1T5by_rcB4_ to i8*), i8* null, i8* null }>, align 8, !type !0, !vcall_visibility !1
/checkout/src/test/codegen/virtual-function-elimination.rs:96:14: error: undefined variable: TYPE0
/checkout/src/test/codegen/virtual-function-elimination.rs:96:14: error: undefined variable: TYPE0
// CHECK: ![[TYPE0]] = !{i64 0, !"[[MANGLED_TYPE0]]"}
             ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/virtual-function-elimination/virtual-function-elimination.ll:146:185: note: with "MANGLED_TYPE0" equal to "NtCsfRpWlKdQPZn_28virtual_function_elimination1T"
 %23 = call { i8*, i1 } @llvm.type.checked.load(i8* nonnull bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.3 to i8*), i32 24, metadata !"NtCsfRpWlKdQPZn_28virtual_function_elimination1V")
                                                                                                                                                                                        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/virtual-function-elimination/virtual-function-elimination.ll:185:1: note: possible intended match here
!0 = !{i64 0, !"NtCsfRpWlKdQPZn_28virtual_function_elimination1T"}

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/virtual-function-elimination/virtual-function-elimination.ll
Check file: /checkout/src/test/codegen/virtual-function-elimination.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'virtual_function_elimination.c870555b-cgu.0' 
check:4'0      X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "virtual_function_elimination.c870555b-cgu.0" 
check:4'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
check:4'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "x86_64-unknown-linux-gnu" 
check:4'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:4'0      ~
            6: %S = type {} 
check:4'0      ~~~~~~~~~~~~~
            .
            .
            .
           10: %"unwind::libunwind::_Unwind_Context" = type { [0 x i8] } 
check:4'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           11:  
check:4'0      ~
           12: @alloc141 = private unnamed_addr constant <{ [33 x i8] }> <{ [33 x i8] c"/checkout/library/alloc/src/rc.rs" }>, align 1 
check:4'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13: @alloc136 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [33 x i8] }>, <{ [33 x i8] }>* @alloc141, i32 0, i32 0, i32 0), [16 x i8] c"!\00\00\00\00\00\00\00t\01\00\00\1B\00\00\00" }>, align 8 
check:4'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14: @str.0 = internal constant [43 x i8] c"attempt to dereference a misaligned pointer" 
check:4'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           15: @vtable.1 = private unnamed_addr constant <{ i8*, [16 x i8], i8*, i8*, i8*, i8*, i8* }> <{ i8* bitcast (void (%S*)* @_RINvNtCs1KUvY9FIIPZ_4core3ptr13drop_in_placeNtCsfRpWlKdQPZn_28virtual_function_elimination1SEBI_ to i8*), [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", i8* bitcast (i32 (%S*)* @_RNvYNtCsfRpWlKdQPZn_28virtual_function_elimination1SNtB4_1T4usedB4_ to i8*), i8* bitcast (i32 (%S*)* @_RNvYNtCsfRpWlKdQPZn_28virtual_function_elimination1SNtB4_1T22used_through_sub_traitB4_ to i8*), i8* bitcast (i32 (i64*)* @_RNvYNtCsfRpWlKdQPZn_28virtual_function_elimination1SNtB4_1T5by_rcB4_ to i8*), i8* null, i8* null }>, align 8, !type !0, !vcall_visibility !1 
check:4'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:4'1                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   ?                                           possible intended match
           16: @vtable.2 = private unnamed_addr constant <{ i8*, [16 x i8], i8*, i8*, i8*, i8*, i8*, i8*, i8* }> <{ i8* bitcast (void (%S*)* @_RINvNtCs1KUvY9FIIPZ_4core3ptr13drop_in_placeNtCsfRpWlKdQPZn_28virtual_function_elimination1SEBI_ to i8*), [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", i8* bitcast (i32 (%S*)* @_RNvYNtCsfRpWlKdQPZn_28virtual_function_elimination1SNtB4_1T4usedB4_ to i8*), i8* bitcast (i32 (%S*)* @_RNvYNtCsfRpWlKdQPZn_28virtual_function_elimination1SNtB4_1T22used_through_sub_traitB4_ to i8*), i8* bitcast (i32 (i64*)* @_RNvYNtCsfRpWlKdQPZn_28virtual_function_elimination1SNtB4_1T5by_rcB4_ to i8*), i8* null, i8* null, i8* bitcast (i32 (%S*)* @_RNvYNtCsfRpWlKdQPZn_28virtual_function_elimination1SNtB4_1U13subtrait_usedB4_ to i8*), i8* null }>, align 8, !type !2, !vcall_visibility !1 
check:4'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           17: @vtable.3 = private unnamed_addr constant <{ i8*, [16 x i8], i8* }> <{ i8* bitcast (void (%S*)* @_RINvNtCs1KUvY9FIIPZ_4core3ptr13drop_in_placeNtCsfRpWlKdQPZn_28virtual_function_elimination1SEBI_ to i8*), [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", i8* bitcast (i32 (%S*)* @_RNvXs0_CsfRpWlKdQPZn_28virtual_function_eliminationNtB5_1SNtB5_1V15public_function to i8*) }>, align 8, !type !3, !vcall_visibility !4 
check:4'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18:  
check:4'0      ~
           19: ; core::ptr::drop_in_place::<virtual_function_elimination::S> 
check:4'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           20: ; Function Attrs: inlinehint mustprogress nofree norecurse nosync nounwind nonlazybind readnone uwtable willreturn 
check:4'0      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
            .
          141:  %_5.i = call i32 %19({}* noundef nonnull align 1 %_3.0) 
          142:  %20 = call { i8*, i1 } @llvm.type.checked.load(i8* bitcast (<{ i8*, [16 x i8], i8*, i8*, i8*, i8*, i8*, i8*, i8* }>* @vtable.2 to i8*), i32 32, metadata !"NtCsfRpWlKdQPZn_28virtual_function_elimination1U") 
          143:  %21 = extractvalue { i8*, i1 } %20, 0 
          144:  %22 = bitcast i8* %21 to i32 ({}*)* 
          145:  %_7.i = call i32 %22({}* noundef nonnull align 1 %_3.0) 
          146:  %23 = call { i8*, i1 } @llvm.type.checked.load(i8* nonnull bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.3 to i8*), i32 24, metadata !"NtCsfRpWlKdQPZn_28virtual_function_elimination1V") 
check:96'0                                                                                                                                                                                             X error: match failed for invalid pattern
check:96'1                                                                                                                                                                                               undefined variable: TYPE0
check:96'2                                                                                                                                                                                               with "MANGLED_TYPE0" equal to "NtCsfRpWlKdQPZn_28virtual_function_elimination1T"
          147:  %24 = extractvalue { i8*, i1 } %23, 0 
check:96'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          148:  %25 = bitcast i8* %24 to i32 ({}*)* 
check:96'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          149:  %26 = call i32 %25({}* noundef nonnull align 1 %_3.0) 
check:96'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          150:  ret void 
check:96'0     ~~~~~~~~~~
          151: } 
check:96'0     ~~
            .
            .
            .
          180: attributes #7 = { nounwind } 
check:96'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          181: attributes #8 = { noreturn } 
check:96'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          182:  
check:96'0     ~
          183: !llvm.module.flags = !{!5, !6, !7} 
check:96'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          184:  
check:96'0     ~
          185: !0 = !{i64 0, !"NtCsfRpWlKdQPZn_28virtual_function_elimination1T"} 
check:96'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:96'3     ?                                                                   possible intended match
          186: !1 = !{i64 2} 
check:96'0     ~~~~~~~~~~~~~~
          187: !2 = !{i64 0, !"NtCsfRpWlKdQPZn_28virtual_function_elimination1U"} 
check:96'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          188: !3 = !{i64 0, !"NtCsfRpWlKdQPZn_28virtual_function_elimination1V"} 
check:96'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          189: !4 = !{i64 1} 
check:96'0     ~~~~~~~~~~~~~~
          190: !5 = !{i32 7, !"PIC Level", i32 2} 
check:96'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
