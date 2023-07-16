plain
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 308 tests
ii....i........i....i.i.......iii........ii......i...............i.ii................i.............. 100/308
i...............i...iii........i..i..F...........i.......i..F.....F......i...i...i.....ii..iiii..... 200/308
........iii.............F..........i.ii.i......i.......i...........................iiiiii........... 300/308
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [codegen] codegen/issue-37945.rs stdout ----


error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll" "/checkout/src/test/codegen/issue-37945.rs" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/issue-37945.rs:20:16: error: CHECK-NEXT: expected string not found in input
// CHECK-NEXT: [[B:%.*]] = icmp eq i32* %xs.0, %xs.1
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll:11:2: note: scanning from here
 %_12.i = icmp eq i32* %xs.1, %xs.0
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll:11:4: note: possible intended match here
 %_12.i = icmp eq i32* %xs.1, %xs.0
   ^
/checkout/src/test/codegen/issue-37945.rs:31:16: error: CHECK-NEXT: expected string not found in input
// CHECK-NEXT: [[D:%.*]] = icmp eq i32* %xs.0, %xs.1
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll:20:2: note: scanning from here
 %_12.i = icmp eq i32* %xs.1, %xs.0
 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll:20:4: note: possible intended match here
 %_12.i = icmp eq i32* %xs.1, %xs.0

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-37945/issue-37945.ll
Check file: /checkout/src/test/codegen/issue-37945.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
           .
           .
           .
           .
           6: ; Function Attrs: nofree nounwind nonlazybind uwtable willreturn
           7: define zeroext i1 @is_empty_1(i32* nonnull %xs.0, i32* %xs.1) unnamed_addr #0 {
           8: start:
           9:  %0 = icmp ne i32* %xs.1, null
          10:  tail call void @llvm.assume(i1 %0)
          11:  %_12.i = icmp eq i32* %xs.1, %xs.0
next:20'0      X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
next:20'1        ?                                possible intended match
          12:  ret i1 %_12.i
next:20'0     ~~~~~~~~~~~~~~
          13: }
next:20'0     ~
          14: 
next:20'0     ~
          15: ; Function Attrs: nofree nounwind nonlazybind uwtable willreturn
next:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          16: define zeroext i1 @is_empty_2(i32* nonnull %xs.0, i32* %xs.1) unnamed_addr #0 {
next:20'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          17: start:
          18:  %0 = icmp ne i32* %xs.1, null
          19:  tail call void @llvm.assume(i1 %0)
          20:  %_12.i = icmp eq i32* %xs.1, %xs.0
next:31'0      X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
next:31'1        ?                                possible intended match
          21:  ret i1 %_12.i
next:31'0     ~~~~~~~~~~~~~~
          22: }
next:31'0     ~
          23: 
next:31'0     ~
          24: ; Function Attrs: nofree nosync nounwind willreturn
next:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          25: declare void @llvm.assume(i1 noundef) #1
next:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
>>>>>>


------------------------------------------


---- [codegen] codegen/issue-75659.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75659/issue-75659.ll" "/checkout/src/test/codegen/issue-75659.rs" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/issue-75659.rs:28:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: memchr
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75659/issue-75659.ll:37:34: note: found here
 br i1 %1, label %_ZN4core5slice6memchr6memchr17h404692f2b348a885E.exit, label %bb12.i.i
                                 ^~~~~~
/checkout/src/test/codegen/issue-75659.rs:36:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: memchr
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75659/issue-75659.ll:61:34: note: found here
 br i1 %1, label %_ZN4core5slice6memchr6memchr17h404692f2b348a885E.exit, label %bb12.i.i
                                 ^~~~~~
/checkout/src/test/codegen/issue-75659.rs:44:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: memchr
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75659/issue-75659.ll:91:34: note: found here
 br i1 %1, label %_ZN4core5slice6memchr6memchr17h404692f2b348a885E.exit, label %bb12.i.i
                                 ^~~~~~
/checkout/src/test/codegen/issue-75659.rs:53:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: slice_contains
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75659/issue-75659.ll:145:81: note: found here
 br i1 %1, label %"_ZN54_$LT$i8$u20$as$u20$core..slice..cmp..SliceContains$GT$14slice_contains17h603dc7e9f6027013E.exit", label %bb12.i.i.i

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75659/issue-75659.ll
Check file: /checkout/src/test/codegen/issue-75659.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
        .
        .
        .
        .
       32: define zeroext i1 @foo3(i8 %0, [3 x i8]* noalias readonly align 1 dereferenceable(3) %data) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
       33: start:
       34:  %ptr.i.i = getelementptr [3 x i8], [3 x i8]* %data, i64 0, i64 0
       35:  %_3.i.i.i = load i8, i8* %ptr.i.i, align 1, !alias.scope !22, !noalias !27
       36:  %1 = icmp eq i8 %_3.i.i.i, %0
       37:  br i1 %1, label %_ZN4core5slice6memchr6memchr17h404692f2b348a885E.exit, label %bb12.i.i
not:28                                      !~~~~~                                                  error: no match expected
       38: 
       39: bb12.i.i: ; preds = %start
       40:  %2 = getelementptr inbounds [3 x i8], [3 x i8]* %data, i64 0, i64 1
       41:  %_3.i.i.i.1 = load i8, i8* %2, align 1, !alias.scope !22, !noalias !27
       42:  %3 = icmp eq i8 %_3.i.i.i.1, %0
        .
        .
        .
       56: define zeroext i1 @foo4(i8 %0, [4 x i8]* noalias readonly align 1 dereferenceable(4) %data) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
       57: start:
       58:  %ptr.i.i = getelementptr [4 x i8], [4 x i8]* %data, i64 0, i64 0
       59:  %_3.i.i.i = load i8, i8* %ptr.i.i, align 1, !alias.scope !32, !noalias !37
       60:  %1 = icmp eq i8 %_3.i.i.i, %0
       61:  br i1 %1, label %_ZN4core5slice6memchr6memchr17h404692f2b348a885E.exit, label %bb12.i.i
not:36                                      !~~~~~                                                  error: no match expected
       62: 
       63: bb12.i.i: ; preds = %start
       64:  %2 = getelementptr inbounds [4 x i8], [4 x i8]* %data, i64 0, i64 1
       65:  %_3.i.i.i.1 = load i8, i8* %2, align 1, !alias.scope !32, !noalias !37
       66:  %3 = icmp eq i8 %_3.i.i.i.1, %0
        .
        .
        .
       86: define zeroext i1 @foo8(i8 %0, [8 x i8]* noalias readonly align 1 dereferenceable(8) %data) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
       87: start:
       88:  %ptr.i.i = getelementptr [8 x i8], [8 x i8]* %data, i64 0, i64 0
       89:  %_3.i.i.i = load i8, i8* %ptr.i.i, align 1, !alias.scope !42, !noalias !47
       90:  %1 = icmp eq i8 %_3.i.i.i, %0
       91:  br i1 %1, label %_ZN4core5slice6memchr6memchr17h404692f2b348a885E.exit, label %bb12.i.i
not:44                                      !~~~~~                                                  error: no match expected
       92: 
       93: bb12.i.i: ; preds = %start
       94:  %2 = getelementptr inbounds [8 x i8], [8 x i8]* %data, i64 0, i64 1
       95:  %_3.i.i.i.1 = load i8, i8* %2, align 1, !alias.scope !42, !noalias !47
       96:  %3 = icmp eq i8 %_3.i.i.i.1, %0
        .
        .
        .
      140: define zeroext i1 @foo8_i8(i8 %0, [8 x i8]* noalias nocapture readonly align 1 dereferenceable(8) %data) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
      141: start:
      142:  %ptr.i.i.i = getelementptr [8 x i8], [8 x i8]* %data, i64 0, i64 0
      143:  %_3.i.i.i.i = load i8, i8* %ptr.i.i.i, align 1, !alias.scope !52, !noalias !59
      144:  %1 = icmp eq i8 %_3.i.i.i.i, %0
      145:  br i1 %1, label %"_ZN54_$LT$i8$u20$as$u20$core..slice..cmp..SliceContains$GT$14slice_contains17h603dc7e9f6027013E.exit", label %bb12.i.i.i
not:53                                                                                     !~~~~~~~~~~~~~                                              error: no match expected
      146: 
      147: bb12.i.i.i: ; preds = %start
      148:  %2 = getelementptr inbounds [8 x i8], [8 x i8]* %data, i64 0, i64 1
      149:  %_3.i.i.i.1.i = load i8, i8* %2, align 1, !alias.scope !52, !noalias !59
      150:  %3 = icmp eq i8 %_3.i.i.i.1.i, %0
        .
        .
>>>>>>


------------------------------------------


---- [codegen] codegen/mem-replace-direct-memcpy.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll" "/checkout/src/test/codegen/mem-replace-direct-memcpy.rs" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/mem-replace-direct-memcpy.rs:18:11: error: CHECK: expected string not found in input
// CHECK: ; core::ptr::read
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll:1:1: note: scanning from here
; ModuleID = 'mem_replace_direct_memcpy.8127b936-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll:9:1: note: possible intended match here
; core::mem::replace


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/mem-replace-direct-memcpy/mem-replace-direct-memcpy.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'mem_replace_direct_memcpy.8127b936-cgu.0'
check:18'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "mem_replace_direct_memcpy.8127b936-cgu.0"
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "x86_64-unknown-linux-gnu"
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5: 
check:18'0     ~
            6: %"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [6 x i64] }
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            7: %"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            8: 
check:18'0     ~
            9: ; core::mem::replace
check:18'0     ~~~~~~~~~~~~~~~~~~~~
check:18'1     ?                    possible intended match
           10: ; Function Attrs: inlinehint nonlazybind uwtable
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           11: define internal i8 @_ZN4core3mem7replace17hb23ffc0688cdc8d2E(i8* noalias align 1 dereferenceable(1) %dest, i8 %src) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           12: start:
check:18'0     ~~~~~~
           13:  %0 = alloca { i8*, i32 }, align 8
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           14:  %tmp = alloca i8, align 1
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>


------------------------------------------


---- [codegen] codegen/remap_path_prefix/main.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll" "/checkout/src/test/codegen/remap_path_prefix/main.rs" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/remap_path_prefix/main.rs:28:11: error: CHECK: expected string not found in input
// CHECK: !DIFile(filename: "/the/aux-src/remap_path_prefix_aux.rs", directory: ""
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll:281:85: note: scanning from here
!11 = !DIFile(filename: "/the/src/remap_path_prefix/main.rs", directory: "/the/cwd/", checksumkind: CSK_MD5, checksum: "d5f4f2ccf77c6ec2c9275674029d64c5")
                                                                                    ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll:295:5: note: possible intended match here
!25 = !DIFile(filename: "/the/src/remap_path_prefix/main.rs", directory: "/the/cwd/")

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/remap_path_prefix/main/main.ll
Check file: /checkout/src/test/codegen/remap_path_prefix/main.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
          276: !6 = !{!7}
          277: !7 = !DISubrange(count: 6, lowerBound: 0)
          278: !8 = !DIGlobalVariableExpression(var: !9, expr: !DIExpression())
          279: !9 = distinct !DIGlobalVariable(name: "FILE_PATH", linkageName: "_ZN4main9FILE_PATH17h6392252c3b6cc15aE", scope: !10, file: !11, line: 16, type: !12, isLocal: true, isDefinition: true, align: 8)
          280: !10 = !DINamespace(name: "main", scope: null)
          281: !11 = !DIFile(filename: "/the/src/remap_path_prefix/main.rs", directory: "/the/cwd/", checksumkind: CSK_MD5, checksum: "d5f4f2ccf77c6ec2c9275674029d64c5")
check:28'0                                                                                         X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
          282: !12 = !DICompositeType(tag: DW_TAG_structure_type, name: "&str", file: !2, size: 128, align: 64, elements: !13, templateParams: !19, identifier: "7ef2a91eecc7bcf4b4aaea2dbce79437")
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          283: !13 = !{!14, !17}
check:28'0     ~~~~~~~~~~~~~~~~~
          284: !14 = !DIDerivedType(tag: DW_TAG_member, name: "data_ptr", scope: !12, file: !2, baseType: !15, size: 64, align: 64)
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          285: !15 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "*const u8", baseType: !16, size: 64, align: 64, dwarfAddressSpace: 0)
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          286: !16 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
            .
          290: !20 = !{i32 7, !"PIC Level", i32 2}
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          291: !21 = !{i32 7, !"PIE Level", i32 2}
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          292: !22 = !{i32 2, !"RtLibUseGOT", i32 1}
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          293: !23 = !{i32 2, !"Debug Info Version", i32 3}
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          294: !24 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !25, producer: "clang LLVM (rustc version 1.59.0-nightly (16a5fec04 2021-12-13))", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, enums: !19, globals: !26)
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          295: !25 = !DIFile(filename: "/the/src/remap_path_prefix/main.rs", directory: "/the/cwd/")
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:28'1         ?                                                                                 possible intended match
          296: !26 = !{!0, !8}
check:28'0     ~~~~~~~~~~~~~~~
          297: !27 = distinct !DISubprogram(name: "__rust_begin_short_backtrace<fn(), ()>", linkageName: "_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hdbe3811668810f60E", scope: !29, file: !28, line: 119, type: !32, scopeLine: 119, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition | DISPFlagOptimized, unit: !24, templateParams: !41, retainedNodes: !37)
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          298: !28 = !DIFile(filename: "/checkout/library/std/src/sys_common/backtrace.rs", directory: "", checksumkind: CSK_MD5, checksum: "6a433c65ecb3674065f414f3ec71fc02")
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          299: !29 = !DINamespace(name: "backtrace", scope: !30)
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          300: !30 = !DINamespace(name: "sys_common", scope: !31)
check:28'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>

---
test result: FAILED. 255 passed; 4 failed; 49 ignored; 0 measured; 0 filtered out; finished in 3.45s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:11:07
