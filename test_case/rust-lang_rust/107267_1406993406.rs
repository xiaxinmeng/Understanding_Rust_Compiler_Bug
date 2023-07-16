plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:ece894d15649f0f9d7884f915dc821f00bd0418b)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
failures:

---- [codegen] tests/codegen/set-discriminant-invalid.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/set-discriminant-invalid/set-discriminant-invalid.ll" "/checkout/tests/codegen/set-discriminant-invalid.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/set-discriminant-invalid.rs:26:17: error: CHECK-NEXT: is not on the line after the previous match
 // CHECK-NEXT: load
                ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/set-discriminant-invalid/set-discriminant-invalid.ll:15:7: note: 'next' match was here
 %2 = load i8, i8* %0, align 1, !range !2, !noundef !3
      ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/set-discriminant-invalid/set-discriminant-invalid.ll:13:24: note: previous match ended here
 call void @llvm.trap()
                       ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/set-discriminant-invalid/set-discriminant-invalid.ll:14:1: note: non-matching line after previous match is here
 %1 = bitcast i8* %0 to %"Error::Api"*

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/set-discriminant-invalid/set-discriminant-invalid.ll
Check file: /checkout/tests/codegen/set-discriminant-invalid.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
         1: ; ModuleID = 'set_discriminant_invalid.a76713b9-cgu.0' 
         2: source_filename = "set_discriminant_invalid.a76713b9-cgu.0" 
         3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
         4: target triple = "x86_64-unknown-linux-gnu" 
         5:  
         6: %"Error::Api" = type { %ApiError } 
         7: %ApiError = type { [0 x i8] } 
         8:  
         9: ; Function Attrs: nonlazybind uwtable 
        10: define i8 @into_error() unnamed_addr #0 { 
        11: start: 
        12:  %0 = alloca i8, align 1 
        13:  call void @llvm.trap() 
        14:  %1 = bitcast i8* %0 to %"Error::Api"* 
        15:  %2 = load i8, i8* %0, align 1, !range !2, !noundef !3 
next:26           !~~~                                              error: match on wrong line
        16:  ret i8 %2 
        17: } 
        18:  
        19: ; Function Attrs: cold noreturn nounwind 
        20: declare void @llvm.trap() #1 
        21:  
        22: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
        23: attributes #1 = { cold noreturn nounwind } 
        24:  
        25: !llvm.module.flags = !{!0, !1} 
        26:  
        27: !0 = !{i32 7, !"PIC Level", i32 2} 
        28: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
        29: !2 = !{i8 0, i8 3} 
        30: !3 = !{} 
------------------------------------------



