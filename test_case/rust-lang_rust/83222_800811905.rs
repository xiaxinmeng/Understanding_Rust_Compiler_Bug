plain
test [run-make] run-make/incr-prev-body-beyond-eof ... ok

failures:

---- [run-make] run-make/issue-36710 stdout ----
error: make failed
status: exit code: 2
command: "make"
stdout:
stdout:
------------------------------------------
musl-g++ -ffunction-sections -fdata-sections -fPIC -m32 -march=i686 -Wl,-melf_i386 -static -Wa,-mrelax-relocations=no -c -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-36710/issue-36710/libfoo.o foo.cpp
Makefile:17: recipe for target '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-36710/issue-36710/libfoo.o' failed
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
/bin/dash: 1: musl-g++: not found
make: *** [/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-36710/issue-36710/libfoo.o] Error 127
------------------------------------------




failures:
    [run-make] run-make/issue-36710

test result: FAILED. 8 passed; 1 failed; 13 ignored; 0 measured; 0 filtered out; finished in 0.60s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-i686-unknown-linux-musl" "--suite" "run-make" "--mode" "run-make" "--target" "i686-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/musl-i686/bin/musl-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0-rust-1.52.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin" "--cc" "/musl-i686/bin/musl-gcc" "--cxx" "musl-g++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m32 -march=i686 -Wl,-melf_i386 -static -Wa,-mrelax-relocations=no" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --host= --target i586-unknown-linux-gnu,i686-unknown-linux-musl
Build completed unsuccessfully in 0:30:45
