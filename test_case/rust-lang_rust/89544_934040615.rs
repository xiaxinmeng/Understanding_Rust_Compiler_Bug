plain
test [run-make] run-make/rustc-macro-dep-files ... ok
test [run-make] run-make/env-dep-info ... ok
test [run-make] run-make/emit-shared-files ... ok
test [run-make] run-make/thumb-none-cortex-m ... ok
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=thumbv6m-none-eabi

failures:


---- [run-make] run-make/dep-graph stdout ----
error: make failed
status: exit status: 2
command: "make"
stdout:
stdout:
------------------------------------------
RUST_DEP_GRAPH=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/dep-graph/dep-graph/dep-graph LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/dep-graph/dep-graph:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/dep-graph/dep-graph -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/dep-graph/dep-graph  -Clinker='arm-none-eabi-gcc' \
        -Cincremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/dep-graph/dep-graph/incr \
        -Zquery-dep-graph -Zdump-dep-graph foo.rs
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error: linking with `arm-none-eabi-gcc` failed: exit status: 1
  |
  = note: "arm-none-eabi-gcc" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/dep-graph/dep-graph/foo.18u5b2jmv51dofh3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/dep-graph/dep-graph/foo.25sg5ox0q16b7auf.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/dep-graph/dep-graph/foo.3c6c78fzwfnp5rr9.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/dep-graph/dep-graph/foo.3pwxpcv7ig8jfb30.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/dep-graph/dep-graph/foo.3wccqld67w709v08.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/dep-graph/dep-graph/foo.4rlnbcoxr7ik1e3w.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/dep-graph/dep-graph/foo.4unz5qo9qikgp26p.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/dep-graph/dep-graph/foo.50rnet0thytpgybx.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/dep-graph/dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-5548ef64e6092712.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-a8f484438681c15e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-b69fd8507c8409af.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-2dff396b99681a6b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-a78bd5aa183c7115.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-846c64d5a2fbc5ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-24c09062b6dc787b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-ce93f351ca41a57b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-5b404d65540eadf8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-514cb174319eb6d4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-37c1534c42bc4f8c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-06d0f7780fda2fd9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-126d4eb258557375.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-88bb1ba4dd271224.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-b4a707e310d7480c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-0b5a376b429677a6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-2a6a2797f7a73818.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-0e3656b1fda5fd7b.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-eecefd843a0dbc02.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/dep-graph/dep-graph/foo" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-nodefaultlibs"
  = note: arm-none-eabi-gcc: error: unrecognized command line option '-m64'

error: aborting due to previous error


make: *** [Makefile:6: all] Error 1
------------------------------------------




failures:
    [run-make] run-make/dep-graph

test result: FAILED. 14 passed; 1 failed; 23 ignored; 0 measured; 0 filtered out; finished in 22.21s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-thumbv6m-none-eabi" "--suite" "run-make" "--mode" "run-make" "--target" "thumbv6m-none-eabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-none-eabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/thumbv6m-none-eabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin" "--cc" "arm-none-eabi-gcc" "--cxx" "arm-none-eabi-g++" "--cflags" "-ffunction-sections -fdata-sections -mthumb -march=armv6s-m" "--ar" "arm-none-eabi-ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:15:46
