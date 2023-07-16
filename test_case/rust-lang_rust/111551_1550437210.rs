plain
test [codegen] tests/codegen/asm-multiple-options.rs ... ignored, only executed when the architecture is x86_64
test [codegen] tests/codegen/asm-options.rs ... ignored, only executed when the architecture is x86_64
test [codegen] tests/codegen/align-byval.rs#wasm ... ok
test [codegen] tests/codegen/addr-of-mutate.rs ... FAILED
test [codegen] tests/codegen/align-byval-vector.rs#x86-darwin ... ok
test [codegen] tests/codegen/align-byval.rs#m68k ... ok
test [codegen] tests/codegen/asm-target-clobbers.rs#base ... ignored, only executed when the architecture is x86_64
test [codegen] tests/codegen/adjustments.rs ... ok
test [codegen] tests/codegen/async-fn-debug-msvc.rs ... ignored, only executed when the target environment is msvc
test [codegen] tests/codegen/align-enum.rs ... ok
test [codegen] tests/codegen/align-enum.rs ... ok
test [codegen] tests/codegen/asm-powerpc-clobbers.rs#powerpc ... ok
test [codegen] tests/codegen/autovectorize-f32x4.rs ... ignored, only executed when the architecture is x86_64
test [codegen] tests/codegen/align-byval-vector.rs#x86-linux ... ok
test [codegen] tests/codegen/align-byval.rs#x86_64-windows ... ok
test [codegen] tests/codegen/array-clone.rs ... ok
test [codegen] tests/codegen/align-byval.rs#x86_64-linux ... ok
test [codegen] tests/codegen/bpf-alu32.rs ... ignored, only executed when the architecture is bpf
---
failures:

---- [codegen] tests/codegen/addr-of-mutate.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/addr-of-mutate/addr-of-mutate.ll" "/checkout/tests/codegen/addr-of-mutate.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
/checkout/tests/codegen/addr-of-mutate.rs:9:11: error: CHECK: expected string not found in input
/checkout/tests/codegen/addr-of-mutate.rs:9:11: error: CHECK: expected string not found in input
// CHECK: i8 @foo(ptr noalias nocapture noundef dereferenceable(128) %x)
          ^
/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/addr-of-mutate/addr-of-mutate.ll:1:1: note: scanning from here
; ModuleID = 'addr_of_mutate.9ef07b3ba1d68205-cgu.0'
^
/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/addr-of-mutate/addr-of-mutate.ll:7:16: note: possible intended match here
define noundef i8 @foo(ptr noalias nocapture noundef align 1 dereferenceable(128) %x) unnamed_addr #0 {

Input file: /checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/addr-of-mutate/addr-of-mutate.ll
Check file: /checkout/tests/codegen/addr-of-mutate.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'addr_of_mutate.9ef07b3ba1d68205-cgu.0' 
check:9'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           2: source_filename = "addr_of_mutate.9ef07b3ba1d68205-cgu.0" 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           3: target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128" 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           4: target triple = "aarch64-unknown-linux-gnu" 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           5:  
check:9'0     ~
           6: ; Function Attrs: nonlazybind uwtable 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           7: define noundef i8 @foo(ptr noalias nocapture noundef align 1 dereferenceable(128) %x) unnamed_addr #0 { 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:9'1                    ?                                                                                         possible intended match
           8: start: 
check:9'0     ~~~~~~~
           9:  %0 = getelementptr inbounds [128 x i8], ptr %x, i64 0, i64 0 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          10:  store i8 1, ptr %0, align 1 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          11:  %1 = getelementptr inbounds [128 x i8], ptr %x, i64 0, i64 0 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          12:  %2 = load i8, ptr %1, align 1, !noundef !2 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          13:  ret i8 %2 
check:9'0     ~~~~~~~~~~~
          14: } 
check:9'0     ~~
          15:  
check:9'0     ~
          16: ; Function Attrs: nonlazybind uwtable 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          17: define noundef zeroext i1 @second(ptr noalias nocapture noundef align 8 dereferenceable(24) %a_ptr_and_b) unnamed_addr #0 { 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          18: start: 
check:9'0     ~~~~~~~
          19:  %0 = getelementptr inbounds { ptr, { i64, i8 } }, ptr %a_ptr_and_b, i32 0, i32 1 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          20:  %self = getelementptr inbounds { i64, i8 }, ptr %0, i32 0, i32 1 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          21:  store i8 1, ptr %self, align 1 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          22:  %1 = getelementptr inbounds { ptr, { i64, i8 } }, ptr %a_ptr_and_b, i32 0, i32 1 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          23:  %2 = getelementptr inbounds { i64, i8 }, ptr %1, i32 0, i32 1 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          24:  %3 = load i8, ptr %2, align 8, !range !3, !noundef !2 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          25:  %4 = trunc i8 %3 to i1 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~
          26:  ret i1 %4 
check:9'0     ~~~~~~~~~~~
          27: } 
check:9'0     ~~
          28:  
check:9'0     ~
          29: ; Function Attrs: nonlazybind uwtable 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          30: define noundef zeroext i1 @third(ptr noalias nocapture noundef readonly align 8 dereferenceable(24) %a_ptr_and_b) unnamed_addr #0 { 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          31: start: 
check:9'0     ~~~~~~~
          32:  %_3 = load ptr, ptr %a_ptr_and_b, align 8, !noundef !2 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          33:  %self = getelementptr inbounds { i32, i8 }, ptr %_3, i32 0, i32 1 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          34:  store i8 1, ptr %self, align 1 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          35:  %0 = getelementptr inbounds { ptr, { i64, i8 } }, ptr %a_ptr_and_b, i32 0, i32 1 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          36:  %1 = getelementptr inbounds { i64, i8 }, ptr %0, i32 0, i32 1 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          37:  %2 = load i8, ptr %1, align 8, !range !3, !noundef !2 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          38:  %3 = trunc i8 %2 to i1 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~
          39:  ret i1 %3 
check:9'0     ~~~~~~~~~~~
          40: } 
check:9'0     ~~
          41:  
check:9'0     ~
          42: attributes #0 = { nonlazybind uwtable "target-cpu"="generic" "target-features"="+v8a,+outline-atomics" } 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          43:  
check:9'0     ~
          44: !llvm.module.flags = !{!0, !1} 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          45:  
check:9'0     ~
          46: !0 = !{i32 8, !"PIC Level", i32 2} 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          47: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          48: !2 = !{} 
check:9'0     ~~~~~~~~~
          49: !3 = !{i8 0, i8 2} 
check:9'0     ~~~~~~~~~~~~~~~~~~~
------------------------------------------



