plain

---- [ui] src/test/ui/suggestions/expected-boxed-future-isnt-pinned.rs stdout ----
diff of stderr:

25    = note: expected struct `Pin<Box<(dyn Future<Output = i32> + Send + 'static)>>`
26               found struct `Box<F>`
27    = help: use `Box::pin`
+ help: try wrapping the expression in `std::pin::Pin`
+    |
+ LL |     std::pin::Pin { pointer: Box::new(x) }
28 
29 error[E0308]: mismatched types
30   --> $DIR/expected-boxed-future-isnt-pinned.rs:19:14

---
To only update this specific test, also pass `--test-args suggestions/expected-boxed-future-isnt-pinned.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/expected-boxed-future-isnt-pinned.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/expected-boxed-future-isnt-pinned" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/expected-boxed-future-isnt-pinned/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/suggestions/expected-boxed-future-isnt-pinned.rs:11:5
   |
   |
LL | fn foo<F: Future<Output=i32> + Send + 'static>(x: F) -> BoxFuture<'static, i32> {
   |        - this type parameter                            ----------------------- expected `Pin<Box<(dyn Future<Output = i32> + Send + 'static)>>` because of return type
LL |     // We could instead use an `async` block, but this way we have no std spans.
LL |     x //~ ERROR mismatched types
   |     ^ expected struct `Pin`, found type parameter `F`
   |
   = note:      expected struct `Pin<Box<(dyn Future<Output = i32> + Send + 'static)>>`
           found type parameter `F`
help: you need to pin and box this expression
   |
LL |     Box::pin(x) //~ ERROR mismatched types
   |     +++++++++ +
error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/expected-boxed-future-isnt-pinned.rs:15:5
   |
   |
LL | fn bar<F: Future<Output=i32> + Send + 'static>(x: F) -> BoxFuture<'static, i32> {
   |                                                         ----------------------- expected `Pin<Box<(dyn Future<Output = i32> + Send + 'static)>>` because of return type
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |     Box::new(x) //~ ERROR mismatched types
   |     ^^^^^^^^^^^ expected struct `Pin`, found struct `Box`
   |
   = note: expected struct `Pin<Box<(dyn Future<Output = i32> + Send + 'static)>>`
              found struct `Box<F>`
   = help: use `Box::pin`
help: try wrapping the expression in `std::pin::Pin`
   |
LL |     std::pin::Pin { pointer: Box::new(x) } //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/expected-boxed-future-isnt-pinned.rs:19:14
   |
   |
LL | fn baz<F: Future<Output=i32> + Send + 'static>(x: F) -> BoxFuture<'static, i32> {
   |        - this type parameter
LL |     Pin::new(x) //~ ERROR mismatched types
   |     -------- ^ expected struct `Box`, found type parameter `F`
   |     arguments to this function are incorrect
   |     arguments to this function are incorrect
   |     help: use `Box::pin` to pin and box this expression: `Box::pin`
   |
   = note:      expected struct `Box<dyn Future<Output = i32> + Send>`
           found type parameter `F`
  --> /checkout/library/core/src/pin.rs:491:18
   |
   |
LL |     pub const fn new(pointer: P) -> Pin<P> {


error[E0277]: `dyn Future<Output = i32> + Send` cannot be unpinned
   |
   |
LL |     Pin::new(x) //~ ERROR mismatched types
   |     ^^^^^^^^ the trait `Unpin` is not implemented for `dyn Future<Output = i32> + Send`
   |
   = note: consider using `Box::pin`
note: required by a bound in `Pin::<P>::new`
   |
   |
LL | impl<P: Deref<Target: Unpin>> Pin<P> {
   |                       ^^^^^ required by this bound in `Pin::<P>::new`

error[E0277]: `dyn Future<Output = i32> + Send` cannot be unpinned
   |
   |
LL |     Pin::new(Box::new(x)) //~ ERROR E0277
   |     ^^^^^^^^ the trait `Unpin` is not implemented for `dyn Future<Output = i32> + Send`
   |
   = note: consider using `Box::pin`
note: required by a bound in `Pin::<P>::new`
   |
   |
LL | impl<P: Deref<Target: Unpin>> Pin<P> {
   |                       ^^^^^ required by this bound in `Pin::<P>::new`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/expected-boxed-future-isnt-pinned.rs:28:5
   |
   |
LL |   fn zap() -> BoxFuture<'static, i32> {
   |               ----------------------- expected `Pin<Box<(dyn Future<Output = i32> + Send + 'static)>>` because of return type
LL | /     async { //~ ERROR mismatched types
LL | |         42
LL | |     }
   | |_____^ expected struct `Pin`, found opaque type
  ::: /checkout/library/core/src/future/mod.rs:72:43
   |
   |
LL |   pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
   |
   |
   = note:   expected struct `Pin<Box<(dyn Future<Output = i32> + Send + 'static)>>`
           found opaque type `impl Future`
help: you need to pin and box this expression
   |
LL ~     Box::pin(async { //~ ERROR mismatched types
LL ~     })
   |

error: aborting due to 6 previous errors
