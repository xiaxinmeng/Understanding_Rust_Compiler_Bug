plain
...................................................iii.............................................. 12500/12556
........................................................
failures:

---- [ui] ui/numeric/uppercase-base-prefix.rs stdout ----

error: /checkout/src/test/ui/numeric/uppercase-base-prefix.rs:4: unexpected note: 'invalid suffix `XABCDEF`'

error: /checkout/src/test/ui/numeric/uppercase-base-prefix.rs:10: unexpected note: 'invalid suffix `O755`'

error: /checkout/src/test/ui/numeric/uppercase-base-prefix.rs:16: unexpected note: 'invalid suffix `B10101010`'

error: /checkout/src/test/ui/numeric/uppercase-base-prefix.rs:22: unexpected note: 'invalid suffix `XABCDEF_u64`'

error: /checkout/src/test/ui/numeric/uppercase-base-prefix.rs:28: unexpected note: 'invalid suffix `O755_u32`'

error: /checkout/src/test/ui/numeric/uppercase-base-prefix.rs:34: unexpected note: 'invalid suffix `B10101010_u8`'
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: 6 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/numeric/uppercase-base-prefix.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numeric/uppercase-base-prefix" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numeric/uppercase-base-prefix/auxiliary"
    Error {
        line_num: 4,
        kind: Some(
            Note,
            Note,
        ),
        msg: "invalid suffix `XABCDEF`",
    Error {
        line_num: 10,
        kind: Some(
            Note,
            Note,
        ),
        msg: "invalid suffix `O755`",
    Error {
        line_num: 16,
        kind: Some(
            Note,
            Note,
        ),
        msg: "invalid suffix `B10101010`",
    Error {
        line_num: 22,
        kind: Some(
            Note,
            Note,
        ),
        msg: "invalid suffix `XABCDEF_u64`",
    Error {
        line_num: 28,
        kind: Some(
            Note,
            Note,
        ),
        msg: "invalid suffix `O755_u32`",
    Error {
        line_num: 34,
        kind: Some(
            Note,
            Note,
        ),
        msg: "invalid suffix `B10101010_u8`",
]


thread '[ui] ui/numeric/uppercase-base-prefix.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1358:13

failures:
    [ui] ui/numeric/uppercase-base-prefix.rs


test result: FAILED. 12434 passed; 1 failed; 121 ignored; 0 measured; 0 filtered out; finished in 118.05s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:38
