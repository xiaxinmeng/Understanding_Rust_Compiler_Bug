plain
.................................................................................................... 9300/11552
.................................................................................................... 9400/11552
.................................................................................i......i........... 9500/11552
.................................................................................................... 9600/11552
............................iiiiiii.iiiiii.i........................................................ 9700/11552
.................................................................................................... 9900/11552
.................................................................................................... 10000/11552
.................................................................................................... 10100/11552
.................................................................................................... 10200/11552
---
failures:

---- [ui] ui/closures/2229_closure_analysis/pattern-matching-should-fail.rs stdout ----

error: /checkout/src/test/ui/closures/2229_closure_analysis/pattern-matching-should-fail.rs:59: unexpected error: '59:15: 59:16: use of possibly-uninitialized variable: `g` [E0381]'
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: /checkout/src/test/ui/closures/2229_closure_analysis/pattern-matching-should-fail.rs:61: unexpected error: '61:19: 61:20: use of possibly-uninitialized variable: `t` [E0381]'

error: /checkout/src/test/ui/closures/2229_closure_analysis/pattern-matching-should-fail.rs:56: expected error not found: use of possibly-uninitialized variable: `g`

error: /checkout/src/test/ui/closures/2229_closure_analysis/pattern-matching-should-fail.rs:56: expected error not found: use of possibly-uninitialized variable: `t`
error: 2 unexpected errors found, 2 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/pattern-matching-should-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/pattern-matching-should-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/pattern-matching-should-fail/auxiliary"
    Error {
        line_num: 59,
        kind: Some(
            Error,
            Error,
        ),
        msg: "59:15: 59:16: use of possibly-uninitialized variable: `g` [E0381]",
    Error {
        line_num: 61,
        kind: Some(
            Error,
            Error,
        ),
        msg: "61:19: 61:20: use of possibly-uninitialized variable: `t` [E0381]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 56,
        kind: Some(
            Error,
        ),
        msg: "use of possibly-uninitialized variable: `g`",
    Error {
        line_num: 56,
        kind: Some(
            Error,
            Error,
        ),
        msg: "use of possibly-uninitialized variable: `t`",
]

thread '[ui] ui/closures/2229_closure_analysis/pattern-matching-should-fail.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1492:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [ui] ui/pattern/move-ref-patterns/borrowck-move-ref-pattern.rs stdout ----

error: /checkout/src/test/ui/pattern/move-ref-patterns/borrowck-move-ref-pattern.rs:45: unexpected error: '45:37: 45:40: use of moved value: `tup` [E0382]'
error: /checkout/src/test/ui/pattern/move-ref-patterns/borrowck-move-ref-pattern.rs:43: expected error not found: use of moved value

error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/move-ref-patterns/borrowck-move-ref-pattern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/move-ref-patterns/borrowck-move-ref-pattern" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/move-ref-patterns/borrowck-move-ref-pattern/auxiliary"
    Error {
        line_num: 45,
        kind: Some(
            Error,
            Error,
        ),
        msg: "45:37: 45:40: use of moved value: `tup` [E0382]",
]

not found errors (from test file): [
    Error {
---
test result: FAILED. 11457 passed; 2 failed; 93 ignored; 0 measured; 0 filtered out; finished in 136.94s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:12
