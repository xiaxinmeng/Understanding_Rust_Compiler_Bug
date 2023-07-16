plain
diff of stderr:

18    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `StructAsync`
19 
20 error[E0271]: expected `fn() -> impl Future<Output = ()> {callback}` to be a fn item that returns `Pin<Box<(dyn Future<Output = ()> + 'static)>>`, but it returns `impl Future<Output = ()>`
-   --> $DIR/issue-98634.rs:45:9
-    |
- LL |         StructAsync { callback }.await;
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Pin`, found opaque type
- note: while checking the return type of the `async fn`
-   --> $DIR/issue-98634.rs:24:21
-    |
-    |
- LL | async fn callback() {}
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |                     ^ checked the `Output` of this `async fn`, found opaque type
-    = note:   expected struct `Pin<Box<(dyn Future<Output = ()> + 'static)>>`
-            found opaque type `impl Future<Output = ()>`
- note: required by a bound in `StructAsync`
-   --> $DIR/issue-98634.rs:9:35
-    |
- LL | pub struct StructAsync<F: Fn() -> Pin<Box<dyn Future<Output = ()>>>> {
-    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `StructAsync`
- 
- error[E0271]: expected `fn() -> impl Future<Output = ()> {callback}` to be a fn item that returns `Pin<Box<(dyn Future<Output = ()> + 'static)>>`, but it returns `impl Future<Output = ()>`
41    |
41    |
42 LL |         StructAsync { callback }.await;

55 LL | pub struct StructAsync<F: Fn() -> Pin<Box<dyn Future<Output = ()>>>> {
56    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `StructAsync`
- error: aborting due to 3 previous errors
+ error: aborting due to 2 previous errors
59 
60 For more information about this error, try `rustc --explain E0271`.
---
To only update this specific test, also pass `--test-args async-await/issue-98634.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-98634.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-98634" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-98634/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0271]: expected `fn() -> impl Future<Output = ()> {callback}` to be a fn item that returns `Pin<Box<(dyn Future<Output = ()> + 'static)>>`, but it returns `impl Future<Output = ()>`
   |
   |
LL |         StructAsync { callback }.await;
   |                       ^^^^^^^^ expected struct `Pin`, found opaque type
note: while checking the return type of the `async fn`
  --> /checkout/src/test/ui/async-await/issue-98634.rs:24:21
   |
   |
LL | async fn callback() {}
   |                     ^ checked the `Output` of this `async fn`, found opaque type
   = note:   expected struct `Pin<Box<(dyn Future<Output = ()> + 'static)>>`
           found opaque type `impl Future<Output = ()>`
note: required by a bound in `StructAsync`
   |
   |
LL | pub struct StructAsync<F: Fn() -> Pin<Box<dyn Future<Output = ()>>>> {
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `StructAsync`

error[E0271]: expected `fn() -> impl Future<Output = ()> {callback}` to be a fn item that returns `Pin<Box<(dyn Future<Output = ()> + 'static)>>`, but it returns `impl Future<Output = ()>`
   |
   |
LL |         StructAsync { callback }.await;
   |                                 ^^^^^^ expected struct `Pin`, found opaque type
note: while checking the return type of the `async fn`
  --> /checkout/src/test/ui/async-await/issue-98634.rs:24:21
   |
   |
LL | async fn callback() {}
   |                     ^ checked the `Output` of this `async fn`, found opaque type
   = note:   expected struct `Pin<Box<(dyn Future<Output = ()> + 'static)>>`
           found opaque type `impl Future<Output = ()>`
note: required by a bound in `StructAsync`
   |
   |
LL | pub struct StructAsync<F: Fn() -> Pin<Box<dyn Future<Output = ()>>>> {
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `StructAsync`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0271`.
------------------------------------------
