plain
test [ui] ui/mpsc_stress.rs ... ok

failures:

---- [ui] ui/panics/issue-47429-short-backtraces.rs#legacy stdout ----
normalized run.stdout:
uploaded "$TEST_BUILD_DIR/panics/issue-47429-short-backtraces.legacy/a", waiting for result
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=arm-unknown-linux-gnueabihf


The actual run.stdout differed from the expected run.stdout.
The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/issue-47429-short-backtraces.legacy/issue-47429-short-backtraces.legacy.run.stdout

error in revision `legacy`: 1 errors occurred comparing run output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "0" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/issue-47429-short-backtraces.legacy/a"
------------------------------------------
------------------------------------------
uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/issue-47429-short-backtraces.legacy/a", waiting for result
------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/panics/issue-47429-short-backtraces.rs:21:5
---

------------------------------------------


---- [ui] ui/panics/issue-47429-short-backtraces.rs#v0 stdout ----
normalized run.stdout:
uploaded "$TEST_BUILD_DIR/panics/issue-47429-short-backtraces.v0/a", waiting for result


The actual run.stdout differed from the expected run.stdout.
The actual run.stdout differed from the expected run.stdout.
Actual run.stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/issue-47429-short-backtraces.v0/issue-47429-short-backtraces.v0.run.stdout

error in revision `v0`: 1 errors occurred comparing run output.
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "run" "0" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/issue-47429-short-backtraces.v0/a"
------------------------------------------
------------------------------------------
uploaded "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/issue-47429-short-backtraces.v0/a", waiting for result
------------------------------------------
stderr:
------------------------------------------
thread 'main' panicked at 'explicit panic', /checkout/src/test/ui/panics/issue-47429-short-backtraces.rs:21:5
---
test result: FAILED. 12024 passed; 2 failed; 169 ignored; 0 measured; 0 filtered out; finished in 588.93s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-arm-unknown-linux-gnueabihf" "--suite" "ui" "--mode" "ui" "--target" "arm-unknown-linux-gnueabihf" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-linux-gnueabihf-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--llvm-version" "13.0.0-rust-1.56.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:24:05
