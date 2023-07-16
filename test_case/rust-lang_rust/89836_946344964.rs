plain

---- [ui] ui/nll/ty-outlives/impl-trait-captures.rs stdout ----
diff of stderr:

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
4 LL | fn foo<'a, T>(x: &T) -> impl Foo<'a> {
6    |                  |
6    |                  |
-    |                  hidden type `&ReFree(DefId(0:8 ~ impl_trait_captures[e9f4]::foo), BrAnon(0)) T` captures the anonymous lifetime defined here
+    |                  hidden type `&ReFree(DefId(0:8 ~ impl_trait_captures[bdef]::foo), BrAnon(0)) T` captures the anonymous lifetime defined here
8    |
- help: to declare that the `impl Trait` captures ReFree(DefId(0:8 ~ impl_trait_captures[e9f4]::foo), BrAnon(0)), you can add an explicit `ReFree(DefId(0:8 ~ impl_trait_captures[e9f4]::foo), BrAnon(0))` lifetime bound
+ help: to declare that the `impl Trait` captures ReFree(DefId(0:8 ~ impl_trait_captures[bdef]::foo), BrAnon(0)), you can add an explicit `ReFree(DefId(0:8 ~ impl_trait_captures[bdef]::foo), BrAnon(0))` lifetime bound
10    |
- LL | fn foo<'a, T>(x: &T) -> impl Foo<'a> + ReFree(DefId(0:8 ~ impl_trait_captures[e9f4]::foo), BrAnon(0)) {
+ LL | fn foo<'a, T>(x: &T) -> impl Foo<'a> + ReFree(DefId(0:8 ~ impl_trait_captures[bdef]::foo), BrAnon(0)) {
13 
14 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/impl-trait-captures/impl-trait-captures.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/ty-outlives/impl-trait-captures.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/impl-trait-captures.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/impl-trait-captures" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zborrowck=mir" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/impl-trait-captures/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
   |
   |
LL | fn foo<'a, T>(x: &T) -> impl Foo<'a> {
   |                  |
   |                  |
   |                  hidden type `&ReFree(DefId(0:8 ~ impl_trait_captures[bdef]::foo), BrAnon(0)) T` captures the anonymous lifetime defined here
   |
help: to declare that the `impl Trait` captures ReFree(DefId(0:8 ~ impl_trait_captures[bdef]::foo), BrAnon(0)), you can add an explicit `ReFree(DefId(0:8 ~ impl_trait_captures[bdef]::foo), BrAnon(0))` lifetime bound
   |
LL | fn foo<'a, T>(x: &T) -> impl Foo<'a> + ReFree(DefId(0:8 ~ impl_trait_captures[bdef]::foo), BrAnon(0)) {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0700`.
---
test result: FAILED. 12188 passed; 1 failed; 117 ignored; 0 measured; 0 filtered out; finished in 108.54s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:11:17
