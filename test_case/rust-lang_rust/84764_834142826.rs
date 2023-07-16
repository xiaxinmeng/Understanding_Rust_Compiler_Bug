plain

Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu
failures:

---- [run-make] run-make-fulldeps/issue-64153 stdout ----
error: make failed
status: exit status: 2
command: "make"
stdout:
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/issue-64153/issue-64153:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/issue-64153/issue-64153 -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/issue-64153/issue-64153  --crate-type rlib upstream.rs -o /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/issue-64153/issue-64153/libupstream.rlib -Ccodegen-units=1
LD_LIBRARY_PATH="/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/issue-64153/issue-64153:/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-bootstrap-tools/aarch64-unknown-linux-gnu/release/deps:/checkout/obj/build/aarch64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/issue-64153/issue-64153 -L /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/issue-64153/issue-64153  --crate-type staticlib downstream.rs -Clto -Ccodegen-units=1 -o /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/issue-64153/issue-64153/libdownstream.a
# Dump all the symbols from the staticlib into `syms`
"/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin"/llvm-objdump -t /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/issue-64153/issue-64153/libdownstream.a > /checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/issue-64153/issue-64153/syms
------------------------------------------
stderr:
------------------------------------------
warning: ignoring --out-dir flag due to -o flag
warning: ignoring --out-dir flag due to -o flag

warning: 1 warning emitted

warning: ignoring --out-dir flag due to -o flag

warning: 1 warning emitted

/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/llvm-objdump: error: '/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps/issue-64153/issue-64153/libdownstream.a': The file was not recognized as a valid object file
make: *** [Makefile:20: all] Error 1
------------------------------------------




failures:
    [run-make] run-make-fulldeps/issue-64153

test result: FAILED. 212 passed; 1 failed; 8 ignored; 0 measured; 0 filtered out; finished in 69.11s



command did not execute successfully: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/lib/rustlib/aarch64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-aarch64-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "aarch64-unknown-linux-gnu" "--host" "aarch64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0-rust-1.54.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 1:34:44
