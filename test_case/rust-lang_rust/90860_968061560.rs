plain

---- [ui (nll)] ui/closures/closure-bounds-static-cant-capture-borrowed.rs stdout ----
diff of stderr:

1 error[E0621]: explicit lifetime required in the type of `x`
3    |
3    |
+ LL |   fn foo(x: &()) {
+    |             --- the type of `x` does not have lifetime `'static'`
4 LL | /     bar(|| {
5 LL | |
6 LL | |         let _ = x;

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-bounds-static-cant-capture-borrowed.nll/closure-bounds-static-cant-capture-borrowed.nll.stderr
To only update this specific test, also pass `--test-args closures/closure-bounds-static-cant-capture-borrowed.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-bounds-static-cant-capture-borrowed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-bounds-static-cant-capture-borrowed.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-bounds-static-cant-capture-borrowed.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0621]: explicit lifetime required in the type of `x`
   |
   |
LL |   fn foo(x: &()) {
   |             --- the type of `x` does not have lifetime `'static'`
LL | /     bar(|| {
LL | |         //~^ ERROR explicit lifetime required in the type of `x` [E0621]
LL | |         let _ = x;
LL | |     })
   | |______^ lifetime `'static` required

error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
   |
LL |     bar(|| {
LL |     bar(|| {
   |         ^^ may outlive borrowed value `x`
LL |         //~^ ERROR explicit lifetime required in the type of `x` [E0621]
LL |         let _ = x;
   |                 - `x` is borrowed here
   |
note: function requires argument type to outlive `'static`
   |
LL | /     bar(|| {
LL | /     bar(|| {
LL | |         //~^ ERROR explicit lifetime required in the type of `x` [E0621]
LL | |         let _ = x;
LL | |     })
   | |______^
help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
LL |     bar(move || {
   |         ++++

error: aborting due to 2 previous errors
---

---- [ui (nll)] ui/lifetimes/issue-90600-expected-return-static-indirect.rs stdout ----
diff of stderr:

+ error[E0597]: `foo` does not live long enough
+    |
+    |
+ LL |     let refcell = RefCell::new(&mut foo);
+    |                                ^^^^^^^^ borrowed value does not live long enough
+ LL |     let read = &refcell as &RefCell<dyn Read>;
+    |                -------- cast requires that `foo` is borrowed for `'static`
+ LL | }
+ LL | }
+    | - `foo` dropped here while still borrowed
1 error[E0621]: explicit lifetime required in the type of `foo`
-   --> $DIR/issue-90600-expected-return-static-indirect.rs:10:16
+   --> $DIR/issue-90600-expected-return-static-indirect.rs:8:16
3    |
3    |
4 LL | fn inner(mut foo: &[u8]) {
5    |                   ----- the type of `foo` does not have lifetime `'static'`
- ...
- ...
- LL |     read_thing(read);
-    |                ^^^^ lifetime `'static` required
+ LL |     let refcell = RefCell::new(&mut foo);
+ LL |     let read = &refcell as &RefCell<dyn Read>;
+    |                ^^^^^^^^ lifetime `'static` required
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
11 
- For more information about this error, try `rustc --explain E0621`.
- For more information about this error, try `rustc --explain E0621`.
+ Some errors have detailed explanations: E0597, E0621.
+ For more information about an error, try `rustc --explain E0597`.
13 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90600-expected-return-static-indirect.nll/issue-90600-expected-return-static-indirect.nll.stderr
To only update this specific test, also pass `--test-args lifetimes/issue-90600-expected-return-static-indirect.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/issue-90600-expected-return-static-indirect.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90600-expected-return-static-indirect.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-90600-expected-return-static-indirect.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0597]: `foo` does not live long enough
   |
   |
LL |     let refcell = RefCell::new(&mut foo);
   |                                ^^^^^^^^ borrowed value does not live long enough
LL |     let read = &refcell as &RefCell<dyn Read>;
   |                -------- cast requires that `foo` is borrowed for `'static`
LL | }
LL | }
   | - `foo` dropped here while still borrowed
error[E0621]: explicit lifetime required in the type of `foo`
  --> /checkout/src/test/ui/lifetimes/issue-90600-expected-return-static-indirect.rs:8:16
   |
   |
LL | fn inner(mut foo: &[u8]) {
   |                   ----- the type of `foo` does not have lifetime `'static'`
LL |     let refcell = RefCell::new(&mut foo);
LL |     let read = &refcell as &RefCell<dyn Read>;
   |                ^^^^^^^^ lifetime `'static` required
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0597, E0621.
For more information about an error, try `rustc --explain E0597`.
---

9 error[E0621]: explicit lifetime required in the type of `u`
10   --> $DIR/regions-static-bound.rs:14:5
11    |
+ LL | fn error(u: &(), v: &()) {
+    |             --- the type of `u` does not have lifetime `'static'`
12 LL |     static_id(&u);
13    |     ^^^^^^^^^^^^^ lifetime `'static` required

15 error[E0621]: explicit lifetime required in the type of `v`
16   --> $DIR/regions-static-bound.rs:16:5
17    |
17    |
+ LL | fn error(u: &(), v: &()) {
+    |                     --- the type of `v` does not have lifetime `'static'`
+ ...
18 LL |     static_id_indirect(&v);
19    |     ^^^^^^^^^^^^^^^^^^^^^^ lifetime `'static` required


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound.migrate.nll/regions-static-bound.migrate.nll.stderr
To only update this specific test, also pass `--test-args regions/regions-static-bound.rs`


error in revision `migrate`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-static-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "migrate" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound.migrate.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound.migrate.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-static-bound.rs:9:5
   |
LL | fn static_id_wrong_way<'a>(t: &'a ()) -> &'static () where 'static: 'a {
   |                        -- lifetime `'a` defined here
LL |     t //[migrate]~ ERROR E0312
   |     ^ returning this value requires that `'a` must outlive `'static`
error[E0621]: explicit lifetime required in the type of `u`
  --> /checkout/src/test/ui/regions/regions-static-bound.rs:14:5
   |
   |
LL | fn error(u: &(), v: &()) {
   |             --- the type of `u` does not have lifetime `'static'`
LL |     static_id(&u); //[migrate]~ ERROR explicit lifetime required in the type of `u` [E0621]
   |     ^^^^^^^^^^^^^ lifetime `'static` required
error[E0621]: explicit lifetime required in the type of `v`
  --> /checkout/src/test/ui/regions/regions-static-bound.rs:16:5
   |
   |
LL | fn error(u: &(), v: &()) {
   |                     --- the type of `v` does not have lifetime `'static'`
...
LL |     static_id_indirect(&v); //[migrate]~ ERROR explicit lifetime required in the type of `v` [E0621]
   |     ^^^^^^^^^^^^^^^^^^^^^^ lifetime `'static` required
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0621`.

---
    [ui (nll)] ui/regions/regions-static-bound.rs#migrate

test result: FAILED. 12249 passed; 3 failed; 146 ignored; 0 measured; 0 filtered out; finished in 110.21s

Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.58.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always" "--compare-mode" "nll"


Build completed unsuccessfully in 0:20:55
