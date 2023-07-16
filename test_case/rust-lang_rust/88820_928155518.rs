plain
test [codegen] codegen/riscv-abi/riscv64-lp64d-abi.rs ... ignored
test [codegen] codegen/riscv-abi/riscv64-lp64f-lp64d-abi.rs ... ignored
test [codegen] codegen/pgo-instrumentation.rs ... ok
test [codegen] codegen/personality_lifetimes.rs ... ok
test [codegen] codegen/pie-relocation-model.rs ... ok
test [codegen] codegen/refs.rs ... ok
test [codegen] codegen/packed.rs ... ok
test [codegen] codegen/sanitizer-memory-track-orgins.rs#MSAN-0 ... ok
test [codegen] codegen/sanitizer-memory-track-orgins.rs#MSAN-1 ... ok
---
test [assembly] assembly/asm/arm-modifiers.rs ... ok
test [assembly] assembly/asm/powerpc-types.rs#powerpc64 ... ok
test [assembly] assembly/asm/mips-types.rs#mips64 ... ok
test [assembly] assembly/asm/powerpc-types.rs#powerpc ... ok
test [assembly] assembly/pie-relocation-model.rs#x64 ... FAILED
test [assembly] assembly/x86_64-fortanix-unknown-sgx-lvi-generic-load.rs ... ignored
test [assembly] assembly/x86_64-fortanix-unknown-sgx-lvi-generic-ret.rs ... ignored
test [assembly] assembly/x86_64-fortanix-unknown-sgx-lvi-inline-assembly.rs ... ignored
test [assembly] assembly/x86_64-sse_crc.rs ... ignored
---
test [assembly] assembly/asm/x86-types.rs#i686 ... ok

failures:

---- [assembly] assembly/pie-relocation-model.rs#x64 stdout ----

error in revision `x64`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/pie-relocation-model.rs" "-Zthreads=1" "--cfg" "x64" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/assembly/pie-relocation-model.x64/pie-relocation-model.s" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--target" "x86_64-unknown-linux-gnu" "-Crelocation-model=pie" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/assembly/pie-relocation-model.x64/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0463]: can't find crate for `core`
  |
  = note: the `x86_64-unknown-linux-gnu` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-unknown-linux-gnu`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.


------------------------------------------


---- [assembly] assembly/pic-relocation-model.rs#x64 stdout ----

error in revision `x64`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/pic-relocation-model.rs" "-Zthreads=1" "--cfg" "x64" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/assembly/pic-relocation-model.x64/pic-relocation-model.s" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--target" "x86_64-unknown-linux-gnu" "-Crelocation-model=pic" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/assembly/pic-relocation-model.x64/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0463]: can't find crate for `core`
  |
  = note: the `x86_64-unknown-linux-gnu` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-unknown-linux-gnu`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.


------------------------------------------



failures:
    [assembly] assembly/pic-relocation-model.rs#x64
    [assembly] assembly/pie-relocation-model.rs#x64
test result: FAILED. 24 passed; 2 failed; 17 ignored; 0 measured; 0 filtered out; finished in 0.39s




command did not execute successfully: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/assembly" "--build-base" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/assembly" "--stage-id" "stage2-aarch64-unknown-linux-gnu" "--suite" "assembly" "--mode" "assembly" "--target" "aarch64-unknown-linux-gnu" "--host" "aarch64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:20:03
