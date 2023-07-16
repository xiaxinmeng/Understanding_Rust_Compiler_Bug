plain
.................................................................................................... 5500/12239
.................................................................................................... 5600/12239
.................................................................................................... 5700/12239
.................................................................................................... 5800/12239
.............................................................2021-10-03T13:14:03.751547Z ERROR compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/issues/auxiliary/xcrate-issue-46112-rexport-core.rs` source not found"
F...................................... 5900/12239
.................................................................................................... 6100/12239
.................................................................................................... 6100/12239
...i................2021-10-03T13:14:05.831172Z ERROR compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/issues/auxiliary/xcrate-issue-61711-b.rs` source not found"
F................................................i.............................. 6200/12239
.................ii.ii.......i...i.................................................................. 6400/12239
.................ii.ii.......i...i.................................................................. 6400/12239
..2021-10-03T13:14:08.750921Z ERROR compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/auxiliary/legacy-const-generics.rs` source not found"
F........................................................................i....i................... 6500/12239
i................................................................................................... 6700/12239
i................................................................................................... 6800/12239
i................................................................................................... 6800/12239
.................................i.......................................2021-10-03T13:14:11.672681Z ERROR compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/auxiliary/lto-rustc-loads-linker-plugin.rs` source not found"
F.......................... 6900/12239
...................i................................................................................ 7100/12239
...............................................................................i.................... 7200/12239
.................................................................................................... 7300/12239
.................................................................................................... 7400/12239
---
failures:

---- [ui] ui/issues/issue-46112.rs stdout ----

error: aux-build `/checkout/src/test/ui/issues/auxiliary/xcrate-issue-46112-rexport-core.rs` source not found
thread '[ui] ui/issues/issue-46112.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2278:9

---- [ui] ui/issues/issue-61711-once-caused-rustc-inf-loop.rs stdout ----


error: aux-build `/checkout/src/test/ui/issues/auxiliary/xcrate-issue-61711-b.rs` source not found
thread '[ui] ui/issues/issue-61711-once-caused-rustc-inf-loop.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2278:9
---- [ui] ui/legacy-const-generics.rs stdout ----

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: aux-build `/checkout/src/test/ui/auxiliary/legacy-const-generics.rs` source not found
thread '[ui] ui/legacy-const-generics.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2278:9
---- [ui] ui/lto-rustc-loads-linker-plugin.rs stdout ----


error: aux-build `/checkout/src/test/ui/auxiliary/lto-rustc-loads-linker-plugin.rs` source not found
thread '[ui] ui/lto-rustc-loads-linker-plugin.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2278:9

failures:
    [ui] ui/issues/issue-46112.rs
    [ui] ui/issues/issue-61711-once-caused-rustc-inf-loop.rs
    [ui] ui/issues/issue-61711-once-caused-rustc-inf-loop.rs
    [ui] ui/legacy-const-generics.rs
    [ui] ui/lto-rustc-loads-linker-plugin.rs

test result: FAILED. 12120 passed; 4 failed; 115 ignored; 0 measured; 0 filtered out; finished in 134.18s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:20
