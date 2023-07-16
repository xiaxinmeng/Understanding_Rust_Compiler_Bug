plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMHV0gyXeIll

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
failures:

---- [codegen] codegen/pie-relocation-model.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--input-file" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/pie-relocation-model/pie-relocation-model.ll" "/Users/runner/work/rust/rust/src/test/codegen/pie-relocation-model.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/Users/runner/work/rust/rust/src/test/codegen/pie-relocation-model.rs:7:11: error: CHECK: expected string not found in input
// CHECK: define dso_local i8 @call_foreign_fn()
          ^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/pie-relocation-model/pie-relocation-model.ll:1:1: note: scanning from here
; ModuleID = 'pie_relocation_model.8d7ed54a-cgu.0'
^
/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/pie-relocation-model/pie-relocation-model.ll:9:10: note: possible intended match here
 %0 = tail call zeroext i8 @foreign_fn()


Input file: /Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen/pie-relocation-model/pie-relocation-model.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
           1: ; ModuleID = 'pie_relocation_model.8d7ed54a-cgu.0' 
check:7'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           2: source_filename = "pie_relocation_model.8d7ed54a-cgu.0" 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           3: target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           4: target triple = "x86_64-apple-macosx10.8.0" 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           5:  
check:7'0     ~
           6: ; Function Attrs: uwtable 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           7: define i8 @call_foreign_fn() unnamed_addr #0 { 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           8: start: 
check:7'0     ~~~~~~~
           9:  %0 = tail call zeroext i8 @foreign_fn() 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:7'1              ?                                possible intended match
          10:  ret i8 %0 
check:7'0     ~~~~~~~~~~~
          11: } 
check:7'0     ~~
          12:  
check:7'0     ~
          13: ; Function Attrs: uwtable 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
          14: declare zeroext i8 @foreign_fn() unnamed_addr #0 
check:7'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
>>>>>>

---
test result: FAILED. 258 passed; 1 failed; 36 ignored; 0 measured; 0 filtered out; finished in 12.38s



command did not execute successfully: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0-tools-bin/compiletest" "--compile-lib-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib" "--run-lib-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "--rustc-path" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "--src-base" "/Users/runner/work/rust/rust/src/test/codegen" "--build-base" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/codegen" "--stage-id" "stage2-x86_64-apple-darwin" "--suite" "codegen" "--mode" "codegen" "--target" "x86_64-apple-darwin" "--host" "x86_64-apple-darwin" "--llvm-filecheck" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/llvm/build/bin/FileCheck" "--nodejs" "/usr/local/bin/node" "--npm" "/usr/local/bin/npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "--docck-python" "/usr/local/opt/python@3.9/bin/python3.9" "--lldb-python" "/usr/bin/python3" "--lldb-version" "lldb-1200.0.44.2\nApple Swift version 5.3.2 (swiftlang-1200.0.45 clang-1200.0.32.28)\n" "--lldb-python-dir" "/Applications/Xcode_12.4.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python3" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 1:18:17
