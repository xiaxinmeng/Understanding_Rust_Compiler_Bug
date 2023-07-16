plain
.................................................................................................... 9300/11542
.................................................................................................... 9400/11542
.......................................................................i......i..................... 9500/11542
.................................................................................................... 9600/11542
.................iiiiiii..iiiiii.i.................................................................. 9700/11542
.................................................................................................... 9900/11542
.................................................................................................... 10000/11542
.................................................................................................... 10100/11542
.................................................................................................... 10200/11542
---

---- [ui] ui/associated-types/defaults-unsound-62211-1.rs stdout ----
diff of stderr:

- error[E0277]: `Self` doesn't implement `std::fmt::Display`
-   --> $DIR/defaults-unsound-62211-1.rs:20:5
-    |
- LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
-    |     |                                                                                |
-    |     |                                                                                |
-    |     |                                                                                required by this bound in `UncheckedCopy::Output`
-    |     `Self` cannot be formatted with the default formatter
-    |
-    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
- help: consider further restricting `Self`
-    |
- LL | trait UncheckedCopy: Sized + std::fmt::Display {
- 
16 error[E0277]: the trait bound `Self: Deref` is not satisfied
17   --> $DIR/defaults-unsound-62211-1.rs:20:5
18    |
18    |

54    |
55 LL | trait UncheckedCopy: Sized + Copy {
+ 
+ 
+ error[E0277]: `Self` doesn't implement `std::fmt::Display`
+    |
+    |
+ LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
+    |     |                                                                                |
+    |     |                                                                                |
+    |     |                                                                                required by this bound in `UncheckedCopy::Output`
+    |     `Self` cannot be formatted with the default formatter
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
+ help: consider further restricting `Self`
+    |
+ LL | trait UncheckedCopy: Sized + std::fmt::Display {
57 
58 error: aborting due to 4 previous errors
59 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-1/defaults-unsound-62211-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/defaults-unsound-62211-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-unsound-62211-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `Self: Deref` is not satisfied
  --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-1.rs:20:5
   |
LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
   |     |                   |
   |     |                   |
   |     |                   required by this bound in `UncheckedCopy::Output`
   |     the trait `Deref` is not implemented for `Self`
help: consider further restricting `Self`
   |
   |
LL | trait UncheckedCopy: Sized + Deref {


error[E0277]: cannot add-assign `&'static str` to `Self`
   |
   |
LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
   |     |                                         |
   |     |                                         |
   |     |                                         required by this bound in `UncheckedCopy::Output`
   |     no implementation for `Self += &'static str`
help: consider further restricting `Self`
   |
   |
LL | trait UncheckedCopy: Sized + AddAssign<&'static str> {


error[E0277]: the trait bound `Self: Copy` is not satisfied
   |
   |
LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
   |     |            |
   |     |            |
   |     |            required by this bound in `UncheckedCopy::Output`
   |     the trait `Copy` is not implemented for `Self`
help: consider further restricting `Self`
   |
   |
LL | trait UncheckedCopy: Sized + Copy {


error[E0277]: `Self` doesn't implement `std::fmt::Display`
   |
   |
LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
   |     |                                                                                |
   |     |                                                                                |
   |     |                                                                                required by this bound in `UncheckedCopy::Output`
   |     `Self` cannot be formatted with the default formatter
   |
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
help: consider further restricting `Self`
   |
LL | trait UncheckedCopy: Sized + std::fmt::Display {

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/associated-types/defaults-unsound-62211-2.rs stdout ----
diff of stderr:

- error[E0277]: `Self` doesn't implement `std::fmt::Display`
-   --> $DIR/defaults-unsound-62211-2.rs:20:5
-    |
- LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
-    |     |                                                                                |
-    |     |                                                                                |
-    |     |                                                                                required by this bound in `UncheckedCopy::Output`
-    |     `Self` cannot be formatted with the default formatter
-    |
-    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
- help: consider further restricting `Self`
-    |
- LL | trait UncheckedCopy: Sized + std::fmt::Display {
- 
16 error[E0277]: the trait bound `Self: Deref` is not satisfied
17   --> $DIR/defaults-unsound-62211-2.rs:20:5
18    |
18    |

54    |
55 LL | trait UncheckedCopy: Sized + Copy {
+ 
+ 
+ error[E0277]: `Self` doesn't implement `std::fmt::Display`
+    |
+    |
+ LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
+    |     |                                                                                |
+    |     |                                                                                |
+    |     |                                                                                required by this bound in `UncheckedCopy::Output`
+    |     `Self` cannot be formatted with the default formatter
+    |
+    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
+ help: consider further restricting `Self`
+    |
+ LL | trait UncheckedCopy: Sized + std::fmt::Display {
57 
58 error: aborting due to 4 previous errors
59 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-2/defaults-unsound-62211-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/defaults-unsound-62211-2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-unsound-62211-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-unsound-62211-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `Self: Deref` is not satisfied
  --> /checkout/src/test/ui/associated-types/defaults-unsound-62211-2.rs:20:5
   |
LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
   |     |                   |
   |     |                   |
   |     |                   required by this bound in `UncheckedCopy::Output`
   |     the trait `Deref` is not implemented for `Self`
help: consider further restricting `Self`
   |
   |
LL | trait UncheckedCopy: Sized + Deref {


error[E0277]: cannot add-assign `&'static str` to `Self`
   |
   |
LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
   |     |                                         |
   |     |                                         |
   |     |                                         required by this bound in `UncheckedCopy::Output`
   |     no implementation for `Self += &'static str`
help: consider further restricting `Self`
   |
   |
LL | trait UncheckedCopy: Sized + AddAssign<&'static str> {


error[E0277]: the trait bound `Self: Copy` is not satisfied
   |
   |
LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
   |     |            |
   |     |            |
   |     |            required by this bound in `UncheckedCopy::Output`
   |     the trait `Copy` is not implemented for `Self`
help: consider further restricting `Self`
   |
   |
LL | trait UncheckedCopy: Sized + Copy {


error[E0277]: `Self` doesn't implement `std::fmt::Display`
   |
   |
LL |     type Output: Copy + Deref<Target = str> + AddAssign<&'static str> + From<Self> + Display = Self;
   |     |                                                                                |
   |     |                                                                                |
   |     |                                                                                required by this bound in `UncheckedCopy::Output`
   |     `Self` cannot be formatted with the default formatter
   |
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
help: consider further restricting `Self`
   |
LL | trait UncheckedCopy: Sized + std::fmt::Display {

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/associated-types/trait-with-supertraits-needing-sized-self.rs stdout ----
diff of stderr:

1 error[E0277]: the size for values of type `Self` cannot be known at compilation time
-   --> $DIR/trait-with-supertraits-needing-sized-self.rs:3:22
3    |
3    |
4 LL | trait ArithmeticOps: Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> {}
-    |                      ^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
+    |                                         ^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
6    | 
7   ::: $SRC_DIR/core/src/ops/arith.rs:LL:COL


- LL | pub trait Add<Rhs = Self> {
-    |               --- required by this bound in `Add`
+ LL | pub trait Sub<Rhs = Self> {
+    |               --- required by this bound in `Sub`
12 help: consider further restricting `Self`
13    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/trait-with-supertraits-needing-sized-self/trait-with-supertraits-needing-sized-self.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/trait-with-supertraits-needing-sized-self.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/trait-with-supertraits-needing-sized-self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/trait-with-supertraits-needing-sized-self" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/trait-with-supertraits-needing-sized-self/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `Self` cannot be known at compilation time
   |
   |
LL | trait ArithmeticOps: Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> {}
   |                                         ^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  ::: /checkout/library/core/src/ops/arith.rs:181:15
   |
   |
LL | pub trait Sub<Rhs = Self> {
   |               --- required by this bound in `Sub`
help: consider further restricting `Self`
   |
   |
LL | trait ArithmeticOps: Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Sized {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/suggestions/missing-trait-bounds-for-method-call.rs stdout ----
diff of stderr:

8    |              ^^^ method cannot be called on `&Foo<T>` due to unsatisfied trait bounds
10    = note: the following trait bounds were not satisfied:
10    = note: the following trait bounds were not satisfied:
-            `T: Bar`
-            which is required by `Foo<T>: Bar`
13            `T: Default`
+            which is required by `Foo<T>: Bar`
+            `T: Bar`
14            which is required by `Foo<T>: Bar`
15 help: consider restricting the type parameters to satisfy the trait bounds


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-trait-bounds-for-method-call/missing-trait-bounds-for-method-call.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-trait-bounds-for-method-call/missing-trait-bounds-for-method-call.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/missing-trait-bounds-for-method-call.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/missing-trait-bounds-for-method-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-trait-bounds-for-method-call" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-trait-bounds-for-method-call/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: the method `foo` exists for reference `&Foo<T>`, but its trait bounds were not satisfied
   |
   |
LL | struct Foo<T> {
   | ------------- doesn't satisfy `Foo<T>: Bar`
...
LL |         self.foo();
   |              ^^^ method cannot be called on `&Foo<T>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
           `T: Default`
           `T: Default`
           which is required by `Foo<T>: Bar`
           `T: Bar`
           which is required by `Foo<T>: Bar`
help: consider restricting the type parameters to satisfy the trait bounds
   |
LL | struct Foo<T> where T: Bar, T: Default {


error[E0599]: the method `foo` exists for reference `&Fin<T>`, but its trait bounds were not satisfied
   |
   |
LL | struct Fin<T> where T: Bar {
   | -------------------------- doesn't satisfy `Fin<T>: Bar`
...
LL |         self.foo();
   |              ^^^ method cannot be called on `&Fin<T>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
           `T: Default`
           `T: Default`
           which is required by `Fin<T>: Bar`
help: consider restricting the type parameter to satisfy the trait bound
   |
LL | struct Fin<T> where T: Bar, T: Default {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.

------------------------------------------


---- [ui] ui/traits/item-privacy.rs stdout ----
diff of stderr:

116    |     ^^^^ `assoc_const::C` cannot be made into an object
117    |
118    = help: consider moving `C` to another trait
-    = help: consider moving `B` to another trait
120    = help: consider moving `A` to another trait
+    = help: consider moving `B` to another trait
121 note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
123    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/item-privacy/item-privacy.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/item-privacy.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/item-privacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/item-privacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/item-privacy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: no method named `a` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- method `a` not found for this
...
LL |     S.a(); //~ ERROR no method named `a` found
   |       ^ method not found in `S`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `method::A` defines an item `a`, perhaps you need to implement it
   |
LL |     trait A {
   |     ^^^^^^^


error[E0599]: no method named `b` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- method `b` not found for this
...
LL |     S.b(); //~ ERROR no method named `b` found
   |       ^ method not found in `S`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use method::B;
LL | use method::B;
   |

error[E0624]: associated function `a` is private
   |
   |
LL |     c.a(); //~ ERROR associated function `a` is private
   |       ^ private associated function

error[E0599]: no function or associated item named `a` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- function or associated item `a` not found for this
...
LL |     S::a(&S);
   |        ^ function or associated item not found in `S`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `method::A` defines an item `a`, perhaps you need to implement it
   |
LL |     trait A {
   |     ^^^^^^^


error[E0599]: no function or associated item named `b` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- function or associated item `b` not found for this
...
LL |     S::b(&S);
   |        ^ function or associated item not found in `S`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use method::B;
LL | use method::B;
   |

error[E0624]: associated function `a` is private
   |
   |
LL |     C::a(&S); //~ ERROR associated function `a` is private
   |        ^ private associated function

error[E0599]: no associated item named `A` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- associated item `A` not found for this
...
LL |     S::A; //~ ERROR no associated item named `A` found
   |        ^ associated item not found in `S`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `assoc_const::A` defines an item `A`, perhaps you need to implement it
   |
LL |     trait A {
   |     ^^^^^^^


error[E0599]: no associated item named `B` found for struct `S` in the current scope
---
test result: FAILED. 11444 passed; 5 failed; 93 ignored; 0 measured; 0 filtered out; finished in 136.73s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:54
