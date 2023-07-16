plain
failures:

---- [codegen] codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi/riscv64-lp64-lp64f-lp64d-abi.ll" "/checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:18:11: error: CHECK: expected string not found in input
// CHECK: define{{.*}} zeroext i1 @f_scalar_0(i1{{.+}} zeroext %a)
          ^
/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi/riscv64-lp64-lp64f-lp64d-abi.ll:11:22: note: scanning from here
define void @f_void() unnamed_addr #0 {
                     ^
/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi/riscv64-lp64-lp64f-lp64d-abi.ll:17:1: note: possible intended match here
define zeroext i1 @f_scalar_0(i1 zeroext %a) unnamed_addr #0 {

Input file: /checkout/obj/build/i686-unknown-linux-gnu/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi/riscv64-lp64-lp64f-lp64d-abi.ll
Some tests failed in compiletest suite=codegen mode=codegen host=i686-unknown-linux-gnu target=i686-unknown-linux-gnu
Check file: /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs
Check file: /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
            6: %Tiny = type { i16, i16, i16, i16 } 
            7: %SmallAligned = type { i128 } 
            8: %Large = type { i64, i64, i64, i64 } 
            9:  
           10: ; Function Attrs: nonlazybind uwtable 
           11: define void @f_void() unnamed_addr #0 { 
check:18'0                          X~~~~~~~~~~~~~~~~~~ error: no match found
           12: start: 
check:18'0     ~~~~~~~
           13:  ret void 
check:18'0     ~~~~~~~~~~
           14: } 
check:18'0     ~~
           15:  
check:18'0     ~
           16: ; Function Attrs: nonlazybind uwtable 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           17: define zeroext i1 @f_scalar_0(i1 zeroext %a) unnamed_addr #0 { 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:18'1     ?                                                               possible intended match
           18: start: 
check:18'0     ~~~~~~~
           19:  ret i1 %a 
check:18'0     ~~~~~~~~~~~
           20: } 
check:18'0     ~~
           21:  
check:18'0     ~
           22: ; Function Attrs: nonlazybind uwtable 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] codegen/union-abi.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/union-abi/union-abi.ll" "/checkout/src/test/codegen/union-abi.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/union-abi.rs:20:11: error: CHECK: expected string not found in input
// CHECK: define void @test_UnionI64x4(<4 x i64>* {{.*}} %_1)
          ^
/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/union-abi/union-abi.ll:1:1: note: scanning from here
; ModuleID = 'union_abi.875e1513-cgu.0'
^
/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/union-abi/union-abi.ll:21:1: note: possible intended match here
define void @test_UnionI64x4_(<4 x i64>* %_1) unnamed_addr #0 {

Input file: /checkout/obj/build/i686-unknown-linux-gnu/test/codegen/union-abi/union-abi.ll
Check file: /checkout/src/test/codegen/union-abi.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'union_abi.875e1513-cgu.0' 
check:20'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "union_abi.875e1513-cgu.0" 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-f64:32:64-f80:32-n8:16:32-S128" 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "i686-unknown-linux-gnu" 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:20'0     ~
            6: %UnionI64x4I64 = type { [4 x i64] } 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
            .
           16: bb1: ; preds = %bb1, %start 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           17:  br label %bb1 
check:20'0     ~~~~~~~~~~~~~~~
           18: } 
check:20'0     ~~
           19:  
check:20'0     ~
           20: ; Function Attrs: nonlazybind uwtable 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           21: define void @test_UnionI64x4_(<4 x i64>* %_1) unnamed_addr #0 { 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:20'1     ?                                                                possible intended match
           22: start: 
check:20'0     ~~~~~~~
           23:  br label %bb1 
check:20'0     ~~~~~~~~~~~~~~~
           24:  
check:20'0     ~
           25: bb1: ; preds = %bb1, %start 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           26:  br label %bb1 
check:20'0     ~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
