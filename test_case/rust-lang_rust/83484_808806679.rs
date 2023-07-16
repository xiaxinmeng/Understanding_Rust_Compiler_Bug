plain
.................................................................................................... 9400/11719
.................................................................................................... 9500/11719
............................................................i......i................................ 9600/11719
.................................................................................................... 9700/11719
......iiiiiii..iiiiii.i............................................................................. 9800/11719
.................................................................................................... 10000/11719
.................................................................................................... 10100/11719
.................................................................................................... 10200/11719
.................................................................................................... 10300/11719
---
.................................................................................................... 10800/11719
.................................................................................................... 10900/11719
.................................................................................................... 11000/11719
............................ii...................................................................... 11100/11719
...............................................................................F.F.................. 11200/11719
.................................................................................................... 11400/11719
.................................................................................................... 11500/11719
.................................................................................................... 11600/11719
..........i.i....................................................................................... 11700/11719
..........i.i....................................................................................... 11700/11719
...................
failures:

---- [ui] ui/box/alloc-unstable-fail.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/box/alloc-unstable-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/box/alloc-unstable-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/box/alloc-unstable-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

- error[E0747]: type provided when a constant was expected
-   --> $DIR/inferred_const.rs:6:19
-    |
- LL |     let a = foo::<_, 2>([0, 1, 2]);
-    |
-    |
-    = help: const arguments cannot yet be inferred with `_`
- error: aborting due to previous error
- 
- For more information about this error, try `rustc --explain E0747`.
- 
- 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/inferred_const/inferred_const.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/min_const_generics/inferred_const.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/min_const_generics/inferred_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/inferred_const/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/inferred_const/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------


---- [ui] ui/did_you_mean/bad-assoc-ty.rs stdout ----

error: /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:49: expected error not found: the type placeholder `_` is not allowed within types on item signatures
error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/bad-assoc-ty" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/bad-assoc-ty/auxiliary"
    Error {
        line_num: 49,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the type placeholder `_` is not allowed within types on item signatures",
]

thread '[ui] ui/did_you_mean/bad-assoc-ty.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1492:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [ui] ui/inference/cannot-infer-partial-try-return.rs stdout ----

error: /checkout/src/test/ui/inference/cannot-infer-partial-try-return.rs:18: unexpected error: '18:29: 18:46: type annotations needed for the closure `fn() -> Result<(), QualifiedError<_>>` [E0282]'
error: /checkout/src/test/ui/inference/cannot-infer-partial-try-return.rs:19: expected error not found: type annotations needed

error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/cannot-infer-partial-try-return.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-partial-try-return" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-partial-try-return/auxiliary"
    Error {
        line_num: 18,
        kind: Some(
            Error,
            Error,
        ),
        msg: "18:29: 18:46: type annotations needed for the closure `fn() -> Result<(), QualifiedError<_>>` [E0282]",
]

not found errors (from test file): [
    Error {
---
thread '[ui] ui/inference/cannot-infer-partial-try-return.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1492:13

---- [ui] ui/parser/issue-14303-fncall.rs stdout ----

error: /checkout/src/test/ui/parser/issue-14303-fncall.rs:13: unexpected error: '13:26: 13:27: inferred provided when a lifetime was expected [E0747]'

error: /checkout/src/test/ui/parser/issue-14303-fncall.rs:13: expected error not found: type provided when a lifetime was expected
error: 1 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-14303-fncall.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-fncall" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-fncall/auxiliary"
    Error {
        line_num: 13,
        kind: Some(
            Error,
            Error,
        ),
        msg: "13:26: 13:27: inferred provided when a lifetime was expected [E0747]",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 13,
        kind: Some(
            Error,
        ),
        msg: "type provided when a lifetime was expected",
]

thread '[ui] ui/parser/issue-14303-fncall.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1492:13


---- [ui] ui/privacy/associated-item-privacy-trait.rs stdout ----

error: /checkout/src/test/ui/privacy/associated-item-privacy-trait.rs:119: expected error not found: type `priv_parent_substs::Priv` is private
error: 0 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/associated-item-privacy-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-trait/auxiliary"
    Error {
        line_num: 119,
        kind: Some(
            Error,
            Error,
        ),
        msg: "type `priv_parent_substs::Priv` is private",
]

thread '[ui] ui/privacy/associated-item-privacy-trait.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1492:13


---- [ui] ui/typeck/typeck_type_placeholder_item.rs#full_tait stdout ----

error in revision `full_tait`: /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:80: expected error not found: the type placeholder `_` is not allowed within types on item signatures

error in revision `full_tait`: /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:163: expected error not found: the type placeholder `_` is not allowed within types on item signatures

error in revision `full_tait`: /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:166: expected error not found: the type placeholder `_` is not allowed within types on item signatures

error in revision `full_tait`: /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:180: expected error not found: the type placeholder `_` is not allowed within types on item signatures

error in revision `full_tait`: /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:186: expected error not found: the type placeholder `_` is not allowed within types on item signatures

error in revision `full_tait`: /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:225: expected error not found: the type placeholder `_` is not allowed within types on item signatures

error in revision `full_tait`: 0 unexpected errors found, 6 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full_tait" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item.full_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item.full_tait/auxiliary"
    Error {
        line_num: 80,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the type placeholder `_` is not allowed within types on item signatures",
    Error {
        line_num: 163,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the type placeholder `_` is not allowed within types on item signatures",
    Error {
        line_num: 166,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the type placeholder `_` is not allowed within types on item signatures",
    Error {
        line_num: 180,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the type placeholder `_` is not allowed within types on item signatures",
    Error {
        line_num: 186,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the type placeholder `_` is not allowed within types on item signatures",
    Error {
        line_num: 225,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the type placeholder `_` is not allowed within types on item signatures",
]


thread '[ui] ui/typeck/typeck_type_placeholder_item.rs#full_tait' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1492:13
---- [ui] ui/typeck/typeck_type_placeholder_item.rs#min_tait stdout ----


error in revision `min_tait`: /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:80: expected error not found: the type placeholder `_` is not allowed within types on item signatures

error in revision `min_tait`: /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:163: expected error not found: the type placeholder `_` is not allowed within types on item signatures

error in revision `min_tait`: /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:166: expected error not found: the type placeholder `_` is not allowed within types on item signatures

error in revision `min_tait`: /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:180: expected error not found: the type placeholder `_` is not allowed within types on item signatures

error in revision `min_tait`: /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:186: expected error not found: the type placeholder `_` is not allowed within types on item signatures

error in revision `min_tait`: /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:225: expected error not found: the type placeholder `_` is not allowed within types on item signatures

error in revision `min_tait`: 0 unexpected errors found, 6 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min_tait" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item.min_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item.min_tait/auxiliary"
    Error {
        line_num: 80,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the type placeholder `_` is not allowed within types on item signatures",
    Error {
        line_num: 163,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the type placeholder `_` is not allowed within types on item signatures",
    Error {
        line_num: 166,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the type placeholder `_` is not allowed within types on item signatures",
    Error {
        line_num: 180,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the type placeholder `_` is not allowed within types on item signatures",
    Error {
        line_num: 186,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the type placeholder `_` is not allowed within types on item signatures",
    Error {
        line_num: 225,
        kind: Some(
            Error,
            Error,
        ),
        msg: "the type placeholder `_` is not allowed within types on item signatures",
]


thread '[ui] ui/typeck/typeck_type_placeholder_item.rs#min_tait' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1492:13

failures:
    [ui] ui/box/alloc-unstable-fail.rs
    [ui] ui/const-generics/min_const_generics/inferred_const.rs
---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:27
