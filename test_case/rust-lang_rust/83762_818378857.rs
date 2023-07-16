plain
.................................................................................................... 9400/11754
.................................................................................................... 9500/11754
........................................................................................i......i.... 9600/11754
.................................................................................................... 9700/11754
..................................iiiiiii..iiiiii.i................................................. 9800/11754
.................................................................................................... 10000/11754
.................................................................................................... 10100/11754
.................................................................................................... 10200/11754
.................................................................................................... 10300/11754
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.10s

 finished in 0.160 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii...........iiii........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.33s

 finished in 2.393 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
---- [ui] rustdoc-ui/renamed-lint-still-applies.rs stdout ----
diff of stderr:

31    |
32 LL | #![deny(rustdoc::non_autolinks)]
33    |         ^^^^^^^^^^^^^^^^^^^^^^
+    = note: bare URLs are not automatically turned into clickable links
35 error: aborting due to 2 previous errors; 1 warning emitted
36 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/renamed-lint-still-applies/renamed-lint-still-applies.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args renamed-lint-still-applies.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/renamed-lint-still-applies.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/renamed-lint-still-applies" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/renamed-lint-still-applies/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: lint `rustdoc::non_autolinks` has been renamed to `rustdoc::bare_urls`
   |
   |
LL | #![deny(rustdoc::non_autolinks)]
   |         ^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `rustdoc::bare_urls`
   = note: `#[warn(renamed_and_removed_lints)]` on by default

error: unresolved link to `x`
  --> /checkout/src/test/rustdoc-ui/renamed-lint-still-applies.rs:5:6
  --> /checkout/src/test/rustdoc-ui/renamed-lint-still-applies.rs:5:6
   |
LL | //! [x]
   |      ^ no item named `x` in scope
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/renamed-lint-still-applies.rs:2:9
   |
   |
LL | #![deny(broken_intra_doc_links)]
   |         ^^^^^^^^^^^^^^^^^^^^^^
   = note: `#[deny(rustdoc::broken_intra_doc_links)]` implied by `#[deny(broken_intra_doc_links)]`
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: this URL is not a hyperlink
   |
   |
LL | //! http://example.com
   |     ^^^^^^^^^^^^^^^^^^ help: use an automatic link instead: `<http://example.com>`
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/renamed-lint-still-applies.rs:8:9
   |
   |
LL | #![deny(rustdoc::non_autolinks)]
   |         ^^^^^^^^^^^^^^^^^^^^^^
   = note: bare URLs are not automatically turned into clickable links
error: aborting due to 2 previous errors; 1 warning emitted


------------------------------------------
---
test result: FAILED. 116 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.62s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:28:58
