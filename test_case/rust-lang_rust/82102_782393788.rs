plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMS9hg0wslbS

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
failures:

---- [codegen] codegen/debug-compile-unit-path.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--input-file" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/debug-compile-unit-path/debug-compile-unit-path.ll" "/Users/runner/work/rust/rust/src/test/codegen/debug-compile-unit-path.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/Users/runner/work/rust/rust/src/test/codegen/debug-compile-unit-path.rs:8:15: error: CHECK-DAG: expected string not found in input
// CHECK-DAG: [[FILE:![0-9]*]] = !DIFile(filename: "/base/debug-compile-unit-path.rs", directory: "/cwd/")
              ^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/debug-compile-unit-path/debug-compile-unit-path.ll:1:1: note: scanning from here
; ModuleID = 'debug_compile_unit_path.3a1fbbbh-cgu.0'
^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/debug-compile-unit-path/debug-compile-unit-path.ll:13:1: note: possible intended match here
!4 = !DIFile(filename: "/base/debug-compile-unit-path.rs/@/debug_compile_unit_path.3a1fbbbh-cgu.0", directory: "/cwd/")

Input file: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/debug-compile-unit-path/debug-compile-unit-path.ll
Check file: /Users/runner/work/rust/rust/src/test/codegen/debug-compile-unit-path.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
         1: ; ModuleID = 'debug_compile_unit_path.3a1fbbbh-cgu.0'
dag:8'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
         2: source_filename = "debug_compile_unit_path.3a1fbbbh-cgu.0"
dag:8'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         3: target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
dag:8'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         4: target triple = "x86_64-apple-macosx10.8.0"
dag:8'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         5: 
dag:8'0     ~
         6: !llvm.module.flags = !{!0, !1, !2}
dag:8'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         7: !llvm.dbg.cu = !{!3}
dag:8'0     ~~~~~~~~~~~~~~~~~~~~
         8: 
dag:8'0     ~
         9: !0 = !{i32 7, !"PIC Level", i32 2}
dag:8'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        10: !1 = !{i32 2, !"Dwarf Version", i32 2}
dag:8'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        11: !2 = !{i32 2, !"Debug Info Version", i32 3}
dag:8'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        12: !3 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !4, producer: "clang LLVM (rustc version 1.52.0-nightly (79516cd72 2021-02-19))", isOptimized: true, runtimeVersion: 0, emissionKind: FullDebug, enums: !5)
dag:8'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        13: !4 = !DIFile(filename: "/base/debug-compile-unit-path.rs/@/debug_compile_unit_path.3a1fbbbh-cgu.0", directory: "/cwd/")
dag:8'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
dag:8'1     ?                                                                                                                       possible intended match
        14: !5 = !{}
dag:8'0     ~~~~~~~~
>>>>>>
------------------------------------------




failures:
    [codegen] codegen/debug-compile-unit-path.rs

test result: FAILED. 206 passed; 1 failed; 36 ignored; 0 measured; 0 filtered out; finished in 12.66s



command did not execute successfully: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/work/rust/rust/src/test/codegen" "--build-base" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen" "--stage-id" "stage2-x86_64-apple-darwin" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python@3.9/bin/python3.9" "--lldb-python" "/usr/bin/python3" "--lldb-version" "lldb-1200.0.44.2\nApple Swift version 5.3.2 (swiftlang-1200.0.45 clang-1200.0.32.28)\n" "--lldb-python-dir" "/Applications/Xcode_12.3.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python3" "--llvm-version" "11.0.1-rust-1.52.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /Users/runner/work/rust/rust/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 1:34:50
