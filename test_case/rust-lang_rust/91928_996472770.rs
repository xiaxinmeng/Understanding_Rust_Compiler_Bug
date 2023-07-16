plain
.................................................................................................... 1200/12489
........i........................................................................................... 1300/12489
........................................i........................................................... 1400/12489
.................................................................................................... 1500/12489
.............................F.....................F................................................ 1600/12489
.................................................................................................... 1800/12489
............i....................................................................................... 1900/12489
.................................................................................................... 2000/12489
..............................................................................i..................... 2100/12489
---
.................................................................................................... 5700/12489
.................................................................................................... 5800/12489
.................................................................................................... 5900/12489
.................................................................................................... 6000/12489
...................F..F.......i..................................................................... 6100/12489
...........i........................................................................................ 6300/12489
............................................................................i....................... 6400/12489
...........................ii.ii........i...i....................................................... 6500/12489
.................................................................................................... 6600/12489
---

11 note: required by a bound in `Option::<T>::or_else`
12   --> $SRC_DIR/core/src/option.rs:LL:COL
13    |
- LL |     pub fn or_else<F: FnOnce() -> Option<T>>(self, f: F) -> Option<T> {
-    |                       ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Option::<T>::or_else`
+ LL |         F: ~const FnOnce() -> Option<T>,
+    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Option::<T>::or_else`
17 error: aborting due to previous error
18 


---
To only update this specific test, also pass `--test-args closures/closure-expected.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-expected.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-expected" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-expected/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: expected a `FnOnce<()>` closure, found `{integer}`
   |
   |
LL |     let y = x.or_else(4);
   |               ------- ^ expected an `FnOnce<()>` closure, found `{integer}`
   |               required by a bound introduced by this call
   |
   |
   = help: the trait `FnOnce<()>` is not implemented for `{integer}`
   = note: wrap the `{integer}` in a closure with no arguments: `|| { /* code */ }`
note: required by a bound in `Option::<T>::or_else`
   |
   |
LL |         F: ~const FnOnce() -> Option<T>,
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Option::<T>::or_else`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.

---

10 note: required by a bound in `Option::<T>::map`
11   --> $SRC_DIR/core/src/option.rs:LL:COL
12    |
- LL |     pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> {
-    |                      ^^^^^^^^^^^^^^ required by this bound in `Option::<T>::map`
+ LL |         F: ~const FnOnce(T) -> U,
+    |            ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Option::<T>::map`
16 error: aborting due to previous error
17 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/coerce-unsafe-to-closure/coerce-unsafe-to-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/coerce-unsafe-to-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/coerce-unsafe-to-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/coerce-unsafe-to-closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/coerce-unsafe-to-closure/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: expected a `FnOnce<(&str,)>` closure, found `unsafe extern "rust-intrinsic" fn(_) -> _ {transmute::<_, _>}`
   |
   |
LL |     let x: Option<&[u8]> = Some("foo").map(std::mem::transmute);
   |                                        --- ^^^^^^^^^^^^^^^^^^^ expected an `FnOnce<(&str,)>` closure, found `unsafe extern "rust-intrinsic" fn(_) -> _ {transmute::<_, _>}`
   |                                        required by a bound introduced by this call
   |
   |
   = help: the trait `FnOnce<(&str,)>` is not implemented for `unsafe extern "rust-intrinsic" fn(_) -> _ {transmute::<_, _>}`
note: required by a bound in `Option::<T>::map`
   |
   |
LL |         F: ~const FnOnce(T) -> U,
   |            ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Option::<T>::map`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.

---

23 note: required by a bound in `Option::<T>::and_then`
24   --> $SRC_DIR/core/src/option.rs:LL:COL
25    |
- LL |     pub fn and_then<U, F: FnOnce(T) -> Option<U>>(self, f: F) -> Option<U> {
-    |                           ^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Option::<T>::and_then`
+ LL |         F: ~const FnOnce(T) -> Option<U>,
+    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Option::<T>::and_then`
29 error: aborting due to 2 previous errors
30 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr/malformed_closure/ruby_style_closure/ruby_style_closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args expr/malformed_closure/ruby_style_closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/expr/malformed_closure/ruby_style_closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr/malformed_closure/ruby_style_closure" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr/malformed_closure/ruby_style_closure/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find value `x` in this scope
  --> /checkout/src/test/ui/expr/malformed_closure/ruby_style_closure.rs:13:14
   |
LL |         Some(x * 2)


error[E0277]: expected a `FnOnce<({integer},)>` closure, found `Option<_>`
   |
LL |       let p = Some(45).and_then({
   |  ______________________--------_^
   | |                      |
   | |                      |
   | |                      required by a bound introduced by this call
LL | |         //~^ expected a `FnOnce<({integer},)>` closure, found `Option<_>`
LL | |         |x| println!("doubling {}", x);
LL | |         Some(x * 2)
   | |         -----------
LL | |         //~^ ERROR: cannot find value `x` in this scope
LL | |     });
   | |_____^ expected an `FnOnce<({integer},)>` closure, found `Option<_>`
   |
   = help: the trait `FnOnce<({integer},)>` is not implemented for `Option<_>`
note: required by a bound in `Option::<T>::and_then`
   |
   |
LL |         F: ~const FnOnce(T) -> Option<U>,
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Option::<T>::and_then`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0425.
For more information about an error, try `rustc --explain E0277`.
---

11 note: required by a bound in `Option::<T>::map`
12   --> $SRC_DIR/core/src/option.rs:LL:COL
13    |
- LL |     pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> {
-    |                      ^^^^^^^^^^^^^^ required by this bound in `Option::<T>::map`
+ LL |         F: ~const FnOnce(T) -> U,
+    |            ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Option::<T>::map`
17 error: aborting due to previous error
18 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47706-trait/issue-47706-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-47706-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-47706-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47706-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47706-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0593]: function is expected to take a single 0-tuple as argument, but it takes 2 distinct arguments
   |
   |
LL |     fn f(&self, _: ()) {
   |     ------------------ takes 2 distinct arguments
LL |         None::<()>.map(Self::f);
   |                    --- ^^^^^^^ expected function that takes a single 0-tuple as argument
   |                    required by a bound introduced by this call
   |
note: required by a bound in `Option::<T>::map`
  --> /checkout/library/core/src/option.rs:869:12
  --> /checkout/library/core/src/option.rs:869:12
   |
LL |         F: ~const FnOnce(T) -> U,
   |            ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Option::<T>::map`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0593`.

---

12 note: required by a bound in `Option::<T>::map`
13   --> $SRC_DIR/core/src/option.rs:LL:COL
14    |
- LL |     pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> {
-    |                      ^^^^^^^^^^^^^^ required by this bound in `Option::<T>::map`
+ LL |         F: ~const FnOnce(T) -> U,
+    |            ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Option::<T>::map`
18 error[E0593]: function is expected to take 0 arguments, but it takes 1 argument
19   --> $DIR/issue-47706.rs:27:9



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47706/issue-47706.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-47706.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-47706.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47706" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47706/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0593]: function is expected to take 1 argument, but it takes 2 arguments
  --> /checkout/src/test/ui/issues/issue-47706.rs:11:22
   |
LL |     pub fn new(foo: Option<i32>, _: ()) -> Foo {
   |     ------------------------------------------ takes 2 arguments
...
LL |         self.foo.map(Foo::new)
   |                  --- ^^^^^^^^ expected function that takes 1 argument
   |                  required by a bound introduced by this call
   |
note: required by a bound in `Option::<T>::map`
  --> /checkout/library/core/src/option.rs:869:12
  --> /checkout/library/core/src/option.rs:869:12
   |
LL |         F: ~const FnOnce(T) -> U,
   |            ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Option::<T>::map`
error[E0593]: function is expected to take 0 arguments, but it takes 1 argument
  --> /checkout/src/test/ui/issues/issue-47706.rs:27:9
   |
LL |     Bar(i32),
LL |     Bar(i32),
   |     -------- takes 1 argument
...
LL |     foo(Qux::Bar);
   |     --- ^^^^^^^^ expected function that takes 0 arguments
   |     required by a bound introduced by this call
   |
note: required by a bound in `foo`
  --> /checkout/src/test/ui/issues/issue-47706.rs:22:8
  --> /checkout/src/test/ui/issues/issue-47706.rs:22:8
   |
LL | fn foo<F>(f: F)
   |    --- required by a bound in this
LL | where
LL |     F: Fn(),
   |        ^^^^ required by this bound in `foo`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0593`.


------------------------------------------


---- [ui] ui/suggestions/as-ref-2.rs stdout ----
diff of stderr:

11 note: this function takes ownership of the receiver `self`, which moves `foo`
12   --> $SRC_DIR/core/src/option.rs:LL:COL
13    |
- LL |     pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U> {
-    |                                      ^^^^
+ LL |     pub const fn map<U, F>(self, f: F) -> Option<U>
+    |                            ^^^^
16 help: consider calling `.as_ref()` to borrow the type's contents
17    |
18 LL |     let _x: Option<Struct> = foo.as_ref().map(|s| bar(&s));

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/as-ref-2/as-ref-2.stderr
To only update this specific test, also pass `--test-args suggestions/as-ref-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/as-ref-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/as-ref-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/as-ref-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: use of moved value: `foo`
  --> /checkout/src/test/ui/suggestions/as-ref-2.rs:12:14
   |
LL |     let foo = Some(Struct);
   |         --- move occurs because `foo` has type `Option<Struct>`, which does not implement the `Copy` trait
LL |     let _x: Option<Struct> = foo.map(|s| bar(&s));
   |                                  ---------------- `foo` moved due to this method call
LL |     let _y = foo; //~ERROR use of moved value: `foo`
   |              ^^^ value used here after move
   |
note: this function takes ownership of the receiver `self`, which moves `foo`
   |
   |
LL |     pub const fn map<U, F>(self, f: F) -> Option<U>
   |                            ^^^^
help: consider calling `.as_ref()` to borrow the type's contents
   |
LL |     let _x: Option<Struct> = foo.as_ref().map(|s| bar(&s));

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
---
test result: FAILED. 12364 passed; 6 failed; 119 ignored; 0 measured; 0 filtered out; finished in 160.32s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:41
