plain
failures:

---- [codegen] tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll" "/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:24:12: error: CHECK: expected string not found in input
 // CHECK: call void @llvm.memcpy.{{.+}}({{.*}} align 16 {{.*}} align 4 {{.*}} 16, i1 false)
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:7:27: note: scanning from here
define void @build_array_s(ptr noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, ptr noalias nocapture noundef readonly dereferenceable(16) %x) unnamed_addr #0 {
                          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:9:2: note: possible intended match here
 call void @llvm.memcpy.p0.p0.i32(ptr align 8 %0, ptr align 4 %x, i32 16, i1 false)
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:32:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:32:12: error: CHECK: expected string not found in input
 // CHECK: store <4 x float> %[[VAL:.+]], {{ptr %0|.+>\* %.+}}, align 16
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:16:40: note: scanning from here
 %1 = load <4 x float>, ptr %x, align 4
                                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:17:2: note: possible intended match here
 store <4 x float> %1, ptr %0, align 8
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:39:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:39:12: error: CHECK: expected string not found in input
 // CHECK: call void @llvm.memcpy.{{.+}}({{.*}} align 16 {{.*}} align 4 {{.*}} 16, i1 false)
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:22:27: note: scanning from here
define void @build_array_t(ptr noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, ptr noalias nocapture noundef readonly dereferenceable(16) %x) unnamed_addr #0 {
                          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:24:2: note: possible intended match here
 call void @llvm.memcpy.p0.p0.i32(ptr align 8 %0, ptr align 4 %x, i32 16, i1 false)
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:47:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:47:12: error: CHECK: expected string not found in input
 // CHECK: store <4 x float> %[[VAL:.+]], {{ptr %0|.+>\* %.+}}, align 16
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:31:40: note: scanning from here
 %1 = load <4 x float>, ptr %x, align 4
                                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:32:2: note: possible intended match here
 store <4 x float> %1, ptr %0, align 8
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:54:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:54:12: error: CHECK: expected string not found in input
 // CHECK: store float %a, {{.+}}, align 16
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:37:27: note: scanning from here
define void @build_array_u(ptr noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, ptr noalias nocapture noundef readonly dereferenceable(16) %x) unnamed_addr #0 {
                          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:47:2: note: possible intended match here
 store float %a, ptr %0, align 8
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:66:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:66:12: error: CHECK: expected string not found in input
 // CHECK: store <4 x float> %[[VAL:.+]], {{ptr %0|.+>\* %.+}}, align 16
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:60:40: note: scanning from here
 %1 = load <4 x float>, ptr %x, align 4
                                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:61:2: note: possible intended match here
 store <4 x float> %1, ptr %0, align 8

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll
Check file: /checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'simd_intrinsic_transmute_array.5139c7eb-cgu.0' 
            2: source_filename = "simd_intrinsic_transmute_array.5139c7eb-cgu.0" 
            3: target datalayout = "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64" 
            4: target triple = "arm-unknown-linux-android" 
            5:  
            6: ; Function Attrs: nonlazybind uwtable 
            7: define void @build_array_s(ptr noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, ptr noalias nocapture noundef readonly dereferenceable(16) %x) unnamed_addr #0 { 
check:24'0                               X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            8: start: 
check:24'0     ~~~~~~~
            9:  call void @llvm.memcpy.p0.p0.i32(ptr align 8 %0, ptr align 4 %x, i32 16, i1 false) 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:24'1      ?                                                                                   possible intended match
           10:  ret void 
check:24'0     ~~~~~~~~~~
           11: } 
check:24'0     ~~
           12:  
check:24'0     ~
           13: ; Function Attrs: nonlazybind uwtable 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14: define void @build_array_transmute_s(ptr noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, ptr noalias nocapture noundef readonly dereferenceable(16) %x) unnamed_addr #0 { 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           15: start: 
           16:  %1 = load <4 x float>, ptr %x, align 4 
check:32'0                                            X error: no match found
           17:  store <4 x float> %1, ptr %0, align 8 
check:32'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:32'1      ?                                      possible intended match
           18:  ret void 
check:32'0     ~~~~~~~~~~
           19: } 
check:32'0     ~~
           20:  
check:32'0     ~
           21: ; Function Attrs: nonlazybind uwtable 
check:32'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           22: define void @build_array_t(ptr noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, ptr noalias nocapture noundef readonly dereferenceable(16) %x) unnamed_addr #0 { 
check:32'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
check:39'0                               X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           23: start: 
check:39'0     ~~~~~~~
           24:  call void @llvm.memcpy.p0.p0.i32(ptr align 8 %0, ptr align 4 %x, i32 16, i1 false) 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:39'1      ?                                                                                   possible intended match
           25:  ret void 
check:39'0     ~~~~~~~~~~
           26: } 
check:39'0     ~~
           27:  
check:39'0     ~
           28: ; Function Attrs: nonlazybind uwtable 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           29: define void @build_array_transmute_t(ptr noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, ptr noalias nocapture noundef readonly dereferenceable(16) %x) unnamed_addr #0 { 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           30: start: 
           31:  %1 = load <4 x float>, ptr %x, align 4 
check:47'0                                            X error: no match found
           32:  store <4 x float> %1, ptr %0, align 8 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:47'1      ?                                      possible intended match
           33:  ret void 
check:47'0     ~~~~~~~~~~
           34: } 
check:47'0     ~~
           35:  
check:47'0     ~
           36: ; Function Attrs: nonlazybind uwtable 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           37: define void @build_array_u(ptr noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, ptr noalias nocapture noundef readonly dereferenceable(16) %x) unnamed_addr #0 { 
check:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
check:54'0                               X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           38: start: 
check:54'0     ~~~~~~~
           39:  %1 = getelementptr inbounds [4 x float], ptr %x, i32 0, i32 0 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           40:  %a = load float, ptr %1, align 4, !noundef !2 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           41:  %2 = getelementptr inbounds [4 x float], ptr %x, i32 0, i32 1 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           42:  %b = load float, ptr %2, align 4, !noundef !2 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           43:  %3 = getelementptr inbounds [4 x float], ptr %x, i32 0, i32 2 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           44:  %c = load float, ptr %3, align 4, !noundef !2 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           45:  %4 = getelementptr inbounds [4 x float], ptr %x, i32 0, i32 3 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           46:  %d = load float, ptr %4, align 4, !noundef !2 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           47:  store float %a, ptr %0, align 8 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:54'1      ?                                possible intended match
           48:  %5 = getelementptr inbounds <4 x float>, ptr %0, i32 0, i32 1 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           49:  store float %b, ptr %5, align 4 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           50:  %6 = getelementptr inbounds <4 x float>, ptr %0, i32 0, i32 2 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           51:  store float %c, ptr %6, align 8 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           52:  %7 = getelementptr inbounds <4 x float>, ptr %0, i32 0, i32 3 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           53:  store float %d, ptr %7, align 4 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           54:  ret void 
check:54'0     ~~~~~~~~~~
           55: } 
check:54'0     ~~
           56:  
check:54'0     ~
           57: ; Function Attrs: nonlazybind uwtable 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           58: define void @build_array_transmute_u(ptr noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, ptr noalias nocapture noundef readonly dereferenceable(16) %x) unnamed_addr #0 { 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           59: start: 
           60:  %1 = load <4 x float>, ptr %x, align 4 
check:66'0                                            X error: no match found
           61:  store <4 x float> %1, ptr %0, align 8 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:66'1      ?                                      possible intended match
           62:  ret void 
check:66'0     ~~~~~~~~~~
           63: } 
check:66'0     ~~
           64:  
check:66'0     ~
           65: ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite) 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           66: declare void @llvm.memcpy.p0.p0.i32(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i32, i1 immarg) #1 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           67:  
check:66'0     ~
           68: attributes #0 = { nonlazybind uwtable "target-cpu"="generic" "target-features"="+strict-align,+v5te" } 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           69: attributes #1 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) } 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           70:  
check:66'0     ~
           71: !llvm.module.flags = !{!0, !1} 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           72:  
check:66'0     ~
           73: !0 = !{i32 8, !"PIC Level", i32 2} 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           74: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           75: !2 = !{} 
check:66'0     ~~~~~~~~~
------------------------------------------



