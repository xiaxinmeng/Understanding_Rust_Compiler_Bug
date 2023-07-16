plain
test [ui] ui/conditional-compilation/cfg-arg-invalid-5.rs ... ok
test [ui] ui/conditional-compilation/cfg-attr-crate-2.rs ... ok
test [ui] ui/command/command-argv0-debug.rs ... ok
test [ui] ui/conditional-compilation/cfg-attr-multi-invalid-1.rs ... ok
test [ui] ui/command/command-create-pidfd.rs ... ok
test [ui] ui/conditional-compilation/cfg-attr-invalid-predicate.rs ... ok
test [ui] ui/conditional-compilation/cfg-attr-multi-invalid-2.rs ... ok
test [ui] ui/command/command-uid-gid.rs ... ok
test [ui] ui/conditional-compilation/cfg-attr-cfg-2.rs ... ok
---
failures:

---- [ui] ui/abi/stack-probes-lto.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/stack-probes-lto.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/abi/stack-probes-lto/a" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/abi/stack-probes-lto/auxiliary"
------------------------------------------

------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=i686-unknown-linux-gnu target=i686-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=i686-unknown-linux-gnu target=i686-unknown-linux-gnu
stderr:
------------------------------------------
rustc: /checkout/src/llvm-project/llvm/lib/CodeGen/RegAllocFast.cpp:883: void {anonymous}::RegAllocFast::defineVirtReg(llvm::MachineInstr&, unsigned int, llvm::Register, bool): Assertion `PhysReg != 0 && "Register not assigned"' failed.
error: ran out of registers during register allocation

------------------------------------------



---- [ui] ui/panic-runtime/lto-unwind.rs stdout ----

error: test compilation failed although it shouldn't!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/lto-unwind.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/panic-runtime/lto-unwind/a" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-C" "panic=unwind" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/panic-runtime/lto-unwind/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
rustc: /checkout/src/llvm-project/llvm/lib/CodeGen/RegAllocFast.cpp:883: void {anonymous}::RegAllocFast::defineVirtReg(llvm::MachineInstr&, unsigned int, llvm::Register, bool): Assertion `PhysReg != 0 && "Register not assigned"' failed.
------------------------------------------



---
test result: FAILED. 11584 passed; 2 failed; 132 ignored; 0 measured; 0 filtered out; finished in 121.12s



command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/i686-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "i686-unknown-linux-gnu" "--host" "i686-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/i686-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0-rust-1.53.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:23:01
