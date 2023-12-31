plain
running 12219 tests
.................................................................................................... 100/12219
........................................iiiiiiiiiii...........i..........ii....ii................... 200/12219
.................................................................................................... 300/12219
..................................................................................................F. 400/12219
.......F...F........................................................................................ 500/12219
.................................................................................................... 600/12219
................F.............................F..........................i.......................... 700/12219
.................................................................................................... 900/12219
.................................................................................................... 1000/12219
.................................................................................................... 1100/12219
..................................................................i................................. 1200/12219
---
diff of stderr:

2   --> $DIR/project-fn-ret-contravariant.rs:45:5
3    |
4 LL | fn transmute<'a,'b>(x: &'a u32, y: &'b u32) -> (&'a u32, &'b u32) {
-    |                        |
-    |                        this parameter and the return type are declared with different lifetimes...
+    |                                    -------     ------------------
+    |                                    |
+    |                                    |
+    |                                    this parameter and the return type are declared with different lifetimes...
8 ...
9 LL |    (a, b)
10    |     ^ ...but data from `y` is returned here
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
13   --> $DIR/project-fn-ret-contravariant.rs:45:8
14    |
14    |
15 LL | fn transmute<'a,'b>(x: &'a u32, y: &'b u32) -> (&'a u32, &'b u32) {
-    |                                    |
-    |                                    this parameter and the return type are declared with different lifetimes...
+    |                        -------                 ------------------
+    |                        |
+    |                        |
+    |                        this parameter and the return type are declared with different lifetimes...
19 ...
20 LL |    (a, b)
21    |        ^ ...but data from `x` is returned here

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-contravariant.krisskross/project-fn-ret-contravariant.krisskross.stderr
To only update this specific test, also pass `--test-args associated-types/cache/project-fn-ret-contravariant.rs`


error in revision `krisskross`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/cache/project-fn-ret-contravariant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "krisskross" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-contravariant.krisskross" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-contravariant.krisskross/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0623]: lifetime mismatch
  --> /checkout/src/test/ui/associated-types/cache/project-fn-ret-contravariant.rs:45:5
   |
LL | fn transmute<'a,'b>(x: &'a u32, y: &'b u32) -> (&'a u32, &'b u32) {
   |                                    |
   |                                    this parameter and the return type are declared with different lifetimes...
...
...
LL |    (a, b) //[krisskross]~ ERROR lifetime mismatch [E0623]
   |     ^ ...but data from `y` is returned here
error[E0623]: lifetime mismatch
  --> /checkout/src/test/ui/associated-types/cache/project-fn-ret-contravariant.rs:45:8
   |
   |
LL | fn transmute<'a,'b>(x: &'a u32, y: &'b u32) -> (&'a u32, &'b u32) {
   |                        |
   |                        this parameter and the return type are declared with different lifetimes...
...
...
LL |    (a, b) //[krisskross]~ ERROR lifetime mismatch [E0623]
   |        ^ ...but data from `x` is returned here
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0623`.

---
diff of stderr:

2   --> $DIR/project-fn-ret-invariant.rs:54:22
3    |
4 LL | fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
-    |                                      |
-    |                                      this parameter and the return type are declared with different lifetimes...
+    |                         --------                  --------------------
+    |                         |
+    |                         |
+    |                         this parameter and the return type are declared with different lifetimes...
8 LL |     let a = bar(foo, y);
9    |                      ^ ...but data from `x` is returned here

12   --> $DIR/project-fn-ret-invariant.rs:56:9
13    |
13    |
14 LL | fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
-    |                                      |
-    |                                      this parameter and the return type are declared with different lifetimes...
+    |                         --------                  --------------------
+    |                         |
+    |                         |
+    |                         this parameter and the return type are declared with different lifetimes...
18 ...
19 LL |     (a, b)
20    |         ^ ...but data from `x` is returned here

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.krisskross/project-fn-ret-invariant.krisskross.stderr
To only update this specific test, also pass `--test-args associated-types/cache/project-fn-ret-invariant.rs`


error in revision `krisskross`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "krisskross" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.krisskross" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.krisskross/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0623]: lifetime mismatch
  --> /checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs:54:22
   |
LL | fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
   |                         |
   |                         this parameter and the return type are declared with different lifetimes...
   |                         this parameter and the return type are declared with different lifetimes...
LL |     let a = bar(foo, y); //[krisskross]~ ERROR E0623
   |                      ^ ...but data from `x` is returned here
error[E0623]: lifetime mismatch
  --> /checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs:56:9
   |
   |
LL | fn transmute<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
   |                         |
   |                         this parameter and the return type are declared with different lifetimes...
...
...
LL |     (a, b) //[krisskross]~ ERROR E0623
   |         ^ ...but data from `x` is returned here
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0623`.

---
diff of stderr:

2   --> $DIR/project-fn-ret-invariant.rs:40:20
3    |
4 LL | fn baz<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
-    |                                |
-    |                                this parameter and the return type are declared with different lifetimes...
+    |                   --------                  --------------------
+    |                   |
+    |                   |
+    |                   this parameter and the return type are declared with different lifetimes...
8 ...
9 LL |     let b = bar(f, y);
10    |                    ^ ...but data from `x` is returned here

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.oneuse/project-fn-ret-invariant.oneuse.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/cache/project-fn-ret-invariant.rs`

error in revision `oneuse`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "oneuse" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.oneuse" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/cache/project-fn-ret-invariant.oneuse/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0623]: lifetime mismatch
  --> /checkout/src/test/ui/associated-types/cache/project-fn-ret-invariant.rs:40:20
   |
LL | fn baz<'a, 'b>(x: Type<'a>, y: Type<'b>) -> (Type<'a>, Type<'b>) {
   |                   |
   |                   this parameter and the return type are declared with different lifetimes...
...
...
LL |     let b = bar(f, y); //[oneuse]~ ERROR lifetime mismatch [E0623]
   |                    ^ ...but data from `x` is returned here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0623`.

---
diff of stderr:

2   --> $DIR/issue-63388-1.rs:14:9
3    |
4 LL |         &'a self, foo: &dyn Foo
-    |         -------- this parameter and the returned future are declared with different lifetimes...
+    |                        -------- this parameter and the returned future are declared with different lifetimes...
6 LL |     ) -> &dyn Foo
8    |          |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1/issue-63388-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issues/issue-63388-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-63388-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0623]: lifetime mismatch
  --> /checkout/src/test/ui/async-await/issues/issue-63388-1.rs:14:9
   |
LL |         &'a self, foo: &dyn Foo
   |                        -------- this parameter and the returned future are declared with different lifetimes...
LL |     ) -> &dyn Foo
   |          |
   |          |
   |          this `async fn` implicitly returns an `impl Future<Output = &dyn Foo>`
LL |     {
LL |         foo  //~ ERROR lifetime mismatch
   |         ^^^ ...but data from `foo` is held across an await point here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0623`.

---
diff of stderr:

2   --> $DIR/ret-impl-trait-one.rs:10:65
3    |
4 LL | async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> {
-    |                                           |                     |
-    |                                           |                     |
-    |                                           |                     ...but data from `b` is held across an await point here
-    |                                           |                     this `async fn` implicitly returns an `impl Future<Output = impl Trait<'a>>`
-    |                                           this parameter and the returned future are declared with different lifetimes...
+    |                                                      |          |
+    |                                                      |          |
+    |                                                      |          ...but data from `b` is held across an await point here
+    |                                                      |          this `async fn` implicitly returns an `impl Future<Output = impl Trait<'a>>`
+    |                                                      this parameter and the returned future are declared with different lifetimes...
11 error: aborting due to previous error
12 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one/ret-impl-trait-one.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/multiple-lifetimes/ret-impl-trait-one.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0623]: lifetime mismatch
  --> /checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.rs:10:65
   |
LL | async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> {
   |                                                      |          |
   |                                                      |          |
   |                                                      |          ...but data from `b` is held across an await point here
   |                                                      |          this `async fn` implicitly returns an `impl Future<Output = impl Trait<'a>>`
   |                                                      this parameter and the returned future are declared with different lifetimes...
error: aborting due to previous error

For more information about this error, try `rustc --explain E0623`.

---
diff of stderr:

13   --> $DIR/regions-free-region-ordering-callee.rs:18:24
14    |
15 LL | fn ordering3<'a, 'b>(x: &'a usize, y: &'b usize) -> &'a &'b usize {
-    |                                       |
-    |                                       this parameter and the return type are declared with different lifetimes...
+    |                         ---------                   -------------
+    |                         |
+    |                         |
+    |                         this parameter and the return type are declared with different lifetimes...
19 LL |     // Do not infer an ordering from the return value.
20 LL |     let z: &'b usize = &*x;
21    |                        ^^^ ...but data from `x` is returned here

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-free-region-ordering-callee/regions-free-region-ordering-callee.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-free-region-ordering-callee.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-free-region-ordering-callee.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-free-region-ordering-callee" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-free-region-ordering-callee/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0623]: lifetime mismatch
  --> /checkout/src/test/ui/regions/regions-free-region-ordering-callee.rs:13:5
   |
LL | fn ordering2<'a, 'b>(x: &'a &'b usize, y: &'a usize) -> &'b usize {
   |                         |
   |                         this parameter and the return type are declared with different lifetimes...
   |                         this parameter and the return type are declared with different lifetimes...
LL |     // However, it is not safe to assume that 'b <= 'a
LL |     &*y //~ ERROR lifetime mismatch [E0623]
   |     ^^^ ...but data from `x` is returned here
error[E0623]: lifetime mismatch
  --> /checkout/src/test/ui/regions/regions-free-region-ordering-callee.rs:18:24
   |
   |
LL | fn ordering3<'a, 'b>(x: &'a usize, y: &'b usize) -> &'a &'b usize {
   |                         |
   |                         this parameter and the return type are declared with different lifetimes...
LL |     // Do not infer an ordering from the return value.
LL |     // Do not infer an ordering from the return value.
LL |     let z: &'b usize = &*x;
   |                        ^^^ ...but data from `x` is returned here
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0623`.

---
test result: FAILED. 12098 passed; 6 failed; 115 ignored; 0 measured; 0 filtered out; finished in 128.98s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:42
