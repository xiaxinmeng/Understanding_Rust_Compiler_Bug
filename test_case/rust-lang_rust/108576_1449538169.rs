plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:e58f560dd5e72b61cf9aeebf432d862b23ac76a8)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 208 tests
..........................................................i............................. 88/208
.........................F............F.F....F.............F............................ 176/208
Some tests failed in compiletest suite=rustdoc-ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:


---- [ui] tests/rustdoc-ui/invalid-toplevel-const.rs stdout ----


1 error[E0121]: the placeholder `_` is not allowed within types on item signatures for static items
-   --> $DIR/invalid-toplevel-const.rs:2:31
+   --> $DIR/invalid-toplevel-const.rs:1:31
3    |
4 LL | static CONST: Option<dyn Fn(& _)> = None;
5    |                               ^ not allowed in type signatures

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-toplevel-const/invalid-toplevel-const.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args invalid-toplevel-const.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/invalid-toplevel-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-toplevel-const" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-toplevel-const/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0121]: the placeholder `_` is not allowed within types on item signatures for static items
  --> /checkout/tests/rustdoc-ui/invalid-toplevel-const.rs:1:31
   |
LL | static CONST: Option<dyn Fn(& _)> = None;
   |                               ^ not allowed in type signatures
error: aborting due to previous error

For more information about this error, try `rustc --explain E0121`.
------------------------------------------
---

1 error[E0107]: missing generics for associated type `SVec::Item`
+   --> $DIR/issue-105742.rs:10:21
+    |
+ LL |     <Self as SVec>::Item,
+    |                     ^^^^ expected 1 lifetime argument
+    |
+ note: associated type defined here, with 1 lifetime parameter: `'a`
+    |
+    |
+ LL |     type Item<'a, T>;
+ help: add missing lifetime argument
+    |
+    |
+ LL |     <Self as SVec>::Item<'a>,
+ 
+ error[E0107]: missing generics for associated type `SVec::Item`
+   --> $DIR/issue-105742.rs:10:21
+    |
+    |
+ LL |     <Self as SVec>::Item,
+    |                     ^^^^ expected 1 generic argument
+    |
+ note: associated type defined here, with 1 generic parameter: `T`
+    |
+    |
+ LL |     type Item<'a, T>;
+ help: add missing generic argument
+    |
+    |
+ LL |     <Self as SVec>::Item<T>,
+ 
+ error[E0107]: missing generics for associated type `SVec::Item`
+   --> $DIR/issue-105742.rs:11:37
+    |
+    |
+ LL |     Output = <Index<<Self as SVec>::Item,
+    |                                     ^^^^ expected 1 lifetime argument
+    |
+ note: associated type defined here, with 1 lifetime parameter: `'a`
+    |
+    |
+ LL |     type Item<'a, T>;
+ help: add missing lifetime argument
+    |
+    |
+ LL |     Output = <Index<<Self as SVec>::Item<'a>,
+ 
+ error[E0107]: missing generics for associated type `SVec::Item`
+   --> $DIR/issue-105742.rs:11:37
+    |
+    |
+ LL |     Output = <Index<<Self as SVec>::Item,
+    |                                     ^^^^ expected 1 generic argument
+    |
+ note: associated type defined here, with 1 generic parameter: `T`
+    |
+    |
+ LL |     type Item<'a, T>;
+ help: add missing generic argument
+    |
+    |
+ LL |     Output = <Index<<Self as SVec>::Item<T>,
+ 
+ error[E0107]: missing generics for associated type `SVec::Item`
+   --> $DIR/issue-105742.rs:12:30
+    |
+    |
+ LL |     Output = <Self as SVec>::Item> as SVec>::Item,
+    |                              ^^^^ expected 1 lifetime argument
+    |
+ note: associated type defined here, with 1 lifetime parameter: `'a`
+    |
+    |
+ LL |     type Item<'a, T>;
+ help: add missing lifetime argument
+    |
+    |
+ LL |     Output = <Self as SVec>::Item<'a>> as SVec>::Item,
+ 
+ error[E0107]: missing generics for associated type `SVec::Item`
+   --> $DIR/issue-105742.rs:12:30
+    |
+    |
+ LL |     Output = <Self as SVec>::Item> as SVec>::Item,
+    |                              ^^^^ expected 1 generic argument
+    |
+ note: associated type defined here, with 1 generic parameter: `T`
+    |
+    |
+ LL |     type Item<'a, T>;
+ help: add missing generic argument
+    |
+    |
+ LL |     Output = <Self as SVec>::Item<T>> as SVec>::Item,
+ 
+ error[E0107]: missing generics for associated type `SVec::Item`
+   --> $DIR/issue-105742.rs:12:46
+    |
+    |
+ LL |     Output = <Self as SVec>::Item> as SVec>::Item,
+    |                                              ^^^^ expected 1 lifetime argument
+    |
+ note: associated type defined here, with 1 lifetime parameter: `'a`
+    |
+    |
+ LL |     type Item<'a, T>;
+ help: add missing lifetime argument
+    |
+    |
+ LL |     Output = <Self as SVec>::Item> as SVec>::Item<'a>,
+ 
+ error[E0107]: missing generics for associated type `SVec::Item`
+   --> $DIR/issue-105742.rs:12:46
+    |
+    |
+ LL |     Output = <Self as SVec>::Item> as SVec>::Item,
+    |                                              ^^^^ expected 1 generic argument
+    |
+ note: associated type defined here, with 1 generic parameter: `T`
+    |
+    |
+ LL |     type Item<'a, T>;
+ help: add missing generic argument
+    |
+    |
+ LL |     Output = <Self as SVec>::Item> as SVec>::Item<T>,
+ 
+ error[E0107]: missing generics for associated type `SVec::Item`
+   --> $DIR/issue-105742.rs:5:40
+    |
+    |
+ LL | pub fn next<'a, T>(s: &'a mut dyn SVec<Item = T, Output = T>) {
+    |                                        ^^^^ expected 1 lifetime argument
+    |
+ note: associated type defined here, with 1 lifetime parameter: `'a`
+    |
+    |
+ LL |     type Item<'a, T>;
+ help: add missing lifetime argument
+    |
+    |
+ LL | pub fn next<'a, T>(s: &'a mut dyn SVec<Item<'_> = T, Output = T>) {
+ 
+ error[E0107]: missing generics for associated type `SVec::Item`
+   --> $DIR/issue-105742.rs:5:40
+    |
+    |
+ LL | pub fn next<'a, T>(s: &'a mut dyn SVec<Item = T, Output = T>) {
+    |                                        ^^^^ expected 1 generic argument
+    |
+ note: associated type defined here, with 1 generic parameter: `T`
+    |
+    |
+ LL |     type Item<'a, T>;
+ help: add missing generic argument
+    |
+    |
+ LL | pub fn next<'a, T>(s: &'a mut dyn SVec<Item<T> = T, Output = T>) {
+ 
+ error[E0107]: missing generics for associated type `SVec::Item`
+   --> $DIR/issue-105742.rs:10:21
+    |
+    |
+ LL |     <Self as SVec>::Item,
+    |                     ^^^^ expected 1 lifetime argument
+    |
+ note: associated type defined here, with 1 lifetime parameter: `'a`
+    |
+    |
+ LL |     type Item<'a, T>;
+ help: add missing lifetime argument
+    |
+    |
+ LL |     <Self as SVec>::Item<'a>,
+ 
+ error[E0107]: missing generics for associated type `SVec::Item`
+   --> $DIR/issue-105742.rs:10:21
+    |
+    |
+ LL |     <Self as SVec>::Item,
+    |                     ^^^^ expected 1 generic argument
+    |
+ note: associated type defined here, with 1 generic parameter: `T`
+    |
+    |
+ LL |     type Item<'a, T>;
+ help: add missing generic argument
+    |
+    |
+ LL |     <Self as SVec>::Item<T>,
+ 
+ error[E0107]: missing generics for associated type `SVec::Item`
+   --> $DIR/issue-105742.rs:11:37
+    |
+    |
+ LL |     Output = <Index<<Self as SVec>::Item,
+    |                                     ^^^^ expected 1 lifetime argument
+    |
+ note: associated type defined here, with 1 lifetime parameter: `'a`
+    |
+    |
+ LL |     type Item<'a, T>;
+ help: add missing lifetime argument
+    |
+    |
+ LL |     Output = <Index<<Self as SVec>::Item<'a>,
+ 
+ error[E0107]: missing generics for associated type `SVec::Item`
+   --> $DIR/issue-105742.rs:11:37
+    |
+    |
+ LL |     Output = <Index<<Self as SVec>::Item,
+    |                                     ^^^^ expected 1 generic argument
+    |
+ note: associated type defined here, with 1 generic parameter: `T`
+    |
+    |
+ LL |     type Item<'a, T>;
+ help: add missing generic argument
+    |
+    |
+ LL |     Output = <Index<<Self as SVec>::Item<T>,
+ 
+ error[E0107]: missing generics for associated type `SVec::Item`
+   --> $DIR/issue-105742.rs:12:30
+    |
+    |
+ LL |     Output = <Self as SVec>::Item> as SVec>::Item,
+    |                              ^^^^ expected 1 lifetime argument
+    |
+ note: associated type defined here, with 1 lifetime parameter: `'a`
+    |
+    |
+ LL |     type Item<'a, T>;
+ help: add missing lifetime argument
+    |
+    |
+ LL |     Output = <Self as SVec>::Item<'a>> as SVec>::Item,
+ 
+ error[E0107]: missing generics for associated type `SVec::Item`
+   --> $DIR/issue-105742.rs:12:30
+    |
+    |
+ LL |     Output = <Self as SVec>::Item> as SVec>::Item,
+    |                              ^^^^ expected 1 generic argument
+    |
+ note: associated type defined here, with 1 generic parameter: `T`
+    |
+    |
+ LL |     type Item<'a, T>;
+ help: add missing generic argument
+    |
+    |
+ LL |     Output = <Self as SVec>::Item<T>> as SVec>::Item,
+ 
+ error[E0107]: missing generics for associated type `SVec::Item`
+   --> $DIR/issue-105742.rs:12:46
+    |
+    |
+ LL |     Output = <Self as SVec>::Item> as SVec>::Item,
+    |                                              ^^^^ expected 1 lifetime argument
+    |
+ note: associated type defined here, with 1 lifetime parameter: `'a`
+    |
+    |
+ LL |     type Item<'a, T>;
+ help: add missing lifetime argument
+    |
+    |
+ LL |     Output = <Self as SVec>::Item> as SVec>::Item<'a>,
+ 
+ error[E0107]: missing generics for associated type `SVec::Item`
+   --> $DIR/issue-105742.rs:12:46
+    |
+    |
+ LL |     Output = <Self as SVec>::Item> as SVec>::Item,
+    |                                              ^^^^ expected 1 generic argument
+    |
+ note: associated type defined here, with 1 generic parameter: `T`
+    |
+    |
+ LL |     type Item<'a, T>;
+ help: add missing generic argument
+    |
+    |
+ LL |     Output = <Self as SVec>::Item> as SVec>::Item<T>,
+ 
+ error[E0038]: the trait `SVec` cannot be made into an object
+   --> $DIR/issue-105742.rs:5:31
+    |
+    |
+ LL | pub fn next<'a, T>(s: &'a mut dyn SVec<Item = T, Output = T>) {
+    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `SVec` cannot be made into an object
+ note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
+   --> $DIR/issue-105742.rs:9:17
+    |
+    |
+ LL |    pub trait SVec: Index<
+    |  ____________----__^
+    | |            this trait cannot be made into an object...
+    | |            this trait cannot be made into an object...
+ LL | |      <Self as SVec>::Item,
+ LL | |/     Output = <Index<<Self as SVec>::Item,
+ LL | ||     Output = <Self as SVec>::Item> as SVec>::Item,
+    | ||_________________________________________________^ ...because it uses `Self` as a type parameter
+ LL | |  > {
+    | |__^ ...because it uses `Self` as a type parameter
+ error[E0107]: missing generics for associated type `SVec::Item`
2   --> $DIR/issue-105742.rs:16:38
3    |
3    |
4 LL |     fn len(&self) -> <Self as SVec>::Item;

30 LL |     fn len(&self) -> <Self as SVec>::Item<T>;
32 
- error: aborting due to 2 previous errors
+ error: aborting due to 21 previous errors
34 
---
To only update this specific test, also pass `--test-args issue-105742.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/issue-105742.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-105742" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-105742/auxiliary" "-Znormalize-docs"
stdout: none
--- stderr -------------------------------
error[E0107]: missing generics for associated type `SVec::Item`
   |
   |
LL |     <Self as SVec>::Item,
   |                     ^^^^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
   |
   |
LL |     type Item<'a, T>;
help: add missing lifetime argument
   |
   |
LL |     <Self as SVec>::Item<'a>,

error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:10:21
   |
   |
LL |     <Self as SVec>::Item,
   |                     ^^^^ expected 1 generic argument
   |
note: associated type defined here, with 1 generic parameter: `T`
   |
   |
LL |     type Item<'a, T>;
help: add missing generic argument
   |
   |
LL |     <Self as SVec>::Item<T>,

error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:11:37
   |
   |
LL |     Output = <Index<<Self as SVec>::Item,
   |                                     ^^^^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
   |
   |
LL |     type Item<'a, T>;
help: add missing lifetime argument
   |
   |
LL |     Output = <Index<<Self as SVec>::Item<'a>,

error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:11:37
   |
   |
LL |     Output = <Index<<Self as SVec>::Item,
   |                                     ^^^^ expected 1 generic argument
   |
note: associated type defined here, with 1 generic parameter: `T`
   |
   |
LL |     type Item<'a, T>;
help: add missing generic argument
   |
   |
LL |     Output = <Index<<Self as SVec>::Item<T>,

error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:12:30
   |
   |
LL |     Output = <Self as SVec>::Item> as SVec>::Item,
   |                              ^^^^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
   |
   |
LL |     type Item<'a, T>;
help: add missing lifetime argument
   |
   |
LL |     Output = <Self as SVec>::Item<'a>> as SVec>::Item,

error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:12:30
   |
   |
LL |     Output = <Self as SVec>::Item> as SVec>::Item,
   |                              ^^^^ expected 1 generic argument
   |
note: associated type defined here, with 1 generic parameter: `T`
   |
   |
LL |     type Item<'a, T>;
help: add missing generic argument
   |
   |
LL |     Output = <Self as SVec>::Item<T>> as SVec>::Item,

error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:12:46
   |
   |
LL |     Output = <Self as SVec>::Item> as SVec>::Item,
   |                                              ^^^^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
   |
   |
LL |     type Item<'a, T>;
help: add missing lifetime argument
   |
   |
LL |     Output = <Self as SVec>::Item> as SVec>::Item<'a>,

error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:12:46
   |
   |
LL |     Output = <Self as SVec>::Item> as SVec>::Item,
   |                                              ^^^^ expected 1 generic argument
   |
note: associated type defined here, with 1 generic parameter: `T`
   |
   |
LL |     type Item<'a, T>;
help: add missing generic argument
   |
   |
LL |     Output = <Self as SVec>::Item> as SVec>::Item<T>,

error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:5:40
   |
   |
LL | pub fn next<'a, T>(s: &'a mut dyn SVec<Item = T, Output = T>) {
   |                                        ^^^^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
   |
   |
LL |     type Item<'a, T>;
help: add missing lifetime argument
   |
   |
LL | pub fn next<'a, T>(s: &'a mut dyn SVec<Item<'_> = T, Output = T>) {

error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:5:40
   |
   |
LL | pub fn next<'a, T>(s: &'a mut dyn SVec<Item = T, Output = T>) {
   |                                        ^^^^ expected 1 generic argument
   |
note: associated type defined here, with 1 generic parameter: `T`
   |
   |
LL |     type Item<'a, T>;
help: add missing generic argument
   |
   |
LL | pub fn next<'a, T>(s: &'a mut dyn SVec<Item<T> = T, Output = T>) {

error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:10:21
   |
   |
LL |     <Self as SVec>::Item,
   |                     ^^^^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
   |
   |
LL |     type Item<'a, T>;
help: add missing lifetime argument
   |
   |
LL |     <Self as SVec>::Item<'a>,

error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:10:21
   |
   |
LL |     <Self as SVec>::Item,
   |                     ^^^^ expected 1 generic argument
   |
note: associated type defined here, with 1 generic parameter: `T`
   |
   |
LL |     type Item<'a, T>;
help: add missing generic argument
   |
   |
LL |     <Self as SVec>::Item<T>,

error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:11:37
   |
   |
LL |     Output = <Index<<Self as SVec>::Item,
   |                                     ^^^^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
   |
   |
LL |     type Item<'a, T>;
help: add missing lifetime argument
   |
   |
LL |     Output = <Index<<Self as SVec>::Item<'a>,

error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:11:37
   |
   |
LL |     Output = <Index<<Self as SVec>::Item,
   |                                     ^^^^ expected 1 generic argument
   |
note: associated type defined here, with 1 generic parameter: `T`
   |
   |
LL |     type Item<'a, T>;
help: add missing generic argument
   |
   |
LL |     Output = <Index<<Self as SVec>::Item<T>,

error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:12:30
   |
   |
LL |     Output = <Self as SVec>::Item> as SVec>::Item,
   |                              ^^^^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
   |
   |
LL |     type Item<'a, T>;
help: add missing lifetime argument
   |
   |
LL |     Output = <Self as SVec>::Item<'a>> as SVec>::Item,

error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:12:30
   |
   |
LL |     Output = <Self as SVec>::Item> as SVec>::Item,
   |                              ^^^^ expected 1 generic argument
   |
note: associated type defined here, with 1 generic parameter: `T`
   |
   |
LL |     type Item<'a, T>;
help: add missing generic argument
   |
   |
LL |     Output = <Self as SVec>::Item<T>> as SVec>::Item,

error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:12:46
   |
   |
LL |     Output = <Self as SVec>::Item> as SVec>::Item,
   |                                              ^^^^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
   |
   |
LL |     type Item<'a, T>;
help: add missing lifetime argument
   |
   |
LL |     Output = <Self as SVec>::Item> as SVec>::Item<'a>,

error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:12:46
   |
   |
LL |     Output = <Self as SVec>::Item> as SVec>::Item,
   |                                              ^^^^ expected 1 generic argument
   |
note: associated type defined here, with 1 generic parameter: `T`
   |
   |
LL |     type Item<'a, T>;
help: add missing generic argument
   |
   |
LL |     Output = <Self as SVec>::Item> as SVec>::Item<T>,

error[E0038]: the trait `SVec` cannot be made into an object
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:5:31
   |
   |
LL | pub fn next<'a, T>(s: &'a mut dyn SVec<Item = T, Output = T>) {
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `SVec` cannot be made into an object
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:9:17
   |
   |
LL |    pub trait SVec: Index<
   |  ____________----__^
   | |            this trait cannot be made into an object...
   | |            this trait cannot be made into an object...
LL | |      <Self as SVec>::Item,
LL | |/     Output = <Index<<Self as SVec>::Item,
LL | ||     Output = <Self as SVec>::Item> as SVec>::Item,
   | ||_________________________________________________^ ...because it uses `Self` as a type parameter
LL | |  > {
   | |__^ ...because it uses `Self` as a type parameter
error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:16:38
   |
   |
LL |     fn len(&self) -> <Self as SVec>::Item;
   |                                      ^^^^ expected 1 lifetime argument
   |
note: associated type defined here, with 1 lifetime parameter: `'a`
   |
   |
LL |     type Item<'a, T>;
help: add missing lifetime argument
   |
   |
LL |     fn len(&self) -> <Self as SVec>::Item<'_>;

error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:16:38
   |
   |
LL |     fn len(&self) -> <Self as SVec>::Item;
   |                                      ^^^^ expected 1 generic argument
   |
note: associated type defined here, with 1 generic parameter: `T`
   |
   |
LL |     type Item<'a, T>;
help: add missing generic argument
   |
   |
LL |     fn len(&self) -> <Self as SVec>::Item<T>;

error: aborting due to 21 previous errors

Some errors have detailed explanations: E0038, E0107.
---
---- [ui] tests/rustdoc-ui/issue-106226.rs stdout ----
diff of stderr:

- error[E0308]: mismatched types
-   --> $DIR/issue-106226.rs:2:14
+ error[E0121]: the placeholder `_` is not allowed within types on item signatures for type aliases
3    |
3    |
4 LL | type F = [_; ()];
+    |           ^ not allowed in type signatures
6 
7 error: aborting due to previous error
8 
---
To only update this specific test, also pass `--test-args issue-106226.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/issue-106226.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-106226" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-106226/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0121]: the placeholder `_` is not allowed within types on item signatures for type aliases
   |
   |
LL | type F = [_; ()];
   |           ^ not allowed in type signatures
error: aborting due to previous error

For more information about this error, try `rustc --explain E0121`.
------------------------------------------
------------------------------------------


---- [ui] tests/rustdoc-ui/issue-79465.rs stdout ----
diff of stderr:

4 LL | pub fn f1<T>(x: T::A) {}
5    |                    ^ associated type `A` not found
- error[E0220]: associated type `A` not found for `T`
-   --> $DIR/issue-79465.rs:1:20
-    |
-    |
- LL | pub fn f1<T>(x: T::A) {}
-    |                    ^ associated type `A` not found
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
14 
15 For more information about this error, try `rustc --explain E0220`.
---
To only update this specific test, also pass `--test-args issue-79465.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/issue-79465.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-79465" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-79465/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0220]: associated type `A` not found for `T`
   |
   |
LL | pub fn f1<T>(x: T::A) {}
   |                    ^ associated type `A` not found
error: aborting due to previous error

For more information about this error, try `rustc --explain E0220`.
------------------------------------------
------------------------------------------


---- [ui] tests/rustdoc-ui/issue-96287.rs stdout ----
diff of stderr:

4 LL | pub type Foo<V> = impl Trait<V::Assoc>;
5    |                                 ^^^^^ there is a similarly named associated type `Assoc` in the trait `TraitWithAssoc`
6 
- error[E0220]: associated type `Assoc` not found for `V`
-   --> $DIR/issue-96287.rs:7:33
-    |
- LL | pub type Foo<V> = impl Trait<V::Assoc>;
-    |                                 ^^^^^ there is a similarly named associated type `Assoc` in the trait `TraitWithAssoc`
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
14 
15 For more information about this error, try `rustc --explain E0220`.
---
To only update this specific test, also pass `--test-args issue-96287.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/issue-96287.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-96287" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-96287/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0220]: associated type `Assoc` not found for `V`
   |
   |
LL | pub type Foo<V> = impl Trait<V::Assoc>;
   |                                 ^^^^^ there is a similarly named associated type `Assoc` in the trait `TraitWithAssoc`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0220`.
------------------------------------------
