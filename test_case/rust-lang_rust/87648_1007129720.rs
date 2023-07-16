plain
.................................................................................................... 1800/12542
................i................................................................................... 1900/12542
.................................................................................................... 2000/12542
.....................................................................................i.............. 2100/12542
...........................................................F..F..................................... 2200/12542
.................................................................................................... 2400/12542
.................................................................................................... 2500/12542
.................................................................................................... 2600/12542
.................................................................................................... 2700/12542
---
---- [ui] ui/associated-consts/assoc-const.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/assoc-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/assoc-const/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/assoc-const/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0220]: associated type `N` not found for `Foo`
  --> /checkout/src/test/ui/associated-consts/assoc-const.rs:13:15
   |
LL | fn foo<F: Foo<N=3>>() {}
   |               ^ associated type `N` not found
error: aborting due to previous error

For more information about this error, try `rustc --explain E0220`.


------------------------------------------


---- [ui] ui/const-generics/parser-error-recovery/issue-89013-no-kw.rs stdout ----
diff of stderr:

- error: cannot constrain an associated constant to a value
-   --> $DIR/issue-89013-no-kw.rs:9:10
+ error[E0107]: this trait takes 1 generic argument but 0 generic arguments were supplied
3    |
3    |
4 LL | impl Foo<N = 3> for Bar {
-    |          -^^^-
-    |          |   |
-    |          |   ...cannot be constrained to this value
-    |          this associated constant...
+    |      ^^^ expected 1 generic argument
+    |
+ note: trait defined here, with 1 generic parameter: `N`
+    |
+    |
+ LL | trait Foo<const N: usize> {
+    |       ^^^       -
+ help: add missing generic argument
+    |
+ LL | impl Foo<N, N = 3> for Bar {
9 
10 error[E0229]: associated type bindings are not allowed here
11   --> $DIR/issue-89013-no-kw.rs:9:10

---
19 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/parser-error-recovery/issue-89013-no-kw/issue-89013-no-kw.stderr
To only update this specific test, also pass `--test-args const-generics/parser-error-recovery/issue-89013-no-kw.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/parser-error-recovery/issue-89013-no-kw.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/parser-error-recovery/issue-89013-no-kw" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/parser-error-recovery/issue-89013-no-kw/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0107]: this trait takes 1 generic argument but 0 generic arguments were supplied
   |
   |
LL | impl Foo<N = 3> for Bar {
   |      ^^^ expected 1 generic argument
   |
note: trait defined here, with 1 generic parameter: `N`
   |
   |
LL | trait Foo<const N: usize> {
help: add missing generic argument
   |
   |
LL | impl Foo<N, N = 3> for Bar {

error[E0229]: associated type bindings are not allowed here
  --> /checkout/src/test/ui/const-generics/parser-error-recovery/issue-89013-no-kw.rs:9:10
   |
   |
LL | impl Foo<N = 3> for Bar {
   |          ^^^^^ associated type not allowed here
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0107, E0229.
For more information about an error, try `rustc --explain E0107`.
For more information about an error, try `rustc --explain E0107`.

------------------------------------------


---- [ui] ui/const-generics/parser-error-recovery/issue-89013.rs stdout ----
diff of stderr:

10 LL + impl Foo<N = 3> for Bar {
12 
- error: cannot constrain an associated constant to a value
-   --> $DIR/issue-89013.rs:9:10
-   --> $DIR/issue-89013.rs:9:10
+ error[E0107]: this trait takes 1 generic argument but 0 generic arguments were supplied
15    |
15    |
16 LL | impl Foo<N = const 3> for Bar {
-    |          -^^^^^^^^^-
-    |          |         ...cannot be constrained to this value
-    |          this associated constant...
+    |      ^^^ expected 1 generic argument
+    |
+    |
+ note: trait defined here, with 1 generic parameter: `N`
+    |
+    |
+ LL | trait Foo<const N: usize> {
+    |       ^^^       -
+ help: add missing generic argument
+    |
+ LL | impl Foo<N, N = const 3> for Bar {
21 
22 error[E0229]: associated type bindings are not allowed here
23   --> $DIR/issue-89013.rs:9:10

---
To only update this specific test, also pass `--test-args const-generics/parser-error-recovery/issue-89013.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/parser-error-recovery/issue-89013.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/parser-error-recovery/issue-89013" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/parser-error-recovery/issue-89013/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected lifetime, type, or constant, found keyword `const`
  --> /checkout/src/test/ui/const-generics/parser-error-recovery/issue-89013.rs:9:14
   |
LL | impl Foo<N = const 3> for Bar {
   |
   |
help: the `const` keyword is only needed in the definition of the type
   |
LL - impl Foo<N = const 3> for Bar {
LL + impl Foo<N = 3> for Bar {


error[E0107]: this trait takes 1 generic argument but 0 generic arguments were supplied
   |
   |
LL | impl Foo<N = const 3> for Bar {
   |      ^^^ expected 1 generic argument
   |
note: trait defined here, with 1 generic parameter: `N`
   |
   |
LL | trait Foo<const N: usize> {
help: add missing generic argument
   |
   |
LL | impl Foo<N, N = const 3> for Bar {

error[E0229]: associated type bindings are not allowed here
  --> /checkout/src/test/ui/const-generics/parser-error-recovery/issue-89013.rs:9:10
   |
   |
LL | impl Foo<N = const 3> for Bar {
   |          ^^^^^^^^^^^ associated type not allowed here
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0107, E0229.
For more information about an error, try `rustc --explain E0107`.
---
---- [ui] ui/parser/recover-assoc-const-constraint.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/recover-assoc-const-constraint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-assoc-const-constraint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-assoc-const-constraint/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
test result: FAILED. 12419 passed; 4 failed; 119 ignored; 0 measured; 0 filtered out; finished in 159.97s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:15:17
