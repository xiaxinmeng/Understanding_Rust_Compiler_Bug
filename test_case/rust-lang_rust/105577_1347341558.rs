plain
running 391 tests
i.....i..............i....i..ii.................iii........ii.i........i................ 88/391
..ii.................i............i....i............i....i...iii........i..i.....i.iii.i 176/391
ii.........i.iii...i..i....................i..ii...i.....ii..i.ii....i...............ii. 264/391
.......................i.i.ii.i.iF.F...FF....i..i....i....i..iii........i..ii........... 352/391
..........iiiiiiii.i.........F......F..

---- [codegen] src/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask.rs stdout ----


error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask/simd-intrinsic-generic-bitmask.ll" "/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:38:12: error: CHECK: expected string not found in input
 // CHECK: [[A:%[0-9]+]] = lshr <2 x i32> %{{x|_2}}, <i32 31, i32 31>
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask/simd-intrinsic-generic-bitmask.ll:7:23: note: scanning from here
define i8 @bitmask_int(<2 x i32>* noalias nocapture noundef dereferenceable(8) %x) unnamed_addr #0 {
                      ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask/simd-intrinsic-generic-bitmask.ll:12:2: note: possible intended match here
 %2 = lshr <2 x i32> %1, <i32 31, i32 31>
/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:48:12: error: CHECK: expected string not found in input
/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:48:12: error: CHECK: expected string not found in input
 // CHECK: [[A:%[0-9]+]] = lshr <2 x i32> %{{x|_2}}, <i32 31, i32 31>
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask/simd-intrinsic-generic-bitmask.ll:23:24: note: scanning from here
define i8 @bitmask_uint(<2 x i32>* noalias nocapture noundef dereferenceable(8) %x) unnamed_addr #0 {
                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask/simd-intrinsic-generic-bitmask.ll:28:2: note: possible intended match here
 %2 = lshr <2 x i32> %1, <i32 31, i32 31>
/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:58:12: error: CHECK: expected string not found in input
/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask.rs:58:12: error: CHECK: expected string not found in input
 // CHECK: [[A:%[0-9]+]] = lshr <16 x i8> %{{x|_2}}, <i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7>
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask/simd-intrinsic-generic-bitmask.ll:39:26: note: scanning from here
define i16 @bitmask_int16(<16 x i8>* noalias nocapture noundef dereferenceable(16) %x) unnamed_addr #0 {
                         ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask/simd-intrinsic-generic-bitmask.ll:45:2: note: possible intended match here
 %3 = lshr <16 x i8> %2, <i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7>

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask/simd-intrinsic-generic-bitmask.ll
Check file: /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-bitmask.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'simd_intrinsic_generic_bitmask.e1f6f7bc-cgu.0' 
            2: source_filename = "simd_intrinsic_generic_bitmask.e1f6f7bc-cgu.0" 
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
            4: target triple = "x86_64-unknown-linux-gnu" 
            5:  
            6: ; Function Attrs: nonlazybind uwtable 
            7: define i8 @bitmask_int(<2 x i32>* noalias nocapture noundef dereferenceable(8) %x) unnamed_addr #0 { 
check:38'0                           X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            8: start: 
check:38'0     ~~~~~~~
            9:  %0 = alloca i8, align 1 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
           10:  call void @llvm.lifetime.start.p0i8(i64 1, i8* %0) 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           11:  %1 = load <2 x i32>, <2 x i32>* %x, align 8 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           12:  %2 = lshr <2 x i32> %1, <i32 31, i32 31> 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:38'1      ?                                         possible intended match
           13:  %3 = trunc <2 x i32> %2 to <2 x i1> 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14:  %4 = bitcast <2 x i1> %3 to i2 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           15:  %5 = zext i2 %4 to i8 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~
           16:  store i8 %5, i8* %0, align 1 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           17:  %6 = load i8, i8* %0, align 1 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18:  call void @llvm.lifetime.end.p0i8(i64 1, i8* %0) 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           19:  ret i8 %6 
check:38'0     ~~~~~~~~~~~
           20: } 
check:38'0     ~~
           21:  
check:38'0     ~
           22: ; Function Attrs: nonlazybind uwtable 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           23: define i8 @bitmask_uint(<2 x i32>* noalias nocapture noundef dereferenceable(8) %x) unnamed_addr #0 { 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~
check:48'0                            X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           24: start: 
check:48'0     ~~~~~~~
           25:  %0 = alloca i8, align 1 
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
           26:  call void @llvm.lifetime.start.p0i8(i64 1, i8* %0) 
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           27:  %1 = load <2 x i32>, <2 x i32>* %x, align 8 
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           28:  %2 = lshr <2 x i32> %1, <i32 31, i32 31> 
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:48'1      ?                                         possible intended match
           29:  %3 = trunc <2 x i32> %2 to <2 x i1> 
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           30:  %4 = bitcast <2 x i1> %3 to i2 
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           31:  %5 = zext i2 %4 to i8 
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~
           32:  store i8 %5, i8* %0, align 1 
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           33:  %6 = load i8, i8* %0, align 1 
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           34:  call void @llvm.lifetime.end.p0i8(i64 1, i8* %0) 
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           35:  ret i8 %6 
check:48'0     ~~~~~~~~~~~
           36: } 
check:48'0     ~~
           37:  
check:48'0     ~
           38: ; Function Attrs: nonlazybind uwtable 
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           39: define i16 @bitmask_int16(<16 x i8>* noalias nocapture noundef dereferenceable(16) %x) unnamed_addr #0 { 
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
check:58'0                              X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           40: start: 
check:58'0     ~~~~~~~
           41:  %0 = alloca i16, align 2 
check:58'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           42:  %1 = bitcast i16* %0 to i8* 
check:58'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           43:  call void @llvm.lifetime.start.p0i8(i64 2, i8* %1) 
check:58'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           44:  %2 = load <16 x i8>, <16 x i8>* %x, align 16 
check:58'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           45:  %3 = lshr <16 x i8> %2, <i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7, i8 7> 
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
check:58'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:58'1      ?                                                                                                                         possible intended match
           46:  %4 = trunc <16 x i8> %3 to <16 x i1> 
check:58'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           47:  %5 = bitcast <16 x i1> %4 to i16 
check:58'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           48:  store i16 %5, i16* %0, align 2 
check:58'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           49:  %6 = load i16, i16* %0, align 2 
check:58'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           50:  %7 = bitcast i16* %0 to i8* 
check:58'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] src/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert/simd-intrinsic-generic-extract-insert.ll" "/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert.rs:24:12: error: CHECK: expected string not found in input
 // CHECK: extractelement <4 x float> %{{v|_3}}, i32 %i
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert/simd-intrinsic-generic-extract-insert.ll:7:24: note: scanning from here
define float @extract_m(<4 x float>* noalias nocapture noundef dereferenceable(16) %v, i32 %i) unnamed_addr #0 {
                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert/simd-intrinsic-generic-extract-insert.ll:13:7: note: possible intended match here
 %3 = extractelement <4 x float> %2, i32 %i
/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert.rs:31:12: error: CHECK: expected string not found in input
/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert.rs:31:12: error: CHECK: expected string not found in input
 // CHECK: extractelement <4 x float> %{{v|_3}}, i32 %i
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert/simd-intrinsic-generic-extract-insert.ll:22:24: note: scanning from here
define float @extract_s(<4 x float>* noalias nocapture noundef dereferenceable(16) %v, i32 %i) unnamed_addr #0 {
                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert/simd-intrinsic-generic-extract-insert.ll:28:7: note: possible intended match here
 %3 = extractelement <4 x float> %2, i32 %i
/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert.rs:38:12: error: CHECK: expected string not found in input
/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert.rs:38:12: error: CHECK: expected string not found in input
 // CHECK: insertelement <4 x float> %{{v|_4}}, float %j, i32 %i
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert/simd-intrinsic-generic-extract-insert.ll:37:22: note: scanning from here
define void @insert_m(<4 x float>* noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, <4 x float>* noalias nocapture noundef dereferenceable(16) %v, i32 %i, float %j) unnamed_addr #0 {
                     ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert/simd-intrinsic-generic-extract-insert.ll:40:7: note: possible intended match here
 %2 = insertelement <4 x float> %1, float %j, i32 %i
/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert.rs:45:12: error: CHECK: expected string not found in input
/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert.rs:45:12: error: CHECK: expected string not found in input
 // CHECK: insertelement <4 x float> %{{v|_4}}, float %j, i32 %i
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert/simd-intrinsic-generic-extract-insert.ll:46:22: note: scanning from here
define void @insert_s(<4 x float>* noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, <4 x float>* noalias nocapture noundef dereferenceable(16) %v, i32 %i, float %j) unnamed_addr #0 {
                     ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert/simd-intrinsic-generic-extract-insert.ll:49:7: note: possible intended match here
 %2 = insertelement <4 x float> %1, float %j, i32 %i

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert/simd-intrinsic-generic-extract-insert.ll
Check file: /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-generic-extract-insert.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'simd_intrinsic_generic_extract_insert.0edbc00a-cgu.0' 
            2: source_filename = "simd_intrinsic_generic_extract_insert.0edbc00a-cgu.0" 
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
            4: target triple = "x86_64-unknown-linux-gnu" 
            5:  
            6: ; Function Attrs: nonlazybind uwtable 
            7: define float @extract_m(<4 x float>* noalias nocapture noundef dereferenceable(16) %v, i32 %i) unnamed_addr #0 { 
check:24'0                            X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            8: start: 
check:24'0     ~~~~~~~
            9:  %0 = alloca float, align 4 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           10:  %1 = bitcast float* %0 to i8* 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           11:  call void @llvm.lifetime.start.p0i8(i64 4, i8* %1) 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           12:  %2 = load <4 x float>, <4 x float>* %v, align 16 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13:  %3 = extractelement <4 x float> %2, i32 %i 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:24'1           ?                                      possible intended match
           14:  store float %3, float* %0, align 4 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           15:  %4 = load float, float* %0, align 4 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           16:  %5 = bitcast float* %0 to i8* 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           17:  call void @llvm.lifetime.end.p0i8(i64 4, i8* %5) 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18:  ret float %4 
check:24'0     ~~~~~~~~~~~~~~
           19: } 
check:24'0     ~~
           20:  
check:24'0     ~
           21: ; Function Attrs: nonlazybind uwtable 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           22: define float @extract_s(<4 x float>* noalias nocapture noundef dereferenceable(16) %v, i32 %i) unnamed_addr #0 { 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~
check:31'0                            X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           23: start: 
check:31'0     ~~~~~~~
           24:  %0 = alloca float, align 4 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           25:  %1 = bitcast float* %0 to i8* 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           26:  call void @llvm.lifetime.start.p0i8(i64 4, i8* %1) 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           27:  %2 = load <4 x float>, <4 x float>* %v, align 16 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           28:  %3 = extractelement <4 x float> %2, i32 %i 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:31'1           ?                                      possible intended match
           29:  store float %3, float* %0, align 4 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           30:  %4 = load float, float* %0, align 4 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           31:  %5 = bitcast float* %0 to i8* 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           32:  call void @llvm.lifetime.end.p0i8(i64 4, i8* %5) 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           33:  ret float %4 
check:31'0     ~~~~~~~~~~~~~~
           34: } 
check:31'0     ~~
           35:  
check:31'0     ~
           36: ; Function Attrs: nonlazybind uwtable 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           37: define void @insert_m(<4 x float>* noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, <4 x float>* noalias nocapture noundef dereferenceable(16) %v, i32 %i, float %j) unnamed_addr #0 { 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~
check:38'0                          X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           38: start: 
check:38'0     ~~~~~~~
           39:  %1 = load <4 x float>, <4 x float>* %v, align 16 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           40:  %2 = insertelement <4 x float> %1, float %j, i32 %i 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:38'1           ?                                               possible intended match
           41:  store <4 x float> %2, <4 x float>* %0, align 16 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           42:  ret void 
check:38'0     ~~~~~~~~~~
           43: } 
check:38'0     ~~
           44:  
check:38'0     ~
           45: ; Function Attrs: nonlazybind uwtable 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           46: define void @insert_s(<4 x float>* noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, <4 x float>* noalias nocapture noundef dereferenceable(16) %v, i32 %i, float %j) unnamed_addr #0 { 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~
check:45'0                          X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           47: start: 
check:45'0     ~~~~~~~
           48:  %1 = load <4 x float>, <4 x float>* %v, align 16 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           49:  %2 = insertelement <4 x float> %1, float %j, i32 %i 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:45'1           ?                                               possible intended match
           50:  store <4 x float> %2, <4 x float>* %0, align 16 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           51:  ret void 
check:45'0     ~~~~~~~~~~
           52: } 
check:45'0     ~~
           53:  
check:45'0     ~
           54: ; Function Attrs: argmemonly nofree nosync nounwind willreturn 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] src/test/codegen/simd_arith_offset.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd_arith_offset/simd_arith_offset.ll" "/checkout/src/test/codegen/simd_arith_offset.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/simd_arith_offset.rs:24:12: error: CHECK: expected string not found in input
 // CHECK: getelementptr i8, <8 x {{i8\*|ptr}}> %_3, <8 x i64> %_4
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd_arith_offset/simd_arith_offset.ll:7:19: note: scanning from here
define void @smoke(<8 x i8*>* noalias nocapture noundef sret(<8 x i8*>) dereferenceable(64) %0, <8 x i8*>* noalias nocapture noundef dereferenceable(64) %ptrs, <8 x i64>* noalias nocapture noundef dereferenceable(64) %offsets) unnamed_addr #0 {
                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd_arith_offset/simd_arith_offset.ll:11:7: note: possible intended match here
 %3 = getelementptr i8, <8 x i8*> %1, <8 x i64> %2

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd_arith_offset/simd_arith_offset.ll
Check file: /checkout/src/test/codegen/simd_arith_offset.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'simd_arith_offset.2d137f9a-cgu.0' 
            2: source_filename = "simd_arith_offset.2d137f9a-cgu.0" 
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
            4: target triple = "x86_64-unknown-linux-gnu" 
            5:  
            6: ; Function Attrs: nonlazybind uwtable 
            7: define void @smoke(<8 x i8*>* noalias nocapture noundef sret(<8 x i8*>) dereferenceable(64) %0, <8 x i8*>* noalias nocapture noundef dereferenceable(64) %ptrs, <8 x i64>* noalias nocapture noundef dereferenceable(64) %offsets) unnamed_addr #0 { 
check:24'0                       X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            8: start: 
check:24'0     ~~~~~~~
            9:  %1 = load <8 x i8*>, <8 x i8*>* %ptrs, align 64 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           10:  %2 = load <8 x i64>, <8 x i64>* %offsets, align 64 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           11:  %3 = getelementptr i8, <8 x i8*> %1, <8 x i64> %2 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:24'1           ?                                             possible intended match
           12:  store <8 x i8*> %3, <8 x i8*>* %0, align 64 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13:  ret void 
check:24'0     ~~~~~~~~~~
           14: } 
check:24'0     ~~
           15:  
check:24'0     ~
           16: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:24'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll" "/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:25:12: error: CHECK: expected string not found in input
 // CHECK: call void @llvm.memcpy.{{.+}}({{.*}}, i{{[0-9]+}} 16, i1 false)
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:12:89: note: scanning from here
 call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 16 %2, i8* align 4 %3, i64 16, i1 false)
/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:33:12: error: CHECK: expected string not found in input
/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:33:12: error: CHECK: expected string not found in input
 // CHECK: call void @llvm.memcpy.{{.+}}({{.*}}, i{{[0-9]+}} 16, i1 false)
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:22:89: note: scanning from here
 call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 16 %2, i8* align 4 %3, i64 16, i1 false)
/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:41:12: error: CHECK: expected string not found in input
/checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs:41:12: error: CHECK: expected string not found in input
 // CHECK: call void @llvm.memcpy.{{.+}}({{.*}}, i{{[0-9]+}} 16, i1 false)
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:32:88: note: scanning from here
 call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %2, i8* align 4 %3, i64 16, i1 false)
                                                                                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll:37:3: note: possible intended match here
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #1

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array/simd-intrinsic-transmute-array.ll
Check file: /checkout/src/test/codegen/simd-intrinsic/simd-intrinsic-transmute-array.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
            7: define void @build_array_s(<4 x float>* noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, [4 x float]* noalias nocapture noundef readonly dereferenceable(16) %x) unnamed_addr #0 { 
            8: start: 
            9:  %1 = bitcast <4 x float>* %0 to [4 x float]* 
           10:  %2 = bitcast [4 x float]* %1 to i8* 
           11:  %3 = bitcast [4 x float]* %x to i8* 
           12:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 16 %2, i8* align 4 %3, i64 16, i1 false) 
check:25                                                                                               X error: no match found
           13:  ret void 
           14: } 
check:25       ~~
           15:  
check:25       ~
check:25       ~
           16: ; Function Attrs: nonlazybind uwtable 
check:25       ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           17: define void @build_array_t(<4 x float>* noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, [4 x float]* noalias nocapture noundef readonly dereferenceable(16) %x) unnamed_addr #0 { 
           18: start: 
           18: start: 
           19:  %1 = bitcast <4 x float>* %0 to [4 x float]* 
           20:  %2 = bitcast [4 x float]* %1 to i8* 
           21:  %3 = bitcast [4 x float]* %x to i8* 
           22:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 16 %2, i8* align 4 %3, i64 16, i1 false) 
check:33                                                                                               X error: no match found
           23:  ret void 
           24: } 
check:33       ~~
           25:  
check:33       ~
check:33       ~
           26: ; Function Attrs: nonlazybind uwtable 
check:33       ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           27: define void @build_array_u(<4 x float>* noalias nocapture noundef sret(<4 x float>) dereferenceable(16) %0, [4 x float]* noalias nocapture noundef dereferenceable(16) %x) unnamed_addr #0 { 
           28: start: 
           28: start: 
           29:  %1 = bitcast <4 x float>* %0 to [4 x float]* 
           30:  %2 = bitcast [4 x float]* %1 to i8* 
           31:  %3 = bitcast [4 x float]* %x to i8* 
           32:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %2, i8* align 4 %3, i64 16, i1 false) 
check:41'0                                                                                            X error: no match found
           33:  ret void 
check:41'0     ~~~~~~~~~~
           34: } 
check:41'0     ~~
           35:  
check:41'0     ~
           36: ; Function Attrs: argmemonly nofree nounwind willreturn 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           37: declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #1 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:41'1       ?                                                                                                                          possible intended match
           38:  
check:41'0     ~
           39: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           40: attributes #1 = { argmemonly nofree nounwind willreturn } 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           41:  
check:41'0     ~
           42: !llvm.module.flags = !{!0, !1} 
check:41'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] src/test/codegen/var-names.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/var-names/var-names.ll" "/checkout/src/test/codegen/var-names.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/var-names.rs:12:17: error: CHECK-NEXT: expected string not found in input
 // CHECK-NEXT: %e = mul i32 %c, %a
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/var-names/var-names.ll:9:21: note: scanning from here
 %c = add i32 %a, %b
