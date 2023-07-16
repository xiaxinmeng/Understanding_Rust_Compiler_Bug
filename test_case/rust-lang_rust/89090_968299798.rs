plain

---- [ui] ui/dyn-keyword/dyn-2018-edition-lint.rs stdout ----
diff of stderr:

31 LL + fn function(x: &SomeTrait, y: Box<dyn SomeTrait>) {
33 
- error: aborting due to 2 previous errors
+ error: trait objects without an explicit `dyn` are deprecated
+   --> $DIR/dyn-2018-edition-lint.rs:9:14
+   --> $DIR/dyn-2018-edition-lint.rs:9:14
+    |
+ LL |     let _x: &SomeTrait = todo!();
+    |
+    = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
+    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
+ help: use `dyn`
+ help: use `dyn`
+    |
+ LL -     let _x: &SomeTrait = todo!();
+ LL +     let _x: &dyn SomeTrait = todo!();
+ 
+ error: trait objects without an explicit `dyn` are deprecated
+   --> $DIR/dyn-2018-edition-lint.rs:4:17
+    |
+    |
+ LL | fn function(x: &SomeTrait, y: Box<SomeTrait>) {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |
+    = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
+    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
+ help: use `dyn`
+ help: use `dyn`
+    |
+ LL - fn function(x: &SomeTrait, y: Box<SomeTrait>) {
+ LL + fn function(x: &dyn SomeTrait, y: Box<SomeTrait>) {
+ 
+ error: trait objects without an explicit `dyn` are deprecated
+   --> $DIR/dyn-2018-edition-lint.rs:4:17
+    |
+    |
+ LL | fn function(x: &SomeTrait, y: Box<SomeTrait>) {
+    |
+    = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
+    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
+ help: use `dyn`
+ help: use `dyn`
+    |
+ LL - fn function(x: &SomeTrait, y: Box<SomeTrait>) {
+ LL + fn function(x: &dyn SomeTrait, y: Box<SomeTrait>) {
+ 
+ error: trait objects without an explicit `dyn` are deprecated
+   --> $DIR/dyn-2018-edition-lint.rs:4:35
+    |
+    |
+ LL | fn function(x: &SomeTrait, y: Box<SomeTrait>) {
+    |
+    = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
+    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
+ help: use `dyn`
+ help: use `dyn`
+    |
+ LL - fn function(x: &SomeTrait, y: Box<SomeTrait>) {
+ LL + fn function(x: &SomeTrait, y: Box<dyn SomeTrait>) {
+ 
+ error: trait objects without an explicit `dyn` are deprecated
+   --> $DIR/dyn-2018-edition-lint.rs:4:35
+    |
+    |
+ LL | fn function(x: &SomeTrait, y: Box<SomeTrait>) {
+    |
+    = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
+    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
+ help: use `dyn`
+ help: use `dyn`
+    |
+ LL - fn function(x: &SomeTrait, y: Box<SomeTrait>) {
+ LL + fn function(x: &SomeTrait, y: Box<dyn SomeTrait>) {
+ 
+ error: aborting due to 7 previous errors
35 
36 
---
To only update this specific test, also pass `--test-args dyn-keyword/dyn-2018-edition-lint.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-keyword/dyn-2018-edition-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-keyword/dyn-2018-edition-lint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-keyword/dyn-2018-edition-lint/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/dyn-keyword/dyn-2018-edition-lint.rs:4:17
   |
LL | fn function(x: &SomeTrait, y: Box<SomeTrait>) {
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/dyn-keyword/dyn-2018-edition-lint.rs:2:8
   |
   |
LL | #[deny(bare_trait_objects)]
   |        ^^^^^^^^^^^^^^^^^^
   = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
   |
LL - fn function(x: &SomeTrait, y: Box<SomeTrait>) {
LL + fn function(x: &dyn SomeTrait, y: Box<SomeTrait>) {

error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/dyn-keyword/dyn-2018-edition-lint.rs:4:35
   |
   |
LL | fn function(x: &SomeTrait, y: Box<SomeTrait>) {
   |
   = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL - fn function(x: &SomeTrait, y: Box<SomeTrait>) {
LL + fn function(x: &SomeTrait, y: Box<dyn SomeTrait>) {

error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/dyn-keyword/dyn-2018-edition-lint.rs:9:14
   |
   |
LL |     let _x: &SomeTrait = todo!();
   |
   = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     let _x: &SomeTrait = todo!();
LL +     let _x: &dyn SomeTrait = todo!();

error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/dyn-keyword/dyn-2018-edition-lint.rs:4:17
   |
   |
LL | fn function(x: &SomeTrait, y: Box<SomeTrait>) {
   |
   = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL - fn function(x: &SomeTrait, y: Box<SomeTrait>) {
LL + fn function(x: &dyn SomeTrait, y: Box<SomeTrait>) {

error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/dyn-keyword/dyn-2018-edition-lint.rs:4:17
   |
   |
LL | fn function(x: &SomeTrait, y: Box<SomeTrait>) {
   |
   = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL - fn function(x: &SomeTrait, y: Box<SomeTrait>) {
LL + fn function(x: &dyn SomeTrait, y: Box<SomeTrait>) {

error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/dyn-keyword/dyn-2018-edition-lint.rs:4:35
   |
   |
LL | fn function(x: &SomeTrait, y: Box<SomeTrait>) {
   |
   = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL - fn function(x: &SomeTrait, y: Box<SomeTrait>) {
LL + fn function(x: &SomeTrait, y: Box<dyn SomeTrait>) {

error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/dyn-keyword/dyn-2018-edition-lint.rs:4:35
   |
   |
LL | fn function(x: &SomeTrait, y: Box<SomeTrait>) {
   |
   = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL - fn function(x: &SomeTrait, y: Box<SomeTrait>) {
LL + fn function(x: &SomeTrait, y: Box<dyn SomeTrait>) {

error: aborting due to 7 previous errors



------------------------------------------


---- [ui] ui/suggestions/issue-61963.rs stdout ----
diff of stderr:

31 LL + dyn pub struct Foo {
33 
- error: aborting due to 2 previous errors
+ error: trait objects without an explicit `dyn` are deprecated
+   --> $DIR/issue-61963.rs:22:14
+   --> $DIR/issue-61963.rs:22:14
+    |
+ LL |     bar: Box<Bar>,
+    |
+    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
+    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
+ help: use `dyn`
+ help: use `dyn`
+    |
+ LL -     bar: Box<Bar>,
+ LL +     bar: Box<dyn Bar>,
+ 
+ error: trait objects without an explicit `dyn` are deprecated
+   --> $DIR/issue-61963.rs:22:14
+    |
+    |
+ LL |     bar: Box<Bar>,
+    |
+    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
+    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
+ help: use `dyn`
+ help: use `dyn`
+    |
+ LL -     bar: Box<Bar>,
+ LL +     bar: Box<dyn Bar>,
+ 
+ error: trait objects without an explicit `dyn` are deprecated
+   --> $DIR/issue-61963.rs:18:1
+    |
+    |
+ LL | pub struct Foo {
+    | ^^^
+    |
+    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
+    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
+ help: use `dyn`
+    |
+ LL - pub struct Foo {
+ LL + dyn pub struct Foo {
+ 
+ error: trait objects without an explicit `dyn` are deprecated
+   --> $DIR/issue-61963.rs:18:1
+    |
+    |
+ LL | pub struct Foo {
+    | ^^^
+    |
+    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
+    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
+ help: use `dyn`
+    |
+ LL - pub struct Foo {
+ LL + dyn pub struct Foo {
+ 
+ error: trait objects without an explicit `dyn` are deprecated
+   --> $DIR/issue-61963.rs:18:1
+    |
+    |
+ LL | pub struct Foo {
+    | ^^^
+    |
+    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
+    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
+ help: use `dyn`
+    |
+ LL - pub struct Foo {
+ LL + dyn pub struct Foo {
+ 
+ error: aborting due to 7 previous errors
35 
36 
---
To only update this specific test, also pass `--test-args suggestions/issue-61963.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-61963.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-61963" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-61963/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/suggestions/issue-61963.rs:22:14
   |
LL |     bar: Box<Bar>,
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/suggestions/issue-61963.rs:3:9
   |
   |
LL | #![deny(bare_trait_objects)]
   |         ^^^^^^^^^^^^^^^^^^
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
   |
LL -     bar: Box<Bar>,
LL +     bar: Box<dyn Bar>,

error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/suggestions/issue-61963.rs:18:1
   |
   |
LL | pub struct Foo {
   | ^^^
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
   |
LL - pub struct Foo {
LL + dyn pub struct Foo {

error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/suggestions/issue-61963.rs:22:14
   |
   |
LL |     bar: Box<Bar>,
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     bar: Box<Bar>,
LL +     bar: Box<dyn Bar>,

error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/suggestions/issue-61963.rs:22:14
   |
   |
LL |     bar: Box<Bar>,
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL -     bar: Box<Bar>,
LL +     bar: Box<dyn Bar>,

error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/suggestions/issue-61963.rs:18:1
   |
   |
LL | pub struct Foo {
   | ^^^
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
   |
LL - pub struct Foo {
LL + dyn pub struct Foo {

error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/suggestions/issue-61963.rs:18:1
   |
   |
LL | pub struct Foo {
   | ^^^
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
   |
LL - pub struct Foo {
LL + dyn pub struct Foo {

error: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/suggestions/issue-61963.rs:18:1
   |
   |
LL | pub struct Foo {
   | ^^^
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
   |
LL - pub struct Foo {
LL + dyn pub struct Foo {

error: aborting due to 7 previous errors


---
test result: FAILED. 12282 passed; 2 failed; 111 ignored; 0 measured; 0 filtered out; finished in 134.91s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:53
