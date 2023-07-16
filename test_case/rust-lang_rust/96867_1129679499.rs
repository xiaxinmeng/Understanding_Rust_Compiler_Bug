plain
failures:

---- [codegen] src/test/codegen/remap_path_prefix/issue-73167-remap-std.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/issue-73167-remap-std/issue-73167-remap-std.ll" "/checkout/src/test/codegen/remap_path_prefix/issue-73167-remap-std.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/remap_path_prefix/issue-73167-remap-std.rs:10:11: error: CHECK: expected string not found in input
// CHECK: !DIFile(filename: "/rustc/xyz/library/std/src/panic.rs", directory: ""
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/issue-73167-remap-std/issue-73167-remap-std.ll:1:1: note: scanning from here
; ModuleID = 'issue_73167_remap_std.f531c0cf-cgu.0'
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/issue-73167-remap-std/issue-73167-remap-std.ll
Check file: /checkout/src/test/codegen/remap_path_prefix/issue-73167-remap-std.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
          1: ; ModuleID = 'issue_73167_remap_std.f531c0cf-cgu.0' 
check:10     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
          2: source_filename = "issue_73167_remap_std.f531c0cf-cgu.0" 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          3: target datalayout = "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-f64:32:64-f80:32-n8:16:32-S128" 
          4: target triple = "i586-unknown-linux-gnu" 
check:10     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          5:  
check:10     ~
check:10     ~
          6: %"[closure@<core::ops::range::Range<usize> as core::slice::index::SliceIndex<[u8]>>::get_unchecked::{closure#0}]" = type { i32*, i32*, { [0 x i8]*, i32 }* } 
          .
          .
          .
>>>>>>
