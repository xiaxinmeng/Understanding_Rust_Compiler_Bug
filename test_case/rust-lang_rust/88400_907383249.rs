plain
---- [ui] ui/wf/hir-wf-check-erase-regions.rs stdout ----
diff of stderr:

3    |
4 LL |     type IntoIter = std::iter::Flatten<std::slice::Iter<'a, T>>;
5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `&T` is not an iterator
+    | 
7   ::: $SRC_DIR/core/src/iter/adapters/flatten.rs:LL:COL
8    |
8    |
9 LL | pub struct Flatten<I: Iterator<Item: IntoIterator>> {
17    |
18 LL |     fn into_iter(self) -> Self::IntoIter {
18 LL |     fn into_iter(self) -> Self::IntoIter {
19    |                           ^^^^^^^^^^^^^^ `&T` is not an iterator
+    | 
21   ::: $SRC_DIR/core/src/iter/adapters/flatten.rs:LL:COL
22    |
22    |
23 LL | pub struct Flatten<I: Iterator<Item: IntoIterator>> {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/hir-wf-check-erase-regions/hir-wf-check-erase-regions.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args wf/hir-wf-check-erase-regions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/wf/hir-wf-check-erase-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/hir-wf-check-erase-regions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "incremental=tmp/wf/hir-wf-check-erase-regions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/hir-wf-check-erase-regions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `&T` is not an iterator
  --> /checkout/src/test/ui/wf/hir-wf-check-erase-regions.rs:7:5
   |
LL |     type IntoIter = std::iter::Flatten<std::slice::Iter<'a, T>>; //~ ERROR `&T` is not an iterator
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `&T` is not an iterator
  ::: /checkout/library/core/src/iter/adapters/flatten.rs:150:38
   |
   |
LL | pub struct Flatten<I: Iterator<Item: IntoIterator>> {
   |                                      ------------ required by this bound in `Flatten`
   |
   = help: the trait `Iterator` is not implemented for `&T`
   = note: required because of the requirements on the impl of `IntoIterator` for `&T`
error[E0277]: `&T` is not an iterator
  --> /checkout/src/test/ui/wf/hir-wf-check-erase-regions.rs:10:27
   |
   |
LL |     fn into_iter(self) -> Self::IntoIter { //~ ERROR `&T` is not an iterator
   |                           ^^^^^^^^^^^^^^ `&T` is not an iterator
  ::: /checkout/library/core/src/iter/adapters/flatten.rs:150:38
   |
   |
LL | pub struct Flatten<I: Iterator<Item: IntoIterator>> {
   |                                      ------------ required by this bound in `Flatten`
   |
   = help: the trait `Iterator` is not implemented for `&T`
   = note: required because of the requirements on the impl of `IntoIterator` for `&T`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.

---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "beta" "--color" "always"


Build completed unsuccessfully in 0:10:40
