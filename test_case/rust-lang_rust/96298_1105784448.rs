plain

---- [ui] src/test/ui/async-await/issue-68112.rs stdout ----
diff of stderr:

43    = help: the trait `Sync` is not implemented for `RefCell<i32>`
44    = note: required because of the requirements on the impl of `Send` for `Arc<RefCell<i32>>`
45    = note: required because it appears within the type `[static generator@$DIR/issue-68112.rs:47:31: 47:36]`
-    = note: required because it appears within the type `from_generator::GenFuture<[static generator@$DIR/issue-68112.rs:47:31: 47:36]>`
+    = note: required because it appears within the type `std::future::from_generator::GenFuture<[static generator@$DIR/issue-68112.rs:47:31: 47:36]>`
47    = note: required because it appears within the type `impl Future<Output = Arc<RefCell<i32>>>`
48    = note: required because it appears within the type `impl Future<Output = Arc<RefCell<i32>>>`
49    = note: required because it appears within the type `impl Future<Output = Arc<RefCell<i32>>>`

50    = note: required because it appears within the type `{ResumeTy, impl Future<Output = Arc<RefCell<i32>>>, (), i32, Ready<i32>}`
51    = note: required because it appears within the type `[static generator@$DIR/issue-68112.rs:55:26: 59:6]`
-    = note: required because it appears within the type `from_generator::GenFuture<[static generator@$DIR/issue-68112.rs:55:26: 59:6]>`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
53    = note: required because it appears within the type `impl Future<Output = ()>`
54 note: required by a bound in `require_send`
55   --> $DIR/issue-68112.rs:11:25

---
To only update this specific test, also pass `--test-args async-await/issue-68112.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-68112.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-68112" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-68112/auxiliary"
stdout: none
--- stderr -------------------------------
error: future cannot be sent between threads safely
   |
LL |     require_send(send_fut);
LL |     require_send(send_fut);
   |     ^^^^^^^^^^^^ future created by async block is not `Send`
   |
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
note: future is not `Send` as it awaits another future which is not `Send`
   |
   |
LL |         let _ = non_send_fut.await;
   |                 ^^^^^^^^^^^^ await occurs here on type `impl Future<Output = Arc<RefCell<i32>>>`, which is not `Send`
note: required by a bound in `require_send`
   |
   |
LL | fn require_send(_: impl Send) {}
   |                         ^^^^ required by this bound in `require_send`
error: future cannot be sent between threads safely
  --> /checkout/src/test/ui/async-await/issue-68112.rs:43:5
   |
LL |     require_send(send_fut);
LL |     require_send(send_fut);
   |     ^^^^^^^^^^^^ future created by async block is not `Send`
   |
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
note: future is not `Send` as it awaits another future which is not `Send`
   |
   |
LL |         let _ = make_non_send_future1().await;
   |                 ^^^^^^^^^^^^^^^^^^^^^^^ await occurs here on type `impl Future<Output = Arc<RefCell<i32>>>`, which is not `Send`
note: required by a bound in `require_send`
   |
   |
LL | fn require_send(_: impl Send) {}
   |                         ^^^^ required by this bound in `require_send`

error[E0277]: `RefCell<i32>` cannot be shared between threads safely
   |
LL |     require_send(send_fut);
LL |     require_send(send_fut);
   |     ^^^^^^^^^^^^ `RefCell<i32>` cannot be shared between threads safely
   |
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
   = note: required because of the requirements on the impl of `Send` for `Arc<RefCell<i32>>`
   = note: required because it appears within the type `[static generator@/checkout/src/test/ui/async-await/issue-68112.rs:47:31: 47:36]`
   = note: required because it appears within the type `std::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/issue-68112.rs:47:31: 47:36]>`
   = note: required because it appears within the type `impl Future<Output = Arc<RefCell<i32>>>`
   = note: required because it appears within the type `impl Future<Output = Arc<RefCell<i32>>>`
   = note: required because it appears within the type `impl Future<Output = Arc<RefCell<i32>>>`
   = note: required because it appears within the type `{ResumeTy, impl Future<Output = Arc<RefCell<i32>>>, (), i32, Ready<i32>}`
   = note: required because it appears within the type `[static generator@/checkout/src/test/ui/async-await/issue-68112.rs:55:26: 59:6]`
   = note: required because it appears within the type `std::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/issue-68112.rs:55:26: 59:6]>`
   = note: required because it appears within the type `impl Future<Output = ()>`
note: required by a bound in `require_send`
   |
   |
LL | fn require_send(_: impl Send) {}
   |                         ^^^^ required by this bound in `require_send`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/partial-drop-partial-reinit.rs stdout ----
diff of stderr:

13    = note: required because it appears within the type `(NotSend,)`
14    = note: required because it appears within the type `{ResumeTy, (NotSend,), impl Future<Output = ()>, ()}`
15    = note: required because it appears within the type `[static generator@$DIR/partial-drop-partial-reinit.rs:22:16: 27:2]`
-    = note: required because it appears within the type `from_generator::GenFuture<[static generator@$DIR/partial-drop-partial-reinit.rs:22:16: 27:2]>`
17    = note: required because it appears within the type `impl Future<Output = ()>`
18    = note: required because it appears within the type `impl Future<Output = ()>`
18    = note: required because it appears within the type `impl Future<Output = ()>`
19 note: required by a bound in `gimme_send`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/partial-drop-partial-reinit/partial-drop-partial-reinit.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/partial-drop-partial-reinit.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/partial-drop-partial-reinit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/partial-drop-partial-reinit" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/partial-drop-partial-reinit/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `NotSend` cannot be sent between threads safely
   |
   |
LL |     gimme_send(foo());
   |     ---------- ^^^^^ `NotSend` cannot be sent between threads safely
   |     required by a bound introduced by this call
...
LL | async fn foo() {
LL | async fn foo() {
   |                - within this `impl Future<Output = ()>`
   |
   = help: within `impl Future<Output = ()>`, the trait `Send` is not implemented for `NotSend`
   = note: required because it appears within the type `(NotSend,)`
   = note: required because it appears within the type `{ResumeTy, (NotSend,), impl Future<Output = ()>, ()}`
   = note: required because it appears within the type `[static generator@/checkout/src/test/ui/async-await/partial-drop-partial-reinit.rs:22:16: 27:2]`
   = note: required because it appears within the type `std::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/partial-drop-partial-reinit.rs:22:16: 27:2]>`
   = note: required because it appears within the type `impl Future<Output = ()>`
   = note: required because it appears within the type `impl Future<Output = ()>`
note: required by a bound in `gimme_send`
   |
   |
LL | fn gimme_send<T: Send>(t: T) {
   |                  ^^^^ required by this bound in `gimme_send`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/chalkify/bugs/async.rs stdout ----
diff of stderr:

7 LL | | }
8    | |_^ the trait `Generator<ResumeTy>` is not implemented for `[static generator@$DIR/async.rs:7:29: 9:2]`
- note: required by a bound in `from_generator`
+ note: required by a bound in `std::future::from_generator`
11   --> $SRC_DIR/core/src/future/mod.rs:LL:COL
12    |
12    |
13 LL |     T: Generator<ResumeTy, Yield = ()>,

-    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `from_generator`
+    |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `std::future::from_generator`
15 
16 error[E0280]: the requirement `<[static generator@$DIR/async.rs:7:29: 9:2] as Generator<ResumeTy>>::Yield == ()` is not satisfied

22 LL | | }
23    | |_^
24    |
24    |
- note: required by a bound in `from_generator`
+ note: required by a bound in `std::future::from_generator`
26   --> $SRC_DIR/core/src/future/mod.rs:LL:COL
27    |
28 LL |     T: Generator<ResumeTy, Yield = ()>,

-    |                            ^^^^^^^^^^ required by this bound in `from_generator`
+    |                            ^^^^^^^^^^ required by this bound in `std::future::from_generator`
30 
31 error[E0280]: the requirement `<impl Future<Output = u32> as Future>::Output == u32` is not satisfied


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async/async.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async/async.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args chalkify/bugs/async.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/chalkify/bugs/async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "chalk" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `[static generator@/checkout/src/test/ui/chalkify/bugs/async.rs:7:29: 9:2]: Generator<ResumeTy>` is not satisfied
   |
LL |   async fn foo(x: u32) -> u32 {
   |  _____________________________^
LL | |     x
LL | |     x
LL | | }
   | |_^ the trait `Generator<ResumeTy>` is not implemented for `[static generator@/checkout/src/test/ui/chalkify/bugs/async.rs:7:29: 9:2]`
note: required by a bound in `std::future::from_generator`
  --> /checkout/library/core/src/future/mod.rs:74:8
   |
   |
LL |     T: Generator<ResumeTy, Yield = ()>,
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `std::future::from_generator`

error[E0280]: the requirement `<[static generator@/checkout/src/test/ui/chalkify/bugs/async.rs:7:29: 9:2] as Generator<ResumeTy>>::Yield == ()` is not satisfied
   |
LL |   async fn foo(x: u32) -> u32 {
   |  _____________________________^
LL | |     x
LL | |     x
LL | | }
   | |_^
   |
note: required by a bound in `std::future::from_generator`
  --> /checkout/library/core/src/future/mod.rs:74:28
   |
LL |     T: Generator<ResumeTy, Yield = ()>,
   |                            ^^^^^^^^^^ required by this bound in `std::future::from_generator`

error[E0280]: the requirement `<impl Future<Output = u32> as Future>::Output == u32` is not satisfied
   |
LL |   async fn foo(x: u32) -> u32 {
   |  _____________________________^
LL | |     x
