plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3040325909b538d8ad81ad89a04b7a91b109c313)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
failures:

---- [codegen] tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll" "/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:38:12: error: CHECK: expected string not found in input
 // CHECK: %1 = load <4 x float>, {{.*}} %x, align 4
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:27:27: note: scanning from here
define void @build_array_u(<4 x float>* noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, [4 x float]* noalias nocapture noundef readonly dereferenceable(16) %x) unnamed_addr #0 {
                          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:30:12: note: possible intended match here
 %2 = load <4 x float>, <4 x float>* %1, align 4

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll
Check file: /checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'simd_intrinsic_transmute_array.5139c7eb-cgu.0' 
            2: source_filename = "simd_intrinsic_transmute_array.5139c7eb-cgu.0" 
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
            4: target triple = "x86_64-unknown-linux-gnu" 
            5:  
            6: ; Function Attrs: nonlazybind uwtable 
            7: define void @build_array_s(<4 x float>* noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, [4 x float]* noalias nocapture noundef readonly dereferenceable(16) %x) unnamed_addr #0 { 
            8: start: 
            9:  %1 = bitcast <4 x float>* %0 to [4 x float]* 
           10:  %2 = bitcast [4 x float]* %1 to i8* 
           11:  %3 = bitcast [4 x float]* %x to i8* 
           12:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 16 %2, i8* align 4 %3, i64 16, i1 false) 
           13:  ret void 
           14: } 
           15:  
           16: ; Function Attrs: nonlazybind uwtable 
           17: define void @build_array_t(<4 x float>* noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, [4 x float]* noalias nocapture noundef readonly dereferenceable(16) %x) unnamed_addr #0 { 
           18: start: 
           19:  %1 = bitcast <4 x float>* %0 to [4 x float]* 
           20:  %2 = bitcast [4 x float]* %1 to i8* 
           21:  %3 = bitcast [4 x float]* %x to i8* 
           22:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 16 %2, i8* align 4 %3, i64 16, i1 false) 
           23:  ret void 
           24: } 
           25:  
           26: ; Function Attrs: nonlazybind uwtable 
           27: define void @build_array_u(<4 x float>* noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, [4 x float]* noalias nocapture noundef readonly dereferenceable(16) %x) unnamed_addr #0 { 
check:38'0                               X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           28: start: 
check:38'0     ~~~~~~~
           29:  %1 = bitcast [4 x float]* %x to <4 x float>* 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           30:  %2 = load <4 x float>, <4 x float>* %1, align 4 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:38'1                ?                                      possible intended match
           31:  store <4 x float> %2, <4 x float>* %0, align 16 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           32:  ret void 
check:38'0     ~~~~~~~~~~
           33: } 
check:38'0     ~~
           34:  
check:38'0     ~
           35: ; Function Attrs: argmemonly nofree nounwind willreturn 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           36: declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #1 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           37:  
check:38'0     ~
           38: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           39: attributes #1 = { argmemonly nofree nounwind willreturn } 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           40:  
check:38'0     ~
           41: !llvm.module.flags = !{!0, !1} 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           42:  
check:38'0     ~
           43: !0 = !{i32 7, !"PIC Level", i32 2} 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           44: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------



