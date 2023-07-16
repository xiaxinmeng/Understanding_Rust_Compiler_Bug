plain
test [run-make] run-make/emit-shared-files ... ok

failures:

---- [run-make] run-make/incremental-session-fail stdout ----
error: make failed
status: exit status: 2
command: "make"
stdout:
stdout:
------------------------------------------
echo /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/incremental-session-fail/incremental-session-fail
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/incremental-session-fail/incremental-session-fail
mkdir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/incremental-session-fail/incremental-session-fail/session
# Make it so that rustc will fail to create a session directory.
chmod a-w /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/incremental-session-fail/incremental-session-fail/session
# Check exit code is 1 for an error, and not 101 for ICE.
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/incremental-session-fail/incremental-session-fail:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/incremental-session-fail/incremental-session-fail -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/incremental-session-fail/incremental-session-fail  foo.rs --crate-type=rlib -C incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/incremental-session-fail/incremental-session-fail/session > /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/incremental-session-fail/incremental-session-fail/build-output 2>&1; [ $? -eq 1 ]
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
make: *** [Makefile:12: all] Error 1
------------------------------------------




failures:
    [run-make] run-make/incremental-session-fail

test result: FAILED. 10 passed; 1 failed; 15 ignored; 0 measured; 0 filtered out; finished in 1.81s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-wasm32-unknown-emscripten" "--suite" "run-make" "--mode" "run-make" "--target" "wasm32-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/latest/bin/node" "--npm" "/emsdk-portable/node/latest/bin/npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.1-rust-1.54.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin" "--cc" "emcc" "--cxx" "em++" "--cflags" "-ffunction-sections -fdata-sections -fPIC" "--ar" "emar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 --host= --target wasm32-unknown-emscripten --exclude library/alloc
Build completed unsuccessfully in 0:21:53
