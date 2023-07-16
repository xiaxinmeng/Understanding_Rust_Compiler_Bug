plain
Some tests failed in compiletest suite=codegen mode=codegen host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu

failures:

---- [codegen] src/test/codegen/align-byval.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/align-byval/align-byval.ll" "/checkout/src/test/codegen/align-byval.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/align-byval.rs:14:12: error: CHECK: expected string not found in input
 // CHECK: declare void @f({{.*}}byval(%Foo) align 16{{.*}})
           ^
/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/align-byval/align-byval.ll:1:1: note: scanning from here
; ModuleID = 'align_byval.33f0fd8f-cgu.0'
^
/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/align-byval/align-byval.ll:63:2: note: possible intended match here
 %_1 = alloca %Foo, align 16


Input file: /checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/align-byval/align-byval.ll
Check file: /checkout/src/test/codegen/align-byval.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'align_byval.33f0fd8f-cgu.0' 
check:14'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "align_byval.33f0fd8f-cgu.0" 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128" 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "aarch64-unknown-linux-gnu" 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:14'0     ~
            6: %Foo = type { [16 x i32] } 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
           58:  
           58:  
check:14'0     ~
           59: ; align_byval::main 
check:14'0     ~~~~~~~~~~~~~~~~~~~~
           60: ; Function Attrs: nonlazybind uwtable 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           61: define internal void @_ZN11align_byval4main17h5c26326f2afbd37eE() unnamed_addr #1 { 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           62: start: 
check:14'0     ~~~~~~~
           63:  %_1 = alloca %Foo, align 16 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:14'1      ?                            possible intended match
           64:  call void @llvm.lifetime.start.p0(i64 64, ptr nonnull %_1) 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           65:  store i32 1, ptr %_1, align 16 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           66:  %_2.sroa.4.0._1.sroa_idx = getelementptr inbounds i8, ptr %_1, i64 4 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           67:  store i32 1, ptr %_2.sroa.4.0._1.sroa_idx, align 4 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           68:  %_2.sroa.5.0._1.sroa_idx = getelementptr inbounds i8, ptr %_1, i64 8 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
