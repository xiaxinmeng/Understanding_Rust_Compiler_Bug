plain
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 12190 tests
.................................................................................................... 100/12190
..............................................ii.......F.......ii..............F.................... 200/12190
.................................................................................................... 400/12190
.................................................................................................... 500/12190
.................................................................................................... 600/12190
..........................................................i......................................... 700/12190
---
failures:

---- [ui] ui/asm/bad-reg.rs stdout ----

error: /checkout/src/test/ui/asm/bad-reg.rs:62: unexpected error: '62:34: 62:48: register class `ymm_reg` requires the `avx` target feature'

error: /checkout/src/test/ui/asm/bad-reg.rs:64: unexpected error: '64:34: 64:49: register class `ymm_reg` requires the `avx` target feature'

error: /checkout/src/test/ui/asm/bad-reg.rs:66: unexpected error: '66:34: 66:53: register class `ymm_reg` requires the `avx` target feature'
error: 3 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/bad-reg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-reg" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "target-feature=+avx2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-reg/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
    Error {
        line_num: 62,
        kind: Some(
            Error,
            Error,
        ),
        msg: "62:34: 62:48: register class `ymm_reg` requires the `avx` target feature",
    Error {
        line_num: 64,
        kind: Some(
            Error,
            Error,
        ),
        msg: "64:34: 64:49: register class `ymm_reg` requires the `avx` target feature",
    Error {
        line_num: 66,
        kind: Some(
            Error,
            Error,
        ),
        msg: "66:34: 66:53: register class `ymm_reg` requires the `avx` target feature",
]

thread '[ui] ui/asm/bad-reg.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1532:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [ui] ui/asm/type-check-3.rs stdout ----

error: /checkout/src/test/ui/asm/type-check-3.rs:25: unexpected error: '25:22: 25:50: register class `ymm_reg` requires the `avx` target feature'

error: /checkout/src/test/ui/asm/type-check-3.rs:26: unexpected error: '26:20: 26:33: register class `kreg` requires at least one target feature: avx512bw, avx512f'

error: /checkout/src/test/ui/asm/type-check-3.rs:27: unexpected error: '27:20: 27:33: register class `kreg` requires at least one target feature: avx512bw, avx512f'

error: /checkout/src/test/ui/asm/type-check-3.rs:39: unexpected error: '39:20: 39:36: register class `ymm_reg` requires the `avx` target feature'

error: /checkout/src/test/ui/asm/type-check-3.rs:41: unexpected error: '41:20: 41:51: register class `ymm_reg` requires the `avx` target feature'

error: /checkout/src/test/ui/asm/type-check-3.rs:45: unexpected error: '45:22: 45:38: register class `ymm_reg` requires the `avx` target feature'

error: /checkout/src/test/ui/asm/type-check-3.rs:46: unexpected error: '46:22: 46:53: register class `ymm_reg` requires the `avx` target feature'
error: /checkout/src/test/ui/asm/type-check-3.rs:12: expected error not found: type `i128` cannot be used with this register class


error: /checkout/src/test/ui/asm/type-check-3.rs:14: expected error not found: type `__m128` cannot be used with this register class

error: /checkout/src/test/ui/asm/type-check-3.rs:16: expected error not found: type `__m256` cannot be used with this register class
error: /checkout/src/test/ui/asm/type-check-3.rs:18: expected error not found: type `u8` cannot be used with this register class


error: /checkout/src/test/ui/asm/type-check-3.rs:27: expected error not found: `avx512bw` target feature is not enabled
error: /checkout/src/test/ui/asm/type-check-3.rs:32: expected warning not found: formatting may not be suitable for sub-register argument

error: /checkout/src/test/ui/asm/type-check-3.rs:34: expected warning not found: formatting may not be suitable for sub-register argument


error: /checkout/src/test/ui/asm/type-check-3.rs:36: expected warning not found: formatting may not be suitable for sub-register argument

error: /checkout/src/test/ui/asm/type-check-3.rs:39: expected warning not found: formatting may not be suitable for sub-register argument

error: /checkout/src/test/ui/asm/type-check-3.rs:50: expected error not found: type `i8` cannot be used with this register class

error: /checkout/src/test/ui/asm/type-check-3.rs:62: expected error not found: incompatible types for asm inout argument

error: /checkout/src/test/ui/asm/type-check-3.rs:64: expected error not found: incompatible types for asm inout argument

error: /checkout/src/test/ui/asm/type-check-3.rs:66: expected error not found: incompatible types for asm inout argument
error: /checkout/src/test/ui/asm/type-check-3.rs:82: expected error not found: constants cannot refer to statics

error: /checkout/src/test/ui/asm/type-check-3.rs:85: expected error not found: constants cannot refer to statics


error: /checkout/src/test/ui/asm/type-check-3.rs:88: expected error not found: constants cannot refer to statics

error: 7 unexpected errors found, 16 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/type-check-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "target-feature=+avx512f" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/type-check-3/auxiliary"
    Error {
        line_num: 25,
        kind: Some(
            Error,
            Error,
        ),
        msg: "25:22: 25:50: register class `ymm_reg` requires the `avx` target feature",
    Error {
        line_num: 26,
        kind: Some(
            Error,
            Error,
        ),
        msg: "26:20: 26:33: register class `kreg` requires at least one target feature: avx512bw, avx512f",
    Error {
        line_num: 27,
        kind: Some(
            Error,
            Error,
        ),
        msg: "27:20: 27:33: register class `kreg` requires at least one target feature: avx512bw, avx512f",
    Error {
        line_num: 39,
        kind: Some(
            Error,
            Error,
        ),
        msg: "39:20: 39:36: register class `ymm_reg` requires the `avx` target feature",
    Error {
        line_num: 41,
        kind: Some(
            Error,
            Error,
        ),
        msg: "41:20: 41:51: register class `ymm_reg` requires the `avx` target feature",
    Error {
        line_num: 45,
        kind: Some(
            Error,
            Error,
        ),
        msg: "45:22: 45:38: register class `ymm_reg` requires the `avx` target feature",
    Error {
        line_num: 46,
        kind: Some(
            Error,
            Error,
        ),
        msg: "46:22: 46:53: register class `ymm_reg` requires the `avx` target feature",
]

not found errors (from test file): [
    Error {
---
        line_num: 14,
        kind: Some(
            Error,
        ),
        msg: "type `__m128` cannot be used with this register class",
    Error {
        line_num: 16,
        kind: Some(
            Error,
            Error,
        ),
        msg: "type `__m256` cannot be used with this register class",
    Error {
        line_num: 18,
        kind: Some(
            Error,
---
        line_num: 27,
        kind: Some(
            Error,
        ),
        msg: "`avx512bw` target feature is not enabled",
    Error {
        line_num: 32,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "formatting may not be suitable for sub-register argument",
    Error {
        line_num: 34,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "formatting may not be suitable for sub-register argument",
    Error {
        line_num: 36,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "formatting may not be suitable for sub-register argument",
    Error {
        line_num: 39,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "formatting may not be suitable for sub-register argument",
    Error {
        line_num: 50,
        kind: Some(
            Error,
---
        line_num: 62,
        kind: Some(
            Error,
        ),
        msg: "incompatible types for asm inout argument",
    Error {
        line_num: 64,
        kind: Some(
            Error,
            Error,
        ),
        msg: "incompatible types for asm inout argument",
    Error {
        line_num: 66,
        kind: Some(
            Error,
            Error,
        ),
        msg: "incompatible types for asm inout argument",
    Error {
        line_num: 82,
        kind: Some(
            Error,
---
test result: FAILED. 12086 passed; 2 failed; 102 ignored; 0 measured; 0 filtered out; finished in 127.42s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:20
