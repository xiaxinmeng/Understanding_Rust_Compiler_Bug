plain

8 note: required by a bound in `Send`
9   --> $SRC_DIR/core/src/marker.rs:LL:COL
10    |
- LL | pub unsafe auto trait Send {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Send`
+ LL | / pub unsafe auto trait Send {
+ LL | |     // empty.
+ LL | | }
+    | |_^ required by this bound in `Send`
14    |
14    |
15 LL | trait Case1 where <<Self as Case1>::C as Iterator>::Item: Send {

25 note: required by a bound in `Iterator`
26   --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
- LL | pub trait Iterator {
- LL | pub trait Iterator {
-    | ^^^^^^^^^^^^^^^^^^ required by this bound in `Iterator`
+ LL | / pub trait Iterator {
+ LL | |     /// The type of the elements being iterated over.
+ LL | |     #[stable(feature = "rust1", since = "1.0.0")]
+ LL | |     type Item;
+ LL | |     }
+ LL | | }
+ LL | | }
+    | |_^ required by this bound in `Iterator`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
31    |
31    |
32 LL | trait Case1 where <<Self as Case1>::C as Iterator>::Item: Iterator {
42 note: required by a bound in `Sync`
43   --> $SRC_DIR/core/src/marker.rs:LL:COL
44    |
- LL | pub unsafe auto trait Sync {
- LL | pub unsafe auto trait Sync {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Sync`
+ LL | / pub unsafe auto trait Sync {
+ LL | |     // FIXME(estebank): once support to add notes in `rustc_on_unimplemented`
+ LL | |     // lands in beta, and it has been extended to check whether a closure is
+ LL | |     // anywhere in the requirement chain, extend it as such (#48534):
+ LL | |     // Empty
+ LL | | }
+    | |_^ required by this bound in `Sync`
47 help: consider further restricting the associated type
47 help: consider further restricting the associated type
48    |
49 LL | trait Case1 where <<Self as Case1>::C as Iterator>::Item: Sync {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/bad-bounds-on-assoc-in-trait/bad-bounds-on-assoc-in-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-type-bounds/bad-bounds-on-assoc-in-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/bad-bounds-on-assoc-in-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/bad-bounds-on-assoc-in-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/bad-bounds-on-assoc-in-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `<<Self as Case1>::C as Iterator>::Item` cannot be sent between threads safely
   |
   |
LL |     type C: Clone + Iterator<Item: Send + Iterator<Item: for<'a> Lam<&'a u8, App: Debug>> + Sync>;
   |                                    ^^^^ `<<Self as Case1>::C as Iterator>::Item` cannot be sent between threads safely
   |
   = help: the trait `Send` is not implemented for `<<Self as Case1>::C as Iterator>::Item`
note: required by a bound in `Send`
   |
   |
LL | / pub unsafe auto trait Send {
LL | |     // empty.
LL | | }
   | |_^ required by this bound in `Send`
   |
   |
LL | trait Case1 where <<Self as Case1>::C as Iterator>::Item: Send {


error[E0277]: `<<Self as Case1>::C as Iterator>::Item` is not an iterator
   |
   |
LL |     type C: Clone + Iterator<Item: Send + Iterator<Item: for<'a> Lam<&'a u8, App: Debug>> + Sync>;
   |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `<<Self as Case1>::C as Iterator>::Item` is not an iterator
   |
   = help: the trait `Iterator` is not implemented for `<<Self as Case1>::C as Iterator>::Item`
note: required by a bound in `Iterator`
   |
LL | / pub trait Iterator {
LL | |     /// The type of the elements being iterated over.
LL | |     /// The type of the elements being iterated over.
LL | |     #[stable(feature = "rust1", since = "1.0.0")]
LL | |     type Item;
LL | |     }
LL | | }
LL | | }
   | |_^ required by this bound in `Iterator`
   |
   |
LL | trait Case1 where <<Self as Case1>::C as Iterator>::Item: Iterator {


error[E0277]: `<<Self as Case1>::C as Iterator>::Item` cannot be shared between threads safely
   |
   |
LL |     type C: Clone + Iterator<Item: Send + Iterator<Item: for<'a> Lam<&'a u8, App: Debug>> + Sync>;
   |                                                                                             ^^^^ `<<Self as Case1>::C as Iterator>::Item` cannot be shared between threads safely
   |
   = help: the trait `Sync` is not implemented for `<<Self as Case1>::C as Iterator>::Item`
note: required by a bound in `Sync`
   |
   |
LL | / pub unsafe auto trait Sync {
LL | |     // FIXME(estebank): once support to add notes in `rustc_on_unimplemented`
LL | |     // lands in beta, and it has been extended to check whether a closure is
LL | |     // anywhere in the requirement chain, extend it as such (#48534):
LL | |     // Empty
LL | | }
   | |_^ required by this bound in `Sync`
help: consider further restricting the associated type
help: consider further restricting the associated type
   |
LL | trait Case1 where <<Self as Case1>::C as Iterator>::Item: Sync {

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.
---

8 note: required by a bound in `Debug`
9   --> $SRC_DIR/core/src/fmt/mod.rs:LL:COL
10    |
- LL | pub trait Debug {
-    | ^^^^^^^^^^^^^^^ required by this bound in `Debug`
+ LL | / pub trait Debug {
+ LL | |     /// Formats the value using the given formatter.
+ LL | |     ///
+ LL | |     /// # Examples
+ ...  |
+ LL | |     fn fmt(&self, f: &mut Formatter<'_>) -> Result;
+ LL | | }
+    | |_^ required by this bound in `Debug`
14    |
14    |
15 LL | trait Case1 where <<Self as Case1>::A as Iterator>::Item: Debug {
24 note: required by a bound in `Default`
25   --> $SRC_DIR/core/src/default.rs:LL:COL
26    |
26    |
- LL | pub trait Default: Sized {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Default`
+ LL | / pub trait Default: Sized {
+ LL | |     /// Returns the "default value" for a type.
+ LL | |     ///
+ LL | |     /// Default values are often some kind of initial value, identity value, or anything else that
+ ...  |
+ LL | |     fn default() -> Self;
+ LL | | }
+    | |_^ required by this bound in `Default`
30    |
30    |
31 LL | pub trait Foo where <<Self as Foo>::Out as Baz>::Assoc: Default { type Out: Baz<Assoc: Default>; }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/bounds-on-assoc-in-trait/bounds-on-assoc-in-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-type-bounds/bounds-on-assoc-in-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/bounds-on-assoc-in-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/bounds-on-assoc-in-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/bounds-on-assoc-in-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: `<<Self as Case1>::A as Iterator>::Item` doesn't implement `Debug`
   |
   |
LL |     type A: Iterator<Item: Debug>;
   |                            ^^^^^ `<<Self as Case1>::A as Iterator>::Item` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for `<<Self as Case1>::A as Iterator>::Item`
note: required by a bound in `Debug`
   |
LL | / pub trait Debug {
LL | |     /// Formats the value using the given formatter.
LL | |     ///
LL | |     ///
LL | |     /// # Examples
...  |
LL | |     fn fmt(&self, f: &mut Formatter<'_>) -> Result;
LL | | }
   | |_^ required by this bound in `Debug`
   |
   |
LL | trait Case1 where <<Self as Case1>::A as Iterator>::Item: Debug {


error[E0277]: the trait bound `<<Self as Foo>::Out as Baz>::Assoc: Default` is not satisfied
   |
   |
LL | pub trait Foo { type Out: Baz<Assoc: Default>; }
   |                                      ^^^^^^^ the trait `Default` is not implemented for `<<Self as Foo>::Out as Baz>::Assoc`
note: required by a bound in `Default`
  --> /checkout/library/core/src/default.rs:85:1
   |
   |
LL | / pub trait Default: Sized {
LL | |     /// Returns the "default value" for a type.
LL | |     ///
LL | |     /// Default values are often some kind of initial value, identity value, or anything else that
LL | |     fn default() -> Self;
LL | | }
   | |_^ required by this bound in `Default`
help: consider further restricting the associated type
help: consider further restricting the associated type
   |
LL | pub trait Foo where <<Self as Foo>::Out as Baz>::Assoc: Default { type Out: Baz<Assoc: Default>; }

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
---

142 note: required by a bound in `Copy`
143   --> $SRC_DIR/core/src/marker.rs:LL:COL
144    |
- LL | pub trait Copy: Clone {
-    | ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Copy`
+ LL | / pub trait Copy: Clone {
+ LL | |     // Empty.
+ LL | | }
+    | |_^ required by this bound in `Copy`
148    |
148    |
149 LL | trait _Tr3 where <<Self as _Tr3>::A as Iterator>::Item: Copy {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_type_bounds/feature-gate-associated_type_bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-associated_type_bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_type_bounds" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-associated_type_bounds/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:15:22
   |
LL |     type A: Iterator<Item: Copy>;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:19:22
   |
   |
LL |     type B: Iterator<Item: 'static>;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:23:20
   |
   |
LL | struct _St1<T: Tr1<As1: Tr2>> {
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:30:18
   |
   |
LL | enum _En1<T: Tr1<As1: Tr2>> {
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:37:19
   |
   |
LL | union _Un1<T: Tr1<As1: Tr2>> {
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:44:37
   |
   |
LL | type _TaWhere1<T> where T: Iterator<Item: Copy> = T;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:47:22
   |
   |
LL | fn _apit(_: impl Tr1<As1: Copy>) {}
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:49:26
   |
   |
LL | fn _apit_dyn(_: &dyn Tr1<As1: Copy>) {}
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:52:24
   |
   |
LL | fn _rpit() -> impl Tr1<As1: Copy> { S1 }
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:55:31
   |
   |
LL | fn _rpit_dyn() -> Box<dyn Tr1<As1: Copy>> { Box::new(S1) }
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:58:23
   |
   |
LL | const _cdef: impl Tr1<As1: Copy> = S1;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:64:24
   |
   |
LL | static _sdef: impl Tr1<As1: Copy> = S1;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
error[E0658]: associated type bounds are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-associated_type_bounds.rs:71:21
   |
   |
LL |     let _: impl Tr1<As1: Copy> = S1;
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable

error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | const _cdef: impl Tr1<As1: Copy> = S1;


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL | static _sdef: impl Tr1<As1: Copy> = S1;


error[E0562]: `impl Trait` not allowed outside of function and method return types
   |
   |
LL |     let _: impl Tr1<As1: Copy> = S1;


error[E0277]: the trait bound `<<Self as _Tr3>::A as Iterator>::Item: Copy` is not satisfied
   |
   |
LL |     type A: Iterator<Item: Copy>;
   |                            ^^^^ the trait `Copy` is not implemented for `<<Self as _Tr3>::A as Iterator>::Item`
note: required by a bound in `Copy`
  --> /checkout/library/core/src/marker.rs:385:1
   |
   |
LL | / pub trait Copy: Clone {
LL | |     // Empty.
LL | | }
   | |_^ required by this bound in `Copy`
   |
   |
LL | trait _Tr3 where <<Self as _Tr3>::A as Iterator>::Item: Copy {

error: aborting due to 17 previous errors

Some errors have detailed explanations: E0277, E0562, E0658.
---

---- [ui] ui/traits/issue-85735.rs stdout ----
diff of stderr:

8 note: required by a bound in `FnMut`
9   --> $SRC_DIR/core/src/ops/function.rs:LL:COL
10    |
- LL | pub trait FnMut<Args>: FnOnce<Args> {
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `FnMut`
+ LL | / pub trait FnMut<Args>: FnOnce<Args> {
+ LL | |     /// Performs the call operation.
+ LL | |     #[unstable(feature = "fn_traits", issue = "29625")]
+ LL | |     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
+ LL | | }
+    | |_^ required by this bound in `FnMut`
14 error: aborting due to previous error
15 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-85735/issue-85735.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-85735.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-85735.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-85735" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-85735/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-85735.rs:7:8
   |
LL |     T: FnMut(&'a ()),
   |        ^^^^^^^^^^^^^ cannot infer type for type parameter `T`
   |
   = note: cannot satisfy `T: FnMut<(&'a (),)>`
note: required by a bound in `FnMut`
   |
   |
LL | / pub trait FnMut<Args>: FnOnce<Args> {
LL | |     /// Performs the call operation.
LL | |     #[unstable(feature = "fn_traits", issue = "29625")]
LL | |     extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
LL | | }
   | |_^ required by this bound in `FnMut`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.

---
test result: FAILED. 12087 passed; 4 failed; 102 ignored; 0 measured; 0 filtered out; finished in 136.97s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:38
