plain
diff of stderr:

313    | ||_________________________________________________^ ...because it uses `Self` as a type parameter
314 ...  |
315 LL | |
- LL | |      > {
-    | |______^ ...because it uses `Self` as a type parameter
+ LL | |  > {
+    | |__^ ...because it uses `Self` as a type parameter
319 error[E0107]: missing generics for associated type `SVec::Item`
320   --> $DIR/issue-105742.rs:15:21



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-105742/issue-105742.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issue-105742.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/issue-105742.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-105742" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-105742/auxiliary" "-Znormalize-docs"
stdout: none
error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:15:21
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:15:21
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:24:37
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:24:37
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:33:30
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:33:30
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:33:46
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:33:46
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:4:40
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:4:40
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:15:21
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:15:21
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:24:37
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:24:37
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:33:30
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:33:30
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:33:46
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:33:46
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:4:31
   |
   |
LL | pub fn next<'a, T>(s: &'a mut dyn SVec<Item = T, Output = T>) {
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `SVec` cannot be made into an object
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:14:17
   |
   |
LL |    pub trait SVec: Index<
   |  ____________----__^
   | |            this trait cannot be made into an object...
   | |            this trait cannot be made into an object...
LL | |      <Self as SVec>::Item,
LL | |      //~^ expected 1 lifetime argument
LL | |      //~| expected 1 generic argument
...  |
LL | |/     Output = <Index<<Self as SVec>::Item,
LL | ||     //~^ expected 1 lifetime argument
LL | ||     //~| expected 1 generic argument
LL | ||     //~| missing generics for associated type `SVec::Item`
...  ||
LL | ||     //~| missing generics for associated type `SVec::Item`
LL | ||     Output = <Self as SVec>::Item> as SVec>::Item,
   | ||_________________________________________________^ ...because it uses `Self` as a type parameter
...  |
LL | |      //~| missing generics for associated type `SVec::Item`
LL | |  > {
   | |__^ ...because it uses `Self` as a type parameter
error[E0107]: missing generics for associated type `SVec::Item`
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:15:21
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:15:21
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:24:37
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:24:37
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:33:30
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:33:30
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:33:46
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:33:46
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:53:38
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
  --> /checkout/tests/rustdoc-ui/issue-105742.rs:53:38
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
