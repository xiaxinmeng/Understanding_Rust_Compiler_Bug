
   |
   = note: error from rustc: unterminated double quote string
help: mark blocks that do not contain Rust code as text
   |
---


---- [ui] rustdoc-ui/issue-58473-2.rs stdout ----
normalized stdout:
time: 0.028; rss: 279MB\tcreate_renderer
time: 0.000; rss: 279MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-58473-2/issue-58473-2.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issue-58473-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/issue-58473-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-58473-2" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-58473-2/auxiliary"
------------------------------------------
------------------------------------------
time: 0.028; rss: 279MB create_renderer
time: 0.000; rss: 279MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] rustdoc-ui/issue-58473.rs stdout ----
normalized stdout:
time: 0.028; rss: 279MB\tcreate_renderer
time: 0.000; rss: 279MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-58473/issue-58473.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-58473/issue-58473.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issue-58473.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/issue-58473.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-58473" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-58473/auxiliary"
------------------------------------------
------------------------------------------
time: 0.028; rss: 279MB create_renderer
time: 0.000; rss: 279MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] rustdoc-ui/issue-74134.rs#public stdout ----
normalized stdout:
time: 0.015; rss: 278MB\tcreate_renderer
time: 0.000; rss: 278MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-74134.public/issue-74134.public.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issue-74134.rs`

error in revision `public`: 1 errors occurred comparing output.
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/issue-74134.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "public" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-74134.public" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-74134.public/auxiliary"
------------------------------------------
------------------------------------------
time: 0.015; rss: 278MB create_renderer
time: 0.000; rss: 278MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
warning: public documentation for `public_item` links to private item `PrivateType`
   |
   |
LL |     /// [`PrivateType`]
   |
   |
   = note: `#[warn(private_intra_doc_links)]` on by default
   = note: this link will resolve properly if you pass `--document-private-items`
warning: 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] rustdoc-ui/issue-74134.rs#private stdout ----
normalized stdout:
time: 0.015; rss: 279MB\tcreate_renderer
time: 0.000; rss: 279MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-74134.private/issue-74134.private.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issue-74134.rs`

error in revision `private`: 1 errors occurred comparing output.
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/issue-74134.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "private" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-74134.private" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--document-private-items" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-74134.private/auxiliary"
------------------------------------------
------------------------------------------
time: 0.015; rss: 279MB create_renderer
time: 0.000; rss: 279MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
warning: public documentation for `public_item` links to private item `PrivateType`
   |
   |
LL |     /// [`PrivateType`]
   |
   |
   = note: `#[warn(private_intra_doc_links)]` on by default
   = note: this link resolves only because you passed `--document-private-items`, but will break without
warning: 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] rustdoc-ui/private-doc-test.rs stdout ----
normalized stdout:
time: 0.014; rss: 278MB\tcreate_renderer
time: 0.000; rss: 278MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/private-doc-test/private-doc-test.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/private-doc-test/private-doc-test.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args private-doc-test.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/private-doc-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/private-doc-test" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/private-doc-test/auxiliary"
------------------------------------------
------------------------------------------
time: 0.014; rss: 278MB create_renderer
time: 0.000; rss: 278MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] rustdoc-ui/range-pattern.rs stdout ----
normalized stdout:
time: 0.014; rss: 279MB\tcreate_renderer
time: 0.000; rss: 279MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/range-pattern/range-pattern.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args range-pattern.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/range-pattern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/range-pattern" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/range-pattern/auxiliary"
------------------------------------------
------------------------------------------
time: 0.014; rss: 279MB create_renderer
time: 0.000; rss: 279MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] rustdoc-ui/unused-braces-lint.rs stdout ----
normalized stdout:
time: 0.014; rss: 279MB\tcreate_renderer
time: 0.000; rss: 279MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unused-braces-lint/unused-braces-lint.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unused-braces-lint/unused-braces-lint.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused-braces-lint.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/unused-braces-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unused-braces-lint" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unused-braces-lint/auxiliary"
------------------------------------------
------------------------------------------
time: 0.014; rss: 279MB create_renderer
time: 0.000; rss: 279MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------


------------------------------------------


---- [ui] rustdoc-ui/unused.rs stdout ----
normalized stdout:
time: 0.014; rss: 279MB\tcreate_renderer
time: 0.000; rss: 279MB\trenderer_after_krate


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unused/unused.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unused/unused.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/unused.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unused" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unused/auxiliary"
------------------------------------------
------------------------------------------
time: 0.014; rss: 279MB create_renderer
time: 0.000; rss: 279MB renderer_after_krate
------------------------------------------
stderr:
------------------------------------------

---
test result: FAILED. 70 passed; 27 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.31s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:33:20
