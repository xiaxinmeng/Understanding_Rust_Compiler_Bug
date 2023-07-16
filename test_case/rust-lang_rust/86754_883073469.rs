plain

---- [ui (nll)] ui/async-await/issues/issue-62097.rs stdout ----
diff of stderr:

14 help: to force the closure to take ownership of `self` (and any other referenced variables), use the `move` keyword
15    |
16 LL |         foo(move || self.bar()).await;
+    |             ^^^^
18 
18 
19 error[E0521]: borrowed data escapes outside of associated function


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097.nll/issue-62097.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issues/issue-62097.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62097.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0373]: closure may outlive the current function, but it borrows `self`, which is owned by the current function
   |
   |
LL |         foo(|| self.bar()).await;
   |             ^^ ---- `self` is borrowed here
   |             |
   |             may outlive borrowed value `self`
   |
note: function requires argument type to outlive `'static`
   |
   |
LL |         foo(|| self.bar()).await;
   |         ^^^^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `self` (and any other referenced variables), use the `move` keyword
   |
LL |         foo(move || self.bar()).await;


error[E0521]: borrowed data escapes outside of associated function
   |
   |
LL |     pub async fn run_dummy_fn(&self) { //~ ERROR E0759
   |                               ----- `self` is a reference that is only valid in the associated function body
LL |         foo(|| self.bar()).await;
   |         ^^^^^^^^^^^^^^^^^^ `self` escapes the associated function body here
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0373, E0521.
For more information about an error, try `rustc --explain E0373`.
For more information about an error, try `rustc --explain E0373`.

------------------------------------------


---- [ui (nll)] ui/closures/closure-bounds-static-cant-capture-borrowed.rs stdout ----
diff of stderr:

27 help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
29 LL |     bar(move || {
-    |         ^^^^^^^
+    |         ^^^^
31 
31 
32 error: aborting due to 2 previous errors
33 


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
LL | /     bar(|| {
LL | /     bar(|| {
LL | |         //~^ ERROR explicit lifetime required in the type of `x` [E0621]
LL | |         let _ = x;
LL | |     })
   | |______^ lifetime `'static` required

error[E0373]: closure may outlive the current function, but it borrows `x`, which is owned by the current function
   |
   |
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
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
   |         ^^^^

error: aborting due to 2 previous errors
---
test result: FAILED. 12038 passed; 2 failed; 125 ignored; 0 measured; 0 filtered out; finished in 107.49s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.1-rust-1.55.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always" "--compare-mode" "nll"


Build completed unsuccessfully in 0:20:47
