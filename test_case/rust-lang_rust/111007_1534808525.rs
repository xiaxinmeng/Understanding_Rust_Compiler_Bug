plain
failures:

---- [codegen] tests/codegen/mem-replace-big-type.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-big-type/mem-replace-big-type.ll" "/checkout/tests/codegen/mem-replace-big-type.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
/checkout/tests/codegen/mem-replace-big-type.rs:29:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/mem-replace-big-type.rs:29:12: error: CHECK: expected string not found in input
 // CHECK: call void @llvm.memcpy.{{.+}}({{i8\*|ptr}} align 8 %0, {{i8\*|ptr}} align 8 %dest, i{{.*}} 56, i1 false)
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-big-type/mem-replace-big-type.ll:10:104: note: scanning from here
define internal void @_ZN4core3mem7replace17hadfcb6d8afd1fe7aE(ptr noalias nocapture noundef sret(%Big) dereferenceable(56) %0, ptr noalias noundef align 8 dereferenceable(56) %dest, ptr noalias nocapture noundef dereferenceable(56) %src) unnamed_addr #0 {
                                                                                                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-big-type/mem-replace-big-type.ll:14:2: note: possible intended match here
 call void @llvm.memcpy.p0.p0.i32(ptr align 8 %result, ptr align 8 %dest, i32 56, i1 false)

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-big-type/mem-replace-big-type.ll
Check file: /checkout/tests/codegen/mem-replace-big-type.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'mem_replace_big_type.917c66af1ea2490d-cgu.0' 
            2: source_filename = "mem_replace_big_type.917c66af1ea2490d-cgu.0" 
            3: target datalayout = "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-f64:32:64-f80:32-n8:16:32-S128" 
            4: target triple = "i586-unknown-linux-gnu" 
            5:  
            6: %Big = type { [7 x i64] } 
            8: ; core::mem::replace 
            8: ; core::mem::replace 
            9: ; Function Attrs: inlinehint nonlazybind uwtable 
           10: define internal void @_ZN4core3mem7replace17hadfcb6d8afd1fe7aE(ptr noalias nocapture noundef sret(%Big) dereferenceable(56) %0, ptr noalias noundef align 8 dereferenceable(56) %dest, ptr noalias nocapture noundef dereferenceable(56) %src) unnamed_addr #0 { 
check:29'0                                                                                                            X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           11: start: 
check:29'0     ~~~~~~~
           12:  %result = alloca %Big, align 8 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13:  call void @llvm.lifetime.start.p0(i64 56, ptr %result) 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14:  call void @llvm.memcpy.p0.p0.i32(ptr align 8 %result, ptr align 8 %dest, i32 56, i1 false) 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:29'1      ?                                                                                           possible intended match
           15:  call void @llvm.memcpy.p0.p0.i32(ptr align 8 %dest, ptr align 8 %src, i32 56, i1 false) 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           16:  call void @llvm.memcpy.p0.p0.i32(ptr align 8 %0, ptr align 8 %result, i32 56, i1 false) 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           17:  call void @llvm.lifetime.end.p0(i64 56, ptr %result) 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18:  ret void 
check:29'0     ~~~~~~~~~~
           19: } 
check:29'0     ~~
           20:  
check:29'0     ~
           21: ; mem_replace_big_type::replace_big 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           22: ; Function Attrs: nonlazybind uwtable 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           23: define void @_ZN20mem_replace_big_type11replace_big17h2750f1c70310ee55E(ptr noalias nocapture noundef sret(%Big) dereferenceable(56) %0, ptr noalias noundef align 8 dereferenceable(56) %dst, ptr noalias nocapture noundef dereferenceable(56) %src) unnamed_addr #1 { 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           24: start: 
check:29'0     ~~~~~~~
           25: ; call core::mem::replace 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           26:  call void @_ZN4core3mem7replace17hadfcb6d8afd1fe7aE(ptr noalias nocapture noundef sret(%Big) dereferenceable(56) %0, ptr noalias noundef align 8 dereferenceable(56) %dst, ptr noalias nocapture noundef dereferenceable(56) %src) 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           27:  ret void 
check:29'0     ~~~~~~~~~~
           28: } 
check:29'0     ~~
           29:  
check:29'0     ~
           30: ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite) 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           31: declare void @llvm.memcpy.p0.p0.i32(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i32, i1 immarg) #2 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           32:  
check:29'0     ~
           33: ; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           34: declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #3 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           35:  
check:29'0     ~
           36: ; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           37: declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #3 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           38:  
check:29'0     ~
           39: attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="pentium" } 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           40: attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="pentium" } 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           41: attributes #2 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) } 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           42: attributes #3 = { nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) } 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           43:  
check:29'0     ~
           44: !llvm.module.flags = !{!0, !1} 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           45:  
check:29'0     ~
           46: !0 = !{i32 8, !"PIC Level", i32 2} 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           47: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:29'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------



