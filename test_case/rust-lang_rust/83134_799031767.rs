plain
test [run-make] run-make/llvm-outputs ... ok
test [run-make] run-make/rustc-macro-dep-files ... ok
test [run-make] run-make/env-dep-info ... ok
test [run-make] run-make/thumb-none-cortex-m ... ok
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=thumbv6m-none-eabi

failures:

---- [run-make] run-make/incr-prev-body-beyond-eof stdout ----
---- [run-make] run-make/incr-prev-body-beyond-eof stdout ----

error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
mkdir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/incr-prev-body-beyond-eof/incr-prev-body-beyond-eof/src
mkdir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/incr-prev-body-beyond-eof/incr-prev-body-beyond-eof/incr
cp a.rs /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/incr-prev-body-beyond-eof/incr-prev-body-beyond-eof/src/main.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/incr-prev-body-beyond-eof/incr-prev-body-beyond-eof:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/incr-prev-body-beyond-eof/incr-prev-body-beyond-eof -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/incr-prev-body-beyond-eof/incr-prev-body-beyond-eof  -Clinker='arm-none-eabi-gcc' -C incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/incr-prev-body-beyond-eof/incr-prev-body-beyond-eof/incr /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/incr-prev-body-beyond-eof/incr-prev-body-beyond-eof/src/main.rs --target thumbv6m-none-eabi
------------------------------------------
stderr:
------------------------------------------
error[E0463]: can't find crate for `std`
error[E0463]: can't find crate for `std`
  |
  = note: the `thumbv6m-none-eabi` target may not be installed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
For more information about this error, try `rustc --explain E0463`.
make: *** [Makefile:14: all] Error 1
------------------------------------------



---- [run-make] run-make/issue-36710 stdout ----
error: make failed
status: exit code: 2
command: "make"
stdout:
stdout:
------------------------------------------
arm-none-eabi-g++ -ffunction-sections -fdata-sections -mthumb -march=armv6s-m -c -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-36710/issue-36710/libfoo.o foo.cpp
arm-none-eabi-ar crus /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-36710/issue-36710/libfoo.a /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-36710/issue-36710/libfoo.o
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-36710/issue-36710:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-36710/issue-36710 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/issue-36710/issue-36710  -Clinker='arm-none-eabi-gcc' foo.rs -lfoo -lstdc++ --target thumbv6m-none-eabi
------------------------------------------
stderr:
------------------------------------------
error[E0463]: can't find crate for `std`
error[E0463]: can't find crate for `std`
  |
  = note: the `thumbv6m-none-eabi` target may not be installed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
For more information about this error, try `rustc --explain E0463`.
make: *** [Makefile:10: foo] Error 1
------------------------------------------



---
test result: FAILED. 8 passed; 2 failed; 12 ignored; 0 measured; 0 filtered out; finished in 19.56s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-thumbv6m-none-eabi" "--suite" "run-make" "--mode" "run-make" "--target" "thumbv6m-none-eabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-none-eabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/thumbv6m-none-eabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--llvm-version" "12.0.0-rust-1.52.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin" "--cc" "arm-none-eabi-gcc" "--cxx" "arm-none-eabi-g++" "--cflags" "-ffunction-sections -fdata-sections -mthumb -march=armv6s-m" "--ar" "arm-none-eabi-ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --host= --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make
Build completed unsuccessfully in 0:15:20
