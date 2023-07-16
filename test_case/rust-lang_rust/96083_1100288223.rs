plain
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu

failures:

---- [codegen] src/test/codegen/simd_arith_offset.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd_arith_offset/simd_arith_offset.ll" "/checkout/src/test/codegen/simd_arith_offset.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/simd_arith_offset.rs:23:12: error: CHECK: expected string not found in input
 // CHECK: getelementptr i8, <8 x i8*> %_3, <8 x i64> %_4
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd_arith_offset/simd_arith_offset.ll:7:19: note: scanning from here
define void @smoke(<8 x i8*>* noalias nocapture noundef sret(<8 x i8*>) dereferenceable(32) %0, <8 x i8*>* noalias nocapture noundef dereferenceable(32) %ptrs, <8 x i32>* noalias nocapture noundef dereferenceable(32) %offsets) unnamed_addr #0 {
                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd_arith_offset/simd_arith_offset.ll:11:7: note: possible intended match here
 %1 = getelementptr i8, <8 x i8*> %_3, <8 x i32> %_4


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/simd_arith_offset/simd_arith_offset.ll
Check file: /checkout/src/test/codegen/simd_arith_offset.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'simd_arith_offset.2d137f9a-cgu.0' 
            2: source_filename = "simd_arith_offset.2d137f9a-cgu.0" 
            3: target datalayout = "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-f64:32:64-f80:32-n8:16:32-S128" 
            4: target triple = "i586-unknown-linux-gnu" 
            5:  
            6: ; Function Attrs: nonlazybind uwtable 
            7: define void @smoke(<8 x i8*>* noalias nocapture noundef sret(<8 x i8*>) dereferenceable(32) %0, <8 x i8*>* noalias nocapture noundef dereferenceable(32) %ptrs, <8 x i32>* noalias nocapture noundef dereferenceable(32) %offsets) unnamed_addr #0 { 
check:23'0                       X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            8: start: 
check:23'0     ~~~~~~~
            9:  %_3 = load <8 x i8*>, <8 x i8*>* %ptrs, align 32 
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           10:  %_4 = load <8 x i32>, <8 x i32>* %offsets, align 32 
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           11:  %1 = getelementptr i8, <8 x i8*> %_3, <8 x i32> %_4 
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:23'1           ?                                               possible intended match
           12:  store <8 x i8*> %1, <8 x i8*>* %0, align 32 
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13:  br label %bb1 
check:23'0     ~~~~~~~~~~~~~~~
           14:  
check:23'0     ~
           15: bb1: ; preds = %start 
check:23'0     ~~~~~~~~~~~~~~~~~~~~~~
           16:  ret void 
check:23'0     ~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
