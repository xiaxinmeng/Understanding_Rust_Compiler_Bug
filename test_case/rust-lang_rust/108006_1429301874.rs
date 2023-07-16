plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:b9dd95b10bcfe24d57bf54db874f81a7c8315a80)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---

1 error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:13:21
-    |
- LL |     <Self as SVec>::Item,
-    |                     ^^^^ expected 1 lifetime argument
-    |
- note: associated type defined here, with 1 lifetime parameter: `'a`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing lifetime argument
-    |
-    |
- LL |     <Self as SVec>::Item<'a>,
- 
- error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:13:21
-    |
-    |
- LL |     <Self as SVec>::Item,
-    |                     ^^^^ expected 1 generic argument
-    |
- note: associated type defined here, with 1 generic parameter: `T`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing generic argument
-    |
-    |
- LL |     <Self as SVec>::Item<T>,
- 
- error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:18:37
-    |
-    |
- LL |     Output = <Index<<Self as SVec>::Item,
-    |                                     ^^^^ expected 1 lifetime argument
-    |
- note: associated type defined here, with 1 lifetime parameter: `'a`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing lifetime argument
-    |
-    |
- LL |     Output = <Index<<Self as SVec>::Item<'a>,
- 
- error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:18:37
-    |
-    |
- LL |     Output = <Index<<Self as SVec>::Item,
-    |                                     ^^^^ expected 1 generic argument
-    |
- note: associated type defined here, with 1 generic parameter: `T`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing generic argument
-    |
-    |
- LL |     Output = <Index<<Self as SVec>::Item<T>,
- 
- error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:23:30
-    |
-    |
- LL |     Output = <Self as SVec>::Item> as SVec>::Item,
-    |                              ^^^^ expected 1 lifetime argument
-    |
- note: associated type defined here, with 1 lifetime parameter: `'a`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing lifetime argument
-    |
-    |
- LL |     Output = <Self as SVec>::Item<'a>> as SVec>::Item,
- 
- error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:23:30
-    |
-    |
- LL |     Output = <Self as SVec>::Item> as SVec>::Item,
-    |                              ^^^^ expected 1 generic argument
-    |
- note: associated type defined here, with 1 generic parameter: `T`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing generic argument
-    |
-    |
- LL |     Output = <Self as SVec>::Item<T>> as SVec>::Item,
- 
- error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:23:46
-    |
-    |
- LL |     Output = <Self as SVec>::Item> as SVec>::Item,
-    |                                              ^^^^ expected 1 lifetime argument
-    |
- note: associated type defined here, with 1 lifetime parameter: `'a`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing lifetime argument
-    |
-    |
- LL |     Output = <Self as SVec>::Item> as SVec>::Item<'a>,
- 
- error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:23:46
-    |
-    |
- LL |     Output = <Self as SVec>::Item> as SVec>::Item,
-    |                                              ^^^^ expected 1 generic argument
-    |
- note: associated type defined here, with 1 generic parameter: `T`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing generic argument
-    |
-    |
- LL |     Output = <Self as SVec>::Item> as SVec>::Item<T>,
- 
- error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:5:40
-    |
-    |
- LL | pub fn next<'a, T>(s: &'a mut dyn SVec<Item = T, Output = T>) {
-    |                                        ^^^^ expected 1 lifetime argument
-    |
- note: associated type defined here, with 1 lifetime parameter: `'a`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing lifetime argument
-    |
-    |
- LL | pub fn next<'a, T>(s: &'a mut dyn SVec<Item<'_> = T, Output = T>) {
- 
- error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:5:40
-    |
-    |
- LL | pub fn next<'a, T>(s: &'a mut dyn SVec<Item = T, Output = T>) {
-    |                                        ^^^^ expected 1 generic argument
-    |
- note: associated type defined here, with 1 generic parameter: `T`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing generic argument
-    |
-    |
- LL | pub fn next<'a, T>(s: &'a mut dyn SVec<Item<T> = T, Output = T>) {
- 
- error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:13:21
-    |
-    |
- LL |     <Self as SVec>::Item,
-    |                     ^^^^ expected 1 lifetime argument
-    |
- note: associated type defined here, with 1 lifetime parameter: `'a`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing lifetime argument
-    |
-    |
- LL |     <Self as SVec>::Item<'a>,
- 
- error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:13:21
-    |
-    |
- LL |     <Self as SVec>::Item,
-    |                     ^^^^ expected 1 generic argument
-    |
- note: associated type defined here, with 1 generic parameter: `T`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing generic argument
-    |
-    |
- LL |     <Self as SVec>::Item<T>,
- 
- error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:18:37
-    |
-    |
- LL |     Output = <Index<<Self as SVec>::Item,
-    |                                     ^^^^ expected 1 lifetime argument
-    |
- note: associated type defined here, with 1 lifetime parameter: `'a`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing lifetime argument
-    |
-    |
- LL |     Output = <Index<<Self as SVec>::Item<'a>,
- 
- error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:18:37
-    |
-    |
- LL |     Output = <Index<<Self as SVec>::Item,
-    |                                     ^^^^ expected 1 generic argument
-    |
- note: associated type defined here, with 1 generic parameter: `T`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing generic argument
-    |
-    |
- LL |     Output = <Index<<Self as SVec>::Item<T>,
- 
- error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:23:30
-    |
-    |
- LL |     Output = <Self as SVec>::Item> as SVec>::Item,
-    |                              ^^^^ expected 1 lifetime argument
-    |
- note: associated type defined here, with 1 lifetime parameter: `'a`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing lifetime argument
-    |
-    |
- LL |     Output = <Self as SVec>::Item<'a>> as SVec>::Item,
- 
- error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:23:30
-    |
-    |
- LL |     Output = <Self as SVec>::Item> as SVec>::Item,
-    |                              ^^^^ expected 1 generic argument
-    |
- note: associated type defined here, with 1 generic parameter: `T`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing generic argument
-    |
-    |
- LL |     Output = <Self as SVec>::Item<T>> as SVec>::Item,
- 
- error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:23:46
-    |
-    |
- LL |     Output = <Self as SVec>::Item> as SVec>::Item,
-    |                                              ^^^^ expected 1 lifetime argument
-    |
- note: associated type defined here, with 1 lifetime parameter: `'a`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing lifetime argument
-    |
-    |
- LL |     Output = <Self as SVec>::Item> as SVec>::Item<'a>,
- 
- error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:23:46
-    |
-    |
- LL |     Output = <Self as SVec>::Item> as SVec>::Item,
-    |                                              ^^^^ expected 1 generic argument
-    |
- note: associated type defined here, with 1 generic parameter: `T`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing generic argument
-    |
-    |
- LL |     Output = <Self as SVec>::Item> as SVec>::Item<T>,
- 
- error[E0038]: the trait `SVec` cannot be made into an object
-   --> $DIR/issue-105742.rs:5:31
-    |
-    |
- LL | pub fn next<'a, T>(s: &'a mut dyn SVec<Item = T, Output = T>) {
-    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `SVec` cannot be made into an object
- note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
-   --> $DIR/issue-105742.rs:12:17
-    |
-    |
- LL |    pub trait SVec: Index<
-    |  ____________----__^
-    | |            this trait cannot be made into an object...
-    | |            this trait cannot be made into an object...
- LL | |      <Self as SVec>::Item,
- LL | |
- LL | |
- ...  |
- LL | |/     Output = <Index<<Self as SVec>::Item,
- LL | ||
- LL | ||
- LL | ||
- LL | ||
- LL | ||     Output = <Self as SVec>::Item> as SVec>::Item,
-    | ||_________________________________________________^ ...because it uses `Self` as a type parameter
- LL | |
- LL | |
- LL | |  > {
-    | |__^ ...because it uses `Self` as a type parameter
- error[E0107]: missing generics for associated type `SVec::Item`
319   --> $DIR/issue-105742.rs:35:38
320    |
320    |
321 LL |     fn len(&self) -> <Self as SVec>::Item;

347 LL |     fn len(&self) -> <Self as SVec>::Item<T>;
349 
- error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:35:38
-    |
-    |
- LL |     fn len(&self) -> <Self as SVec>::Item;
-    |                                      ^^^^ expected 1 lifetime argument
-    |
- note: associated type defined here, with 1 lifetime parameter: `'a`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing lifetime argument
-    |
-    |
- LL |     fn len(&self) -> <Self as SVec>::Item<'_>;
+ error: aborting due to 2 previous errors
365 
- error[E0107]: missing generics for associated type `SVec::Item`
-   --> $DIR/issue-105742.rs:35:38
-   --> $DIR/issue-105742.rs:35:38
-    |
- LL |     fn len(&self) -> <Self as SVec>::Item;
-    |                                      ^^^^ expected 1 generic argument
-    |
- note: associated type defined here, with 1 generic parameter: `T`
-   --> $DIR/issue-105742.rs:33:10
-    |
- LL |     type Item<'a, T>;
- help: add missing generic argument
-    |
-    |
- LL |     fn len(&self) -> <Self as SVec>::Item<T>;
- 
- error: aborting due to 23 previous errors
- 
- Some errors have detailed explanations: E0038, E0107.
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:35:38
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

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0107`.
