plain
failures:

---- [codegen] src/test/codegen/merge-functions.rs#merge stdout ----

error in revision `merge`: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/merge-functions.merge/merge-functions.ll" "/checkout/src/test/codegen/merge-functions.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,merge"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/merge-functions.rs:7:11: error: CHECK: expected string not found in input
// CHECK: @func2 = {{.*}}alias{{.*}}@func1
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/merge-functions.merge/merge-functions.ll:1:1: note: scanning from here
; ModuleID = 'merge_functions.a24fab76-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/merge-functions.merge/merge-functions.ll:1:20: note: possible intended match here
; ModuleID = 'merge_functions.a24fab76-cgu.0'

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/merge-functions.merge/merge-functions.ll
Check file: /checkout/src/test/codegen/merge-functions.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'merge_functions.a24fab76-cgu.0' 
check:7'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
check:7'1                        ?                           possible intended match
           2: source_filename = "merge_functions.a24fab76-cgu.0" 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           4: target triple = "x86_64-unknown-linux-gnu" 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           5:  
check:7'0     ~
           6: ; Function Attrs: nonlazybind uwtable 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
>>>>>>
------------------------------------------
