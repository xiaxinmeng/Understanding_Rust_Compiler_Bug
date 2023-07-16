plain
.................................................................................................... 9400/11823
.................................................................................................... 9500/11823
.................................................................................................... 9600/11823
...........................................i......i................................................. 9700/11823
.........................................................................................iiiiiii..ii 9800/11823
.................................................................................................... 10000/11823
.................................................................................................... 10100/11823
.................................................................................................... 10200/11823
.................................................................................................... 10300/11823
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.165 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii...........iiii........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.36s

 finished in 2.428 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
diff of stderr:

14   --> $DIR/unknown-renamed-lints.rs:7:9
15    |
16 LL | #![deny(rustdoc::x)]
-    |         ^^^^^^^^^^
+    |         ^^^^^^^^^^ help: did you mean: `rustdoc::all`
18 
19 error: lint `intra_doc_link_resolution_failure` has been renamed to `rustdoc::broken_intra_doc_links`
20   --> $DIR/unknown-renamed-lints.rs:9:9

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unknown-renamed-lints/unknown-renamed-lints.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unknown-renamed-lints.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/unknown-renamed-lints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unknown-renamed-lints" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unknown-renamed-lints/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown lint: `x`
  --> /checkout/src/test/rustdoc-ui/unknown-renamed-lints.rs:5:9
   |
LL | #![deny(x)]
   |
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/unknown-renamed-lints.rs:1:9
   |
   |
LL | #![deny(unknown_lints)]

error: unknown lint: `rustdoc::x`
  --> /checkout/src/test/rustdoc-ui/unknown-renamed-lints.rs:7:9
   |
   |
LL | #![deny(rustdoc::x)]
   |         ^^^^^^^^^^ help: did you mean: `rustdoc::all`

error: lint `intra_doc_link_resolution_failure` has been renamed to `rustdoc::broken_intra_doc_links`
   |
   |
LL | #![deny(intra_doc_link_resolution_failure)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `rustdoc::broken_intra_doc_links`
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/unknown-renamed-lints.rs:3:9
   |
   |
LL | #![deny(renamed_and_removed_lints)]


error: lint `non_autolinks` has been renamed to `rustdoc::bare_urls`
   |
   |
LL | #![deny(non_autolinks)]
   |         ^^^^^^^^^^^^^ help: use the new name: `rustdoc::bare_urls`

error: lint `rustdoc::non_autolinks` has been renamed to `rustdoc::bare_urls`
   |
   |
LL | #![deny(rustdoc::non_autolinks)]
   |         ^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `rustdoc::bare_urls`

error: lint `rustdoc` has been removed: use `rustdoc::all` instead
   |
   |
LL | #![deny(rustdoc)]

error: unknown lint: `rustdoc::intra_doc_link_resolution_failure`
  --> /checkout/src/test/rustdoc-ui/unknown-renamed-lints.rs:24:9
   |
   |
LL | #![deny(rustdoc::intra_doc_link_resolution_failure)]

error: Compilation failed, aborting rustdoc

error: aborting due to 8 previous errors
---
test result: FAILED. 118 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.60s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:31:01
