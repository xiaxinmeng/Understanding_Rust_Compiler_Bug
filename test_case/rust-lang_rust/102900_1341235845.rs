plain
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      OS Loader Version: 540.120.3~22
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMc/lnDP9R5+
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 12
hw.byteorder: 1234
---

---- [codegen] src/test/codegen/pgo-counter-bias.rs stdout ----
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-apple-darwin target=x86_64-apple-darwin

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/ci-llvm/bin/FileCheck" "--input-file" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/pgo-counter-bias/pgo-counter-bias.ll" "/Users/runner/work/rust/rust/src/test/codegen/pgo-counter-bias.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/Users/runner/work/rust/rust/src/test/codegen/pgo-counter-bias.rs:7:11: error: CHECK: expected string not found in input
// CHECK: @__llvm_profile_counter_bias = {{.*}}global
          ^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/pgo-counter-bias/pgo-counter-bias.ll:1:1: note: scanning from here
; ModuleID = 'pgo_counter_bias.bb92779b-cgu.0'
^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/pgo-counter-bias/pgo-counter-bias.ll:2:6: note: possible intended match here
source_filename = "pgo_counter_bias.bb92779b-cgu.0"

Input file: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/pgo-counter-bias/pgo-counter-bias.ll
Check file: /Users/runner/work/rust/rust/src/test/codegen/pgo-counter-bias.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'pgo_counter_bias.bb92779b-cgu.0' 
check:7'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           2: source_filename = "pgo_counter_bias.bb92779b-cgu.0" 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:7'1          ?                                               possible intended match
           3: target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           4: target triple = "x86_64-apple-macosx10.8.0" 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           5:  
check:7'0     ~
           6: %0 = type { [2 x i64], i64 } 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           7: %1 = type { i64, [2 x i64] } 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
>>>>>>
------------------------------------------
