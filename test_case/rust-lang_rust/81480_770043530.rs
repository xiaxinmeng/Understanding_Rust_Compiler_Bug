plain

---- [ui] ui/suggestions/non-existent-field-present-in-subfield.rs stdout ----
diff of stderr:

1 error[E0609]: no field `c` on type `Foo`
-   --> $DIR/issue-81220.rs:39:20
3    |
3    |
- LL |   let test = fooer.c;
-    |                    ^ unknown field
+ LL |     let test = fooer.c;
+    |                      ^ unknown field
6    |
7    = note: available fields are: `first`, `second`, `third`
8 help: one of the expressions' fields has a field of the same name
9    |
9    |
- LL |   let test = fooer.first.bar.c;
-    |                    ^^^^^^^^^^
+ LL |     let test = fooer.first.bar.c;
12 
12 
13 error[E0609]: no field `test` on type `Foo`
-   --> $DIR/issue-81220.rs:42:21
15    |
15    |
- LL |   let test2 = fooer.test;
-    |                     ^^^^ unknown field
+ LL |     let test2 = fooer.test;
18    |
18    |
19    = note: available fields are: `first`, `second`, `third`
20 help: one of the expressions' fields has a field of the same name
21    |
21    |
- LL |   let test2 = fooer.first.bar.c.test;
-    |                     ^^^^^^^^^^^^
+ LL |     let test2 = fooer.first.bar.c.test;
24 
24 
25 error[E0609]: no field `f` on type `Foo`
-   --> $DIR/issue-81220.rs:46:21
27    |
27    |
- LL |   let test3 = fooer.f;
-    |                     ^ unknown field
+ LL |     let test3 = fooer.f;
+    |                       ^ unknown field
30    |
31    = note: available fields are: `first`, `second`, `third`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/non-existent-field-present-in-subfield/non-existent-field-present-in-subfield.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/non-existent-field-present-in-subfield/non-existent-field-present-in-subfield.stderr
diff of fixed:

3 // run-rustfix
4 
5 struct Foo {
-   first : Bar,
-   second : u32,
-   third : u32,
+     first: Bar,
+     second: u32,
+     third: u32,
10 
11 struct Bar {


-   bar : C,
+     bar: C,
14 
15 struct C {


-   c : D,
+     c: D,
18 
- struct D { 
- struct D { 
-   test : E,
+ struct D {
+     test: E,
22 
- struct E { 
- struct E { 
-   e : F,
+ struct E {
+     e: F,
26 
- struct F { 
- struct F { 
-   f : u32,
+ struct F {
+     f: u32,
30 
31 fn main() {


-   let f = F { f : 6 };
-   let e = E { e : f };
-   let d = D { test : e };
-   let c = C { c : d };
-   let bar = Bar { bar : c};
-   let fooer = Foo { first : bar, second : 4, third : 5 };
+     let f = F { f: 6 };
+     let e = E { e: f };
+     let d = D { test: e };
+     let c = C { c: d };
+     let bar = Bar { bar: c };
+     let fooer = Foo { first: bar, second: 4, third: 5 };
38 
-   let test = fooer.first.bar.c;
+     let test = fooer.first.bar.c;
40     //~^ ERROR: no field
41 
-   let test2 = fooer.first.bar.c.test;
+     let test2 = fooer.first.bar.c.test;
43     //~^ ERROR: no field
44 
-   // No suggestion if nesting is deeper than 4 field levels
-   let test3 = fooer.f;
+     // No suggestion if nesting is deeper than 4 field levels
+     let test3 = fooer.f;
47     //~^ ERROR: no field
49 


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/non-existent-field-present-in-subfield/non-existent-field-present-in-subfield.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/non-existent-field-present-in-subfield.rs`
error: 2 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/non-existent-field-present-in-subfield.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/non-existent-field-present-in-subfield" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/non-existent-field-present-in-subfield/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0609]: no field `c` on type `Foo`
   |
   |
LL |     let test = fooer.c;
   |                      ^ unknown field
   |
   = note: available fields are: `first`, `second`, `third`
help: one of the expressions' fields has a field of the same name
   |
LL |     let test = fooer.first.bar.c;


error[E0609]: no field `test` on type `Foo`
   |
   |
LL |     let test2 = fooer.test;
   |
   |
   = note: available fields are: `first`, `second`, `third`
help: one of the expressions' fields has a field of the same name
   |
LL |     let test2 = fooer.first.bar.c.test;


error[E0609]: no field `f` on type `Foo`
   |
   |
LL |     let test3 = fooer.f;
   |                       ^ unknown field
   |
   = note: available fields are: `first`, `second`, `third`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0609`.

---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:45
