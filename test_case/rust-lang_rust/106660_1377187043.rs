plain
test [codegen] src/test/codegen/lto-removes-invokes.rs ... ok

failures:

---- [codegen] src/test/codegen/move-operands.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/move-operands/move-operands.ll" "/checkout/src/test/codegen/move-operands.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/move-operands.rs:9:12: error: CHECK: expected string not found in input
 // CHECK: call void @llvm.memcpy.{{.*}}({{i8\*|ptr}} align 8 %{{.*}}, {{i8\*|ptr}} align 8 %{{.*}}, i64 256, i1 false)
           ^
/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/move-operands/move-operands.ll:1:1: note: scanning from here
; ModuleID = 'move_operands.4ab9fcdf-cgu.0'
^
/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/move-operands/move-operands.ll:11:2: note: possible intended match here
 call void @llvm.memcpy.p0.p0.i32(ptr align 4 %_3, ptr align 4 %a, i32 256, i1 false)

Input file: /checkout/obj/build/i686-unknown-linux-gnu/test/codegen/move-operands/move-operands.ll
Check file: /checkout/src/test/codegen/move-operands.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'move_operands.4ab9fcdf-cgu.0' 
check:9'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           2: source_filename = "move_operands.4ab9fcdf-cgu.0" 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           3: target datalayout = "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-f64:32:64-f80:32-n8:16:32-S128" 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           4: target triple = "i686-unknown-linux-gnu" 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           5:  
check:9'0     ~
           6: ; Function Attrs: nonlazybind uwtable 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           7: define void @f(ptr noalias nocapture noundef dereferenceable(256) %a, ptr noundef nonnull %b) unnamed_addr #0 { 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           8: start: 
check:9'0     ~~~~~~~
           9:  %_3 = alloca [32 x i64], align 4 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          10:  call void @llvm.lifetime.start.p0(i64 256, ptr %_3) 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          11:  call void @llvm.memcpy.p0.p0.i32(ptr align 4 %_3, ptr align 4 %a, i32 256, i1 false) 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:9'1      ?                                                                                     possible intended match
          12:  call void %b(ptr noalias nocapture noundef dereferenceable(256) %a, ptr noalias nocapture noundef dereferenceable(256) %_3) 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          13:  call void @llvm.lifetime.end.p0(i64 256, ptr %_3) 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          14:  ret void 
check:9'0     ~~~~~~~~~~
          15: } 
check:9'0     ~~
          16:  
check:9'0     ~
          17: ; Function Attrs: argmemonly nocallback nofree nounwind willreturn 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          18: declare void @llvm.memcpy.p0.p0.i32(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i32, i1 immarg) #1 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          19:  
check:9'0     ~
          20: ; Function Attrs: argmemonly nocallback nofree nosync nounwind willreturn 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          21: declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #2 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          22:  
check:9'0     ~
          23: ; Function Attrs: argmemonly nocallback nofree nosync nounwind willreturn 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          24: declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #2 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          25:  
check:9'0     ~
          26: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="pentium4" } 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          27: attributes #1 = { argmemonly nocallback nofree nounwind willreturn } 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          28: attributes #2 = { argmemonly nocallback nofree nosync nounwind willreturn } 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          29:  
check:9'0     ~
          30: !llvm.module.flags = !{!0, !1} 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          31:  
check:9'0     ~
          32: !0 = !{i32 7, !"PIC Level", i32 2} 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          33: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------



