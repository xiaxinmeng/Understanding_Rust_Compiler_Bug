plain
.................................................................................................... 9300/11542
.................................................................................................... 9400/11542
.......................................................................i......i..................... 9500/11542
.................................................................................................... 9600/11542
.................iiiiiii..iiiiii.i.................................................................. 9700/11542
.................................................................................................... 9900/11542
.................................................................................................... 10000/11542
.................................................................................................... 10100/11542
.................................................................................................... 10200/11542
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.066 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i...i.ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.38s

 finished in 2.450 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---

---- [ui] rustdoc-ui/check-doc-alias-attr.rs stdout ----
diff of stderr:

- error: doc alias attribute expects a string `#[doc(alias = "a")]` or a list of strings `#[doc(alias("a", "b")]`
+ error: doc alias attribute expects a string `#[doc(alias = "a")]` or a list of strings `#[doc(alias("a", "b"))]`
3    |
3    |
4 LL | #[doc(alias)]
5    |       ^^^^^
6 
6 
- error: doc alias attribute expects a string `#[doc(alias = "a")]` or a list of strings `#[doc(alias("a", "b")]`
+ error: doc alias attribute expects a string `#[doc(alias = "a")]` or a list of strings `#[doc(alias("a", "b"))]`
9    |
9    |
10 LL | #[doc(alias = 0)]

54 LL | #[doc(alias = "")]
56 
56 
- error: `#[doc(alias("a")]` expects string literals
+ error: `#[doc(alias("a"))]` expects string literals
59    |
59    |
60 LL | #[doc(alias(0))]

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-doc-alias-attr/check-doc-alias-attr.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-doc-alias-attr/check-doc-alias-attr.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args check-doc-alias-attr.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/check-doc-alias-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-doc-alias-attr" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/check-doc-alias-attr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: doc alias attribute expects a string `#[doc(alias = "a")]` or a list of strings `#[doc(alias("a", "b"))]`
   |
   |
LL | #[doc(alias)] //~ ERROR


error: doc alias attribute expects a string `#[doc(alias = "a")]` or a list of strings `#[doc(alias("a", "b"))]`
   |
   |
LL | #[doc(alias = 0)] //~ ERROR


error: '\"' character isn't allowed in `#[doc(alias = "...")]`
   |
   |
LL | #[doc(alias = "\"")] //~ ERROR


error: '\n' character isn't allowed in `#[doc(alias = "...")]`
   |
   |
LL | #[doc(alias = "\n")] //~ ERROR


error: '\n' character isn't allowed in `#[doc(alias = "...")]`
   |
   |
LL |   #[doc(alias = "
   |  _______________^
LL | | ")] //~^ ERROR


error: '\t' character isn't allowed in `#[doc(alias = "...")]`
   |
   |
LL | #[doc(alias = "\t")] //~ ERROR


error: `#[doc(alias = "...")]` cannot start or end with ' '
   |
   |
LL | #[doc(alias = " hello")] //~ ERROR


error: `#[doc(alias = "...")]` cannot start or end with ' '
   |
   |
LL | #[doc(alias = "hello ")] //~ ERROR


error: `#[doc(alias = "...")]` attribute cannot have empty value
   |
   |
LL | #[doc(alias = "")] //~ ERROR


error: `#[doc(alias("a"))]` expects string literals
   |
   |
LL | #[doc(alias(0))] //~ ERROR


error: '\"' character isn't allowed in `#[doc(alias("..."))]`
   |
   |
LL | #[doc(alias("\""))] //~ ERROR


error: '\n' character isn't allowed in `#[doc(alias("..."))]`
   |
   |
LL | #[doc(alias("\n"))] //~ ERROR


error: '\n' character isn't allowed in `#[doc(alias("..."))]`
   |
   |
LL |   #[doc(alias("
   |  _____________^
LL | | "))] //~^ ERROR


error: '\t' character isn't allowed in `#[doc(alias("..."))]`
   |
   |
LL | #[doc(alias("\t"))] //~ ERROR


error: `#[doc(alias("..."))]` cannot start or end with ' '
   |
   |
LL | #[doc(alias(" hello"))] //~ ERROR


error: `#[doc(alias("..."))]` cannot start or end with ' '
   |
   |
LL | #[doc(alias("hello "))] //~ ERROR


error: `#[doc(alias("..."))]` attribute cannot have empty value
   |
   |
LL | #[doc(alias(""))] //~ ERROR

error: aborting due to 17 previous errors


---
test result: FAILED. 108 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.99s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:34:35
