plain
.................................................................................................... 9400/11760
.................................................................................................... 9500/11760
...........................................................................................i......i. 9600/11760
.................................................................................................... 9700/11760
.....................................iiiiiii..iiiiii.i.............................................. 9800/11760
.................................................................................................... 10000/11760
.................................................................................................... 10100/11760
.................................................................................................... 10200/11760
.................................................................................................... 10300/11760
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 123 tests
iiiiiiiiiiiiiiiiiiiiiiiiii.......................................................................... 100/123
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Fii..iii...............


---- [assembly] assembly/stack-protector/stack-protector-target-support.rs#r75 stdout ----

error in revision `r75`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/stack-protector/stack-protector-target-support.rs" "-Zthreads=1" "--cfg" "r75" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-target-support.r75/stack-protector-target-support.s" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "x86_64-fortanix-unknown-sgx" "-C" "stack-protector=all" "-C" "opt-level=2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-target-support.r75/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
rustc -Cllvm-args="..." with: Unknown command line argument '--x86-experimental-lvi-inline-asm-hardening'.  Try: 'rustc -Cllvm-args="..." with --help'
rustc -Cllvm-args="..." with: Did you mean '--x86-experimental-pref-loop-alignment'?
------------------------------------------




failures:
    [assembly] assembly/stack-protector/stack-protector-target-support.rs#r75

test result: FAILED. 91 passed; 1 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.33s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/assembly" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "assembly" "--mode" "assembly" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:29
