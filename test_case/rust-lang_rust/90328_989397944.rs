plain
error: /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:33: expected error not found: can't pass

error: 0 unexpected errors found, 8 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-1.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=i686-pc-windows-msvc" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-1/auxiliary"
    Error {
        line_num: 25,
        kind: Some(
            Error,
---
error: /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:51: expected error not found: intrinsic must be in

error: 0 unexpected errors found, 8 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi/auxiliary"
    Error {
        line_num: 15,
        kind: Some(
            Error,
            Error,
        ),
        msg: "intrinsic must be in",
    Error {
        line_num: 17,
        kind: Some(
            Error,
            Error,
        ),
        msg: "intrinsic must be in",
    Error {
        line_num: 24,
        kind: Some(
            Error,
            Error,
        ),
        msg: "intrinsic must be in",
    Error {
        line_num: 26,
        kind: Some(
            Error,
            Error,
        ),
        msg: "intrinsic must be in",
    Error {
        line_num: 39,
        kind: Some(
            Error,
            Error,
        ),
        msg: "intrinsic must be in",
    Error {
        line_num: 41,
        kind: Some(
            Error,
            Error,
        ),
        msg: "intrinsic must be in",
    Error {
        line_num: 49,
        kind: Some(
            Error,
            Error,
        ),
        msg: "intrinsic must be in",
    Error {
        line_num: 51,
        kind: Some(
            Error,
            Error,
        ),
        msg: "intrinsic must be in",
]

thread '[ui] ui/feature-gates/feature-gate-abi.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1346:13

---
error: /checkout/src/test/ui/feature-gates/feature-gate-abi_amdgpu_kernel.rs:31: expected error not found: is not a supported ABI

error: 0 unexpected errors found, 5 expected errors not found
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi_amdgpu_kernel.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi_amdgpu_kernel" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi_amdgpu_kernel/auxiliary"
    Error {
        line_num: 9,
        kind: Some(
            Error,
            Error,
        ),
        msg: "is not a supported ABI",
    Error {
        line_num: 14,
        kind: Some(
            Error,
            Error,
        ),
        msg: "is not a supported ABI",
    Error {
        line_num: 20,
        kind: Some(
            Error,
            Error,
        ),
        msg: "is not a supported ABI",
    Error {
        line_num: 25,
        kind: Some(
            Error,
            Error,
        ),
        msg: "is not a supported ABI",
    Error {
        line_num: 31,
        kind: Some(
            Error,
            Error,
        ),
        msg: "is not a supported ABI",
]

thread '[ui] ui/feature-gates/feature-gate-abi_amdgpu_kernel.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1346:13


---- [ui] ui/lang-items/lang-item-correct-generics.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lang-items/lang-item-correct-generics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/lang-item-correct-generics" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/lang-item-correct-generics/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: requires `send` lang_item
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/privacy/privacy1.rs stdout ----

error: /checkout/src/test/ui/privacy/privacy1.rs:80: expected error not found: associated function `bar` is private

error: /checkout/src/test/ui/privacy/privacy1.rs:98: expected error not found: associated function `bar` is private

error: /checkout/src/test/ui/privacy/privacy1.rs:105: expected error not found: associated function `bar` is private

error: /checkout/src/test/ui/privacy/privacy1.rs:108: expected error not found: associated function `bar` is private

error: /checkout/src/test/ui/privacy/privacy1.rs:111: expected error not found: associated function `bar2` is private
error: 0 unexpected errors found, 5 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/privacy1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy1/auxiliary"
    Error {
        line_num: 80,
        kind: Some(
            Error,
            Error,
        ),
        msg: "associated function `bar` is private",
    Error {
        line_num: 98,
        kind: Some(
            Error,
            Error,
        ),
        msg: "associated function `bar` is private",
    Error {
        line_num: 105,
        kind: Some(
            Error,
            Error,
        ),
        msg: "associated function `bar` is private",
    Error {
        line_num: 108,
        kind: Some(
            Error,
            Error,
        ),
        msg: "associated function `bar` is private",
    Error {
        line_num: 111,
        kind: Some(
            Error,
            Error,
        ),
        msg: "associated function `bar2` is private",
]

thread '[ui] ui/privacy/privacy1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1346:13

---
test result: FAILED. 12343 passed; 5 failed; 118 ignored; 0 measured; 0 filtered out; finished in 133.46s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:53
