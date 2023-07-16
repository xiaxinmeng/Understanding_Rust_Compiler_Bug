plain
failures:

---- [codegen] src/test/codegen/mem-replace-direct-memcpy.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll" "/checkout/src/test/codegen/mem-replace-direct-memcpy.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/mem-replace-direct-memcpy.rs:21:11: error: CHECK: expected string not found in input
// CHECK: call void @llvm.memcpy.{{.+}}({{i8\*|ptr}} align 1 %{{.*}}, {{i8\*|ptr}} align 1 %src, i{{.*}} 1, i1 false)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll:6:21: note: scanning from here
; core::mem::replace
                    ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll:22:2: note: possible intended match here
 call void @llvm.memcpy.p0.p0.i32(ptr align 1 %dest, ptr align 1 %src1, i32 1, i1 false)

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll
Check file: /checkout/src/test/codegen/mem-replace-direct-memcpy.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'mem_replace_direct_memcpy.6e601deb-cgu.0' 
            2: source_filename = "mem_replace_direct_memcpy.6e601deb-cgu.0" 
            3: target datalayout = "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-f64:32:64-f80:32-n8:16:32-S128" 
            4: target triple = "i586-unknown-linux-gnu" 
            6: ; core::mem::replace 
            6: ; core::mem::replace 
check:21'0                         X error: no match found
            7: ; Function Attrs: inlinehint nonlazybind uwtable 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            8: define internal i8 @_ZN4core3mem7replace17h8eaa2e5bbcc07fd8E(ptr noalias noundef align 1 dereferenceable(1) %dest, i8 %src) unnamed_addr #0 personality ptr @rust_eh_personality { 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            9: start: 
check:21'0     ~~~~~~~
           10:  %0 = alloca { ptr, i32 }, align 4 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           11:  %tmp = alloca i8, align 1 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
           17:  
           17:  
check:21'0     ~
           18: bb4: ; preds = %start 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~
           19:  call void @llvm.lifetime.end.p0(i64 1, ptr %tmp) 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           20:  call void @llvm.lifetime.start.p0(i64 1, ptr %src1) 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           21:  store i8 %src, ptr %src1, align 1 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           22:  call void @llvm.memcpy.p0.p0.i32(ptr align 1 %dest, ptr align 1 %src1, i32 1, i1 false) 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:21'1      ?                                                                                        possible intended match
           23:  call void @llvm.lifetime.end.p0(i64 1, ptr %src1) 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           24:  ret i8 %self 
check:21'0     ~~~~~~~~~~~~~~
           25:  
check:21'0     ~
           26: bb3: ; No predecessors! 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           27:  br i1 true, label %bb2, label %bb1 
check:21'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
