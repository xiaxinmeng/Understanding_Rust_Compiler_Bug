plain

---- [ui] ui/error-codes/E0516.rs stdout ----
diff of stderr:

4 LL |     let x: typeof(92) = 92;
5    |            ^^^^^^^^^^ reserved keyword
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0391]: cycle detected when computing type of `main::{constant#0}`
+   --> $DIR/E0516.rs:2:19
+    |
+ LL |     let x: typeof(92) = 92;
+    |
+    |
+ note: ...which requires type-checking `main::{constant#0}`...
+   --> $DIR/E0516.rs:2:19
+    |
+ LL |     let x: typeof(92) = 92;
+    |                   ^^
+    = note: ...which again requires computing type of `main::{constant#0}`, completing the cycle
+ note: cycle used when type-checking `main`
+   --> $DIR/E0516.rs:1:1
+ LL | fn main() {
+    | ^^^^^^^^^
8 
- For more information about this error, try `rustc --explain E0516`.
---
10 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0516/E0516.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0516.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0516.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0516" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0516/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------
error[E0516]: `typeof` is a reserved keyword but unimplemented
   |
   |
LL |     let x: typeof(92) = 92; //~ ERROR E0516
   |            ^^^^^^^^^^ reserved keyword

error[E0391]: cycle detected when computing type of `main::{constant#0}`
   |
   |
LL |     let x: typeof(92) = 92; //~ ERROR E0516
   |
   |
note: ...which requires type-checking `main::{constant#0}`...
   |
   |
LL |     let x: typeof(92) = 92; //~ ERROR E0516
   |                   ^^
   = note: ...which again requires computing type of `main::{constant#0}`, completing the cycle
note: cycle used when type-checking `main`
   |
LL | fn main() {
   | ^^^^^^^^^

---

---- [ui] ui/issues/issue-29184.rs stdout ----
diff of stderr:

4 LL |     let x: typeof(92) = 92;
5    |            ^^^^^^^^^^ reserved keyword
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0391]: cycle detected when computing type of `main::{constant#0}`
+    |
+    |
+ LL |     let x: typeof(92) = 92;
+    |
+    |
+ note: ...which requires type-checking `main::{constant#0}`...
+    |
+    |
+ LL |     let x: typeof(92) = 92;
+    |                   ^^
+    = note: ...which again requires computing type of `main::{constant#0}`, completing the cycle
+ note: cycle used when type-checking `main`
+    |
+ LL | fn main() {
+    | ^^^^^^^^^
8 
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29184/issue-29184.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-29184.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-29184.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29184" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29184/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0516]: `typeof` is a reserved keyword but unimplemented
   |
   |
LL |     let x: typeof(92) = 92; //~ ERROR `typeof` is a reserved keyword
   |            ^^^^^^^^^^ reserved keyword

error[E0391]: cycle detected when computing type of `main::{constant#0}`
   |
   |
LL |     let x: typeof(92) = 92; //~ ERROR `typeof` is a reserved keyword
   |
   |
note: ...which requires type-checking `main::{constant#0}`...
   |
   |
LL |     let x: typeof(92) = 92; //~ ERROR `typeof` is a reserved keyword
   |                   ^^
   = note: ...which again requires computing type of `main::{constant#0}`, completing the cycle
note: cycle used when type-checking `main`
   |
LL | fn main() {
   | ^^^^^^^^^

---

---- [ui] ui/issues/issue-42060.rs stdout ----
diff of stderr:

20 LL |     let other: typeof(thing) = thing;
21    |                ^^^^^^^^^^^^^ reserved keyword
22 
+ error[E0391]: cycle detected when computing type of `main::{constant#0}`
+    |
+    |
+ LL |     let other: typeof(thing) = thing;
+    |
+    |
+ note: ...which requires type-checking `main::{constant#0}`...
+    |
+    |
+ LL |     let other: typeof(thing) = thing;
+    |                       ^^^^^
+    = note: ...which again requires computing type of `main::{constant#0}`, completing the cycle
+ note: cycle used when type-checking `main`
+    |
+ LL | fn main() {
+    | ^^^^^^^^^
+ 
+ 
23 error[E0516]: `typeof` is a reserved keyword but unimplemented
25    |


26 LL |     <typeof(q)>::N
27    |      ^^^^^^^^^ reserved keyword
- error: aborting due to 4 previous errors
- error: aborting due to 4 previous errors
+ error[E0391]: cycle detected when computing type of `f::{constant#0}`
+    |
+    |
+ LL |     <typeof(q)>::N
+    |
+    |
+ note: ...which requires type-checking `f::{constant#0}`...
+    |
+    |
+ LL |     <typeof(q)>::N
+    |             ^
+    = note: ...which again requires computing type of `f::{constant#0}`, completing the cycle
+ note: cycle used when type-checking `f`
+    |
+ LL | fn f(){
+    | ^^^^^^
30 
---


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42060/issue-42060.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-42060.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-42060.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42060" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42060/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/issues/issue-42060.rs:3:23
   |
LL |     let thing = ();
   |     --------- help: consider using `const` instead of `let`: `const thing`
LL |     let other: typeof(thing) = thing; //~ ERROR attempt to use a non-constant value in a constant
   |                       ^^^^^ non-constant value
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/issues/issue-42060.rs:9:13
   |
LL |     let q = 1;
LL |     let q = 1;
   |     ----- help: consider using `const` instead of `let`: `const q`
LL |     <typeof(q)>::N //~ ERROR attempt to use a non-constant value in a constant
   |             ^ non-constant value

error[E0516]: `typeof` is a reserved keyword but unimplemented
   |
   |
LL |     let other: typeof(thing) = thing; //~ ERROR attempt to use a non-constant value in a constant
   |                ^^^^^^^^^^^^^ reserved keyword

error[E0391]: cycle detected when computing type of `main::{constant#0}`
   |
   |
LL |     let other: typeof(thing) = thing; //~ ERROR attempt to use a non-constant value in a constant
   |
   |
note: ...which requires type-checking `main::{constant#0}`...
   |
   |
LL |     let other: typeof(thing) = thing; //~ ERROR attempt to use a non-constant value in a constant
   |                       ^^^^^
   = note: ...which again requires computing type of `main::{constant#0}`, completing the cycle
note: cycle used when type-checking `main`
   |
LL | fn main() {
   | ^^^^^^^^^


error[E0516]: `typeof` is a reserved keyword but unimplemented
   |
   |
LL |     <typeof(q)>::N //~ ERROR attempt to use a non-constant value in a constant
   |      ^^^^^^^^^ reserved keyword

error[E0391]: cycle detected when computing type of `f::{constant#0}`
   |
   |
LL |     <typeof(q)>::N //~ ERROR attempt to use a non-constant value in a constant
   |
   |
note: ...which requires type-checking `f::{constant#0}`...
   |
   |
LL |     <typeof(q)>::N //~ ERROR attempt to use a non-constant value in a constant
   |             ^
   = note: ...which again requires computing type of `f::{constant#0}`, completing the cycle
note: cycle used when type-checking `f`
   |
LL | fn f(){
   | ^^^^^^

---
test result: FAILED. 11670 passed; 3 failed; 97 ignored; 0 measured; 0 filtered out; finished in 120.42s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:01
