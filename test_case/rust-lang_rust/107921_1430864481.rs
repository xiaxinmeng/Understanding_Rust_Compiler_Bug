plain
failures:

---- [codegen] tests/codegen/inherit_overflow.rs#ASSERT stdout ----

error in revision `ASSERT`: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/inherit_overflow.ASSERT/inherit_overflow.ll" "/checkout/tests/codegen/inherit_overflow.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,ASSERT" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/inherit_overflow.rs:7:12: error: ASSERT: expected string not found in input
// ASSERT: tail call void @_ZN4core9panicking5panic17h
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/inherit_overflow.ASSERT/inherit_overflow.ll:11:21: note: scanning from here
define i8 @assertion() unnamed_addr #0 {
                    ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/inherit_overflow.ASSERT/inherit_overflow.ll:30:1: note: possible intended match here
declare void @_ZN4core9panicking5panic17hc95601e79e34832bE(ptr align 1, i64, ptr align 8) unnamed_addr #2


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/inherit_overflow.ASSERT/inherit_overflow.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'inherit_overflow.d8ba3aac-cgu.0' 
           2: source_filename = "inherit_overflow.d8ba3aac-cgu.0" 
           3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
           4: target triple = "x86_64-unknown-linux-gnu" 
           5:  
           6: @alloc5 = private unnamed_addr constant <{ [39 x i8] }> <{ [39 x i8] c"/checkout/library/core/src/ops/arith.rs" }>, align 1 
           7: @alloc6 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc5, [16 x i8] c"'\00\00\00\00\00\00\00h\00\00\00-\00\00\00" }>, align 8 
           8: @str.0 = internal constant [28 x i8] c"attempt to add with overflow" 
           9:  
          10: ; Function Attrs: nonlazybind uwtable 
          11: define i8 @assertion() unnamed_addr #0 { 
check:7'0                         X~~~~~~~~~~~~~~~~~~~~ error: no match found
          12: start: 
check:7'0     ~~~~~~~
          13:  %0 = call i1 @llvm.expect.i1(i1 true, i1 false) 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          14:  br i1 %0, label %panic, label %bb1 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          15:  
check:7'0     ~
          16: bb1: ; preds = %start 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~
          17:  ret i8 0 
check:7'0     ~~~~~~~~~~
          18:  
check:7'0     ~
          19: panic: ; preds = %start 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~
          20: ; call core::panicking::panic 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          21:  call void @_ZN4core9panicking5panic17hc95601e79e34832bE(ptr align 1 @str.0, i64 28, ptr align 8 @alloc6) #3 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          22:  unreachable 
check:7'0     ~~~~~~~~~~~~~
          23: } 
check:7'0     ~~
          24:  
check:7'0     ~
          25: ; Function Attrs: nocallback nofree nosync nounwind readnone willreturn 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          26: declare i1 @llvm.expect.i1(i1, i1) #1 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          27:  
check:7'0     ~
          28: ; core::panicking::panic 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
          29: ; Function Attrs: cold noinline noreturn nonlazybind uwtable 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          30: declare void @_ZN4core9panicking5panic17hc95601e79e34832bE(ptr align 1, i64, ptr align 8) unnamed_addr #2 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:7'1     ?                                                                                                          possible intended match
          31:  
check:7'0     ~
          32: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          33: attributes #1 = { nocallback nofree nosync nounwind readnone willreturn } 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          34: attributes #2 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          35: attributes #3 = { noreturn } 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          36:  
check:7'0     ~
          37: !llvm.module.flags = !{!0, !1} 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          38:  
check:7'0     ~
          39: !0 = !{i32 7, !"PIC Level", i32 2} 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          40: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------



