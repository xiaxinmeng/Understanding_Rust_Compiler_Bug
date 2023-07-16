plain
.................................................................................................... 7800/11525
.......i........i................................................................................... 7900/11525
i.............................................................................................i..... 8000/11525
.................................................................................................... 8100/11525
.............................................................................................F.F.... 8200/11525
.................................................................................................... 8400/11525
.................................................................................................... 8500/11525
.................................................................................................... 8600/11525
.................................................................................................... 8700/11525
---
.................................................................................................... 9300/11525
.................................................................................................... 9400/11525
.........................................................................i......i................... 9500/11525
.................................................................................................... 9600/11525
............iiiiiii..iiiiii.i....................................................................... 9700/11525
.................................................................................................... 9900/11525
.................................................................................................... 10000/11525
.................................................................................................... 10100/11525
.................................................................................................... 10200/11525
---
failures:

---- [ui] ui/parser/item-free-const-no-body-semantic-fail.rs stdout ----

error: /checkout/src/test/ui/parser/item-free-const-no-body-semantic-fail.rs:6: unexpected error: '6:1: 6:9: free constant item without body'
error: /checkout/src/test/ui/parser/item-free-const-no-body-semantic-fail.rs:6: expected error not found: missing type for `const` item

error: 1 unexpected errors found, 1 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/item-free-const-no-body-semantic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/item-free-const-no-body-semantic-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/item-free-const-no-body-semantic-fail/auxiliary"
    Error {
        line_num: 6,
        kind: Some(
            Error,
            Error,
        ),
        msg: "6:1: 6:9: free constant item without body",
]

not found errors (from test file): [
    Error {
    Error {
        line_num: 6,
        kind: Some(
            Error,
        ),
        msg: "missing type for `const` item",
]

thread '[ui] ui/parser/item-free-const-no-body-semantic-fail.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1492:13


Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---- [ui] ui/parser/item-free-static-no-body-semantic-fail.rs stdout ----
diff of stderr:

15    |         help: provide a definition for the static: `= <expr>;`
16 
17 error: free static item without body
-   --> $DIR/item-free-static-no-body-semantic-fail.rs:9:1
19    |
19    |
20 LL | static mut C: u8;


23    |                 help: provide a definition for the static: `= <expr>;`
24 
25 error: free static item without body
-   --> $DIR/item-free-static-no-body-semantic-fail.rs:10:1
27    |
28 LL | static mut D;
29    | ^^^^^^^^^^^^-



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/item-free-static-no-body-semantic-fail/item-free-static-no-body-semantic-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/item-free-static-no-body-semantic-fail.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/item-free-static-no-body-semantic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/item-free-static-no-body-semantic-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/item-free-static-no-body-semantic-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: free static item without body
   |
   |
LL | static A: u8; //~ ERROR free static item without body
   |             |
   |             |
   |             help: provide a definition for the static: `= <expr>;`

error: free static item without body
   |
   |
LL | static B; //~ ERROR missing type for `static` item
   |         |
   |         |
   |         help: provide a definition for the static: `= <expr>;`

error: free static item without body
   |
   |
LL | static mut C: u8; //~ ERROR free static item without body
   |                 |
   |                 |
   |                 help: provide a definition for the static: `= <expr>;`

error: free static item without body
   |
   |
LL | static mut D; //~ ERROR missing type for `static mut` item
   |             |
   |             |
   |             help: provide a definition for the static: `= <expr>;`
error: aborting due to 4 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/typeck/issue-79040.rs stdout ----
diff of stderr:

1 error[E0369]: cannot add `{integer}` to `&str`
-   --> $DIR/79040.rs:2:25
3    |
3    |
4 LL |     const FOO = "hello" + 1;
5    |                 ------- ^ - {integer}
7    |                 &str
8 
8 
9 error[E0369]: cannot add `{integer}` to `&str`
-   --> $DIR/79040.rs:2:25
11    |
11    |
12 LL |     const FOO = "hello" + 1;
13    |                 ------- ^ - {integer}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-79040/issue-79040.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-79040/issue-79040.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/issue-79040.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-79040.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-79040" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-79040/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0369]: cannot add `{integer}` to `&str`
   |
   |
LL |     const FOO = "hello" + 1;
   |                 ------- ^ - {integer}
   |                 &str


error[E0369]: cannot add `{integer}` to `&str`
   |
   |
LL |     const FOO = "hello" + 1;
   |                 ------- ^ - {integer}
   |                 &str

error: aborting due to 2 previous errors

---
test result: FAILED. 11429 passed; 3 failed; 93 ignored; 0 measured; 0 filtered out; finished in 137.75s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:37
