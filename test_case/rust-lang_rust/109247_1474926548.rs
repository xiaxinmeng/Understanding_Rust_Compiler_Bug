plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
failures:

---- [codegen] tests/codegen/remap_path_prefix/xcrate-generic.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/xcrate-generic/xcrate-generic.ll" "/checkout/tests/codegen/remap_path_prefix/xcrate-generic.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/remap_path_prefix/xcrate-generic.rs:14:11: error: CHECK: expected string not found in input
// CHECK: !DIFile(filename: "/the/aux-src/xcrate-generic.rs", directory: ""
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/xcrate-generic/xcrate-generic.ll:1:1: note: scanning from here
; ModuleID = 'xcrate_generic.12e752f8-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/xcrate-generic/xcrate-generic.ll:25:35: note: possible intended match here
!7 = !DIFile(filename: "/checkout/tests/codegen/remap_path_prefix/xcrate-generic.rs", directory: "", checksumkind: CSK_MD5, checksum: "4ece019f4688790b88988ad5a4ed3ca0")

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/xcrate-generic/xcrate-generic.ll
Check file: /checkout/tests/codegen/remap_path_prefix/xcrate-generic.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'xcrate_generic.12e752f8-cgu.0' 
check:14'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "xcrate_generic.12e752f8-cgu.0" 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "x86_64-unknown-linux-gnu" 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
check:14'0     ~
            6: ; xcrate_generic::foo 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~
            7: ; Function Attrs: nonlazybind uwtable 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            8: define void @_ZN14xcrate_generic3foo17h071f71e594464f94E() unnamed_addr #0 !dbg !6 { 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            9: start: 
check:14'0     ~~~~~~~
           10:  ret void, !dbg !12 
check:14'0     ~~~~~~~~~~~~~~~~~~~~
           11: } 
check:14'0     ~~
           12:  
check:14'0     ~
           13: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14:  
check:14'0     ~
           15: !llvm.module.flags = !{!0, !1, !2, !3} 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           16: !llvm.dbg.cu = !{!4} 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~
           17:  
check:14'0     ~
           18: !0 = !{i32 7, !"PIC Level", i32 2} 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           19: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           20: !2 = !{i32 2, !"Dwarf Version", i32 4} 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           21: !3 = !{i32 2, !"Debug Info Version", i32 3} 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           22: !4 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !5, producer: "clang LLVM (rustc version 1.70.0-nightly (d8698339d 2023-03-18))", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false) 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           23: !5 = !DIFile(filename: "/checkout/tests/codegen/remap_path_prefix/xcrate-generic.rs/@/xcrate_generic.12e752f8-cgu.0", directory: "/checkout/obj") 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           24: !6 = distinct !DISubprogram(name: "foo", linkageName: "_ZN14xcrate_generic3foo17h071f71e594464f94E", scope: !8, file: !7, line: 9, type: !9, scopeLine: 9, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition | DISPFlagOptimized, unit: !4, templateParams: !11, retainedNodes: !11) 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           25: !7 = !DIFile(filename: "/checkout/tests/codegen/remap_path_prefix/xcrate-generic.rs", directory: "", checksumkind: CSK_MD5, checksum: "4ece019f4688790b88988ad5a4ed3ca0") 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:14'1                                       ?                                                                                                                                        possible intended match
           26: !8 = !DINamespace(name: "xcrate_generic", scope: null) 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           27: !9 = !DISubroutineType(types: !10) 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           28: !10 = !{null} 
check:14'0     ~~~~~~~~~~~~~~
           29: !11 = !{} 
check:14'0     ~~~~~~~~~~
           30: !12 = !DILocation(line: 11, column: 2, scope: !6) 
check:14'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------



