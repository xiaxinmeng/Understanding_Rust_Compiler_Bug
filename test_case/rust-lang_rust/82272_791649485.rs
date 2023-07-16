plain
.................................................................................................... 9300/11532
.................................................................................................... 9400/11532
.................................................................................i......i........... 9500/11532
.................................................................................................... 9600/11532
....................iiiiiii..iiiiii.i............................................................... 9700/11532
.................................................................................................... 9900/11532
.................................................................................................... 10000/11532
.................................................................................................... 10100/11532
.................................................................................................... 10200/11532
---

---- [ui] ui/generic-associated-types/issue-81712-cyclic-traits.rs stdout ----
diff of stderr:

1 error[E0107]: missing generics for associated type `C::DType`
-   --> $DIR/issue-81712-cyclic-traits.rs:18:19
3    |
3    |
4 LL |     type CType: C<DType = Self>;
5    |                   ^^^^^^^^^^^^ expected 1 generic argument

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-81712-cyclic-traits/issue-81712-cyclic-traits.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-81712-cyclic-traits/issue-81712-cyclic-traits.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-81712-cyclic-traits.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-81712-cyclic-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-81712-cyclic-traits" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-81712-cyclic-traits/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0107]: missing generics for associated type `C::DType`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL |     type CType: C<DType = Self>;
   |                   ^^^^^^^^^^^^ expected 1 generic argument
   |
note: associated type defined here, with  1 generic parameter : `T`
   |
   |
LL |     type DType<T>: D<T, CType = Self>;
help: add missing generic argument
   |
   |
LL |     type CType: C<DType<T>>;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0107`.
For more information about this error, try `rustc --explain E0107`.

------------------------------------------


---- [ui] ui/suggestions/missing-lifetime-specifier.rs stdout ----

error: /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:18: unexpected error: '18:44: 18:47: missing lifetime specifiers [E0106]'

error: /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:18: unexpected error: '18:44: 18:47: missing lifetime specifiers [E0106]'

error: /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:23: unexpected error: '23:44: 23:45: missing lifetime specifier [E0106]'

error: /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:23: unexpected error: '23:45: 23:48: missing lifetime specifiers [E0106]'

error: /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:23: unexpected error: '23:44: 23:45: missing lifetime specifier [E0106]'

error: /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:23: unexpected error: '23:45: 23:48: missing lifetime specifiers [E0106]'

error: /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:30: unexpected error: '30:48: 30:48: missing lifetime specifiers [E0106]'

error: /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:30: unexpected error: '30:48: 30:48: missing lifetime specifiers [E0106]'

error: /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:35: unexpected error: '35:44: 35:45: missing lifetime specifier [E0106]'

error: /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:35: unexpected error: '35:49: 35:49: missing lifetime specifiers [E0106]'

error: /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:35: unexpected error: '35:44: 35:45: missing lifetime specifier [E0106]'

error: /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:35: unexpected error: '35:49: 35:49: missing lifetime specifiers [E0106]'

error: /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:50: unexpected error: '50:44: 50:45: missing lifetime specifier [E0106]'

error: /checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs:50: unexpected error: '50:44: 50:45: missing lifetime specifier [E0106]'
error: 14 unexpected errors found, 0 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/missing-lifetime-specifier.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-lifetime-specifier" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-lifetime-specifier/auxiliary"
    Error {
        line_num: 18,
        kind: Some(
            Error,
            Error,
        ),
        msg: "18:44: 18:47: missing lifetime specifiers [E0106]",
    Error {
        line_num: 18,
        kind: Some(
            Error,
            Error,
        ),
        msg: "18:44: 18:47: missing lifetime specifiers [E0106]",
    Error {
        line_num: 23,
        kind: Some(
            Error,
            Error,
        ),
        msg: "23:44: 23:45: missing lifetime specifier [E0106]",
    Error {
        line_num: 23,
        kind: Some(
            Error,
            Error,
        ),
        msg: "23:45: 23:48: missing lifetime specifiers [E0106]",
    Error {
        line_num: 23,
        kind: Some(
            Error,
            Error,
        ),
        msg: "23:44: 23:45: missing lifetime specifier [E0106]",
    Error {
        line_num: 23,
        kind: Some(
            Error,
            Error,
        ),
        msg: "23:45: 23:48: missing lifetime specifiers [E0106]",
    Error {
        line_num: 30,
        kind: Some(
            Error,
            Error,
        ),
        msg: "30:48: 30:48: missing lifetime specifiers [E0106]",
    Error {
        line_num: 30,
        kind: Some(
            Error,
            Error,
        ),
        msg: "30:48: 30:48: missing lifetime specifiers [E0106]",
    Error {
        line_num: 35,
        kind: Some(
            Error,
            Error,
        ),
        msg: "35:44: 35:45: missing lifetime specifier [E0106]",
    Error {
        line_num: 35,
        kind: Some(
            Error,
            Error,
        ),
        msg: "35:49: 35:49: missing lifetime specifiers [E0106]",
    Error {
        line_num: 35,
        kind: Some(
            Error,
            Error,
        ),
        msg: "35:44: 35:45: missing lifetime specifier [E0106]",
    Error {
        line_num: 35,
        kind: Some(
            Error,
            Error,
        ),
        msg: "35:49: 35:49: missing lifetime specifiers [E0106]",
    Error {
        line_num: 50,
        kind: Some(
            Error,
            Error,
        ),
        msg: "50:44: 50:45: missing lifetime specifier [E0106]",
    Error {
        line_num: 50,
        kind: Some(
            Error,
            Error,
        ),
        msg: "50:44: 50:45: missing lifetime specifier [E0106]",
]

thread '[ui] ui/suggestions/missing-lifetime-specifier.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1492:13

---
test result: FAILED. 11437 passed; 2 failed; 93 ignored; 0 measured; 0 filtered out; finished in 132.44s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:20
