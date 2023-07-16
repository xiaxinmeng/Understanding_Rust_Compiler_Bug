plain
failures:

---- [ui] ui-fulldeps/issue-15778-pass.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   |     ^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

error[E0050]: method `check_crate` has 3 parameters but the declaration in trait `rustc_lint::LateLintPass::check_crate` has 2
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:30:28
   |
LL |               fn check_crate(&mut self, cx: &LateContext, krate: &rustc_hir::Crate) {
...
...
LL | / fake_lint_pass! {
LL | |     PassOkay,
LL | |     Symbol::intern("crate_okay")
LL | | }
   |
   |
   = note: `check_crate` from trait: `fn(&mut Self, &LateContext<'tcx>)`
   = note: this error originates in the macro `fake_lint_pass` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0050]: method `check_crate` has 3 parameters but the declaration in trait `rustc_lint::LateLintPass::check_crate` has 2
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:30:28
   |
LL |               fn check_crate(&mut self, cx: &LateContext, krate: &rustc_hir::Crate) {
...
...
LL | / fake_lint_pass! {
LL | |     PassRedBlue,
LL | |     Symbol::intern("crate_red"), Symbol::intern("crate_blue")
LL | | }
   |
   |
   = note: `check_crate` from trait: `fn(&mut Self, &LateContext<'tcx>)`
   = note: this error originates in the macro `fake_lint_pass` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0050]: method `check_crate` has 3 parameters but the declaration in trait `rustc_lint::LateLintPass::check_crate` has 2
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate-rpass.rs:30:28
   |
LL |               fn check_crate(&mut self, cx: &LateContext, krate: &rustc_hir::Crate) {
...
...
LL | / fake_lint_pass! {
LL | |     PassGreyGreen,
LL | |     Symbol::intern("crate_grey"), Symbol::intern("crate_green")
LL | | }
   |
   |
   = note: `check_crate` from trait: `fn(&mut Self, &LateContext<'tcx>)`
   = note: this error originates in the macro `fake_lint_pass` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 3 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0050`.


------------------------------------------


---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary"
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

error[E0050]: method `check_crate` has 3 parameters but the declaration in trait `rustc_lint::LateLintPass::check_crate` has 2
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:29:20
   |
LL |     fn check_crate(&mut self, cx: &LateContext, krate: &rustc_hir::Crate) {
   |
   |
   = note: `check_crate` from trait: `fn(&mut Self, &LateContext<'tcx>)`
error: aborting due to previous error; 3 warnings emitted

For more information about this error, try `rustc --explain E0050`.

---
test result: FAILED. 66 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.96s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui-fulldeps" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:35
