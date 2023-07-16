plain
.....................................................................................i.i............ 11800/11894
..............................................................................................
failures:

---- [ui] ui/methods/method-not-found-generic-arg-elision.rs stdout ----

error: /checkout/src/test/ui/methods/method-not-found-generic-arg-elision.rs:71: unexpected error: '71:23: 71:28: no method named `other` found for struct `Point` in the current scope [E0599]'

error: /checkout/src/test/ui/methods/method-not-found-generic-arg-elision.rs:74: unexpected error: '74:29: 74:35: no method named `extend` found for struct `Map` in the current scope [E0599]'
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: /checkout/src/test/ui/methods/method-not-found-generic-arg-elision.rs:79: unexpected error: '79:13: 79:18: no method named `other` found for struct `Wrapper` in the current scope [E0599]'

error: /checkout/src/test/ui/methods/method-not-found-generic-arg-elision.rs:85: unexpected error: '85:13: 85:18: no method named `other` found for struct `Wrapper2` in the current scope [E0599]'

error: /checkout/src/test/ui/methods/method-not-found-generic-arg-elision.rs:88: unexpected error: '88:7: 88:16: no method named `not_found` found for struct `Vec` in the current scope [E0599]'

error: /checkout/src/test/ui/methods/method-not-found-generic-arg-elision.rs:71: expected error not found: no method named `other` found for struct `Point<_>

error: /checkout/src/test/ui/methods/method-not-found-generic-arg-elision.rs:74: expected error not found: no method named `extend` found for struct `Map<_, _>

error: /checkout/src/test/ui/methods/method-not-found-generic-arg-elision.rs:79: expected error not found: no method named `other` found for struct `Wrapper<_>

error: /checkout/src/test/ui/methods/method-not-found-generic-arg-elision.rs:85: expected error not found: no method named `other` found for struct `Wrapper2<'_, _, _>

error: /checkout/src/test/ui/methods/method-not-found-generic-arg-elision.rs:88: expected error not found: no method named `not_found` found for struct `Vec<_, _>
error: 5 unexpected errors found, 5 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-not-found-generic-arg-elision.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-not-found-generic-arg-elision" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-not-found-generic-arg-elision/auxiliary"
    Error {
        line_num: 71,
        kind: Some(
            Error,
            Error,
        ),
        msg: "71:23: 71:28: no method named `other` found for struct `Point` in the current scope [E0599]",
    Error {
        line_num: 74,
        kind: Some(
            Error,
            Error,
        ),
        msg: "74:29: 74:35: no method named `extend` found for struct `Map` in the current scope [E0599]",
    Error {
        line_num: 79,
        kind: Some(
            Error,
            Error,
        ),
        msg: "79:13: 79:18: no method named `other` found for struct `Wrapper` in the current scope [E0599]",
    Error {
        line_num: 85,
        kind: Some(
            Error,
            Error,
        ),
        msg: "85:13: 85:18: no method named `other` found for struct `Wrapper2` in the current scope [E0599]",
    Error {
        line_num: 88,
        kind: Some(
            Error,
            Error,
        ),
        msg: "88:7: 88:16: no method named `not_found` found for struct `Vec` in the current scope [E0599]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 71,
        kind: Some(
            Error,
        ),
        msg: "no method named `other` found for struct `Point<_>",
    Error {
        line_num: 74,
        kind: Some(
            Error,
            Error,
        ),
        msg: "no method named `extend` found for struct `Map<_, _>",
    Error {
        line_num: 79,
        kind: Some(
            Error,
            Error,
        ),
        msg: "no method named `other` found for struct `Wrapper<_>",
    Error {
        line_num: 85,
        kind: Some(
            Error,
            Error,
        ),
        msg: "no method named `other` found for struct `Wrapper2<'_, _, _>",
    Error {
        line_num: 88,
        kind: Some(
            Error,
            Error,
        ),
        msg: "no method named `not_found` found for struct `Vec<_, _>",
]


thread '[ui] ui/methods/method-not-found-generic-arg-elision.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1524:13

failures:
    [ui] ui/methods/method-not-found-generic-arg-elision.rs


test result: FAILED. 11796 passed; 1 failed; 97 ignored; 0 measured; 0 filtered out; finished in 121.95s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:42
