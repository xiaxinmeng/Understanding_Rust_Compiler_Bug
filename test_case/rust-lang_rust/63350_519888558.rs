
failures:

---- [codegen] codegen/issue-44056-macos-tls-align.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: "/Users/ilijatovilo/Developer/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--input-file" "/Users/ilijatovilo/Developer/rust/build/x86_64-apple-darwin/test/codegen/issue-44056-macos-tls-align/issue-44056-macos-tls-align.ll" "/Users/ilijatovilo/Developer/rust/src/test/codegen/issue-44056-macos-tls-align.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/Users/ilijatovilo/Developer/rust/src/test/codegen/issue-44056-macos-tls-align.rs:9:11: error: CHECK: expected string not found in input
// CHECK: @STATIC_VAR_1 = thread_local local_unnamed_addr global <{ [32 x i8] }> zeroinitializer, section "__DATA,__thread_bss", align 4
          ^
/Users/ilijatovilo/Developer/rust/build/x86_64-apple-darwin/test/codegen/issue-44056-macos-tls-align/issue-44056-macos-tls-align.ll:1:1: note: scanning from here
; ModuleID = 'issue_44056_macos_tls_align.3a1fbbbh-cgu.0'
^
/Users/ilijatovilo/Developer/rust/build/x86_64-apple-darwin/test/codegen/issue-44056-macos-tls-align/issue-44056-macos-tls-align.ll:12:1: note: possible intended match here
1   Add missing #![feature(associated_type_bounds)]
@STATIC_VAR_1 = thread_local global <{ [32 x i8] }> zeroinitializer, section "__DATA,__thread_bss", align 4
^
