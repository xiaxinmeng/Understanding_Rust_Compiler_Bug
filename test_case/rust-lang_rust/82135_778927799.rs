plain
.................................................................................................... 9200/11451
.................................................................................................... 9300/11451
.................................................................................................... 9400/11451
..........i......i.................................................................................. 9500/11451
................................................iiiiiii...iiiiiii................................... 9600/11451
.................................................................................................... 9800/11451
.................................................................................................... 9900/11451
.................................................................................................... 10000/11451
.................................................................................................... 10100/11451
---
failures:

---- [ui] ui/async-await/issue-61076.rs stdout ----

error: /checkout/src/test/ui/async-await/issue-61076.rs:58: unexpected note: 'checked the return type of this `async fn`, expected opaque type'

error: /checkout/src/test/ui/async-await/issue-61076.rs:58: expected note not found: the `Output` of this `async fn`'s expected opaque type
error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-61076.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61076" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61076/auxiliary"
    Error {
        line_num: 58,
        kind: Some(
            Note,
            Note,
        ),
        msg: "checked the return type of this `async fn`, expected opaque type",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 58,
        kind: Some(
            Note,
        ),
        msg: "the `Output` of this `async fn`\'s expected opaque type",
]

thread '[ui] ui/async-await/issue-61076.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [ui] ui/suggestions/match-prev-arm-needing-semi.rs stdout ----

error: /checkout/src/test/ui/suggestions/match-prev-arm-needing-semi.rs:16: unexpected note: 'checked the return type of this `async fn`, found opaque type'

error: /checkout/src/test/ui/suggestions/match-prev-arm-needing-semi.rs:17: unexpected note: 'checked the return type of this `async fn`, found opaque type'

error: /checkout/src/test/ui/suggestions/match-prev-arm-needing-semi.rs:17: unexpected note: 'checked the return type of this `async fn`, found opaque type'

error: /checkout/src/test/ui/suggestions/match-prev-arm-needing-semi.rs:16: expected note not found: the `Output` of this `async fn`'s found opaque type

error: /checkout/src/test/ui/suggestions/match-prev-arm-needing-semi.rs:17: expected note not found: the `Output` of this `async fn`'s found opaque type

error: /checkout/src/test/ui/suggestions/match-prev-arm-needing-semi.rs:17: expected note not found: the `Output` of this `async fn`'s found opaque type
error: 3 unexpected errors found, 3 expected errors not found
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/match-prev-arm-needing-semi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-prev-arm-needing-semi" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-prev-arm-needing-semi/auxiliary"
    Error {
        line_num: 16,
        kind: Some(
            Note,
            Note,
        ),
        msg: "checked the return type of this `async fn`, found opaque type",
    Error {
        line_num: 17,
        kind: Some(
            Note,
            Note,
        ),
        msg: "checked the return type of this `async fn`, found opaque type",
    Error {
        line_num: 17,
        kind: Some(
            Note,
            Note,
        ),
        msg: "checked the return type of this `async fn`, found opaque type",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 16,
        kind: Some(
            Note,
        ),
        msg: "the `Output` of this `async fn`\'s found opaque type",
    Error {
        line_num: 17,
        kind: Some(
            Note,
            Note,
        ),
        msg: "the `Output` of this `async fn`\'s found opaque type",
    Error {
        line_num: 17,
        kind: Some(
            Note,
            Note,
        ),
        msg: "the `Output` of this `async fn`\'s found opaque type",
]

thread '[ui] ui/suggestions/match-prev-arm-needing-semi.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1491:13

---
test result: FAILED. 11356 passed; 2 failed; 93 ignored; 0 measured; 0 filtered out; finished in 136.03s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:28
