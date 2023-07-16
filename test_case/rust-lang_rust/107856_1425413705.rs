plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:ca7cab66e2f838703fe12775fbabb05754421ad8)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
failures:

---- [codegen] tests/codegen/optimize-attr-1.rs#NO-OPT stdout ----

error in revision `NO-OPT`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/optimize-attr-1.NO-OPT/optimize-attr-1.ll" "/checkout/tests/codegen/optimize-attr-1.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,NO-OPT" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/optimize-attr-1.rs:11:12: error: NO-OPT: expected string not found in input
// NO-OPT: ret i32 4
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/optimize-attr-1.NO-OPT/optimize-attr-1.ll:15:38: note: scanning from here
define i32 @nothing() unnamed_addr #0 {
                                     ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/optimize-attr-1.NO-OPT/optimize-attr-1.ll:24:2: note: possible intended match here
 ret i32 %_1.0
 ^
/checkout/tests/codegen/optimize-attr-1.rs:21:12: error: NO-OPT: expected string not found in input
// NO-OPT: ret i32 6
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/optimize-attr-1.NO-OPT/optimize-attr-1.ll:33:35: note: scanning from here
define i32 @size() unnamed_addr #1 {
                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/optimize-attr-1.NO-OPT/optimize-attr-1.ll:42:2: note: possible intended match here
 ret i32 %_1.0
 ^
/checkout/tests/codegen/optimize-attr-1.rs:34:12: error: NO-OPT: expected string not found in input
// NO-OPT: ret i32 8
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/optimize-attr-1.NO-OPT/optimize-attr-1.ll:51:36: note: scanning from here
define i32 @speed() unnamed_addr #0 {
                                   ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/optimize-attr-1.NO-OPT/optimize-attr-1.ll:60:2: note: possible intended match here
 ret i32 %_1.0


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/optimize-attr-1.NO-OPT/optimize-attr-1.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'optimize_attr_1.cddac4ce-cgu.0' 
            2: source_filename = "optimize_attr_1.cddac4ce-cgu.0" 
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
            4: target triple = "x86_64-unknown-linux-gnu" 
            5:  
            6: %"core::panic::location::Location<'_>" = type { { [0 x i8]*, i64 }, i32, i32 } 
            7:  
            8: @alloc8 = private unnamed_addr constant <{ [42 x i8] }> <{ [42 x i8] c"/checkout/tests/codegen/optimize-attr-1.rs" }>, align 1 
            9: @alloc5 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [42 x i8] }>, <{ [42 x i8] }>* @alloc8, i32 0, i32 0, i32 0), [16 x i8] c"*\00\00\00\00\00\00\00\10\00\00\00\05\00\00\00" }>, align 8 
           10: @str.0 = internal constant [28 x i8] c"attempt to add with overflow" 
           11: @alloc7 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [42 x i8] }>, <{ [42 x i8] }>* @alloc8, i32 0, i32 0, i32 0), [16 x i8] c"*\00\00\00\00\00\00\00\1B\00\00\00\05\00\00\00" }>, align 8 
           12: @alloc9 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [42 x i8] }>, <{ [42 x i8] }>* @alloc8, i32 0, i32 0, i32 0), [16 x i8] c"*\00\00\00\00\00\00\00(\00\00\00\05\00\00\00" }>, align 8 
           13:  
           14: ; Function Attrs: nonlazybind uwtable 
           15: define i32 @nothing() unnamed_addr #0 { 
check:11'0                                          X~~ error: no match found
           16: start: 
check:11'0     ~~~~~~~
           17:  %0 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 2, i32 2) 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18:  %_1.0 = extractvalue { i32, i1 } %0, 0 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           19:  %_1.1 = extractvalue { i32, i1 } %0, 1 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           20:  %1 = call i1 @llvm.expect.i1(i1 %_1.1, i1 false) 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           21:  br i1 %1, label %panic, label %bb1 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           22:  
check:11'0     ~
           23: bb1: ; preds = %start 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~
           24:  ret i32 %_1.0 
check:11'0     ~~~~~~~~~~~~~~~
check:11'1      ?              possible intended match
           25:  
check:11'0     ~
           26: panic: ; preds = %start 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           27: ; call core::panicking::panic 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           28:  call void @_ZN4core9panicking5panic17h8efe2b7d308956ccE([0 x i8]* align 1 bitcast ([28 x i8]* @str.0 to [0 x i8]*), i64 28, %"core::panic::location::Location<'_>"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc5 to %"core::panic::location::Location<'_>"*)) #5 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           29:  unreachable 
check:11'0     ~~~~~~~~~~~~~
           30: } 
check:11'0     ~~
           31:  
check:11'0     ~
           32: ; Function Attrs: minsize nonlazybind optsize uwtable 
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           33: define i32 @size() unnamed_addr #1 { 
check:11'0     ~~~~~~~~~~~~~~~~
check:21'0                                       X~~ error: no match found
           34: start: 
check:21'0     ~~~~~~~
           35:  %0 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 3, i32 3) 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           36:  %_1.0 = extractvalue { i32, i1 } %0, 0 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           37:  %_1.1 = extractvalue { i32, i1 } %0, 1 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           38:  %1 = call i1 @llvm.expect.i1(i1 %_1.1, i1 false) 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           39:  br i1 %1, label %panic, label %bb1 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           40:  
check:21'0     ~
           41: bb1: ; preds = %start 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~
           42:  ret i32 %_1.0 
check:21'0     ~~~~~~~~~~~~~~~
check:21'1      ?              possible intended match
           43:  
check:21'0     ~
           44: panic: ; preds = %start 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           45: ; call core::panicking::panic 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           46:  call void @_ZN4core9panicking5panic17h8efe2b7d308956ccE([0 x i8]* align 1 bitcast ([28 x i8]* @str.0 to [0 x i8]*), i64 28, %"core::panic::location::Location<'_>"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc7 to %"core::panic::location::Location<'_>"*)) #5 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           47:  unreachable 
check:21'0     ~~~~~~~~~~~~~
           48: } 
check:21'0     ~~
           49:  
check:21'0     ~
           50: ; Function Attrs: nonlazybind uwtable 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           51: define i32 @speed() unnamed_addr #0 { 
check:21'0     ~~~~~~~~~~~~~~~~~
check:34'0                                        X~~ error: no match found
           52: start: 
check:34'0     ~~~~~~~
           53:  %0 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 4, i32 4) 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           54:  %_1.0 = extractvalue { i32, i1 } %0, 0 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           55:  %_1.1 = extractvalue { i32, i1 } %0, 1 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           56:  %1 = call i1 @llvm.expect.i1(i1 %_1.1, i1 false) 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           57:  br i1 %1, label %panic, label %bb1 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           58:  
check:34'0     ~
           59: bb1: ; preds = %start 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~
           60:  ret i32 %_1.0 
check:34'0     ~~~~~~~~~~~~~~~
check:34'1      ?              possible intended match
           61:  
check:34'0     ~
           62: panic: ; preds = %start 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           63: ; call core::panicking::panic 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           64:  call void @_ZN4core9panicking5panic17h8efe2b7d308956ccE([0 x i8]* align 1 bitcast ([28 x i8]* @str.0 to [0 x i8]*), i64 28, %"core::panic::location::Location<'_>"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc9 to %"core::panic::location::Location<'_>"*)) #5 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           65:  unreachable 
check:34'0     ~~~~~~~~~~~~~
           66: } 
check:34'0     ~~
           67:  
check:34'0     ~
           68: ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           69: declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #2 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           70:  
check:34'0     ~
           71: ; Function Attrs: nofree nosync nounwind readnone willreturn 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           72: declare i1 @llvm.expect.i1(i1, i1) #3 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           73:  
check:34'0     ~
           74: ; core::panicking::panic 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
           75: ; Function Attrs: cold noinline noreturn nonlazybind uwtable 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           76: declare void @_ZN4core9panicking5panic17h8efe2b7d308956ccE([0 x i8]* align 1, i64, %"core::panic::location::Location<'_>"* align 8) unnamed_addr #4 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           77:  
check:34'0     ~
           78: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           79: attributes #1 = { minsize nonlazybind optsize uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           80: attributes #2 = { nofree nosync nounwind readnone speculatable willreturn } 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           81: attributes #3 = { nofree nosync nounwind readnone willreturn } 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           82: attributes #4 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           83: attributes #5 = { noreturn } 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           84:  
check:34'0     ~
           85: !llvm.module.flags = !{!0, !1} 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           86:  
check:34'0     ~
           87: !0 = !{i32 7, !"PIC Level", i32 2} 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           88: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------



