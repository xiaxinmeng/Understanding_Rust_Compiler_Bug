plain

---- [ui] src/test/ui/async-await/issue-98634.rs stdout ----
diff of stderr:

- error[E0271]: expected `fn() -> impl Future<Output = ()> {callback}` to be a fn item that returns `Pin<Box<(dyn Future<Output = ()> + 'static)>>`, but it returns `impl Future<Output = ()>`
-   --> $DIR/issue-98634.rs:26:19
+ error[E0670]: `async fn` is not permitted in Rust 2015
3    |
3    |
- LL |     StructAsync { callback }.await;
-    |                   ^^^^^^^^ expected struct `Pin`, found opaque type
- note: while checking the return type of the `async fn`
-   --> $DIR/issue-98634.rs:22:21
-    |
-    |
10 LL | async fn callback() {}
-    |                     ^ checked the `Output` of this `async fn`, found opaque type
-    = note:   expected struct `Pin<Box<(dyn Future<Output = ()> + 'static)>>`
-            found opaque type `impl Future<Output = ()>`
- note: required by a bound in `StructAsync`
-   --> $DIR/issue-98634.rs:7:35
+    | ^^^^^ to use `async fn`, switch to Rust 2018 or later
16    |
- LL | pub struct StructAsync<F: Fn() -> Pin<Box<dyn Future<Output = ()>>>> {
-    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `StructAsync`
+    = help: pass `--edition 2021` to `rustc`
+    = note: for more on editions, read https://doc.rust-lang.org/edition-guide
19 
- error[E0271]: expected `fn() -> impl Future<Output = ()> {callback}` to be a fn item that returns `Pin<Box<(dyn Future<Output = ()> + 'static)>>`, but it returns `impl Future<Output = ()>`
-   --> $DIR/issue-98634.rs:26:5
+ error[E0670]: `async fn` is not permitted in Rust 2015
22    |
22    |
- LL |     StructAsync { callback }.await;
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Pin`, found opaque type
+ LL | async fn main() {
+    | ^^^^^ to use `async fn`, switch to Rust 2018 or later
- note: while checking the return type of the `async fn`
-   --> $DIR/issue-98634.rs:22:21
-   --> $DIR/issue-98634.rs:22:21
+    = help: pass `--edition 2021` to `rustc`
+    = note: for more on editions, read https://doc.rust-lang.org/edition-guide
+ error[E0433]: failed to resolve: use of undeclared crate or module `tokio`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+   --> $DIR/issue-98634.rs:24:3
28    |
28    |
- LL | async fn callback() {}
-    |                     ^ checked the `Output` of this `async fn`, found opaque type
-    = note:   expected struct `Pin<Box<(dyn Future<Output = ()> + 'static)>>`
-            found opaque type `impl Future<Output = ()>`
- note: required by a bound in `StructAsync`
-   --> $DIR/issue-98634.rs:7:35
+ LL | #[tokio::main]
+    |   ^^^^^ use of undeclared crate or module `tokio`
+ 
+ error[E0609]: no field `await` on type `StructAsync<fn() -> impl Future<Output = ()> {callback}>`
35    |
35    |
- LL | pub struct StructAsync<F: Fn() -> Pin<Box<dyn Future<Output = ()>>>> {
-    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `StructAsync`
+ LL |     StructAsync { callback }.await;
+    |
+    = note: available fields are: `callback`
+    = note: available fields are: `callback`
+    = note: to `.await` a `Future`, switch to Rust 2018 or later
+    = help: pass `--edition 2021` to `rustc`
+    = note: for more on editions, read https://doc.rust-lang.org/edition-guide
38 
39 error[E0271]: expected `fn() -> impl Future<Output = ()> {callback}` to be a fn item that returns `Pin<Box<(dyn Future<Output = ()> + 'static)>>`, but it returns `impl Future<Output = ()>`
-   --> $DIR/issue-98634.rs:26:29
41    |
41    |
42 LL |     StructAsync { callback }.await;
-    |                             ^^^^^^ expected struct `Pin`, found opaque type
+    |                   ^^^^^^^^ expected struct `Pin`, found opaque type
45 note: while checking the return type of the `async fn`
46   --> $DIR/issue-98634.rs:22:21


55 LL | pub struct StructAsync<F: Fn() -> Pin<Box<dyn Future<Output = ()>>>> {
56    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `StructAsync`
- error: aborting due to 3 previous errors
- error: aborting due to 3 previous errors
+ error[E0752]: `main` function is not allowed to be `async`
+    |
+ LL | async fn main() {
+ LL | async fn main() {
+    | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
- For more information about this error, try `rustc --explain E0271`.
+ error: aborting due to 6 previous errors
+ 
+ Some errors have detailed explanations: E0271, E0433, E0609, E0670, E0752.
---
To only update this specific test, also pass `--test-args async-await/issue-98634.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-98634.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-98634" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-98634/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0670]: `async fn` is not permitted in Rust 2015
   |
   |
LL | async fn callback() {}
   | ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0670]: `async fn` is not permitted in Rust 2015
   |
LL | async fn main() {
LL | async fn main() {
   | ^^^^^ to use `async fn`, switch to Rust 2018 or later
   |
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide
error[E0433]: failed to resolve: use of undeclared crate or module `tokio`
  --> /checkout/src/test/ui/async-await/issue-98634.rs:24:3
   |
   |
LL | #[tokio::main]
   |   ^^^^^ use of undeclared crate or module `tokio`

error[E0609]: no field `await` on type `StructAsync<fn() -> impl Future<Output = ()> {callback}>`
   |
   |
LL |     StructAsync { callback }.await;
   |
   = note: available fields are: `callback`
   = note: available fields are: `callback`
   = note: to `.await` a `Future`, switch to Rust 2018 or later
   = help: pass `--edition 2021` to `rustc`
   = note: for more on editions, read https://doc.rust-lang.org/edition-guide

error[E0271]: expected `fn() -> impl Future<Output = ()> {callback}` to be a fn item that returns `Pin<Box<(dyn Future<Output = ()> + 'static)>>`, but it returns `impl Future<Output = ()>`
   |
   |
LL |     StructAsync { callback }.await;
   |                   ^^^^^^^^ expected struct `Pin`, found opaque type
note: while checking the return type of the `async fn`
  --> /checkout/src/test/ui/async-await/issue-98634.rs:22:21
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

error[E0752]: `main` function is not allowed to be `async`
   |
LL | async fn main() {
LL | async fn main() {
   | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0271, E0433, E0609, E0670, E0752.
For more information about an error, try `rustc --explain E0271`.
