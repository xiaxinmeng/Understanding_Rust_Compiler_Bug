plain
.................................................................................................... 10300/11934
.................................................................................................... 10400/11934
.................................................................................................... 10500/11934
.................................................................................................... 10600/11934
.................................F...F........i......................................i.............. 10700/11934
.................................................................................................... 10900/11934
.................................................................................................... 11000/11934
.................................................................................................... 11100/11934
.................................................................................................... 11200/11934
---
failures:

---- [ui] ui/symbol-names/const-generics-demangling.rs stdout ----

error: /checkout/src/test/ui/symbol-names/const-generics-demangling.rs:7: unexpected error: '7:1: 7:21: symbol-name(_RMCsaP8qXevlYG3_25const_generics_demanglingINtB0_8UnsignedKhb_E)'

error: /checkout/src/test/ui/symbol-names/const-generics-demangling.rs:7: unexpected error: '7:1: 7:21: demangling(<const_generics_demangling[7e153590edc26969]::Unsigned<11: u8>>)'
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: /checkout/src/test/ui/symbol-names/const-generics-demangling.rs:15: unexpected error: '15:1: 15:21: symbol-name(_RMs_CsaP8qXevlYG3_25const_generics_demanglingINtB2_6SignedKsn98_E)'

error: /checkout/src/test/ui/symbol-names/const-generics-demangling.rs:15: unexpected error: '15:1: 15:21: demangling(<const_generics_demangling[7e153590edc26969]::Signed<-152: i16>>)'

error: /checkout/src/test/ui/symbol-names/const-generics-demangling.rs:23: unexpected error: '23:1: 23:21: symbol-name(_RMs0_CsaP8qXevlYG3_25const_generics_demanglingINtB3_4BoolKb1_E)'

error: /checkout/src/test/ui/symbol-names/const-generics-demangling.rs:23: unexpected error: '23:1: 23:21: demangling(<const_generics_demangling[7e153590edc26969]::Bool<true: bool>>)'

error: /checkout/src/test/ui/symbol-names/const-generics-demangling.rs:31: unexpected error: '31:1: 31:21: symbol-name(_RMs1_CsaP8qXevlYG3_25const_generics_demanglingINtB3_4CharKc2202_E)'

error: /checkout/src/test/ui/symbol-names/const-generics-demangling.rs:31: unexpected error: '31:1: 31:21: demangling(<const_generics_demangling[7e153590edc26969]::Char<'∂': char>>)'

error: /checkout/src/test/ui/symbol-names/const-generics-demangling.rs:7: expected error not found: symbol-name(_RMCs21hi0yVfW1J_25const_generics_demanglingINtB0_8UnsignedKhb_E)

error: /checkout/src/test/ui/symbol-names/const-generics-demangling.rs:7: expected error not found: demangling(<const_generics_demangling[17891616a171812d]::Unsigned<11: u8>>)

error: /checkout/src/test/ui/symbol-names/const-generics-demangling.rs:15: expected error not found: symbol-name(_RMs_Cs21hi0yVfW1J_25const_generics_demanglingINtB2_6SignedKsn98_E)

error: /checkout/src/test/ui/symbol-names/const-generics-demangling.rs:15: expected error not found: demangling(<const_generics_demangling[17891616a171812d]::Signed<-152: i16>>)

error: /checkout/src/test/ui/symbol-names/const-generics-demangling.rs:23: expected error not found: symbol-name(_RMs0_Cs21hi0yVfW1J_25const_generics_demanglingINtB3_4BoolKb1_E)

error: /checkout/src/test/ui/symbol-names/const-generics-demangling.rs:23: expected error not found: demangling(<const_generics_demangling[17891616a171812d]::Bool<true: bool>>)

error: /checkout/src/test/ui/symbol-names/const-generics-demangling.rs:31: expected error not found: symbol-name(_RMs1_Cs21hi0yVfW1J_25const_generics_demanglingINtB3_4CharKc2202_E)

error: /checkout/src/test/ui/symbol-names/const-generics-demangling.rs:31: expected error not found: demangling(<const_generics_demangling[17891616a171812d]::Char<'∂': char>>)
error: 8 unexpected errors found, 8 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/const-generics-demangling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-demangling" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-demangling/auxiliary"
    Error {
        line_num: 7,
        kind: Some(
            Error,
            Error,
        ),
        msg: "7:1: 7:21: symbol-name(_RMCsaP8qXevlYG3_25const_generics_demanglingINtB0_8UnsignedKhb_E)",
    Error {
        line_num: 7,
        kind: Some(
            Error,
            Error,
        ),
        msg: "7:1: 7:21: demangling(<const_generics_demangling[7e153590edc26969]::Unsigned<11: u8>>)",
    Error {
        line_num: 15,
        kind: Some(
            Error,
            Error,
        ),
        msg: "15:1: 15:21: symbol-name(_RMs_CsaP8qXevlYG3_25const_generics_demanglingINtB2_6SignedKsn98_E)",
    Error {
        line_num: 15,
        kind: Some(
            Error,
            Error,
        ),
        msg: "15:1: 15:21: demangling(<const_generics_demangling[7e153590edc26969]::Signed<-152: i16>>)",
    Error {
        line_num: 23,
        kind: Some(
            Error,
            Error,
        ),
        msg: "23:1: 23:21: symbol-name(_RMs0_CsaP8qXevlYG3_25const_generics_demanglingINtB3_4BoolKb1_E)",
    Error {
        line_num: 23,
        kind: Some(
            Error,
            Error,
        ),
        msg: "23:1: 23:21: demangling(<const_generics_demangling[7e153590edc26969]::Bool<true: bool>>)",
    Error {
        line_num: 31,
        kind: Some(
            Error,
            Error,
        ),
        msg: "31:1: 31:21: symbol-name(_RMs1_CsaP8qXevlYG3_25const_generics_demanglingINtB3_4CharKc2202_E)",
    Error {
        line_num: 31,
        kind: Some(
            Error,
            Error,
        ),
        msg: "31:1: 31:21: demangling(<const_generics_demangling[7e153590edc26969]::Char<'∂': char>>)",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 7,
        kind: Some(
            Error,
        ),
        msg: "symbol-name(_RMCs21hi0yVfW1J_25const_generics_demanglingINtB0_8UnsignedKhb_E)",
    Error {
        line_num: 7,
        kind: Some(
            Error,
            Error,
        ),
        msg: "demangling(<const_generics_demangling[17891616a171812d]::Unsigned<11: u8>>)",
    Error {
        line_num: 15,
        kind: Some(
            Error,
            Error,
        ),
        msg: "symbol-name(_RMs_Cs21hi0yVfW1J_25const_generics_demanglingINtB2_6SignedKsn98_E)",
    Error {
        line_num: 15,
        kind: Some(
            Error,
            Error,
        ),
        msg: "demangling(<const_generics_demangling[17891616a171812d]::Signed<-152: i16>>)",
    Error {
        line_num: 23,
        kind: Some(
            Error,
            Error,
        ),
        msg: "symbol-name(_RMs0_Cs21hi0yVfW1J_25const_generics_demanglingINtB3_4BoolKb1_E)",
    Error {
        line_num: 23,
        kind: Some(
            Error,
            Error,
        ),
        msg: "demangling(<const_generics_demangling[17891616a171812d]::Bool<true: bool>>)",
    Error {
        line_num: 31,
        kind: Some(
            Error,
            Error,
        ),
        msg: "symbol-name(_RMs1_Cs21hi0yVfW1J_25const_generics_demanglingINtB3_4CharKc2202_E)",
    Error {
        line_num: 31,
        kind: Some(
            Error,
            Error,
        ),
        msg: "demangling(<const_generics_demangling[17891616a171812d]::Char<'∂': char>>)",
]

thread '[ui] ui/symbol-names/const-generics-demangling.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1524:13


---- [ui] ui/symbol-names/impl1.rs#v0 stdout ----

error in revision `v0`: /checkout/src/test/ui/symbol-names/impl1.rs:32: unexpected error: '32:9: 32:29: symbol-name(_RNvMNtCs2qSCrjELJET_5impl13barNtNtB4_3foo3Foo3baz)'

error in revision `v0`: /checkout/src/test/ui/symbol-names/impl1.rs:32: unexpected error: '32:9: 32:29: demangling(<impl1[1c5860ab79c9e305]::foo::Foo>::baz)'

error in revision `v0`: /checkout/src/test/ui/symbol-names/impl1.rs:32: expected error not found: symbol-name(_RNvMNtCs21hi0yVfW1J_5impl13barNtNtB4_3foo3Foo3baz)

error in revision `v0`: /checkout/src/test/ui/symbol-names/impl1.rs:32: expected error not found: demangling(<impl1[17891616a171812d]::foo::Foo>::baz)

error in revision `v0`: 2 unexpected errors found, 2 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/impl1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "v0" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.v0" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.v0/auxiliary"
    Error {
        line_num: 32,
        kind: Some(
            Error,
            Error,
        ),
        msg: "32:9: 32:29: symbol-name(_RNvMNtCs2qSCrjELJET_5impl13barNtNtB4_3foo3Foo3baz)",
    Error {
        line_num: 32,
        kind: Some(
            Error,
            Error,
        ),
        msg: "32:9: 32:29: demangling(<impl1[1c5860ab79c9e305]::foo::Foo>::baz)",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 32,
        kind: Some(
            Error,
        ),
        msg: "symbol-name(_RNvMNtCs21hi0yVfW1J_5impl13barNtNtB4_3foo3Foo3baz)",
    Error {
        line_num: 32,
        kind: Some(
            Error,
            Error,
        ),
        msg: "demangling(<impl1[17891616a171812d]::foo::Foo>::baz)",
]


thread '[ui] ui/symbol-names/impl1.rs#v0' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1524:13

failures:
    [ui] ui/symbol-names/const-generics-demangling.rs
    [ui] ui/symbol-names/impl1.rs#v0
    [ui] ui/symbol-names/impl1.rs#v0

test result: FAILED. 11835 passed; 2 failed; 97 ignored; 0 measured; 0 filtered out; finished in 100.10s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:10:20
