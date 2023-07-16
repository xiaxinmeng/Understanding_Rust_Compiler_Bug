plain

---- [ui] ui/async-await/no-const-async.rs stdout ----
diff of stderr:

7    |     |     `async` because of this
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
8    |     `const` because of this
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0277]: the trait bound `[static generator@$DIR/no-const-async.rs:4:24: 4:26]: Generator<ResumeTy>` is not satisfied
+    |
+    |
+ LL | pub const async fn x() {}
+    |                        ^^ the trait `Generator<ResumeTy>` is not implemented for `[static generator@$DIR/no-const-async.rs:4:24: 4:26]`
+   ::: $SRC_DIR/core/src/future/mod.rs:LL:COL
+    |
+    |
+ LL |     T: Generator<ResumeTy, Yield = ()>,
+    |        ------------------------------- required by this bound in `from_generator`
+ error: aborting due to 2 previous errors
+ 
+ For more information about this error, try `rustc --explain E0277`.
12 
---
To only update this specific test, also pass `--test-args async-await/no-const-async.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/no-const-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-const-async" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-const-async/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: functions cannot be both `const` and `async`
   |
   |
LL | pub const async fn x() {}
   | ----^^^^^-^^^^^----------
   |     |     |
   |     |     `async` because of this
   |     `const` because of this

error[E0277]: the trait bound `[static generator@/checkout/src/test/ui/async-await/no-const-async.rs:4:24: 4:26]: Generator<ResumeTy>` is not satisfied
   |
   |
LL | pub const async fn x() {}
   |                        ^^ the trait `Generator<ResumeTy>` is not implemented for `[static generator@/checkout/src/test/ui/async-await/no-const-async.rs:4:24: 4:26]`
  ::: /checkout/library/core/src/future/mod.rs:63:8
   |
   |
LL |     T: Generator<ResumeTy, Yield = ()>,
   |        ------------------------------- required by this bound in `from_generator`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/consts/const-eval/issue-49296.rs stdout ----
diff of stderr:

- error[E0080]: evaluation of constant value failed
-   --> $DIR/issue-49296.rs:19:16
+ error[E0277]: the trait bound `&u64: Copy` is not satisfied
3    |
3    |
- LL | const X: u64 = *wat(42);
-    |                ^^^^^^^^ pointer to alloc2 was dereferenced after this allocation got freed
+ LL | const unsafe fn transmute<T: Copy, U: Copy>(t: T) -> U {
+    |                              ---- required by this bound in `transmute`
+ ...
+ LL |     unsafe { transmute(&x) }
+    |                        ^^ the trait `Copy` is not implemented for `&u64`
+    |
+ help: consider removing the leading `&`-reference
+    |
+ LL |     unsafe { transmute(x) }
+    |                       --
+ help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
+    |
+ LL | const fn wat(x: u64) -> &'static u64 where &u64: Copy {
6 
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0277]: the trait bound `&u64: Copy` is not satisfied
+    |
+    |
+ LL | const unsafe fn transmute<T: Copy, U: Copy>(t: T) -> U {
+    |                                       ---- required by this bound in `transmute`
+ ...
+ LL |     unsafe { transmute(&x) }
+    |              ^^^^^^^^^ the trait `Copy` is not implemented for `&u64`
+    |
+ help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
+    |
+ LL | const fn wat(x: u64) -> &'static u64 where &u64: Copy {
8 
- For more information about this error, try `rustc --explain E0080`.
+ error: aborting due to 2 previous errors
+ 
---
To only update this specific test, also pass `--test-args consts/const-eval/issue-49296.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-49296.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-49296" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-49296/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `&u64: Copy` is not satisfied
   |
   |
LL | const unsafe fn transmute<T: Copy, U: Copy>(t: T) -> U {
   |                              ---- required by this bound in `transmute`
...
LL |     unsafe { transmute(&x) }
   |                        ^^ the trait `Copy` is not implemented for `&u64`
   |
help: consider removing the leading `&`-reference
LL |     unsafe { transmute(x) }
   |                       --
   |                       --
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
   |
LL | const fn wat(x: u64) -> &'static u64 where &u64: Copy {


error[E0277]: the trait bound `&u64: Copy` is not satisfied
   |
   |
LL | const unsafe fn transmute<T: Copy, U: Copy>(t: T) -> U {
   |                                       ---- required by this bound in `transmute`
...
LL |     unsafe { transmute(&x) }
   |              ^^^^^^^^^ the trait `Copy` is not implemented for `&u64`
   |
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
   |
LL | const fn wat(x: u64) -> &'static u64 where &u64: Copy {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/parser/fn-header-semantic-fail.rs stdout ----
diff of stderr:

180    |         |     `async` because of this
181    |         `const` because of this
182 
+ error[E0277]: the trait bound `[static generator@$DIR/fn-header-semantic-fail.rs:12:44: 12:46]: Generator<ResumeTy>` is not satisfied
+   --> $DIR/fn-header-semantic-fail.rs:12:44
+    |
+ LL |     const async unsafe extern "C" fn ff5() {} // OK.
+    |                                            ^^ the trait `Generator<ResumeTy>` is not implemented for `[static generator@$DIR/fn-header-semantic-fail.rs:12:44: 12:46]`
+   ::: $SRC_DIR/core/src/future/mod.rs:LL:COL
+    |
+    |
+ LL |     T: Generator<ResumeTy, Yield = ()>,
+    |        ------------------------------- required by this bound in `from_generator`
183 error[E0053]: method `ft1` has an incompatible type for trait
184   --> $DIR/fn-header-semantic-fail.rs:28:24
185    |


212    = note: expected fn pointer `unsafe extern "C" fn()`
213               found fn pointer `unsafe extern "C" fn() -> impl Future`
- error: aborting due to 20 previous errors
- error: aborting due to 20 previous errors
+ error[E0277]: the trait bound `[static generator@$DIR/fn-header-semantic-fail.rs:33:48: 33:50]: Generator<ResumeTy>` is not satisfied
+   --> $DIR/fn-header-semantic-fail.rs:33:48
+    |
+ LL |         const async unsafe extern "C" fn ft5() {}
+    |                                                ^^ the trait `Generator<ResumeTy>` is not implemented for `[static generator@$DIR/fn-header-semantic-fail.rs:33:48: 33:50]`
+   ::: $SRC_DIR/core/src/future/mod.rs:LL:COL
+    |
+    |
+ LL |     T: Generator<ResumeTy, Yield = ()>,
+    |        ------------------------------- required by this bound in `from_generator`
- Some errors have detailed explanations: E0053, E0379, E0706.
- Some errors have detailed explanations: E0053, E0379, E0706.
+ error[E0277]: the trait bound `[static generator@$DIR/fn-header-semantic-fail.rs:45:48: 45:50]: Generator<ResumeTy>` is not satisfied
+   --> $DIR/fn-header-semantic-fail.rs:45:48
+    |
+ LL |         const async unsafe extern "C" fn fi5() {}
+    |                                                ^^ the trait `Generator<ResumeTy>` is not implemented for `[static generator@$DIR/fn-header-semantic-fail.rs:45:48: 45:50]`
+   ::: $SRC_DIR/core/src/future/mod.rs:LL:COL
+    |
+    |
+ LL |     T: Generator<ResumeTy, Yield = ()>,
+    |        ------------------------------- required by this bound in `from_generator`
+ error: aborting due to 23 previous errors
+ 
+ Some errors have detailed explanations: E0053, E0277, E0379, E0706.
218 For more information about an error, try `rustc --explain E0053`.
---
To only update this specific test, also pass `--test-args parser/fn-header-semantic-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/fn-header-semantic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-header-semantic-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-header-semantic-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: functions cannot be both `const` and `async`
   |
   |
LL |     const async unsafe extern "C" fn ff5() {} // OK.
   |     ^^^^^-^^^^^------------------------------
   |     |     |
   |     |     `async` because of this
   |     `const` because of this
error[E0706]: functions in traits cannot be declared `async`
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:16:9
   |
   |
LL |         async fn ft1(); //~ ERROR functions in traits cannot be declared `async`
   |         |
   |         |
   |         `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:18:9
   |
   |
LL |         const fn ft3(); //~ ERROR functions in traits cannot be declared const

error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:20:9
   |
   |
LL |         const async unsafe extern "C" fn ft5();

error[E0706]: functions in traits cannot be declared `async`
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:20:9
   |
   |
LL |         const async unsafe extern "C" fn ft5();
   |               |
   |               |
   |               `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait

error: functions cannot be both `const` and `async`
   |
   |
LL |         const async unsafe extern "C" fn ft5();
   |         ^^^^^-^^^^^----------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this
error[E0706]: functions in traits cannot be declared `async`
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:28:9
   |
   |
LL |         async fn ft1() {} //~ ERROR functions in traits cannot be declared `async`
   |         |
   |         |
   |         `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:31:9
   |
   |
LL |         const fn ft3() {} //~ ERROR functions in traits cannot be declared const

error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:33:9
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}

error[E0706]: functions in traits cannot be declared `async`
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:33:9
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |               |
   |               |
   |               `async` because of this
   |
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait

error: functions cannot be both `const` and `async`
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |         ^^^^^-^^^^^------------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this

error: functions cannot be both `const` and `async`
   |
   |
LL |         const async unsafe extern "C" fn fi5() {}
   |         ^^^^^-^^^^^------------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this

error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
LL |         async fn fe1(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe1(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
LL |         async fn fe1(); //~ ERROR functions in `extern` blocks cannot have qualifiers
LL |         unsafe fn fe2(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe2(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
...
LL |         const fn fe3(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe3(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
...
LL |         extern "C" fn fe4(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe4(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
...
LL |         const async unsafe extern "C" fn fe5(); //~ ERROR functions in `extern` blocks
   |
help: remove the qualifiers
   |
   |
LL |         fn fe5(); //~ ERROR functions in `extern` blocks


error: functions cannot be both `const` and `async`
   |
   |
LL |         const async unsafe extern "C" fn fe5(); //~ ERROR functions in `extern` blocks
   |         ^^^^^-^^^^^----------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this

error[E0277]: the trait bound `[static generator@/checkout/src/test/ui/parser/fn-header-semantic-fail.rs:12:44: 12:46]: Generator<ResumeTy>` is not satisfied
   |
   |
LL |     const async unsafe extern "C" fn ff5() {} // OK.
   |                                            ^^ the trait `Generator<ResumeTy>` is not implemented for `[static generator@/checkout/src/test/ui/parser/fn-header-semantic-fail.rs:12:44: 12:46]`
  ::: /checkout/library/core/src/future/mod.rs:63:8
   |
   |
LL |     T: Generator<ResumeTy, Yield = ()>,
   |        ------------------------------- required by this bound in `from_generator`
error[E0053]: method `ft1` has an incompatible type for trait
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:28:24
   |
   |
LL |         async fn ft1(); //~ ERROR functions in traits cannot be declared `async`
   |                       - type in trait
...
LL |         async fn ft1() {} //~ ERROR functions in traits cannot be declared `async`
   |                        |
   |                        |
   |                        checked the `Output` of this `async fn`, found opaque type
   |
   |
   = note: while checking the return type of the `async fn`
   = note: expected fn pointer `fn()`
              found fn pointer `fn() -> impl Future`
error[E0053]: method `ft5` has an incompatible type for trait
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:33:48
   |
   |
LL |         const async unsafe extern "C" fn ft5();
   |                                               - type in trait
...
LL |         const async unsafe extern "C" fn ft5() {}
   |                                                |
   |                                                |
   |                                                checked the `Output` of this `async fn`, found opaque type
   |
   |
   = note: while checking the return type of the `async fn`
   = note: expected fn pointer `unsafe extern "C" fn()`
              found fn pointer `unsafe extern "C" fn() -> impl Future`

error[E0277]: the trait bound `[static generator@/checkout/src/test/ui/parser/fn-header-semantic-fail.rs:33:48: 33:50]: Generator<ResumeTy>` is not satisfied
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |                                                ^^ the trait `Generator<ResumeTy>` is not implemented for `[static generator@/checkout/src/test/ui/parser/fn-header-semantic-fail.rs:33:48: 33:50]`
  ::: /checkout/library/core/src/future/mod.rs:63:8
   |
   |
LL |     T: Generator<ResumeTy, Yield = ()>,
   |        ------------------------------- required by this bound in `from_generator`

error[E0277]: the trait bound `[static generator@/checkout/src/test/ui/parser/fn-header-semantic-fail.rs:45:48: 45:50]: Generator<ResumeTy>` is not satisfied
   |
   |
LL |         const async unsafe extern "C" fn fi5() {}
   |                                                ^^ the trait `Generator<ResumeTy>` is not implemented for `[static generator@/checkout/src/test/ui/parser/fn-header-semantic-fail.rs:45:48: 45:50]`
  ::: /checkout/library/core/src/future/mod.rs:63:8
   |
   |
LL |     T: Generator<ResumeTy, Yield = ()>,
   |        ------------------------------- required by this bound in `from_generator`
error: aborting due to 23 previous errors

Some errors have detailed explanations: E0053, E0277, E0379, E0706.
For more information about an error, try `rustc --explain E0053`.
---
test result: FAILED. 12070 passed; 3 failed; 102 ignored; 0 measured; 0 filtered out; finished in 106.77s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:55
