plain
test [codegen] codegen/sanitizer-recover.rs#MSAN-RECOVER-LTO ... ignored
test [codegen] codegen/noreturn-uninhabited.rs ... ok
test [codegen] codegen/pic-relocation-model.rs ... ok
test [codegen] codegen/refs.rs ... ok
test [codegen] codegen/sanitizer_cfi_add_canonical_jump_tables_flag.rs ... FAILED
test [codegen] codegen/sanitizer_cfi_emit_type_checks.rs ... FAILED
test [codegen] codegen/nounwind.rs ... ok
test [codegen] codegen/sanitizer_cfi_emit_type_metadata.rs ... FAILED
test [codegen] codegen/personality_lifetimes.rs ... ok
test [codegen] codegen/scalar-pair-bool.rs ... ok
---
Some tests failed in compiletest suite=codegen mode=codegen host=i686-unknown-linux-gnu target=i686-unknown-linux-gnu

failures:

---- [codegen] codegen/sanitizer_cfi_add_canonical_jump_tables_flag.rs stdout ----
error: compilation failed!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/sanitizer_cfi_add_canonical_jump_tables_flag.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/sanitizer_cfi_add_canonical_jump_tables_flag" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clto" "-Zsanitizer=cfi" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/sanitizer_cfi_add_canonical_jump_tables_flag/auxiliary" "--emit=llvm-ir"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cfi sanitizer is not supported for this target
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [codegen] codegen/sanitizer_cfi_emit_type_checks.rs stdout ----
error: compilation failed!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/sanitizer_cfi_emit_type_checks.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/sanitizer_cfi_emit_type_checks" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clto" "-Cno-prepopulate-passes" "-Zsanitizer=cfi" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/sanitizer_cfi_emit_type_checks/auxiliary" "--emit=llvm-ir"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cfi sanitizer is not supported for this target
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [codegen] codegen/sanitizer_cfi_emit_type_metadata.rs stdout ----
error: compilation failed!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/sanitizer_cfi_emit_type_metadata.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/sanitizer_cfi_emit_type_metadata" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clto" "-Cno-prepopulate-passes" "-Zsanitizer=cfi" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen/sanitizer_cfi_emit_type_metadata/auxiliary" "--emit=llvm-ir"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cfi sanitizer is not supported for this target
error: aborting due to previous error


------------------------------------------
------------------------------------------



failures:
    [codegen] codegen/sanitizer_cfi_add_canonical_jump_tables_flag.rs
    [codegen] codegen/sanitizer_cfi_emit_type_checks.rs
    [codegen] codegen/sanitizer_cfi_emit_type_metadata.rs
test result: FAILED. 229 passed; 3 failed; 67 ignored; 0 measured; 0 filtered out; finished in 4.51s




command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/i686-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "codegen" "--mode" "codegen" "--target" "i686-unknown-linux-gnu" "--host" "i686-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.58.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:19:17
