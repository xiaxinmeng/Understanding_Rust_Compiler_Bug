plain

---- [ui (nll)] ui/kindck/kindck-impl-type-params.rs stdout ----
diff of stderr:

93 LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
94    |                                ^^^^^^^^^^^     ^^^^
95    = note: required for the cast to the object type `dyn Gettable<Foo>`
+ help: consider annotating `Foo` with `#[derive(Copy)]`
+    |
+ LL |     #[derive(Copy)]
96 
97 error: aborting due to 6 previous errors
98 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.nll/kindck-impl-type-params.nll.stderr
To only update this specific test, also pass `--test-args kindck/kindck-impl-type-params.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/kindck/kindck-impl-type-params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `T` cannot be sent between threads safely
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:18:13
   |
LL |     let a = &t as &dyn Gettable<T>;
   |             ^^ `T` cannot be sent between threads safely
   |
note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:14:32
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   = note: required for the cast to the object type `dyn Gettable<T>`
help: consider restricting type parameter `T`
   |
   |
LL | fn f<T: std::marker::Send>(val: T) {


error[E0277]: the trait bound `T: Copy` is not satisfied
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:18:13
   |
LL |     let a = &t as &dyn Gettable<T>;
   |             ^^ the trait `Copy` is not implemented for `T`
   |
note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:14:32
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   = note: required for the cast to the object type `dyn Gettable<T>`
help: consider restricting type parameter `T`
   |
   |
LL | fn f<T: std::marker::Copy>(val: T) {


error[E0277]: `T` cannot be sent between threads safely
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:25:31
   |
LL |     let a: &dyn Gettable<T> = &t;
   |                               ^^ `T` cannot be sent between threads safely
   |
note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:14:32
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   = note: required for the cast to the object type `dyn Gettable<T>`
help: consider restricting type parameter `T`
   |
   |
LL | fn g<T: std::marker::Send>(val: T) {


error[E0277]: the trait bound `T: Copy` is not satisfied
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:25:31
   |
LL |     let a: &dyn Gettable<T> = &t;
   |                               ^^ the trait `Copy` is not implemented for `T`
   |
note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:14:32
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   = note: required for the cast to the object type `dyn Gettable<T>`
help: consider restricting type parameter `T`
   |
   |
LL | fn g<T: std::marker::Copy>(val: T) {

error[E0277]: the trait bound `String: Copy` is not satisfied
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:38:13
   |
   |
LL |     let a = t as Box<dyn Gettable<String>>;
   |             ^ the trait `Copy` is not implemented for `String`
   |
   = help: the trait `Gettable<T>` is implemented for `S<T>`
note: required because of the requirements on the impl of `Gettable<String>` for `S<String>`
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:14:32
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   |                                ^^^^^^^^^^^     ^^^^
   = note: required for the cast to the object type `dyn Gettable<String>`
error[E0277]: the trait bound `Foo: Copy` is not satisfied
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:46:37
   |
   |
LL |     let a: Box<dyn Gettable<Foo>> = t;
   |                                     ^ the trait `Copy` is not implemented for `Foo`
   |
   = help: the trait `Gettable<T>` is implemented for `S<T>`
note: required because of the requirements on the impl of `Gettable<Foo>` for `S<Foo>`
  --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:14:32
   |
LL | impl<T: Send + Copy + 'static> Gettable<T> for S<T> {}
   |                                ^^^^^^^^^^^     ^^^^
   = note: required for the cast to the object type `dyn Gettable<Foo>`
help: consider annotating `Foo` with `#[derive(Copy)]`
LL |     #[derive(Copy)]
   |

error: aborting due to 6 previous errors
