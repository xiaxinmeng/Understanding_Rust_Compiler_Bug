plain
---- [ui] ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs stdout ----
diff of stderr:

16    |                          |
17    |                          hidden type `[closure@$DIR/missing-lifetimes-in-signature.rs:20:5: 22:6]` captures the anonymous lifetime defined here
18    |
- help: to declare that the `impl Trait` captures '_, you can add an explicit `'_` lifetime bound
+ help: to declare that the `impl Trait` captures `'_`, you can add an explicit `'_` lifetime bound
20    |
21 LL | fn foo<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature/missing-lifetimes-in-signature.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature/missing-lifetimes-in-signature.stderr
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/lifetimes/missing-lifetimes-in-signature.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0261]: use of undeclared lifetime name `'a`
  --> /checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:38:11
   |
LL | fn baz<G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
   |        -  ^^ undeclared lifetime
   |        |
   |        help: consider introducing lifetime `'a` here: `'a,`
   |
   = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes

error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
   |
   |
LL | fn foo<G, T>(g: G, dest: &mut T) -> impl FnOnce()
   |                          |
   |                          |
   |                          hidden type `[closure@/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:20:5: 22:6]` captures the anonymous lifetime defined here
   |
help: to declare that the `impl Trait` captures `'_`, you can add an explicit `'_` lifetime bound
   |
LL | fn foo<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_


error[E0311]: the parameter type `G` may not live long enough
   |
   |
LL | fn bar<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
   |
   |
note: the parameter type `G` must be valid for the anonymous lifetime defined here...
   |
   |
LL | fn bar<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
   |                          ^^^^^^
note: ...so that the type `[closure@/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:31:5: 33:6]` will meet its required lifetime bounds
   |
   |
LL | fn bar<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
help: consider introducing an explicit lifetime bound
   |
   |
LL | fn bar<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'a


error[E0311]: the parameter type `G` may not live long enough
   |
   |
LL | fn qux<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
   |
   |
note: the parameter type `G` must be valid for the anonymous lifetime defined here...
   |
   |
LL | fn qux<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
   |                                  ^^^^^^
note: ...so that the type `[closure@/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:54:5: 56:6]` will meet its required lifetime bounds
   |
   |
LL | fn qux<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
help: consider introducing an explicit lifetime bound
   |
   |
LL | fn qux<'b, 'a, G: 'b + 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'b
   |        +++     ~~~~~~~                                                  ++++

error[E0311]: the parameter type `G` may not live long enough
   |
   |
LL |     fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
   |
   |
note: the parameter type `G` must be valid for the anonymous lifetime defined here...
   |
   |
LL |     fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
   |                                               ^^^^^^
note: ...so that the type `[closure@/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:63:9: 65:10]` will meet its required lifetime bounds
   |
   |
LL |     fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
help: consider introducing an explicit lifetime bound
   |
   |
LL |     fn qux<'c, 'b, G: 'c + Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'c {
   |            +++     ~~~~~~~                                                           ++++

error[E0621]: explicit lifetime required in the type of `dest`
   |
   |
LL | fn bat<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'a
   |                                  ------     ^^^^^^^^^^^^^^^^^^^^^^^ lifetime `'a` required
   |                                  |
   |                                  help: add explicit lifetime `'a` to the type of `dest`: `&'a mut T`

error[E0309]: the parameter type `G` may not live long enough
   |
   |
LL | fn bak<'a, G, T>(g: G, dest: &'a mut T) -> impl FnOnce() + 'a
   |            -                               ^^^^^^^^^^^^^^^^^^ ...so that the type `[closure@/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs:86:5: 88:6]` will meet its required lifetime bounds
   |            |
   |            help: consider adding an explicit lifetime bound...: `G: 'a`
error: aborting due to 7 previous errors

Some errors have detailed explanations: E0261, E0309, E0621, E0700.
For more information about an error, try `rustc --explain E0261`.
---
test result: FAILED. 12385 passed; 1 failed; 119 ignored; 0 measured; 0 filtered out; finished in 158.06s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:16
