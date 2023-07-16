plain
Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 315 tests
ii....i........i..Fi..i...........iii........ii......i...............i.ii................i.......... 100/315
....i...............i...iii........i..i......F........i.......i...............i...i...i.....ii..iiii 200/315
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...............
failures:


---- [codegen] codegen/align-enum.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-enum/align-enum.ll" "/checkout/src/test/codegen/align-enum.rs" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/align-enum.rs:11:11: error: CHECK: expected string not found in input
// CHECK: %Align64 = type { i32, [15 x i32] }
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-enum/align-enum.ll:1:1: note: scanning from here
; ModuleID = 'align_enum.c4d5c6e3-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-enum/align-enum.ll:6:1: note: possible intended match here
%"Align64::A" = type { [1 x i32], i32, [14 x i32] }
^
/checkout/src/test/codegen/align-enum.rs:22:11: error: CHECK: expected string not found in input
// CHECK: %a64 = alloca %Align64, align 64
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-enum/align-enum.ll:11:20: note: scanning from here
define i32 @align64(i32 %a) unnamed_addr #0 {
                   ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-enum/align-enum.ll:13:2: note: possible intended match here
 %a64 = alloca i32, align 64

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-enum/align-enum.ll
Check file: /checkout/src/test/codegen/align-enum.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'align_enum.c4d5c6e3-cgu.0'
check:11'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "align_enum.c4d5c6e3-cgu.0"
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "x86_64-unknown-linux-gnu"
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5: 
check:11'0     ~
            6: %"Align64::A" = type { [1 x i32], i32, [14 x i32] }
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:11'1     ?                                                   possible intended match
            7: %Nested64 = type { i32, i16, i8, [61 x i8] }
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            8: %"Align64::B" = type { [1 x i32], i32, [14 x i32] }
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            9: 
check:11'0     ~
           10: ; Function Attrs: nonlazybind uwtable
check:11'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           11: define i32 @align64(i32 %a) unnamed_addr #0 {
check:11'0     ~~~~~~~~~~~~~~~~~~~
check:22'0                        X~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           12: start:
check:22'0     ~~~~~~
           13:  %a64 = alloca i32, align 64
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:22'1      ?                           possible intended match
           14:  %0 = bitcast i32* %a64 to i8*
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           15:  call void @llvm.lifetime.start.p0i8(i64 64, i8* %0)
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           16:  %1 = bitcast i32* %a64 to %"Align64::A"*
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           17:  %2 = getelementptr inbounds %"Align64::A", %"Align64::A"* %1, i32 0, i32 1
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18:  store i32 %a, i32* %2, align 4
check:22'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>


------------------------------------------


---- [codegen] codegen/issue-92464.rs stdout ----
thread '[codegen] codegen/issue-92464.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:440:17


failures:
    [codegen] codegen/align-enum.rs
    [codegen] codegen/align-enum.rs
    [codegen] codegen/issue-92464.rs

test result: FAILED. 262 passed; 2 failed; 51 ignored; 0 measured; 0 filtered out; finished in 3.88s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:27
