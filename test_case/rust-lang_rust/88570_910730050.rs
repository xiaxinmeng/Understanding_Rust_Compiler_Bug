plain
test [run-make] run-make/wasm-symbols-not-imported ... ignored
test [run-make] run-make/x86_64-fortanix-unknown-sgx-lvi ... ignored
test [run-make] run-make/wasm-stringify-ints-small ... ok
test [run-make] run-make/fmt-write-bloat ... ok
test [run-make] run-make/native-link-modifier-whole-archive ... FAILED
test [run-make] run-make/const_fn_mir ... ok
test [run-make] run-make/incremental-session-fail ... ok
test [run-make] run-make/llvm-outputs ... ok
test [run-make] run-make/emit-path-unhashed ... ok
test [run-make] run-make/emit-path-unhashed ... ok
test [run-make] run-make/emit-named-files ... ok
test [run-make] run-make/track-path-dep-info ... ok
test [run-make] run-make/rustc-macro-dep-files ... ok
test [run-make] run-make/env-dep-info ... ok
test [run-make] run-make/emit-shared-files ... ok
test [run-make] run-make/thumb-none-cortex-m ... ok
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=thumbv6m-none-eabi

failures:


---- [run-make] run-make/native-link-modifier-whole-archive stdout ----
error: make failed
status: exit status: 2
command: "make"
stdout:
stdout:
------------------------------------------
arm-none-eabi-g++ -ffunction-sections -fdata-sections -mthumb -march=armv6s-m -c -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/native-link-modifier-whole-archive/native-link-modifier-whole-archive/libc_static_lib_with_constructor.o c_static_lib_with_constructor.cpp
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
c_static_lib_with_constructor.cpp:1:10: fatal error: cstdio: No such file or directory
    1 | #include <cstdio>
compilation terminated.
compilation terminated.
make: *** [Makefile:37: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/native-link-modifier-whole-archive/native-link-modifier-whole-archive/libc_static_lib_with_constructor.o] Error 1
------------------------------------------




failures:
    [run-make] run-make/native-link-modifier-whole-archive
test result: FAILED. 14 passed; 1 failed; 20 ignored; 0 measured; 0 filtered out; finished in 20.93s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-thumbv6m-none-eabi" "--suite" "run-make" "--mode" "run-make" "--target" "thumbv6m-none-eabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-none-eabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/thumbv6m-none-eabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--llvm-version" "13.0.0-rust-1.56.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin" "--cc" "arm-none-eabi-gcc" "--cxx" "arm-none-eabi-g++" "--cflags" "-ffunction-sections -fdata-sections -mthumb -march=armv6s-m" "--ar" "arm-none-eabi-ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:15:53
