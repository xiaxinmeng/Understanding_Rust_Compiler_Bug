plain
.................................................................................................... 9500/11842
.................................................................................................... 9600/11842
......................................................i......i...................................... 9700/11842
.................................................................................................... 9800/11842
iiiiiii..iiiiii.i................................................................................... 9900/11842
.................................................................................................... 10100/11842
.................................................................................................... 10200/11842
.................................................................................................... 10300/11842
.................................................................................................... 10400/11842
.................................................................................................... 10400/11842
....................................................F.F............................................. 10500/11842
...................i................................................................................ 10700/11842
.................................................................................................... 10800/11842
.................................................................................................... 10900/11842
.................................................................................................... 11000/11842
---
failures:

---- [ui] ui/suggestions/issue-68049-1.rs stdout ----

error: /checkout/src/test/ui/suggestions/issue-68049-1.rs:7: unexpected error: '7:9: 7:20: cannot assign to `self.0` which is behind a `&` reference [E0594]'
error: 1 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-68049-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-68049-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-68049-1/auxiliary"
    Error {
        line_num: 7,
        kind: Some(
            Error,
            Error,
        ),
        msg: "7:9: 7:20: cannot assign to `self.0` which is behind a `&` reference [E0594]",
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
]

thread '[ui] ui/suggestions/issue-68049-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1524:13
thread '[ui] ui/suggestions/issue-68049-1.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1524:13

---- [ui] ui/suggestions/issue-68049-2.rs stdout ----

error: /checkout/src/test/ui/suggestions/issue-68049-2.rs:9: unexpected error: '9:7: 9:22: cannot assign to `*input` which is behind a `&` reference [E0594]'

error: /checkout/src/test/ui/suggestions/issue-68049-2.rs:17: unexpected error: '17:5: 17:21: cannot assign to `self.0` which is behind a `&` reference [E0594]'
error: 2 unexpected errors found, 0 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-68049-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-68049-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-68049-2/auxiliary"
    Error {
        line_num: 9,
        kind: Some(
            Error,
            Error,
        ),
        msg: "9:7: 9:22: cannot assign to `*input` which is behind a `&` reference [E0594]",
    Error {
        line_num: 17,
        kind: Some(
            Error,
            Error,
        ),
        msg: "17:5: 17:21: cannot assign to `self.0` which is behind a `&` reference [E0594]",
]

thread '[ui] ui/suggestions/issue-68049-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1524:13

---
test result: FAILED. 11744 passed; 2 failed; 96 ignored; 0 measured; 0 filtered out; finished in 121.46s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:54
