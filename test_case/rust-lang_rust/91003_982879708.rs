plain
test [assembly] assembly/x86_64-sse_crc.rs ... ok

failures:

---- [assembly] assembly/sparc-struct-abi.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/sparc-struct-abi/sparc-struct-abi.s" "/checkout/src/test/assembly/sparc-struct-abi.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
------------------------------------------
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
/checkout/src/test/assembly/sparc-struct-abi.rs:31:12: error: CHECK: expected string not found in input
 // CHECK: st %f1, [[PLACE_B:.*]]
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/sparc-struct-abi/sparc-struct-abi.s:17:20: note: scanning from here
 st %f2, [%fp+2039]
                   ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/sparc-struct-abi/sparc-struct-abi.s:18:2: note: possible intended match here
 st %f0, [%fp+2031]

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/sparc-struct-abi/sparc-struct-abi.s
Check file: /checkout/src/test/assembly/sparc-struct-abi.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
           12:  .cfi_register %o7, %i7 
           13:  add %fp, 2031, %i0 
           14:  or %i0, 4, %i0 
           15:  st %f1, [%i0] 
           16:  st %f3, [%fp+2043] 
           17:  st %f2, [%fp+2039] 
check:31'0                        X error: no match found
           18:  st %f0, [%fp+2031] 
check:31'0     ~~~~~~~~~~~~~~~~~~~~
check:31'1      ?                   possible intended match
           19:  ldx [%fp+2039], %i0 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~
           20:  stx %i0, [%fp+2023] 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~
           21:  ldx [%fp+2031], %i0 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~
           22:  call clobber 
check:31'0     ~~~~~~~~~~~~~~
           23:  stx %i0, [%fp+2015] 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>

---
test result: FAILED. 119 passed; 1 failed; 14 ignored; 0 measured; 0 filtered out; finished in 0.52s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/assembly" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "assembly" "--mode" "assembly" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.59.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:18:20
