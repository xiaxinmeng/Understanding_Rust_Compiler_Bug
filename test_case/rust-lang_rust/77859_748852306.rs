plain
.................................................................................................... 9000/11194
.................................................................................................... 9100/11194
....................................................................................i......i........ 9200/11194
.................................................................................................... 9300/11194
.......................iiiiii..iiiiii.i............................................................. 9400/11194
.................................................................................................... 9600/11194
.................................................................................................... 9700/11194
..............................................................test [ui] ui/issues/issue-74564-if-expr-stack-overflow.rs has been running for over 60 seconds
...................................... 9800/11194
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.074 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.48s

 finished in 2.548 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Some tests failed in compiletest suite=rustdoc-ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
............................................................i..................F......
failures:

---- [ui] rustdoc-ui/reference-link-reports-error-once.rs stdout ----

1 error: unresolved link to `ref`
-   --> $DIR/reference-link-reports-error-once.rs:12:10
+   --> $DIR/reference-link-reports-error-once.rs:3:14
+   --> $DIR/reference-link-reports-error-once.rs:3:14
3    |
- LL | /// [a]: ref
-    |          ^^^ no item named `ref` in scope
+ LL | /// Links to [a] [link][a]
+    |              ^^^ no item named `ref` in scope
7 note: the lint level is defined here
8   --> $DIR/reference-link-reports-error-once.rs:1:9



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/reference-link-reports-error-once/reference-link-reports-error-once.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args reference-link-reports-error-once.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/reference-link-reports-error-once.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/reference-link-reports-error-once" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/reference-link-reports-error-once/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unresolved link to `ref`
  --> /checkout/src/test/rustdoc-ui/reference-link-reports-error-once.rs:3:14
   |
LL | /// Links to [a] [link][a]
   |              ^^^ no item named `ref` in scope
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/reference-link-reports-error-once.rs:1:9
   |
   |
LL | #![deny(broken_intra_doc_links)]
   |         ^^^^^^^^^^^^^^^^^^^^^^
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `ref`
  --> /checkout/src/test/rustdoc-ui/reference-link-reports-error-once.rs:6:62
   |
   |
LL | /// Other links to the same target should still emit error: [ref] //~ERROR unresolved link to
   |                                                              ^^^ no item named `ref` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `ref`
  --> /checkout/src/test/rustdoc-ui/reference-link-reports-error-once.rs:7:16
   |
   |
LL | /// Duplicate [ref] //~ERROR unresolved link to
   |                ^^^ no item named `ref` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `ref2`
  --> /checkout/src/test/rustdoc-ui/reference-link-reports-error-once.rs:9:60
   |
   |
LL | /// Other links to other targets should still emit error: [ref2] //~ERROR unresolved link to
   |                                                            ^^^^ no item named `ref2` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `ref2`
  --> /checkout/src/test/rustdoc-ui/reference-link-reports-error-once.rs:10:16
   |
   |
LL | /// Duplicate [ref2] //~ERROR unresolved link to
   |                ^^^^ no item named `ref2` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `ref`
  --> /checkout/src/test/rustdoc-ui/reference-link-reports-error-once.rs:15:6
   |
   |
LL | /// [ref][]
   |      ^^^ no item named `ref` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: aborting due to 6 previous errors


------------------------------------------
---
test result: FAILED. 84 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 6.56s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:35:52
