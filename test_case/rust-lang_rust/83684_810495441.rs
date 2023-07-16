plain
.................................................................................................... 9400/11722
.................................................................................................... 9500/11722
.................................................................i.....i............................ 9600/11722
.................................................................................................... 9700/11722
..........iiiiiii..iiiiii..i........................................................................ 9800/11722
.................................................................................................... 10000/11722
.................................................................................................... 10100/11722
.................................................................................................... 10200/11722
.................................................................................................... 10300/11722
---
 finished in 0.395 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.110 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii..........i....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.33s

 finished in 2.391 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 67 tests
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.......................F.F.........................................

---- [ui] ui-fulldeps/issue-15778-pass.rs stdout ----


error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   |     ^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:67:1
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error[E0609]: no field `span` on type `rustc_hir::Mod<'_>`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:36:67
   |
LL |                                lint.build(&msg).set_span(krate.item.span).emit()
...
...
LL | / fake_lint_pass! {
LL | |     PassOkay,
LL | |     Symbol::intern("crate_okay")
LL | | }
   |
   |
   = note: available fields are: `inner`, `item_ids`


error[E0609]: no field `span` on type `rustc_hir::Mod<'_>`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:36:67
   |
LL |                                lint.build(&msg).set_span(krate.item.span).emit()
...
...
LL | / fake_lint_pass! {
LL | |     PassRedBlue,
LL | |     Symbol::intern("crate_red"), Symbol::intern("crate_blue")
LL | | }
   |
   |
   = note: available fields are: `inner`, `item_ids`


error[E0609]: no field `span` on type `rustc_hir::Mod<'_>`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:36:67
   |
LL |                                lint.build(&msg).set_span(krate.item.span).emit()
...
...
LL | / fake_lint_pass! {
LL | |     PassRedBlue,
LL | |     Symbol::intern("crate_red"), Symbol::intern("crate_blue")
LL | | }
   |
   |
   = note: available fields are: `inner`, `item_ids`


error[E0609]: no field `span` on type `rustc_hir::Mod<'_>`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:36:67
   |
LL |                                lint.build(&msg).set_span(krate.item.span).emit()
...
...
LL | / fake_lint_pass! {
LL | |     PassGreyGreen,
LL | |     Symbol::intern("crate_grey"), Symbol::intern("crate_green")
LL | | }
   |
   |
   = note: available fields are: `inner`, `item_ids`


error[E0609]: no field `span` on type `rustc_hir::Mod<'_>`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:36:67
   |
LL |                                lint.build(&msg).set_span(krate.item.span).emit()
...
...
LL | / fake_lint_pass! {
LL | |     PassGreyGreen,
LL | |     Symbol::intern("crate_grey"), Symbol::intern("crate_green")
LL | | }
   |
   |
   = note: available fields are: `inner`, `item_ids`

error: aborting due to 5 previous errors; 2 warnings emitted

For more information about this error, try `rustc --explain E0609`.
For more information about this error, try `rustc --explain E0609`.

------------------------------------------


---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintPass`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:16:45
   |
LL | use rustc_lint::{LateContext, LateLintPass, LintArray, LintContext, LintPass};
   |                                             ^^^^^^^^^               ^^^^^^^^
warning: unused import: `rustc_ast::attr`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:18:5
   |
LL | use rustc_ast::attr;
LL | use rustc_ast::attr;
   |     ^^^^^^^^^^^^^^^

warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:41:1
   |
LL | #[plugin_registrar]
   | ^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default


error[E0609]: no field `span` on type `rustc_hir::Mod<'_>`
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:34:42
   |
LL |                     .set_span(krate.item.span)
   |
   |
   = note: available fields are: `inner`, `item_ids`
error: aborting due to previous error; 4 warnings emitted

For more information about this error, try `rustc --explain E0609`.

---
test result: FAILED. 65 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 8.72s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui-fulldeps" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:33
