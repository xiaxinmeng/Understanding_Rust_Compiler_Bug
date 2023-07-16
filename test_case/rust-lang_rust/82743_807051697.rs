plain

---- [ui (nll)] ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs stdout ----
diff of stderr:

1 error[E0261]: use of undeclared lifetime name `'a`
-   --> $DIR/missing-lifetimes-in-signature.rs:37:11
3    |
3    |
4 LL | fn baz<G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
5    |        -  ^^ undeclared lifetime
6    |        |
6    |        |
7    |        help: consider introducing lifetime `'a` here: `'a,`
- error: lifetime may not live long enough
-   --> $DIR/missing-lifetimes-in-signature.rs:15:37
-    |
-    |
- LL | fn foo<G, T>(g: G, dest: &mut T) -> impl FnOnce()
-    |                          -          ^^^^^^^^^^^^^ opaque type requires that `'1` must outlive `'static`
-    |                          |
-    |                          let's call the lifetime of this reference `'1`
-    |
- help: to allow this `impl Trait` to capture borrowed data with lifetime `'1`, add `'_` as a bound
-    |
- LL | fn foo<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
+ error: aborting due to previous error
21 
21 
- error[E0311]: the parameter type `G` may not live long enough
-   --> $DIR/missing-lifetimes-in-signature.rs:25:37
-    |
- LL | fn bar<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
-    |
-    |
- note: the parameter type `G` must be valid for the anonymous lifetime defined on the function body at 25:26...
-   --> $DIR/missing-lifetimes-in-signature.rs:25:26
-    |
- LL | fn bar<G, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
- 
- 
- error[E0311]: the parameter type `G` may not live long enough
-   --> $DIR/missing-lifetimes-in-signature.rs:47:45
-    |
- LL | fn qux<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
-    |
-    |
- note: the parameter type `G` must be valid for the anonymous lifetime defined on the function body at 47:34...
-   --> $DIR/missing-lifetimes-in-signature.rs:47:34
-    |
- LL | fn qux<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_
- 
- 
- error[E0311]: the parameter type `G` may not live long enough
-   --> $DIR/missing-lifetimes-in-signature.rs:59:58
-    |
- LL |     fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
-    |
-    |
- note: the parameter type `G` must be valid for the anonymous lifetime defined on the method body at 59:47...
-   --> $DIR/missing-lifetimes-in-signature.rs:59:47
-    |
- LL |     fn qux<'b, G: Get<T> + 'b, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ {
- 
- 
- error[E0311]: the parameter type `G` may not live long enough
-   --> $DIR/missing-lifetimes-in-signature.rs:68:45
-    |
- LL | fn bat<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'a
-    |
-    |
- note: the parameter type `G` must be valid for the anonymous lifetime defined on the function body at 68:34...
-   --> $DIR/missing-lifetimes-in-signature.rs:68:34
-    |
- LL | fn bat<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'a
- 
- 
- error[E0621]: explicit lifetime required in the type of `dest`
-   --> $DIR/missing-lifetimes-in-signature.rs:73:5
-    |
- LL |   fn bat<'a, G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ + 'a
-    |                                    ------ help: add explicit lifetime `'a` to the type of `dest`: `&'a mut T`
- LL | /     move || {
- LL | /     move || {
- LL | |         *dest = g.get();
- LL | |     }
-    | |_____^ lifetime `'a` required
- 
- error[E0309]: the parameter type `G` may not live long enough
-   --> $DIR/missing-lifetimes-in-signature.rs:79:44
-    |
- LL | fn bak<'a, G, T>(g: G, dest: &'a mut T) -> impl FnOnce() + 'a
-    |
-    |
-    = help: consider adding an explicit lifetime bound `G: 'a`...
- error: aborting due to 8 previous errors
- 
- Some errors have detailed explanations: E0261, E0309, E0621.
- For more information about an error, try `rustc --explain E0261`.
- For more information about an error, try `rustc --explain E0261`.
+ For more information about this error, try `rustc --explain E0261`.
93 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.nll/missing-lifetimes-in-signature.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/lifetimes/missing-lifetimes-in-signature.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0261]: use of undeclared lifetime name `'a`
   |
   |
LL | fn baz<G: 'a, T>(g: G, dest: &mut T) -> impl FnOnce() + '_ //~ ERROR undeclared lifetime
   |        -  ^^ undeclared lifetime
   |        |
   |        help: consider introducing lifetime `'a` here: `'a,`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0261`.

---
    [ui (nll)] ui/suggestions/lifetimes/missing-lifetimes-in-signature.rs

test result: FAILED. 11589 passed; 1 failed; 124 ignored; 0 measured; 0 filtered out; finished in 104.56s

Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.0-rust-1.53.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:19:55
