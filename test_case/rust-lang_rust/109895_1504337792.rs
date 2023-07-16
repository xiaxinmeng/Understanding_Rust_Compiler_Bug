plain
failures:

---- [codegen] tests/codegen/issues/issue-101082.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issues/issue-101082/issue-101082.ll" "/checkout/tests/codegen/issues/issue-101082.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/issues/issue-101082.rs:10:12: error: CHECK: expected string not found in input
 // CHECK: ret i64 165
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issues/issue-101082/issue-101082.ll:7:26: note: scanning from here
define noundef i32 @test() unnamed_addr #0 personality ptr @rust_eh_personality {
                         ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issues/issue-101082/issue-101082.ll:9:2: note: possible intended match here
 ret i32 165

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issues/issue-101082/issue-101082.ll
Check file: /checkout/tests/codegen/issues/issue-101082.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'issue_101082.6ae14e6e-cgu.0' 
            2: source_filename = "issue_101082.6ae14e6e-cgu.0" 
            3: target datalayout = "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-f64:32:64-f80:32-n8:16:32-S128" 
            4: target triple = "i586-unknown-linux-gnu" 
            5:  
            6: ; Function Attrs: nonlazybind uwtable 
            7: define noundef i32 @test() unnamed_addr #0 personality ptr @rust_eh_personality { 
check:10'0                              X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            8: bb4.preheader: 
check:10'0     ~~~~~~~~~~~~~~~
            9:  ret i32 165 
check:10'0     ~~~~~~~~~~~~~
check:10'1      ?            possible intended match
           10: } 
check:10'0     ~~
           11:  
check:10'0     ~
           12: ; Function Attrs: nonlazybind uwtable 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13: declare noundef i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #0 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14:  
check:10'0     ~
           15: attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="pentium" } 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           16:  
check:10'0     ~
           17: !llvm.module.flags = !{!0, !1} 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18:  
check:10'0     ~
           19: !0 = !{i32 8, !"PIC Level", i32 2} 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           20: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:10'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------



