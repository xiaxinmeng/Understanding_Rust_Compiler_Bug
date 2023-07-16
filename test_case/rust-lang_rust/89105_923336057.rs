plain
..........................................i.i....................................................... 12100/12154
......................................................
failures:

---- [ui] ui/pattern/usefulness/doc-hidden-non-exhaustive.rs stdout ----

error: /checkout/src/test/ui/pattern/usefulness/doc-hidden-non-exhaustive.rs:20: unexpected error: '20:11: 20:17: non-exhaustive patterns: `B` and `_` not covered [E0004]'

error: /checkout/src/test/ui/pattern/usefulness/doc-hidden-non-exhaustive.rs:25: unexpected error: '25:11: 25:15: non-exhaustive patterns: `Some(B)` and `Some(_)` not covered [E0004]'
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error: /checkout/src/test/ui/pattern/usefulness/doc-hidden-non-exhaustive.rs:20: expected message not found: non-exhaustive patterns: `_` not covered

error: /checkout/src/test/ui/pattern/usefulness/doc-hidden-non-exhaustive.rs:25: expected message not found: non-exhaustive patterns: `Some(_)` not covered
error: 2 unexpected errors found, 2 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/usefulness/doc-hidden-non-exhaustive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/doc-hidden-non-exhaustive" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/doc-hidden-non-exhaustive/auxiliary"
    Error {
        line_num: 20,
        kind: Some(
            Error,
            Error,
        ),
        msg: "20:11: 20:17: non-exhaustive patterns: `B` and `_` not covered [E0004]",
    Error {
        line_num: 25,
        kind: Some(
            Error,
            Error,
        ),
        msg: "25:11: 25:15: non-exhaustive patterns: `Some(B)` and `Some(_)` not covered [E0004]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 20,
        kind: None,
        msg: "non-exhaustive patterns: `_` not covered",
    Error {
        line_num: 25,
        kind: None,
        kind: None,
        msg: "non-exhaustive patterns: `Some(_)` not covered",
]


thread '[ui] ui/pattern/usefulness/doc-hidden-non-exhaustive.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1532:13

failures:
    [ui] ui/pattern/usefulness/doc-hidden-non-exhaustive.rs


test result: FAILED. 12051 passed; 1 failed; 102 ignored; 0 measured; 0 filtered out; finished in 130.39s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:01
