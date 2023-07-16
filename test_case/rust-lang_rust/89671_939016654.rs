plain
diff of stderr:

2   --> $DIR/ordinary-bounds-unrelated.rs:16:74
3    |
4 LL | fn upper_bounds<'a, 'b, 'c, 'd, 'e>(a: Ordinary<'a>, b: Ordinary<'b>) -> impl Trait<'d, 'e>
+    |                     --                                                   ^^^^^^^^^^^^^^^^^^
+    |                     |
+    |                     |
+    |                     hidden type `Ordinary<'b>` captures the lifetime `'b` as defined here
6    |
- note: hidden type `Ordinary<'b>` captures the lifetime `'b` as defined on the function body at 16:21
-   --> $DIR/ordinary-bounds-unrelated.rs:16:21
+ help: to declare that the `impl Trait` captures 'b, you can add an explicit `'b` lifetime bound
9    |
- LL | fn upper_bounds<'a, 'b, 'c, 'd, 'e>(a: Ordinary<'a>, b: Ordinary<'b>) -> impl Trait<'d, 'e>
-    |                     ^^
+ LL | fn upper_bounds<'a, 'b, 'c, 'd, 'e>(a: Ordinary<'a>, b: Ordinary<'b>) -> impl Trait<'d, 'e> + 'b
12 
13 error: aborting due to previous error
14 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/multiple-lifetimes/ordinary-bounds-unrelated.nll/ordinary-bounds-unrelated.nll.stderr
To only update this specific test, also pass `--test-args impl-trait/multiple-lifetimes/ordinary-bounds-unrelated.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/multiple-lifetimes/ordinary-bounds-unrelated.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/multiple-lifetimes/ordinary-bounds-unrelated.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/multiple-lifetimes/ordinary-bounds-unrelated.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
   |
   |
LL | fn upper_bounds<'a, 'b, 'c, 'd, 'e>(a: Ordinary<'a>, b: Ordinary<'b>) -> impl Trait<'d, 'e>
   |                     |
   |                     |
   |                     hidden type `Ordinary<'b>` captures the lifetime `'b` as defined here
   |
help: to declare that the `impl Trait` captures 'b, you can add an explicit `'b` lifetime bound
   |
LL | fn upper_bounds<'a, 'b, 'c, 'd, 'e>(a: Ordinary<'a>, b: Ordinary<'b>) -> impl Trait<'d, 'e> + 'b

error: aborting due to previous error

For more information about this error, try `rustc --explain E0700`.
---
diff of stderr:

2   --> $DIR/ordinary-bounds-unsuited.rs:18:62
3    |
4 LL | fn upper_bounds<'a, 'b>(a: Ordinary<'a>, b: Ordinary<'b>) -> impl Trait<'a, 'b>
+    |                     --                                       ^^^^^^^^^^^^^^^^^^
+    |                     |
+    |                     |
+    |                     hidden type `Ordinary<'b>` captures the lifetime `'b` as defined here
6    |
- note: hidden type `Ordinary<'b>` captures the lifetime `'b` as defined on the function body at 18:21
-   --> $DIR/ordinary-bounds-unsuited.rs:18:21
+ help: to declare that the `impl Trait` captures 'b, you can add an explicit `'b` lifetime bound
9    |
- LL | fn upper_bounds<'a, 'b>(a: Ordinary<'a>, b: Ordinary<'b>) -> impl Trait<'a, 'b>
-    |                     ^^
+ LL | fn upper_bounds<'a, 'b>(a: Ordinary<'a>, b: Ordinary<'b>) -> impl Trait<'a, 'b> + 'b
12 
13 error: aborting due to previous error
14 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/multiple-lifetimes/ordinary-bounds-unsuited.nll/ordinary-bounds-unsuited.nll.stderr
To only update this specific test, also pass `--test-args impl-trait/multiple-lifetimes/ordinary-bounds-unsuited.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/multiple-lifetimes/ordinary-bounds-unsuited.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/multiple-lifetimes/ordinary-bounds-unsuited.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/multiple-lifetimes/ordinary-bounds-unsuited.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
   |
   |
LL | fn upper_bounds<'a, 'b>(a: Ordinary<'a>, b: Ordinary<'b>) -> impl Trait<'a, 'b>
   |                     |
   |                     |
   |                     hidden type `Ordinary<'b>` captures the lifetime `'b` as defined here
   |
help: to declare that the `impl Trait` captures 'b, you can add an explicit `'b` lifetime bound
   |
LL | fn upper_bounds<'a, 'b>(a: Ordinary<'a>, b: Ordinary<'b>) -> impl Trait<'a, 'b> + 'b

error: aborting due to previous error

For more information about this error, try `rustc --explain E0700`.
---

7 LL | |     });
8    | |______^
9    |
- note: the parameter type `T` must be valid for the anonymous lifetime defined on the function body at 19:24...
+ note: the parameter type `T` must be valid for the anonymous lifetime defined here...
12    |
12    |
13 LL | fn func<T: Test>(foo: &Foo, t: T) {

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature-2.nll/missing-lifetimes-in-signature-2.nll.stderr
To only update this specific test, also pass `--test-args suggestions/lifetimes/missing-lifetimes-in-signature-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature-2.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/lifetimes/missing-lifetimes-in-signature-2.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0311]: the parameter type `T` may not live long enough
   |
   |
LL | /     foo.bar(move |_| {
LL | |     //~^ ERROR the parameter type `T` may not live long enough
LL | |         t.test();
LL | |     });
   |
   |
note: the parameter type `T` must be valid for the anonymous lifetime defined here...
   |
   |
LL | fn func<T: Test>(foo: &Foo, t: T) {

error: aborting due to previous error


---
    [ui (nll)] ui/suggestions/lifetimes/missing-lifetimes-in-signature-2.rs

test result: FAILED. 12121 passed; 3 failed; 142 ignored; 0 measured; 0 filtered out; finished in 111.47s

Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always" "--compare-mode" "nll"


Build completed unsuccessfully in 0:19:51
