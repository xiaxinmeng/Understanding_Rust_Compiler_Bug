plain
failures:

---- [codegen] src/test/codegen/issue-86106.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/ci-llvm/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-86106/issue-86106.ll" "/checkout/src/test/codegen/issue-86106.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/issue-86106.rs:9:17: error: CHECK-LABEL: expected string not found in input
// CHECK-LABEL: @string_new = unnamed_addr alias void (ptr), ptr @empty_to_string
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-86106/issue-86106.ll:1:1: note: scanning from here
; ModuleID = 'issue_86106.f0cbdd92-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-86106/issue-86106.ll:21:108: note: possible intended match here
define void @empty_to_string(ptr noalias nocapture noundef writeonly sret(%"alloc::string::String") dereferenceable(24) %0) unnamed_addr #1 personality ptr @rust_eh_personality {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-86106/issue-86106.ll
Check file: /checkout/src/test/codegen/issue-86106.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'issue_86106.f0cbdd92-cgu.0' 
label:9'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           2: source_filename = "issue_86106.f0cbdd92-cgu.0" 
label:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
label:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           4: target triple = "x86_64-unknown-linux-gnu" 
label:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           5:  
label:9'0     ~
           6: %"alloc::string::String" = type { %"alloc::vec::Vec<u8>" } 
label:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
           .
          16:  store i64 0, ptr %_1.sroa.5.0..sroa_idx, align 8 
label:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          17:  ret void 
label:9'0     ~~~~~~~~~~
          18: } 
label:9'0     ~~
          19:  
label:9'0     ~
          20: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind willreturn uwtable 
label:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          21: define void @empty_to_string(ptr noalias nocapture noundef writeonly sret(%"alloc::string::String") dereferenceable(24) %0) unnamed_addr #1 personality ptr @rust_eh_personality { 
label:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
label:9'1                                                                                                                ?                                                                        possible intended match
          22: start: 
label:9'0     ~~~~~~~
          23:  store i64 0, ptr %0, align 8 
label:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          24:  %_10.sroa.4.0..sroa_idx = getelementptr inbounds i8, ptr %0, i64 8 
label:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          25:  store ptr inttoptr (i64 1 to ptr), ptr %_10.sroa.4.0..sroa_idx, align 8 
label:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          26:  %_10.sroa.5.0..sroa_idx = getelementptr inbounds i8, ptr %0, i64 16 
label:9'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
>>>>>>
------------------------------------------
