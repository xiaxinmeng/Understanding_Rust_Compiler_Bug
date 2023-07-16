
test [codegen] src/test/codegen/zip.rs ... ok

failures:

---- [codegen] src/test/codegen/static-relocation-model.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/ci-llvm/bin/FileCheck" "--input-file" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/static-relocation-model/static-relocation-model.ll" "/Users/runner/work/rust/rust/src/test/codegen/static-relocation-model.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/Users/runner/work/rust/rust/src/test/codegen/static-relocation-model.rs:16:11: error: CHECK: expected string not found in input
// CHECK: @extern_static = external dso_local local_unnamed_addr global i8
          ^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/static-relocation-model/static-relocation-model.ll:1:1: note: scanning from here
; ModuleID = 'static_relocation_model.445b1f62-cgu.0'
^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/static-relocation-model/static-relocation-model.ll:6:1: note: possible intended match here
@extern_static = external local_unnamed_addr global i8
^

Input file: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/static-relocation-model/static-relocation-model.ll
Check file: /Users/runner/work/rust/rust/src/test/codegen/static-relocation-model.rs

-dump-input=help explains the following input dump.

Input was:
<<<<<<
            1: ; ModuleID = 'static_relocation_model.445b1f62-cgu.0' 
check:16'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "static_relocation_model.445b1f62-cgu.0" 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "x86_64-apple-macosx10.8.0" 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:16'0     ~
            6: @extern_static = external local_unnamed_addr global i8 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:16'1     ?                                                       possible intended match
            7:  
check:16'0     ~
            8: ; Function Attrs: uwtable 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
            9: define i8 @access_extern() unnamed_addr #0 { 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           10: start: 
check:16'0     ~~~~~~~
           11:  %_1 = tail call i8 @extern_fn() 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
            .
>>>>>>
------------------------------------------



failures:
    [codegen] src/test/codegen/static-relocation-model.rs

test result: FAILED. 344 passed; 1 failed; 39 ignored; 0 measured; 0 filtered out; finished in 19.59s

Build completed unsuccessfully in 0:57:57
== clock drift check ==
  local time: Thu Nov 24 00:16:54 UTC 2022
  network time: Thu, 24 Nov 2022 00:16:54 GMT
== end clock drift check ==
Error: Process completed with exit code 1.
0s
