plain
iiiii.....................i..................i.......................................... 264/13840
........................................................................................ 352/13840
........................................................................................ 440/13840
........................................................................................ 528/13840
........................................................................F............... 616/13840
...............................................................................FF..F.... 704/13840
........................................................................................ 792/13840
..........F..F...i...................................................................... 880/13840
........................................................................................ 1056/13840
........................................................................................ 1144/13840
........................................................................................ 1232/13840
........................................................................................ 1320/13840
---

40 LL | |     panic!()
41 LL | | }
42    | |_^
+ note: required because it appears within the type `impl Future<Output = !>`
+    |
+    |
+ LL | async fn bar2<T>(_: T) -> ! {
+    |                           ^
43    = note: required because it captures the following types: `ResumeTy`, `Option<bool>`, `impl Future<Output = !>`, `()`
44 note: required because it's used within this `async fn` body

51 LL | |     };
52 LL | | }
53    | |_^
53    | |_^
+ note: required because it appears within the type `impl Future<Output = ()>`
+   --> $DIR/async-await-let-else.rs:21:32
+    |
+ LL | async fn foo2(x: Option<bool>) {
54 note: required by a bound in `is_send`
55   --> $DIR/async-await-let-else.rs:19:15
56    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await-let-else.drop-tracking/async-await-let-else.drop-tracking.stderr
To only update this specific test, also pass `--test-args async-await/async-await-let-else.rs`


error in revision `drop-tracking`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-await-let-else.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "drop_tracking" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await-let-else.drop-tracking" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await-let-else.drop-tracking/auxiliary" "--edition=2021" "-Zdrop-tracking=yes"
stdout: none
--- stderr -------------------------------
error: future cannot be sent between threads safely
   |
   |
LL |     is_send(foo(Some(true)));
   |             ^^^^^^^^^^^^^^^ future returned by `foo` is not `Send`
   |
   = help: within `impl Future<Output = ()>`, the trait `Send` is not implemented for `Rc<()>`
note: future is not `Send` as this value is used across an await
   |
   |
LL |         let r = Rc::new(());
   |             - has type `Rc<()>` which is not `Send`
LL |         bar().await
   |              ^^^^^^ await occurs here, with `r` maybe used later
LL |     };
   |     - `r` is later dropped here
  --> /checkout/src/test/ui/async-await/async-await-let-else.rs:19:15
   |
   |
LL | fn is_send<T: Send>(_: T) {}


error[E0277]: `Rc<()>` cannot be sent between threads safely
   |
   |
LL | async fn foo2(x: Option<bool>) {
   |                                - within this `impl Future<Output = ()>`
...
LL |     is_send(foo2(Some(true)));
   |     ------- ^^^^^^^^^^^^^^^^ `Rc<()>` cannot be sent between threads safely
   |     required by a bound introduced by this call
   |
   |
   = help: within `impl Future<Output = ()>`, the trait `Send` is not implemented for `Rc<()>`
note: required because it's used within this `async fn` body
   |
   |
LL |   async fn bar2<T>(_: T) -> ! {
LL | |     panic!()
LL | | }
   | |_^
   | |_^
note: required because it appears within the type `impl Future<Output = !>`
   |
   |
LL | async fn bar2<T>(_: T) -> ! {
   |                           ^
   = note: required because it captures the following types: `ResumeTy`, `Option<bool>`, `impl Future<Output = !>`, `()`
note: required because it's used within this `async fn` body
   |
   |
LL |   async fn foo2(x: Option<bool>) {
   |  ________________________________^
LL | |     let Some(_) = x else {
LL | |         bar2(Rc::new(())).await
LL | |     };
LL | | }
note: required because it appears within the type `impl Future<Output = ()>`
  --> /checkout/src/test/ui/async-await/async-await-let-else.rs:21:32
   |
   |
LL | async fn foo2(x: Option<bool>) {
note: required by a bound in `is_send`
  --> /checkout/src/test/ui/async-await/async-await-let-else.rs:19:15
   |
   |
LL | fn is_send<T: Send>(_: T) {}

error: future cannot be sent between threads safely
  --> /checkout/src/test/ui/async-await/async-await-let-else.rs:52:13
   |
   |
LL |     is_send(foo3(Some(true)));
   |             ^^^^^^^^^^^^^^^^ future returned by `foo3` is not `Send`
   |
   = help: within `impl Future<Output = ()>`, the trait `Send` is not implemented for `Rc<()>`
note: future is not `Send` as this value is used across an await
   |
   |
LL |         (Rc::new(()), bar().await);
   |          -----------       ^^^^^^ await occurs here, with `Rc::new(())` maybe used later
   |          |
   |          has type `Rc<()>` which is not `Send`
note: `Rc::new(())` is later dropped here
   |
   |
LL |         (Rc::new(()), bar().await);
note: required by a bound in `is_send`
  --> /checkout/src/test/ui/async-await/async-await-let-else.rs:19:15
   |
   |
LL | fn is_send<T: Send>(_: T) {}

error: future cannot be sent between threads safely
  --> /checkout/src/test/ui/async-await/async-await-let-else.rs:54:13
   |
   |
LL |     is_send(foo4(Some(true)));
   |             ^^^^^^^^^^^^^^^^ future returned by `foo4` is not `Send`
   |
   = help: within `impl Future<Output = ()>`, the trait `Send` is not implemented for `Rc<()>`
note: future is not `Send` as this value is used across an await
   |
   |
LL |         let r = Rc::new(());
   |             - has type `Rc<()>` which is not `Send`
LL |         bar().await;
   |              ^^^^^^ await occurs here, with `r` maybe used later
LL |     };
LL |     };
   |     - `r` is later dropped here
  --> /checkout/src/test/ui/async-await/async-await-let-else.rs:19:15
   |
   |
LL | fn is_send<T: Send>(_: T) {}

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0277`.
---
diff of stderr:

53 LL | | }
54    | |_^
55 note: required because it appears within the type `impl Future<Output = Arc<RefCell<i32>>>`
+    |
+    |
+ LL | async fn ready2<T>(t: T) -> T {
+    |                             ^
+ note: required because it appears within the type `impl Future<Output = Arc<RefCell<i32>>>`
57    |
57    |
58 LL | fn make_non_send_future2() -> impl Future<Output = Arc<RefCell<i32>>> {
59    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
59    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
60    = note: required because it captures the following types: `ResumeTy`, `impl Future<Output = Arc<RefCell<i32>>>`, `()`, `i32`, `Ready<i32>`
61 note: required because it's used within this `async` block
-   --> $DIR/issue-68112.rs:60:26
63    |
64 LL |       let send_fut = async {
-    |  __________________________^
+    |  ____________________^
+    |  ____________________^
66 LL | |         let non_send_fut = make_non_send_future2();
67 LL | |         let _ = non_send_fut.await;
68 LL | |         ready(0).await;

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-68112.no_drop_tracking/issue-68112.no_drop_tracking.stderr
To only update this specific test, also pass `--test-args async-await/issue-68112.rs`

error in revision `no_drop_tracking`: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-68112.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "no_drop_tracking" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-68112.no_drop_tracking" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-68112.no_drop_tracking/auxiliary" "--edition=2018" "-Zdrop-tracking=no"
stdout: none
--- stderr -------------------------------
error: future cannot be sent between threads safely
   |
LL |     require_send(send_fut);
LL |     require_send(send_fut);
   |                  ^^^^^^^^ future created by async block is not `Send`
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
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
  --> /checkout/src/test/ui/async-await/issue-68112.rs:46:18
   |
LL |     require_send(send_fut);
LL |     require_send(send_fut);
   |                  ^^^^^^^^ future created by async block is not `Send`
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
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
   |     ------------ ^^^^^^^^ `RefCell<i32>` cannot be shared between threads safely
   |     required by a bound introduced by this call
   |
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
   = note: required for `Arc<RefCell<i32>>` to implement `Send`
note: required because it's used within this `async fn` body
   |
LL |   async fn ready2<T>(t: T) -> T {
   |  _______________________________^
LL | |     t
LL | |     t
LL | | }
   | |_^
note: required because it appears within the type `impl Future<Output = Arc<RefCell<i32>>>`
   |
LL | async fn ready2<T>(t: T) -> T {
   |                             ^
   |                             ^
note: required because it appears within the type `impl Future<Output = Arc<RefCell<i32>>>`
   |
   |
LL | fn make_non_send_future2() -> impl Future<Output = Arc<RefCell<i32>>> {
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: required because it captures the following types: `ResumeTy`, `impl Future<Output = Arc<RefCell<i32>>>`, `()`, `i32`, `Ready<i32>`
note: required because it's used within this `async` block
   |
LL |       let send_fut = async {
   |  ____________________^
   |  ____________________^
LL | |         let non_send_fut = make_non_send_future2();
LL | |         let _ = non_send_fut.await;
LL | |         ready(0).await;
LL | |     };
note: required by a bound in `require_send`
  --> /checkout/src/test/ui/async-await/issue-68112.rs:14:25
   |
   |
LL | fn require_send(_: impl Send) {}
   |                         ^^^^ required by this bound in `require_send`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.
---
diff of stderr:

53 LL | | }
54    | |_^
55 note: required because it appears within the type `impl Future<Output = Arc<RefCell<i32>>>`
+    |
+    |
+ LL | async fn ready2<T>(t: T) -> T {
+    |                             ^
+ note: required because it appears within the type `impl Future<Output = Arc<RefCell<i32>>>`
57    |
57    |
58 LL | fn make_non_send_future2() -> impl Future<Output = Arc<RefCell<i32>>> {
59    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
59    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
60    = note: required because it captures the following types: `ResumeTy`, `impl Future<Output = Arc<RefCell<i32>>>`, `()`, `Ready<i32>`
61 note: required because it's used within this `async` block
-   --> $DIR/issue-68112.rs:60:26
63    |
64 LL |       let send_fut = async {
-    |  __________________________^
+    |  ____________________^
+    |  ____________________^
66 LL | |         let non_send_fut = make_non_send_future2();
67 LL | |         let _ = non_send_fut.await;
68 LL | |         ready(0).await;

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-68112.drop_tracking/issue-68112.drop_tracking.stderr
To only update this specific test, also pass `--test-args async-await/issue-68112.rs`


error in revision `drop_tracking`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-68112.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "drop_tracking" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-68112.drop_tracking" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-68112.drop_tracking/auxiliary" "--edition=2018" "-Zdrop-tracking=yes"
stdout: none
--- stderr -------------------------------
error: future cannot be sent between threads safely
   |
LL |     require_send(send_fut);
LL |     require_send(send_fut);
   |                  ^^^^^^^^ future created by async block is not `Send`
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
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
  --> /checkout/src/test/ui/async-await/issue-68112.rs:46:18
   |
LL |     require_send(send_fut);
LL |     require_send(send_fut);
   |                  ^^^^^^^^ future created by async block is not `Send`
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
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
   |     ------------ ^^^^^^^^ `RefCell<i32>` cannot be shared between threads safely
   |     required by a bound introduced by this call
   |
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
   = note: required for `Arc<RefCell<i32>>` to implement `Send`
note: required because it's used within this `async fn` body
   |
LL |   async fn ready2<T>(t: T) -> T {
   |  _______________________________^
LL | |     t
LL | |     t
LL | | }
   | |_^
note: required because it appears within the type `impl Future<Output = Arc<RefCell<i32>>>`
   |
LL | async fn ready2<T>(t: T) -> T {
   |                             ^
   |                             ^
note: required because it appears within the type `impl Future<Output = Arc<RefCell<i32>>>`
   |
   |
LL | fn make_non_send_future2() -> impl Future<Output = Arc<RefCell<i32>>> {
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: required because it captures the following types: `ResumeTy`, `impl Future<Output = Arc<RefCell<i32>>>`, `()`, `Ready<i32>`
note: required because it's used within this `async` block
   |
LL |       let send_fut = async {
   |  ____________________^
   |  ____________________^
LL | |         let non_send_fut = make_non_send_future2();
LL | |         let _ = non_send_fut.await;
LL | |         ready(0).await;
LL | |     };
note: required by a bound in `require_send`
  --> /checkout/src/test/ui/async-await/issue-68112.rs:14:25
   |
   |
LL | fn require_send(_: impl Send) {}
   |                         ^^^^ required by this bound in `require_send`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
---
20    | |_^
+ note: required because it appears within the type `impl Future<Output = ()>`
+   --> $DIR/issue-70935-complex-spans.rs:10:40
+    |
+ LL | async fn baz<T>(_c: impl FnMut() -> T) where T: Future<Output=()> {
+    |                                        ^
21    = note: required because it captures the following types: `ResumeTy`, `impl Future<Output = ()>`, `()`
22 note: required because it's used within this `async` block
-   --> $DIR/issue-70935-complex-spans.rs:16:16
24    |
24    |
- LL |       async move {
-    |  ________________^
+ LL | /     async move {
27 LL | |         baz(|| async{
28 LL | |             foo(tx.clone());
29 LL | |         }).await;

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70935-complex-spans.drop_tracking/issue-70935-complex-spans.drop_tracking.stderr
To only update this specific test, also pass `--test-args async-await/issue-70935-complex-spans.rs`


error in revision `drop_tracking`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-70935-complex-spans.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "drop_tracking" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70935-complex-spans.drop_tracking" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-70935-complex-spans.drop_tracking/auxiliary" "--edition=2018" "-Zdrop-tracking"
stdout: none
--- stderr -------------------------------
error[E0277]: `Sender<i32>` cannot be shared between threads safely
   |
   |
LL | fn foo(tx: std::sync::mpsc::Sender<i32>) -> impl Future + Send {
   |                                             ^^^^^^^^^^^^^^^^^^ `Sender<i32>` cannot be shared between threads safely
   = help: the trait `Sync` is not implemented for `Sender<i32>`
   = help: the trait `Sync` is not implemented for `Sender<i32>`
   = note: required for `&Sender<i32>` to implement `Send`
note: required because it's used within this closure
   |
   |
LL |         baz(|| async{
   |             ^^
note: required because it's used within this `async fn` body
   |
   |
LL |   async fn baz<T>(_c: impl FnMut() -> T) where T: Future<Output=()> {
LL | | }
   | |_^
note: required because it appears within the type `impl Future<Output = ()>`
  --> /checkout/src/test/ui/async-await/issue-70935-complex-spans.rs:10:40
  --> /checkout/src/test/ui/async-await/issue-70935-complex-spans.rs:10:40
   |
LL | async fn baz<T>(_c: impl FnMut() -> T) where T: Future<Output=()> {
   |                                        ^
   = note: required because it captures the following types: `ResumeTy`, `impl Future<Output = ()>`, `()`
note: required because it's used within this `async` block
   |
   |
LL | /     async move {
LL | |         baz(|| async{
LL | |             foo(tx.clone());
LL | |         }).await;
LL | |     }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
---
+   --> $DIR/partial-drop-partial-reinit.rs:31:16
+    |
+ LL | async fn foo() {
+    |                ^
27 note: required by a bound in `gimme_send`
29    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/partial-drop-partial-reinit.drop_tracking/partial-drop-partial-reinit.drop_tracking.stderr
To only update this specific test, also pass `--test-args async-await/partial-drop-partial-reinit.rs`


error in revision `drop_tracking`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/partial-drop-partial-reinit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "drop_tracking" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/partial-drop-partial-reinit.drop_tracking" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/partial-drop-partial-reinit.drop_tracking/auxiliary" "--edition=2021" "-Zdrop-tracking=yes"
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
   = note: required because it captures the following types: `ResumeTy`, `(NotSend,)`, `()`, `impl Future<Output = ()>`
note: required because it's used within this `async fn` body
   |
LL |   async fn foo() {
   |  ________________^
   |  ________________^
LL | |     //~^ NOTE used within this `async fn` body
LL | |     //~| NOTE within this `impl Future
LL | |     let mut x = (NotSend {},);
LL | |     bar().await;
LL | | }
   | |_^
note: required because it appears within the type `impl Future<Output = ()>`
note: required because it appears within the type `impl Future<Output = ()>`
  --> /checkout/src/test/ui/async-await/partial-drop-partial-reinit.rs:31:16
   |
LL | async fn foo() {
   |                ^
note: required by a bound in `gimme_send`
   |
   |
LL | fn gimme_send<T: Send>(t: T) {
   |                  ^^^^ required by this bound in `gimme_send`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
---
+   --> $DIR/partial-drop-partial-reinit.rs:31:16
+    |
+ LL | async fn foo() {
+    |                ^
27 note: required by a bound in `gimme_send`
29    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/partial-drop-partial-reinit.no_drop_tracking/partial-drop-partial-reinit.no_drop_tracking.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/partial-drop-partial-reinit.rs`

error in revision `no_drop_tracking`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/partial-drop-partial-reinit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "no_drop_tracking" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/partial-drop-partial-reinit.no_drop_tracking" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/partial-drop-partial-reinit.no_drop_tracking/auxiliary" "--edition=2021" "-Zdrop-tracking=no"
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
   = note: required because it captures the following types: `ResumeTy`, `(NotSend,)`, `impl Future<Output = ()>`, `()`
note: required because it's used within this `async fn` body
   |
LL |   async fn foo() {
   |  ________________^
   |  ________________^
LL | |     //~^ NOTE used within this `async fn` body
LL | |     //~| NOTE within this `impl Future
LL | |     let mut x = (NotSend {},);
LL | |     bar().await;
LL | | }
   | |_^
note: required because it appears within the type `impl Future<Output = ()>`
note: required because it appears within the type `impl Future<Output = ()>`
  --> /checkout/src/test/ui/async-await/partial-drop-partial-reinit.rs:31:16
   |
LL | async fn foo() {
   |                ^
note: required by a bound in `gimme_send`
   |
   |
LL | fn gimme_send<T: Send>(t: T) {
   |                  ^^^^ required by this bound in `gimme_send`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/async-block.rs#without_feature stdout ----
diff of stderr:

- error[E0658]: `async` blocks are not allowed in constants
-   --> $DIR/async-block.rs:12:47
+ error: fatal error triggered by #[rustc_error]
3    |
3    |
- LL | const _: i32 = { core::mem::ManuallyDrop::new(async { 0 }); 4 };
-    |
-    = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
-    = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable
+ LL | fn main() {}
+ LL | fn main() {}
+    | ^^^^^^^^^
9 
- error[E0658]: `async` blocks are not allowed in statics
-   --> $DIR/async-block.rs:15:51
-    |
- LL | static _FUT: &(dyn Future<Output = ()> + Sync) = &async {};
-    |
-    = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
-    = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable
+ error: aborting due to previous error
---
22 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/async-block.without_feature/async-block.without_feature.stderr
To only update this specific test, also pass `--test-args consts/async-block.rs`


error in revision `without_feature`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/async-block.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "without_feature" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/async-block.without_feature" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/async-block.without_feature/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: fatal error triggered by #[rustc_error]
   |
   |
LL | fn main() {} //[with_feature]~ fatal error triggered by #[rustc_error]

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/generic-associated-types/bugs/issue-100013.rs stdout ----
diff of stderr:

55    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'a` must outlive `'b`
56    |
57    = help: consider adding the following bound: `'a: 'b`
+ help: consider adding 'move' keyword before the nested closure
+    |
+ LL |     move async { // a generator checked for autotrait impl `Send`
58 
59 error: lifetime bound not satisfied
60   --> $DIR/issue-100013.rs:32:5

---
To only update this specific test, also pass `--test-args generic-associated-types/bugs/issue-100013.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/bugs/issue-100013.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-100013" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-100013/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
error: lifetime bound not satisfied
   |
   |
LL | /     async { // a generator checked for autotrait impl `Send`
LL | |         //~^ lifetime bound not satisfied
LL | |         let x = None::<I::Future<'_, '_>>; // a type referencing GAT
LL | |         async {}.await; // a yield point
LL | |     }
   |
note: the lifetime defined here...
  --> /checkout/src/test/ui/generic-associated-types/bugs/issue-100013.rs:17:38
   |
   |
LL |         let x = None::<I::Future<'_, '_>>; // a type referencing GAT
   |                                      ^^
note: ...must outlive the lifetime defined here
   |
   |
LL |         let x = None::<I::Future<'_, '_>>; // a type referencing GAT
   = note: this is a known limitation that will be removed in the future (see issue #100013 <https://github.com/rust-lang/rust/issues/100013> for more information)

error: lifetime bound not satisfied
  --> /checkout/src/test/ui/generic-associated-types/bugs/issue-100013.rs:23:5
  --> /checkout/src/test/ui/generic-associated-types/bugs/issue-100013.rs:23:5
   |
LL | /     async { // a generator checked for autotrait impl `Send`
LL | |         //~^ lifetime bound not satisfied
LL | |         let x = None::<I::Future<'a, 'b>>; // a type referencing GAT
LL | |         //~^ lifetime may not live long enough
LL | |         async {}.await; // a yield point
LL | |     }
   |
note: the lifetime defined here...
  --> /checkout/src/test/ui/generic-associated-types/bugs/issue-100013.rs:22:14
   |
   |
LL | fn call2<'a, 'b, I: FutureIterator>() -> impl Send {
   |              ^^
note: ...must outlive the lifetime defined here
   |
   |
LL | fn call2<'a, 'b, I: FutureIterator>() -> impl Send {
   = note: this is a known limitation that will be removed in the future (see issue #100013 <https://github.com/rust-lang/rust/issues/100013> for more information)

error: lifetime may not live long enough
  --> /checkout/src/test/ui/generic-associated-types/bugs/issue-100013.rs:25:17
  --> /checkout/src/test/ui/generic-associated-types/bugs/issue-100013.rs:25:17
   |
LL | fn call2<'a, 'b, I: FutureIterator>() -> impl Send {
   |          --  -- lifetime `'b` defined here
   |          |
   |          lifetime `'a` defined here
...
LL |         let x = None::<I::Future<'a, 'b>>; // a type referencing GAT
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
help: consider adding 'move' keyword before the nested closure
   |
LL |     move async { // a generator checked for autotrait impl `Send`

error: lifetime bound not satisfied
  --> /checkout/src/test/ui/generic-associated-types/bugs/issue-100013.rs:32:5
   |
   |
LL | /     async { // a generator checked for autotrait impl `Send`
LL | |         //~^ lifetime bound not satisfied
LL | |         let x = None::<I::Future<'a, 'b>>; // a type referencing GAT
LL | |         async {}.await; // a yield point
LL | |     }
   |
note: the lifetime defined here...
  --> /checkout/src/test/ui/generic-associated-types/bugs/issue-100013.rs:31:18
   |
   |
LL | fn call3<'a: 'b, 'b, I: FutureIterator>() -> impl Send {
   |                  ^^
note: ...must outlive the lifetime defined here
   |
   |
LL | fn call3<'a: 'b, 'b, I: FutureIterator>() -> impl Send {
   = note: this is a known limitation that will be removed in the future (see issue #100013 <https://github.com/rust-lang/rust/issues/100013> for more information)

error: aborting due to 4 previous errors
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/impl-trait/in-trait/default-body-with-rpit.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/in-trait/default-body-with-rpit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/default-body-with-rpit" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/default-body-with-rpit/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
error: concrete type differs from previous defining opaque type use
   |
LL |         ""
LL |         ""
   |         ^^ expected `impl Debug`, got `&'static str`
note: previous use here
  --> /checkout/src/test/ui/impl-trait/in-trait/default-body-with-rpit.rs:10:39
   |
   |
LL |       async fn baz(&self) -> impl Debug {
LL | |         ""
LL | |     }
   | |_____^


error[E0720]: cannot resolve opaque type
  --> /checkout/src/test/ui/impl-trait/in-trait/default-body-with-rpit.rs:10:28
   |
LL |     async fn baz(&self) -> impl Debug {
   |                            ^^^^^^^^^^ cannot resolve opaque type
   |
   = note: these returned values have a concrete "never" type
   = help: this error will resolve once the item's body returns a concrete type
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0720`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/impl-trait/issues/issue-78722.rs stdout ----
diff of stderr:

- error[E0658]: `async` blocks are not allowed in constants
-   --> $DIR/issue-78722.rs:13:20
-    |
- LL |         let f: F = async { 1 };
-    |
-    = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
-    = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable
- 
- 
- error[E0493]: destructor of `F` cannot be evaluated at compile-time
-   --> $DIR/issue-78722.rs:13:13
-    |
- LL |         let f: F = async { 1 };
-    |             ^ the destructor for this type cannot be evaluated in constants
- LL |     }],
-    |     - value is dropped here
- 
- 
19 error[E0271]: expected `impl Future<Output = ()>` to be a future that resolves to `u8`, but it resolves to `()`
21    |

22 LL |         fn concrete_use() -> F {
23    |                              ^ expected `()`, found `u8`
---
To only update this specific test, also pass `--test-args impl-trait/issues/issue-78722.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-78722.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-78722" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-78722/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0271]: expected `impl Future<Output = ()>` to be a future that resolves to `u8`, but it resolves to `()`
   |
LL |         fn concrete_use() -> F {
   |                              ^ expected `()`, found `u8`

---
---- [ui] src/test/ui/regions/issue-72051-member-region-hang.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/issue-72051-member-region-hang.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-72051-member-region-hang" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-72051-member-region-hang/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | pub async fn query<'a>(_: &(), _: &(), _: (&(dyn std::any::Any + 'a),) ) {}
   |                    |
   |                    |
   |                    hidden type `impl Future<Output = ()>` captures the lifetime `'a` as defined here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/self/self_lifetime-async.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/self_lifetime-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_lifetime-async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_lifetime-async/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = &'a ()>` captures lifetime that does not appear in bounds
   |
   |
LL |     async fn bar<'a>(self: &Alias, arg: &'a ()) -> &() { arg }
   |
   |
   = note: hidden type `impl Future<Output = &'a ()>` captures lifetime '_#17r
error: aborting due to previous error

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
