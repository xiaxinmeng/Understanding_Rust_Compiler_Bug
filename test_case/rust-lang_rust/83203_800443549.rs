plain
.................................................................................................... 9300/11680
.................................................................................................... 9400/11680
.................................................................................................... 9500/11680
.......................i......i..................................................................... 9600/11680
.....................................................................iiiiiii..iiiiii.i.............. 9700/11680
.................................................................................................... 9900/11680
.................................................................................................... 10000/11680
.................................................................................................... 10100/11680
.................................................................................................... 10200/11680
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.086 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.40s

 finished in 2.473 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
diff of stderr:

29    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
30 
31 error: lint `rustdoc` has been removed: use `rustdoc::all` instead
-   --> $DIR/unknown-renamed-lints.rs:15:9
+   --> $DIR/unknown-renamed-lints.rs:16:9
33    |
34 LL | #![deny(rustdoc)]

36 
37 error: unknown lint: `rustdoc::intra_doc_link_resolution_failure`
-   --> $DIR/unknown-renamed-lints.rs:19:9
-   --> $DIR/unknown-renamed-lints.rs:19:9
+   --> $DIR/unknown-renamed-lints.rs:20:9
39    |
40 LL | #![deny(rustdoc::intra_doc_link_resolution_failure)]


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unknown-renamed-lints/unknown-renamed-lints.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unknown-renamed-lints/unknown-renamed-lints.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unknown-renamed-lints.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/unknown-renamed-lints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unknown-renamed-lints" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unknown-renamed-lints/auxiliary"
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
   |
   |
LL | #![deny(rustdoc::x)]


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


error: lint `rustdoc` has been removed: use `rustdoc::all` instead
   |
   |
LL | #![deny(rustdoc)]

error: unknown lint: `rustdoc::intra_doc_link_resolution_failure`
  --> /checkout/src/test/rustdoc-ui/unknown-renamed-lints.rs:20:9
   |
   |
LL | #![deny(rustdoc::intra_doc_link_resolution_failure)]

error: Compilation failed, aborting rustdoc

error: aborting due to 6 previous errors
---
1 error: unresolved link to `x`
-   --> $DIR/renamed-lint-still-applies.rs:4:6
+   --> $DIR/renamed-lint-still-applies.rs:5:6
3    |
4 LL | //! [x]
5    |      ^ no item named `x` in scope

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/renamed-lint-still-applies/renamed-lint-still-applies.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/renamed-lint-still-applies/renamed-lint-still-applies.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args renamed-lint-still-applies.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/renamed-lint-still-applies.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/renamed-lint-still-applies" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/renamed-lint-still-applies/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unresolved link to `x`
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
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 107 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.69s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:30:06
