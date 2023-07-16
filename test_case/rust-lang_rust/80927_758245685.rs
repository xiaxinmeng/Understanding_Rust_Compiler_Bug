plain
..............i.............i....................................................................... 8700/11247
.................................................................................................... 8800/11247
.................................................................................................... 8900/11247
.................................................................................................... 9000/11247
.................F................................................F................................. 9100/11247
...........................................i......i................................................. 9300/11247
...........................................i......i................................................. 9300/11247
..................................................................................iiiiii..iiiiii.i.. 9400/11247
.................................................................................................... 9600/11247
.................................................................................................... 9700/11247
.................................................................................................... 9800/11247
.................................................................................................... 9900/11247
---
diff of stderr:

41   --> $DIR/E0424.rs:20:9
42    |
43 LL | fn main () {
-    |    ---- this function can't have a `self` parameter
+    |    ---- this function doesn't have a `self` parameter
45 LL |     let self = "self";
46    |         ^^^^ `self` value is a keyword and may not be bound to variables or shadowed
+    |
+ help: add a `self` receiver parameter to make the associated `fn` a method
+    |
+ LL | fn main (&self) {
47 
48 error: aborting due to 4 previous errors
49 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0424/E0424.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0424.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0424.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0424" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0424/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0424]: expected value, found module `self`
  --> /checkout/src/test/ui/error-codes/E0424.rs:7:9
   |
LL |     fn foo() {
   |        --- this function doesn't have a `self` parameter
LL |         self.bar(); //~ ERROR E0424
   |         ^^^^ `self` value is a keyword only available in methods with a `self` parameter
   |
help: add a `self` receiver parameter to make the associated `fn` a method
   |
LL |     fn foo(&self) {

error[E0424]: expected value, found module `self`
  --> /checkout/src/test/ui/error-codes/E0424.rs:11:9
   |
   |
LL |     fn baz(_: i32) {
   |        --- this function doesn't have a `self` parameter
LL |         self.bar(); //~ ERROR E0424
   |         ^^^^ `self` value is a keyword only available in methods with a `self` parameter
   |
help: add a `self` receiver parameter to make the associated `fn` a method
   |
LL |     fn baz(&self, _: i32) {

error[E0424]: expected value, found module `self`
  --> /checkout/src/test/ui/error-codes/E0424.rs:15:20
   |
   |
LL |     fn qux() {
   |        --- this function doesn't have a `self` parameter
LL |         let _ = || self.bar(); //~ ERROR E0424
   |                    ^^^^ `self` value is a keyword only available in methods with a `self` parameter
   |
help: add a `self` receiver parameter to make the associated `fn` a method
   |
LL |     fn qux(&self) {


error[E0424]: expected unit struct, unit variant or constant, found module `self`
   |
LL | fn main () {
LL | fn main () {
   |    ---- this function doesn't have a `self` parameter
LL |     let self = "self"; //~ ERROR E0424
   |         ^^^^ `self` value is a keyword and may not be bound to variables or shadowed
   |
help: add a `self` receiver parameter to make the associated `fn` a method
LL | fn main (&self) {
   |          ^^^^^

error: aborting due to 4 previous errors
---
3    |
4 LL |         this.a
-    |         ^^^^ not found in this scope
-    |
- help: you might have meant to use `self` here instead
- LL |         self.a
10    |         ^^^^
10    |         ^^^^
- help: if you meant to use `self`, you are also missing a `self` receiver argument
-    |
- LL |     fn a(&self) -> A {
+    |         |
+    |         not found in this scope
+    |         not found in this scope
+    |         help: you might have meant to use `self` here instead
15 
16 error[E0425]: cannot find value `this` in this scope

18    |
18    |
19 LL |         this.b(x);
-    |
-    |
- help: you might have meant to use `self` here instead
-    |
- LL |         self.b(x);
25    |         ^^^^
- help: if you meant to use `self`, you are also missing a `self` receiver argument
-    |
- LL |     fn b(&self, x: i32) {
+    |         |
+    |         not found in this scope
+    |         not found in this scope
+    |         help: you might have meant to use `self` here instead
30 
31 error[E0425]: cannot find value `this` in this scope

33    |
33    |
34 LL |         let _ = || this.a;
-    |
-    |
- help: you might have meant to use `self` here instead
-    |
- LL |         let _ = || self.a;
40    |                    ^^^^
- help: if you meant to use `self`, you are also missing a `self` receiver argument
-    |
- LL |     fn c(&self) {
+    |                    |
+    |                    not found in this scope
+    |                    not found in this scope
+    |                    help: you might have meant to use `self` here instead
46 error: aborting due to 3 previous errors
47 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5099/issue-5099.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-5099.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-5099.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5099" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-5099/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find value `this` in this scope
   |
   |
LL |         this.a //~ ERROR cannot find value `this` in this scope
   |         |
   |         not found in this scope
   |         not found in this scope
   |         help: you might have meant to use `self` here instead

error[E0425]: cannot find value `this` in this scope
   |
   |
LL |         this.b(x); //~ ERROR cannot find value `this` in this scope
   |         |
   |         not found in this scope
   |         not found in this scope
   |         help: you might have meant to use `self` here instead

error[E0425]: cannot find value `this` in this scope
   |
   |
LL |         let _ = || this.a; //~ ERROR cannot find value `this` in this scope
   |                    |
   |                    not found in this scope
   |                    not found in this scope
   |                    help: you might have meant to use `self` here instead
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0425`.

---
diff of stderr:

2   --> $DIR/issue-60057.rs:8:21
3    |
4 LL |             banana: banana
-    |                     ^^^^^^ a field by this name exists in `Self`
+    |                     ^^^^^^ help: you might have meant to use the available field: `self.banana`
6 
7 error[E0425]: cannot find value `banana` in this scope


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60057/issue-60057.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60057/issue-60057.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-60057.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-60057.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60057" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-60057/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find value `banana` in this scope
   |
   |
LL |             banana: banana //~ ERROR cannot find value `banana` in this scope
   |                     ^^^^^^ help: you might have meant to use the available field: `self.banana`

error[E0425]: cannot find value `banana` in this scope
   |
   |
LL |             banana: banana //~ ERROR cannot find value `banana` in this scope
   |                     ^^^^^^ help: you might have meant to use the available field: `self.banana`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0425`.

---
diff of stderr:

29   --> $DIR/issue-2356.rs:39:5
30    |
31 LL |     whiskers -= other;
-    |     ^^^^^^^^ a field by this name exists in `Self`
+    |     ^^^^^^^^ help: you might have meant to use the available field: `self.whiskers`
33 
34 error[E0425]: cannot find function `shave` in this scope

102   --> $DIR/issue-2356.rs:84:5
103    |
103    |
104 LL |     whiskers = 4;
-    |     ^^^^^^^^ a field by this name exists in `Self`
+    |     ^^^^^^^^ help: you might have meant to use the available field: `self.whiskers`
106 
107 error[E0425]: cannot find function `purr_louder` in this scope

114   --> $DIR/issue-2356.rs:92:5
115    |
116 LL | fn main() {
116 LL | fn main() {
-    |    ---- this function can't have a `self` parameter
+    |    ---- this function doesn't have a `self` parameter
118 LL |     self += 1;
119    |     ^^^^ `self` value is a keyword only available in methods with a `self` parameter
+    |
+ help: add a `self` receiver parameter to make the associated `fn` a method
+    |
+ LL | fn main(&self) {
120 
121 error: aborting due to 17 previous errors
122 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-2356/issue-2356.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/issue-2356.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-2356.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-2356" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-2356/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find function `shave` in this scope
   |
   |
LL |     shave();

error[E0425]: cannot find function `clone` in this scope
  --> /checkout/src/test/ui/resolve/issue-2356.rs:24:5
   |
   |
LL |     clone();
   |     ^^^^^ help: you might have meant to call the method: `self.clone`
error[E0425]: cannot find function `default` in this scope
  --> /checkout/src/test/ui/resolve/issue-2356.rs:31:5
   |
LL |     default();
---
   |
LL | use std::default::default;
   |

error[E0425]: cannot find value `whiskers` in this scope
   |
   |
LL |     whiskers -= other;
   |     ^^^^^^^^ help: you might have meant to use the available field: `self.whiskers`

error[E0425]: cannot find function `shave` in this scope
   |
   |
LL |     shave(4);
   |     ^^^^^ help: you might have meant to call the associated function: `Self::shave`

error[E0425]: cannot find function `purr` in this scope
   |
   |
LL |     purr();


error[E0425]: cannot find function `static_method` in this scope
   |
LL |         static_method();
   |         ^^^^^^^^^^^^^ not found in this scope


error[E0425]: cannot find function `purr` in this scope
   |
   |
LL |         purr();


error[E0425]: cannot find function `purr` in this scope
   |
   |
LL |         purr();


error[E0425]: cannot find function `purr` in this scope
   |
   |
LL |         purr();

error[E0424]: expected value, found module `self`
  --> /checkout/src/test/ui/resolve/issue-2356.rs:65:8
   |
   |
LL |   fn meow() {
   |      ---- this function doesn't have a `self` parameter
LL |     if self.whiskers > 3 {
   |        ^^^^ `self` value is a keyword only available in methods with a `self` parameter
   |
help: add a `self` receiver parameter to make the associated `fn` a method
   |
LL |   fn meow(&self) {


error[E0425]: cannot find function `grow_older` in this scope
   |
   |
LL |     grow_older();


error[E0425]: cannot find function `shave` in this scope
   |
   |
LL |     shave();


error[E0425]: cannot find value `whiskers` in this scope
   |
   |
LL |     whiskers = 0;
   |     ^^^^^^^^ help: you might have meant to use the available field: `self.whiskers`

error[E0425]: cannot find value `whiskers` in this scope
   |
   |
LL |     whiskers = 4;
   |     ^^^^^^^^ help: you might have meant to use the available field: `self.whiskers`

error[E0425]: cannot find function `purr_louder` in this scope
   |
   |
LL |     purr_louder();

error[E0424]: expected value, found module `self`
  --> /checkout/src/test/ui/resolve/issue-2356.rs:92:5
   |
   |
LL | fn main() {
   |    ---- this function doesn't have a `self` parameter
LL |     self += 1;
   |     ^^^^ `self` value is a keyword only available in methods with a `self` parameter
   |
help: add a `self` receiver parameter to make the associated `fn` a method
LL | fn main(&self) {
   |         ^^^^^

error: aborting due to 17 previous errors
---
diff of stderr:

2   --> $DIR/unresolved_static_type_field.rs:9:11
3    |
4 LL |         f(cx);
-    |           ^^ a field by this name exists in `Self`
+    |           ^^ help: you might have meant to use the available field: `self.cx`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/unresolved_static_type_field/unresolved_static_type_field.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/unresolved_static_type_field.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/unresolved_static_type_field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/unresolved_static_type_field" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/unresolved_static_type_field/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find value `cx` in this scope
   |
   |
LL |         f(cx);
   |           ^^ help: you might have meant to use the available field: `self.cx`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.

---
diff of stderr:

2   --> $DIR/issue-34255-1.rs:7:9
3    |
4 LL |         input_cells: Vec::new()
-    |         ^^^^^^^^^^^ a field by this name exists in `Self`
+    |         ^^^^^^^^^^^ help: you might have meant to use the available field: `self.input_cells`
6 
7 error[E0214]: parenthesized type parameters may only be used with a `Fn` trait


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-34255-1/issue-34255-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type/ascription/issue-34255-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/ascription/issue-34255-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-34255-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/ascription/issue-34255-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find value `input_cells` in this scope
  --> /checkout/src/test/ui/type/ascription/issue-34255-1.rs:7:9
   |
LL |         input_cells: Vec::new()
   |         ^^^^^^^^^^^ help: you might have meant to use the available field: `self.input_cells`

error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
   |
   |
LL |         input_cells: Vec::new()
   |                           ^^^^^ only `Fn` traits may use parentheses

error[E0107]: wrong number of type arguments: expected at least 1, found 0
   |
   |
LL |         input_cells: Vec::new()
   |                      ^^^^^^^^^^ expected at least 1 type argument
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0107, E0214, E0425.
For more information about an error, try `rustc --explain E0107`.
---
test result: FAILED. 11155 passed; 6 failed; 86 ignored; 0 measured; 0 filtered out; finished in 137.60s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:38
