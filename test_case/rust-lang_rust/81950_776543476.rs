plain
..............................i.i................................................................... 11400/11440
........................................
failures:

---- [ui] ui/81447.rs stdout ----
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


1 error[E0423]: cannot initialize a tuple struct which contains private fields
2   --> $DIR/81447.rs:11:9
- LL |         Test(self) 
+ LL |         Test(self)
5    |         ^^^^
6    |
6    |
7 note: constructor is not visible here due to private fields

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/81447/81447.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args 81447.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/81447.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/81447" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/81447/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0423]: cannot initialize a tuple struct which contains private fields
  --> /checkout/src/test/ui/81447.rs:11:9
LL |         Test(self)
   |         ^^^^
   |
   |
note: constructor is not visible here due to private fields
  --> /checkout/src/test/ui/81447.rs:2:32
   |
LL |     pub struct Test<T: ?Sized>(T);
   |                                ^ private field
error[E0107]: missing generics for struct `Test`
  --> /checkout/src/test/ui/81447.rs:10:23
   |
LL |     fn build(self) -> Test {
LL |     fn build(self) -> Test {
   |                       ^^^^ expected 1 type argument
   |
note: struct defined here, with 1 type parameter: `T`
  --> /checkout/src/test/ui/81447.rs:2:16
   |
LL |     pub struct Test<T: ?Sized>(T);
   |                ^^^^ -
help: use angle brackets to add missing type argument
   |
LL |     fn build(self) -> Test<T> {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0107, E0423.
---
diff of stderr:

5    |         ^^^
6 
7 error[E0425]: cannot find function `bar` in this scope
-   --> $DIR/suggest-self-2.rs:9:9
+   --> $DIR/suggest-self-2.rs:8:9
9    |
10 LL |         bar(&&self, 102);

12 
12 
13 error[E0425]: cannot find function `bar` in this scope
-   --> $DIR/suggest-self-2.rs:13:9
+   --> $DIR/suggest-self-2.rs:11:9
15    |
16 LL |         bar(&mut self, 102, &"str");

18 
18 
19 error[E0425]: cannot find function `bar` in this scope
-   --> $DIR/suggest-self-2.rs:17:9
+   --> $DIR/suggest-self-2.rs:14:9
22 LL |         bar();
23    |         ^^^ not found in this scope

24 
24 
25 error[E0599]: no method named `bar` found for reference `&Foo` in the current scope
-   --> $DIR/suggest-self-2.rs:20:14
+   --> $DIR/suggest-self-2.rs:17:14
28 LL |         self.bar();
29    |              ^^^ method not found in `&Foo`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/suggest-self-2/suggest-self-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args self/suggest-self-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/suggest-self-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/suggest-self-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/suggest-self-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find function `bar` in this scope
   |
LL |         bar(self);
   |         ^^^


error[E0425]: cannot find function `bar` in this scope
   |
   |
LL |         bar(&&self, 102);


error[E0425]: cannot find function `bar` in this scope
   |
   |
LL |         bar(&mut self, 102, &"str");


error[E0425]: cannot find function `bar` in this scope
   |
LL |         bar();
   |         ^^^ not found in this scope


error[E0599]: no method named `bar` found for reference `&Foo` in the current scope
   |
LL |         self.bar();
   |              ^^^ method not found in `&Foo`

---
test result: FAILED. 11346 passed; 2 failed; 92 ignored; 0 measured; 0 filtered out; finished in 149.44s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:47
