plain
failures:

---- [codegen] codegen/fastcall-inreg.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fastcall-inreg/fastcall-inreg.ll" "/checkout/src/test/codegen/fastcall-inreg.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/fastcall-inreg.rs:38:12: error: CHECK: expected string not found in input
 // CHECK: @f6(i1 inreg noundef zeroext %_1, i32 inreg %_2, i32 %_3)
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fastcall-inreg/fastcall-inreg.ll:31:49: note: scanning from here
define x86_fastcallcc void @f5(i64 %_1, i32 %_2) unnamed_addr #0 {
                                                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fastcall-inreg/fastcall-inreg.ll:37:20: note: possible intended match here
define x86_fastcallcc void @f6(i1 inreg zeroext %_1, i32 inreg %_2, i32 %_3) unnamed_addr #0 {


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/fastcall-inreg/fastcall-inreg.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
           26: start: 
           27:  ret void 
           28: } 
           29:  
           30: ; Function Attrs: nonlazybind uwtable 
           31: define x86_fastcallcc void @f5(i64 %_1, i32 %_2) unnamed_addr #0 { 
check:38'0                                                     X~~~~~~~~~~~~~~~~~~ error: no match found
           32: start: 
check:38'0     ~~~~~~~
           33:  ret void 
check:38'0     ~~~~~~~~~~
           34: } 
check:38'0     ~~
           35:  
check:38'0     ~
           36: ; Function Attrs: nonlazybind uwtable 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           37: define x86_fastcallcc void @f6(i1 inreg zeroext %_1, i32 inreg %_2, i32 %_3) unnamed_addr #0 { 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:38'1                        ?                                                                            possible intended match
           38: start: 
check:38'0     ~~~~~~~
           39:  ret void 
check:38'0     ~~~~~~~~~~
           40: } 
check:38'0     ~~
           41:  
check:38'0     ~
           42: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="pentium4" } 
check:38'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi/riscv64-lp64-lp64f-lp64d-abi.ll" "/checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:18:11: error: CHECK: expected string not found in input
// CHECK: define noundef zeroext i1 @f_scalar_0(i1 noundef zeroext %a)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi/riscv64-lp64-lp64f-lp64d-abi.ll:11:22: note: scanning from here
define void @f_void() unnamed_addr #0 {
                     ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi/riscv64-lp64-lp64f-lp64d-abi.ll:17:1: note: possible intended match here
define zeroext i1 @f_scalar_0(i1 zeroext %a) unnamed_addr #0 {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi/riscv64-lp64-lp64f-lp64d-abi.ll
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


---- [codegen] codegen/repr-transparent-aggregates-1.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repr-transparent-aggregates-1/repr-transparent-aggregates-1.ll" "/checkout/src/test/codegen/repr-transparent-aggregates-1.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/repr-transparent-aggregates-1.rs:36:11: error: CHECK: expected string not found in input
// CHECK: define{{.*}}void @test_BigS(%BigS* [[BIGS_RET_ATTRS1:.*]] sret(%BigS) [[BIGS_RET_ATTRS2:.*]], %BigS* [[BIGS_ARG_ATTRS1:.*]] byval(%BigS) [[BIGS_ARG_ATTRS2:.*]])
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repr-transparent-aggregates-1/repr-transparent-aggregates-1.ll:1:1: note: scanning from here
; ModuleID = 'repr_transparent_aggregates_1.e9dae903-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repr-transparent-aggregates-1/repr-transparent-aggregates-1.ll:16:1: note: possible intended match here
define void @test_BigS(%BigS* sret(%BigS) %0, %BigS* byval(%BigS) %_1) unnamed_addr #0 {


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/repr-transparent-aggregates-1/repr-transparent-aggregates-1.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'repr_transparent_aggregates_1.e9dae903-cgu.0' 
check:36'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "repr_transparent_aggregates_1.e9dae903-cgu.0" 
check:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
check:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "x86_64-unknown-linux-gnu" 
check:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:36'0     ~
            6: %BigS = type { [16 x i32] } 
check:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
            .
           11: %TsBigU = type { %BigU } 
check:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
           12: %TuBigU = type { [16 x i32] } 
check:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13: %"TeBigU::Variant" = type { %BigU } 
check:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14:  
check:36'0     ~
           15: ; Function Attrs: nonlazybind uwtable 
check:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           16: define void @test_BigS(%BigS* sret(%BigS) %0, %BigS* byval(%BigS) %_1) unnamed_addr #0 { 
check:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:36'1     ?                                                                                         possible intended match
           17: start: 
check:36'0     ~~~~~~~
           18:  br label %bb1 
check:36'0     ~~~~~~~~~~~~~~~
           19:  
check:36'0     ~
           20: bb1: ; preds = %bb1, %start 
check:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           21:  br label %bb1 
check:36'0     ~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] codegen/union-abi.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/union-abi/union-abi.ll" "/checkout/src/test/codegen/union-abi.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/union-abi.rs:20:11: error: CHECK: expected string not found in input
// CHECK: define void @test_UnionI64x4(<4 x i64>* {{.*}} %_1)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/union-abi/union-abi.ll:1:1: note: scanning from here
; ModuleID = 'union_abi.875e1513-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/union-abi/union-abi.ll:21:1: note: possible intended match here
define void @test_UnionI64x4_(<4 x i64>* %_1) unnamed_addr #0 {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/union-abi/union-abi.ll
Check file: /checkout/src/test/codegen/union-abi.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'union_abi.875e1513-cgu.0' 
check:20'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "union_abi.875e1513-cgu.0" 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "x86_64-unknown-linux-gnu" 
check:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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
