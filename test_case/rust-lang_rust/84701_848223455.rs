plain
1 error: lifetime may not live long enough
-   --> $DIR/ret-impl-trait-one.rs:12:80
+   --> $DIR/ret-impl-trait-one.rs:10:80
3    |
4 LL |   async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> {
5    |  ________________________________--__--__________________________________________^

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.nll/ret-impl-trait-one.nll.stderr
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/multiple-lifetimes/ret-impl-trait-one.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.rs:10:80
   |
LL |   async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> {
   |  ________________________________--__--__________________________________________^
   | |                                |   |
   | |                                |   lifetime `'b` defined here
   | |                                lifetime `'a` defined here
LL | |     //~^ ERROR lifetime mismatch
LL | |     (a, b)
LL | | }
   | |_^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui (nll)] ui/impl-trait/multiple-lifetimes/ordinary-bounds-unrelated.rs stdout ----
diff of stderr:

1 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
-   --> $DIR/ordinary-bounds-unrelated.rs:18:74
3    |
3    |
4 LL | fn upper_bounds<'a, 'b, 'c, 'd, 'e>(a: Ordinary<'a>, b: Ordinary<'b>) -> impl Trait<'d, 'e>


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/multiple-lifetimes/ordinary-bounds-unrelated.nll/ordinary-bounds-unrelated.nll.stderr
To only update this specific test, also pass `--test-args impl-trait/multiple-lifetimes/ordinary-bounds-unrelated.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/multiple-lifetimes/ordinary-bounds-unrelated.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/multiple-lifetimes/ordinary-bounds-unrelated.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/multiple-lifetimes/ordinary-bounds-unrelated.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
   |
   |
LL | fn upper_bounds<'a, 'b, 'c, 'd, 'e>(a: Ordinary<'a>, b: Ordinary<'b>) -> impl Trait<'d, 'e>
   |
   |
   = note: hidden type `Ordinary<'_>` captures lifetime '_#9r
error: aborting due to previous error

For more information about this error, try `rustc --explain E0700`.


------------------------------------------


---- [ui (nll)] ui/impl-trait/multiple-lifetimes/ordinary-bounds-unsuited.rs stdout ----
diff of stderr:

1 error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
-   --> $DIR/ordinary-bounds-unsuited.rs:20:62
3    |
3    |
4 LL | fn upper_bounds<'a, 'b>(a: Ordinary<'a>, b: Ordinary<'b>) -> impl Trait<'a, 'b>


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/multiple-lifetimes/ordinary-bounds-unsuited.nll/ordinary-bounds-unsuited.nll.stderr
To only update this specific test, also pass `--test-args impl-trait/multiple-lifetimes/ordinary-bounds-unsuited.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/multiple-lifetimes/ordinary-bounds-unsuited.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/multiple-lifetimes/ordinary-bounds-unsuited.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/multiple-lifetimes/ordinary-bounds-unsuited.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
   |
   |
LL | fn upper_bounds<'a, 'b>(a: Ordinary<'a>, b: Ordinary<'b>) -> impl Trait<'a, 'b>
   |
   |
   = note: hidden type `Ordinary<'_>` captures lifetime '_#6r
error: aborting due to previous error

For more information about this error, try `rustc --explain E0700`.

---
test result: FAILED. 11787 passed; 3 failed; 123 ignored; 0 measured; 0 filtered out; finished in 103.09s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.1-rust-1.54.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:19:22
