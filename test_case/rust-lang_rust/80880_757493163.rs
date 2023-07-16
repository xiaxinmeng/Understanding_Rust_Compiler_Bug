plain
.................................................................................................... 2100/11246
.................................................................................................... 2200/11246
.................................................................................................... 2300/11246
.................................................................................................... 2400/11246
................................................2021-01-10T15:16:23.708135Z ERROR compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/cross-crate/auxiliary/issue-10031-aux.rs` source not found"
F.............i..i.................................. 2500/11246
.................................................................iiiii.............................. 2700/11246
.................................................................iiiii.............................. 2700/11246
........2021-01-10T15:16:26.996615Z ERROR compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/drop/auxiliary/issue-10028.rs` source not found"
F........................................................................................... 2800/11246
.................................................................................................... 3000/11246
.................................................................................................... 3100/11246
.................................................................................................... 3200/11246
.................................................................................................... 3300/11246
---
.................................................................................................... 9000/11246
.................................................................................................... 9100/11246
.................................................................................................... 9200/11246
..........................................i......i.................................................. 9300/11246
.................................................................................iiiiii..iiiiii.i... 9400/11246
.................................................................................................... 9600/11246
.................................................................................................... 9700/11246
.................................................................................................... 9800/11246
.................................................................................................... 9900/11246
---
failures:

---- [ui] ui/cross-crate/issue-10031.rs stdout ----

error: aux-build `/checkout/src/test/ui/cross-crate/auxiliary/issue-10031-aux.rs` source not found
thread '[ui] ui/cross-crate/issue-10031.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2222:9

---- [ui] ui/drop/issue-10028.rs stdout ----


error: aux-build `/checkout/src/test/ui/drop/auxiliary/issue-10028.rs` source not found
thread '[ui] ui/drop/issue-10028.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2222:9

failures:
    [ui] ui/cross-crate/issue-10031.rs
    [ui] ui/drop/issue-10028.rs
    [ui] ui/drop/issue-10028.rs

test result: FAILED. 11158 passed; 2 failed; 86 ignored; 0 measured; 0 filtered out; finished in 141.47s

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:00
