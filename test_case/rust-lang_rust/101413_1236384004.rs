plain
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.
failures:

---- [codegen] src/test/codegen/abi-efiapi.rs#x86_64 stdout ----

error in revision `x86_64`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-efiapi.x86_64/abi-efiapi.ll" "/checkout/src/test/codegen/abi-efiapi.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,x86_64"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/abi-efiapi.rs:27:11: error: x86_64: expected string not found in input
//x86_64: define dso_local win64cc void @has_efiapi
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-efiapi.x86_64/abi-efiapi.ll:1:1: note: scanning from here
; ModuleID = 'abi_efiapi.cb96f543-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-efiapi.x86_64/abi-efiapi.ll:7:1: note: possible intended match here
define win64cc void @has_efiapi() unnamed_addr #0 {


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/abi-efiapi.x86_64/abi-efiapi.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'abi_efiapi.cb96f543-cgu.0' 
check:27'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "abi_efiapi.cb96f543-cgu.0" 
check:27'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
check:27'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "x86_64-unknown-windows-msvc" 
check:27'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:27'0     ~
            6: ; Function Attrs: noredzone nounwind 
check:27'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            7: define win64cc void @has_efiapi() unnamed_addr #0 { 
check:27'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:27'1     ?                                                    possible intended match
            8: start: 
check:27'0     ~~~~~~~
            9:  ret void 
check:27'0     ~~~~~~~~~~
           10: } 
check:27'0     ~~
           11:  
check:27'0     ~
           12: attributes #0 = { noredzone nounwind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" "target-features"="-mmx,-sse,+soft-float" } 
check:27'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
