plain
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=wasm32-unknown-emscripten

failures:

---- [codegen] src/test/codegen/cold-call-declare-and-call.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/cold-call-declare-and-call/cold-call-declare-and-call.ll" "/checkout/src/test/codegen/cold-call-declare-and-call.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/cold-call-declare-and-call.rs:6:11: error: CHECK: expected string not found in input
// CHECK: define coldcc void @this_should_never_happen(i16
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/cold-call-declare-and-call/cold-call-declare-and-call.ll:1:1: note: scanning from here
; ModuleID = 'cold_call_declare_and_call.8e146746-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/cold-call-declare-and-call/cold-call-declare-and-call.ll:7:11: note: possible intended match here
define dso_local coldcc void @this_should_never_happen(i16 zeroext %x) unnamed_addr #0 {


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/cold-call-declare-and-call/cold-call-declare-and-call.ll
Check file: /checkout/src/test/codegen/cold-call-declare-and-call.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'cold_call_declare_and_call.8e146746-cgu.0' 
check:6'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           2: source_filename = "cold_call_declare_and_call.8e146746-cgu.0" 
check:6'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           3: target datalayout = "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-f128:64-n32:64-S128-ni:1:10:20" 
check:6'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           4: target triple = "wasm32-unknown-emscripten" 
check:6'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           5:  
check:6'0     ~
           6: ; Function Attrs: uwtable 
check:6'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           7: define dso_local coldcc void @this_should_never_happen(i16 zeroext %x) unnamed_addr #0 { 
check:6'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:6'1               ?                                                                               possible intended match
           8: start: 
check:6'0     ~~~~~~~
           9:  ret void 
check:6'0     ~~~~~~~~~~
          10: } 
check:6'0     ~~
          11:  
check:6'0     ~
          12: ; cold_call_declare_and_call::do_things 
check:6'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
>>>>>>
------------------------------------------
