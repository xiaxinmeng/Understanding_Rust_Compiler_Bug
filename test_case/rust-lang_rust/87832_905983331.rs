plain
test [debuginfo-gdb] debuginfo/simple-struct.rs ... ignored
test [debuginfo-gdb] debuginfo/simple-tuple.rs ... ignored
test [debuginfo-gdb] debuginfo/static-method-on-struct-and-enum.rs ... /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/static-method-on-struct-and-enum.gdb/a: 1 file pushed. 9.2 MB/s (54884 bytes in 0.006s)
ok
test [debuginfo-gdb] debuginfo/step-into-match.rs ... /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/step-into-match.gdb/a: 1 file pushed. 10.3 MB/s (54676 bytes in 0.005s)
test [debuginfo-gdb] debuginfo/struct-in-enum.rs ... ignored
test [debuginfo-gdb] debuginfo/struct-in-struct.rs ... /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/struct-in-struct.gdb/a: 1 file pushed. 10.1 MB/s (54544 bytes in 0.005s)
ok
test [debuginfo-gdb] debuginfo/struct-namespace.rs ... ignored
---
test [debuginfo-gdb] debuginfo/vec.rs ... ignored

failures:

---- [debuginfo-gdb] debuginfo/step-into-match.rs stdout ----
NOTE: compiletest thinks it is using GDB without native rust support

error: line not found in debugger output: [...]match x {
status: exit status: 0
command: "arm-linux-androideabi-gdb" "-quiet" "-batch" "-nx" "-command=/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/step-into-match.gdb/step-into-match.debugger.script"
------------------------------------------
------------------------------------------
0xb6f06a30 in ?? ()
Breakpoint 1 at 0xb6f18854: file /checkout/src/test/debuginfo/step-into-match.rs, line 341.
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
warning: Missing auto-load script at offset 0 in section .debug_gdb_scripts
of file /checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/step-into-match.gdb/a.
Use `info auto-load python-scripts [REGEXP]' to list them.
warning: Unable to find dynamic linker breakpoint function.
GDB will be unable to debug shared library initializers
and track explicitly loaded dynamic code.
warning: Unable to find dynamic linker breakpoint function.
GDB will be unable to debug shared library initializers
and track explicitly loaded dynamic code.
/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo/step-into-match.gdb/step-into-match.debugger.script:7: Error in sourced command file:
The "remote" target does not support "run".  Try "help target" or "continue".
------------------------------------------




failures:
    [debuginfo-gdb] debuginfo/step-into-match.rs

test result: FAILED. 82 passed; 1 failed; 50 ignored; 0 measured; 0 filtered out; finished in 120.06s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-linux-androideabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/debuginfo" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/debuginfo" "--stage-id" "stage2-arm-linux-androideabi" "--suite" "debuginfo" "--mode" "debuginfo" "--target" "arm-linux-androideabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/android/ndk/arm-14/bin/arm-linux-androideabi-clang" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/arm-linux-androideabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--llvm-version" "13.0.0-rust-1.56.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "/android/ndk/arm-14" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:30:59
