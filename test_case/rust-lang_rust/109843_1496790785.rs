plain
failures:

---- [codegen] tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll" "/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:24:12: error: CHECK: expected string not found in input
 // CHECK: ret [[USIZE:i[0-9]+]] [[ARRAY_ALIGN:[0-9]+]]
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:21:25: note: scanning from here
define i64 @array_align() unnamed_addr #1 {
                        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:25:2: note: possible intended match here
 ret i64 %0
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:31:18: error: undefined variable: USIZE
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:31:18: error: undefined variable: USIZE
 // CHECK: ret [[USIZE]] [[VECTOR_ALIGN:[0-9]+]]
                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:33:2: note: possible intended match here
 ret i64 %0
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:38:57: error: undefined variable: VECTOR_ALIGN
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:38:57: error: undefined variable: VECTOR_ALIGN
 // CHECK: call void @llvm.memcpy.{{.+}}({{.*}} align [[VECTOR_ALIGN]] {{.*}} align [[ARRAY_ALIGN]] {{.*}}, [[USIZE]] 16, i1 false)
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:38:87: error: undefined variable: ARRAY_ALIGN
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:38:87: error: undefined variable: ARRAY_ALIGN
 // CHECK: call void @llvm.memcpy.{{.+}}({{.*}} align [[VECTOR_ALIGN]] {{.*}} align [[ARRAY_ALIGN]] {{.*}}, [[USIZE]] 16, i1 false)
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:38:111: error: undefined variable: USIZE
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:38:111: error: undefined variable: USIZE
 // CHECK: call void @llvm.memcpy.{{.+}}({{.*}} align [[VECTOR_ALIGN]] {{.*}} align [[ARRAY_ALIGN]] {{.*}}, [[USIZE]] 16, i1 false)
                                                                                                              ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:39:2: note: possible intended match here
 call void @llvm.memcpy.p0.p0.i64(ptr align 16 %0, ptr align 4 %x, i64 16, i1 false)
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:45:74: error: undefined variable: ARRAY_ALIGN
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:45:74: error: undefined variable: ARRAY_ALIGN
 // CHECK: %[[VAL:.+]] = load <4 x float>, {{ptr %x|.+>\* %.+}}, align [[ARRAY_ALIGN]]
                                                                         ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:46:2: note: possible intended match here
 %1 = load <4 x float>, ptr %x, align 4
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:53:57: error: undefined variable: VECTOR_ALIGN
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:53:57: error: undefined variable: VECTOR_ALIGN
 // CHECK: call void @llvm.memcpy.{{.+}}({{.*}} align [[VECTOR_ALIGN]] {{.*}} align [[ARRAY_ALIGN]] {{.*}}, [[USIZE]] 16, i1 false)
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:53:87: error: undefined variable: ARRAY_ALIGN
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:53:87: error: undefined variable: ARRAY_ALIGN
 // CHECK: call void @llvm.memcpy.{{.+}}({{.*}} align [[VECTOR_ALIGN]] {{.*}} align [[ARRAY_ALIGN]] {{.*}}, [[USIZE]] 16, i1 false)
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:53:111: error: undefined variable: USIZE
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:53:111: error: undefined variable: USIZE
 // CHECK: call void @llvm.memcpy.{{.+}}({{.*}} align [[VECTOR_ALIGN]] {{.*}} align [[ARRAY_ALIGN]] {{.*}}, [[USIZE]] 16, i1 false)
                                                                                                              ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:54:2: note: possible intended match here
 call void @llvm.memcpy.p0.p0.i64(ptr align 16 %0, ptr align 4 %x, i64 16, i1 false)
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:60:74: error: undefined variable: ARRAY_ALIGN
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:60:74: error: undefined variable: ARRAY_ALIGN
 // CHECK: %[[VAL:.+]] = load <4 x float>, {{ptr %x|.+>\* %.+}}, align [[ARRAY_ALIGN]]
                                                                         ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:61:2: note: possible intended match here
 %1 = load <4 x float>, ptr %x, align 4
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:68:44: error: undefined variable: VECTOR_ALIGN
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:68:44: error: undefined variable: VECTOR_ALIGN
 // CHECK: store float %a, {{.+}}, align [[VECTOR_ALIGN]]
                                           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:77:2: note: possible intended match here
 store float %a, ptr %0, align 16
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:79:74: error: undefined variable: ARRAY_ALIGN
/checkout/tests/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:79:74: error: undefined variable: ARRAY_ALIGN
 // CHECK: %[[VAL:.+]] = load <4 x float>, {{ptr %x|.+>\* %.+}}, align [[ARRAY_ALIGN]]
                                                                         ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:90:2: note: possible intended match here
 %1 = load <4 x float>, ptr %x, align 4

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
            6: ; core::mem::align_of 
            6: ; core::mem::align_of 
            7: ; Function Attrs: alwaysinline nonlazybind uwtable 
            8: define internal i64 @_ZN4core3mem8align_of17hb75643aa12a33f1aE() unnamed_addr #0 { 
           10:  ret i64 4 
           11: } 
           12:  
           13: ; core::mem::align_of 
           13: ; core::mem::align_of 
           14: ; Function Attrs: alwaysinline nonlazybind uwtable 
           15: define internal i64 @_ZN4core3mem8align_of17hf7bede11ad1f5407E() unnamed_addr #0 { 
           17:  ret i64 16 
           18: } 
           19:  
           19:  
           20: ; Function Attrs: nonlazybind uwtable 
           21: define i64 @array_align() unnamed_addr #1 { 
check:24'0                             X~~~~~~~~~~~~~~~~~~~ error: no match found
           22: start: 
check:24'0     ~~~~~~~
           23: ; call core::mem::align_of 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
           24:  %0 = call i64 @_ZN4core3mem8align_of17hb75643aa12a33f1aE() 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           25:  ret i64 %0 
check:24'0     ~~~~~~~~~~~~
check:24'1      ?           possible intended match
           26: } 
check:24'0     ~~
           27:  
check:24'0     ~
           28: ; Function Attrs: nonlazybind uwtable 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           29: define i64 @vector_align() unnamed_addr #1 { 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
check:31'0                              X~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:31'1                                                   undefined variable: USIZE
           30: start: 
check:31'0     ~~~~~~~
           31: ; call core::mem::align_of 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
           32:  %0 = call i64 @_ZN4core3mem8align_of17hf7bede11ad1f5407E() 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           33:  ret i64 %0 
check:31'0     ~~~~~~~~~~~~
check:31'2      ?           possible intended match
           34: } 
check:31'0     ~~
           35:  
check:31'0     ~
           36: ; Function Attrs: nonlazybind uwtable 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           37: define void @build_array_s(ptr sret(<4 x float>) %0, ptr %x) unnamed_addr #1 { 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
check:38'0                               X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:38'1                                                                                     undefined variable: VECTOR_ALIGN
check:38'2                                                                                     undefined variable: ARRAY_ALIGN
check:38'3                                                                                     undefined variable: USIZE
           38: start: 
check:38'0     ~~~~~~~
           39:  call void @llvm.memcpy.p0.p0.i64(ptr align 16 %0, ptr align 4 %x, i64 16, i1 false) 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:38'4      ?                                                                                    possible intended match
           40:  ret void 
check:38'0     ~~~~~~~~~~
           41: } 
check:38'0     ~~
           42:  
check:38'0     ~
           43: ; Function Attrs: nonlazybind uwtable 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           44: define void @build_array_transmute_s(ptr sret(<4 x float>) %0, ptr %x) unnamed_addr #1 { 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:45'0                                         X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:45'1                                                                                               undefined variable: ARRAY_ALIGN
           45: start: 
check:45'0     ~~~~~~~
           46:  %1 = load <4 x float>, ptr %x, align 4 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:45'2      ?                                       possible intended match
           47:  store <4 x float> %1, ptr %0, align 16 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           48:  ret void 
check:45'0     ~~~~~~~~~~
           49: } 
check:45'0     ~~
           50:  
check:45'0     ~
           51: ; Function Attrs: nonlazybind uwtable 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           52: define void @build_array_t(ptr sret(<4 x float>) %0, ptr %x) unnamed_addr #1 { 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
check:53'0                               X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:53'1                                                                                     undefined variable: VECTOR_ALIGN
check:53'2                                                                                     undefined variable: ARRAY_ALIGN
check:53'3                                                                                     undefined variable: USIZE
           53: start: 
check:53'0     ~~~~~~~
           54:  call void @llvm.memcpy.p0.p0.i64(ptr align 16 %0, ptr align 4 %x, i64 16, i1 false) 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:53'4      ?                                                                                    possible intended match
           55:  ret void 
check:53'0     ~~~~~~~~~~
           56: } 
check:53'0     ~~
           57:  
check:53'0     ~
           58: ; Function Attrs: nonlazybind uwtable 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           59: define void @build_array_transmute_t(ptr sret(<4 x float>) %0, ptr %x) unnamed_addr #1 { 
check:53'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:60'0                                         X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:60'1                                                                                               undefined variable: ARRAY_ALIGN
           60: start: 
check:60'0     ~~~~~~~
           61:  %1 = load <4 x float>, ptr %x, align 4 
check:60'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:60'2      ?                                       possible intended match
           62:  store <4 x float> %1, ptr %0, align 16 
check:60'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           63:  ret void 
check:60'0     ~~~~~~~~~~
           64: } 
check:60'0     ~~
           65:  
check:60'0     ~
           66: ; Function Attrs: nonlazybind uwtable 
check:60'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           67: define void @build_array_u(ptr sret(<4 x float>) %0, ptr %x) unnamed_addr #1 { 
check:60'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
check:68'0                               X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:68'1                                                                                     undefined variable: VECTOR_ALIGN
           68: start: 
check:68'0     ~~~~~~~
           69:  %1 = getelementptr inbounds [4 x float], ptr %x, i64 0, i64 0 
check:68'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           70:  %a = load float, ptr %1, align 4, !noundef !2 
check:68'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           71:  %2 = getelementptr inbounds [4 x float], ptr %x, i64 0, i64 1 
check:68'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           72:  %b = load float, ptr %2, align 4, !noundef !2 
check:68'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           73:  %3 = getelementptr inbounds [4 x float], ptr %x, i64 0, i64 2 
check:68'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           74:  %c = load float, ptr %3, align 4, !noundef !2 
check:68'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           75:  %4 = getelementptr inbounds [4 x float], ptr %x, i64 0, i64 3 
check:68'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           76:  %d = load float, ptr %4, align 4, !noundef !2 
check:68'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           77:  store float %a, ptr %0, align 16 
check:68'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:68'2      ?                                 possible intended match
           78:  %5 = getelementptr inbounds <4 x float>, ptr %0, i32 0, i32 1 
check:68'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           79:  store float %b, ptr %5, align 4 
check:68'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           80:  %6 = getelementptr inbounds <4 x float>, ptr %0, i32 0, i32 2 
check:68'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           81:  store float %c, ptr %6, align 8 
check:68'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           82:  %7 = getelementptr inbounds <4 x float>, ptr %0, i32 0, i32 3 
check:68'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           83:  store float %d, ptr %7, align 4 
check:68'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           84:  ret void 
check:68'0     ~~~~~~~~~~
           85: } 
check:68'0     ~~
           86:  
check:68'0     ~
           87: ; Function Attrs: nonlazybind uwtable 
check:68'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           88: define void @build_array_transmute_u(ptr sret(<4 x float>) %0, ptr %x) unnamed_addr #1 { 
check:68'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:79'0                                         X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:79'1                                                                                               undefined variable: ARRAY_ALIGN
           89: start: 
check:79'0     ~~~~~~~
           90:  %1 = load <4 x float>, ptr %x, align 4 
check:79'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:79'2      ?                                       possible intended match
           91:  store <4 x float> %1, ptr %0, align 16 
check:79'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           92:  ret void 
check:79'0     ~~~~~~~~~~
           93: } 
check:79'0     ~~
           94:  
check:79'0     ~
           95: ; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite) 
check:79'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           96: declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #2 
check:79'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           97:  
check:79'0     ~
           98: attributes #0 = { alwaysinline nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" } 
check:79'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           99: attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" } 
check:79'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          100: attributes #2 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) } 
check:79'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          101:  
check:79'0     ~
          102: !llvm.module.flags = !{!0, !1} 
check:79'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          103:  
check:79'0     ~
          104: !0 = !{i32 8, !"PIC Level", i32 2} 
check:79'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          105: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:79'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          106: !2 = !{} 
check:79'0     ~~~~~~~~~
------------------------------------------



