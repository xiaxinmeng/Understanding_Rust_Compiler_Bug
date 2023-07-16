plain
iiiii.....................i..................i.......................................... 264/13831
........................................................................................ 352/13831
........................................................................................ 440/13831
........................................................................................ 528/13831
......................................................................F.F.............FF 616/13831
.F.........................F..................................................FFF..F.... 704/13831
..........................................F.F......F..................F.F..FFFF..F.F.... 792/13831
.....F....F......i........F............................................................. 880/13831
........................................................................................ 1056/13831
........................................................................................ 1144/13831
........................................................................................ 1232/13831
........................................................................................ 1320/13831
---
........................................................................................ 10032/13831
........................................................................................ 10120/13831
....................................................ii...............i...iii............ 10208/13831
........................................................................................ 10296/13831
.................................................................F...............F...... 10384/13831
........................................................................................ 10560/13831
........................................................................................ 10648/13831
........................................................................................ 10736/13831
........................................................................................ 10824/13831
........................................................................................ 10824/13831
........................................................................................ 10912/13831
............iiiii...i.....ii............................................................ 11000/13831
........................................................................................ 11088/13831
...........i............................................................................ 11176/13831
.....................iiiiii.i..iiiiiiiiiii.i.........................F.................. 11264/13831
.....F..F...F...FFFF.....F...........................F.................................. 11352/13831
........................................................................................ 11528/13831
........................................................................................ 11616/13831
........................................................................................ 11704/13831
........................................................................................ 11792/13831
---
---- [ui] src/test/ui/async-await/async-assoc-fn-anon-lifetimes.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-assoc-fn-anon-lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-assoc-fn-anon-lifetimes" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-assoc-fn-anon-lifetimes/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn assoc(x: &u32, y: B<'_>) {
   |  _______________________________________^
LL | |         async fn nested(x: &u32, y: A<'_, '_>) {}
LL | |     }
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#28r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn assoc(x: &u32, y: B<'_>) {
   |  _______________________________________^
LL | |         async fn nested(x: &u32, y: A<'_, '_>) {}
LL | |     }
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#29r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL |         async fn nested(x: &u32, y: A<'_, '_>) {}
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL |         async fn nested(x: &u32, y: A<'_, '_>) {}
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn assoc2(x: &u32, y: A<'_, '_>) {
   |  ____________________________________________^
LL | |         impl A<'_, '_> {
LL | |             async fn nested_assoc(x: &u32, y: B<'_>) {}
LL | |         }
LL | |     }
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#33r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn assoc2(x: &u32, y: A<'_, '_>) {
   |  ____________________________________________^
LL | |         impl A<'_, '_> {
LL | |             async fn nested_assoc(x: &u32, y: B<'_>) {}
LL | |         }
LL | |     }
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#34r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL |             async fn nested_assoc(x: &u32, y: B<'_>) {}
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#28r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL |             async fn nested_assoc(x: &u32, y: B<'_>) {}
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#29r
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
---

40 LL | |     panic!()
41 LL | | }
42    | |_^
-    = note: required because it captures the following types: `ResumeTy`, `Option<bool>`, `impl Future<Output = !>`, `()`
+ note: required because it appears within the type `impl Future<Output = !>`
+    |
+    |
+ LL | async fn bar2<T>(_: T) -> ! {
+    |                           ^
+    = note: required because it captures the following types: `&mut Context<'_>`, `Option<bool>`, `impl Future<Output = !>`, `()`
44 note: required because it's used within this `async fn` body
46    |

51 LL | |     };
52 LL | | }
52 LL | | }
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
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
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
   = note: required because it captures the following types: `&mut Context<'_>`, `Option<bool>`, `impl Future<Output = !>`, `()`
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
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/async-await/async-await.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "default" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.default/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.default/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = u8>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn async_fn_multiple_args(x: &u8, _y: &u8) -> u8 {
   |  ________________________________________________________^
LL | |     wake_and_yield_once().await;
LL | |     *x
LL | | }
   |
   |
   = note: hidden type `impl Future<Output = u8>` captures lifetime '_#14r

error[E0700]: hidden type for `impl Future<Output = u8>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn async_fn_multiple_args(x: &u8, _y: &u8) -> u8 {
   |  ________________________________________________________^
LL | |     wake_and_yield_once().await;
LL | |     *x
LL | | }
   |
   |
   = note: hidden type `impl Future<Output = u8>` captures lifetime '_#15r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/async-await.rs#thirunsafeck stdout ----

error in revision `thirunsafeck`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thirunsafeck" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.thirunsafeck/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.thirunsafeck/auxiliary" "-Zthir-unsafeck" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = u8>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn async_fn_multiple_args(x: &u8, _y: &u8) -> u8 {
   |  ________________________________________________________^
LL | |     wake_and_yield_once().await;
LL | |     *x
LL | | }
   |
   |
   = note: hidden type `impl Future<Output = u8>` captures lifetime '_#14r

error[E0700]: hidden type for `impl Future<Output = u8>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn async_fn_multiple_args(x: &u8, _y: &u8) -> u8 {
   |  ________________________________________________________^
LL | |     wake_and_yield_once().await;
LL | |     *x
LL | | }
   |
   |
   = note: hidden type `impl Future<Output = u8>` captures lifetime '_#15r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/async-await.rs#nomiropt stdout ----

error in revision `nomiropt`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nomiropt" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.nomiropt/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.nomiropt/auxiliary" "-Z" "mir-opt-level=0" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = u8>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn async_fn_multiple_args(x: &u8, _y: &u8) -> u8 {
   |  ________________________________________________________^
LL | |     wake_and_yield_once().await;
LL | |     *x
LL | | }
   |
   |
   = note: hidden type `impl Future<Output = u8>` captures lifetime '_#14r

error[E0700]: hidden type for `impl Future<Output = u8>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn async_fn_multiple_args(x: &u8, _y: &u8) -> u8 {
   |  ________________________________________________________^
LL | |     wake_and_yield_once().await;
LL | |     *x
LL | | }
   |
   |
   = note: hidden type `impl Future<Output = u8>` captures lifetime '_#15r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/in-trait/async-associated-types.rs stdout ----
diff of stderr:

52    = note: expected `MyTrait<'static, 'static, T>`
53               found `MyTrait<'_, '_, T>`
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error[E0700]: hidden type for `impl Future<Output = (&'a U, &'b T)>` captures lifetime that does not appear in bounds
+    |
+    |
+ LL |       async fn foo(&'a self, key: &'b T) -> (&'a U, &'b T) {
+ LL | |         (self, key)
+ LL | |     }
+    | |_____^
+    |
+    |
+    = note: hidden type `impl Future<Output = (&'a U, &'b T)>` captures lifetime '_#24r
- For more information about this error, try `rustc --explain E0495`.
- For more information about this error, try `rustc --explain E0495`.
+ error[E0700]: hidden type for `impl Future<Output = (&'a U, &'b T)>` captures lifetime that does not appear in bounds
+    |
+    |
+ LL |       async fn foo(&'a self, key: &'b T) -> (&'a U, &'b T) {
+ LL | |         (self, key)
+ LL | |     }
+    | |_____^
+    |
+    |
+    = note: hidden type `impl Future<Output = (&'a U, &'b T)>` captures lifetime '_#25r
+ error: aborting due to 4 previous errors
+ 
+ Some errors have detailed explanations: E0495, E0700.
+ For more information about an error, try `rustc --explain E0495`.
---
To only update this specific test, also pass `--test-args async-await/in-trait/async-associated-types.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/in-trait/async-associated-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/async-associated-types" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/async-associated-types/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements
   |
   |
LL |     async fn foo(&'a self, key: &'b T) -> (&'a U, &'b T) {
   |
   |
note: first, the lifetime cannot outlive the lifetime `'a` as defined here...
   |
   |
LL | impl<'a, 'b, T: Debug + Sized + 'b, U: 'a> MyTrait<'a, 'b, T> for U {
note: ...so that the types are compatible
  --> /checkout/src/test/ui/async-await/in-trait/async-associated-types.rs:19:43
   |
   |
LL |     async fn foo(&'a self, key: &'b T) -> (&'a U, &'b T) {
   |                                           ^^^^^^^^^^^^^^
   = note: expected `(&'a U, &'b T)`
              found `(&U, &T)`
   = note: but, the lifetime must be valid for the static lifetime...
  --> /checkout/src/test/ui/async-await/in-trait/async-associated-types.rs:19:43
   |
   |
LL |     async fn foo(&'a self, key: &'b T) -> (&'a U, &'b T) {
   |                                           ^^^^^^^^^^^^^^
   = note: expected `MyTrait<'static, 'static, T>`
              found `MyTrait<'_, '_, T>`

error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'b` due to conflicting requirements
   |
   |
LL |     async fn foo(&'a self, key: &'b T) -> (&'a U, &'b T) {
   |
   |
note: first, the lifetime cannot outlive the lifetime `'b` as defined here...
   |
   |
LL | impl<'a, 'b, T: Debug + Sized + 'b, U: 'a> MyTrait<'a, 'b, T> for U {
note: ...so that the types are compatible
  --> /checkout/src/test/ui/async-await/in-trait/async-associated-types.rs:19:43
   |
   |
LL |     async fn foo(&'a self, key: &'b T) -> (&'a U, &'b T) {
   |                                           ^^^^^^^^^^^^^^
   = note: expected `(&'a U, &'b T)`
              found `(&U, &T)`
   = note: but, the lifetime must be valid for the static lifetime...
  --> /checkout/src/test/ui/async-await/in-trait/async-associated-types.rs:19:43
   |
   |
LL |     async fn foo(&'a self, key: &'b T) -> (&'a U, &'b T) {
   |                                           ^^^^^^^^^^^^^^
   = note: expected `MyTrait<'static, 'static, T>`
              found `MyTrait<'_, '_, T>`

error[E0700]: hidden type for `impl Future<Output = (&'a U, &'b T)>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn foo(&'a self, key: &'b T) -> (&'a U, &'b T) {
LL | |         (self, key)
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = (&'a U, &'b T)>` captures lifetime '_#24r

error[E0700]: hidden type for `impl Future<Output = (&'a U, &'b T)>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn foo(&'a self, key: &'b T) -> (&'a U, &'b T) {
LL | |         (self, key)
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = (&'a U, &'b T)>` captures lifetime '_#25r
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0495, E0700.
For more information about an error, try `rustc --explain E0495`.
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
-    = note: required because it captures the following types: `ResumeTy`, `impl Future<Output = Arc<RefCell<i32>>>`, `()`, `i32`, `Ready<i32>`
+    = note: required because it captures the following types: `&mut Context<'_>`, `impl Future<Output = Arc<RefCell<i32>>>`, `()`, `i32`, `Ready<i32>`
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
   = note: required because it captures the following types: `&mut Context<'_>`, `impl Future<Output = Arc<RefCell<i32>>>`, `()`, `i32`, `Ready<i32>`
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
------------------------------------------


---- [ui] src/test/ui/async-await/issue-68112.rs#drop_tracking stdout ----
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
-    = note: required because it captures the following types: `ResumeTy`, `impl Future<Output = Arc<RefCell<i32>>>`, `()`, `Ready<i32>`
+    = note: required because it captures the following types: `&mut Context<'_>`, `impl Future<Output = Arc<RefCell<i32>>>`, `()`, `Ready<i32>`
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
   = note: required because it captures the following types: `&mut Context<'_>`, `impl Future<Output = Arc<RefCell<i32>>>`, `()`, `Ready<i32>`
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
------------------------------------------


---- [ui] src/test/ui/async-await/issue-69446-fnmut-capture.rs stdout ----
diff of stderr:

14    |
15    = note: `FnMut` closures only have access to their captured variables while they are executing...
16    = note: ...therefore, they cannot allow references to captured variables to escape
+    = note: requirement occurs because of a mutable reference to `Context<'_>`
+    = note: mutable references are invariant over their type parameter
+    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
18 error: aborting due to previous error
19 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-69446-fnmut-capture/issue-69446-fnmut-capture.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issue-69446-fnmut-capture.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-69446-fnmut-capture.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-69446-fnmut-capture" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-69446-fnmut-capture/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: captured variable cannot escape `FnMut` closure body
   |
LL |       let mut x = Foo;
   |           ----- variable defined here
   |           ----- variable defined here
LL |       bar(move || async { //~ ERROR captured
   |  _______________-_^
   | |               |
   | |               inferred to be a `FnMut` closure
LL | |         x.foo();
LL | |     });
LL | |     });
   | |_____^ returns an `async` block that contains a reference to a captured variable, which then escapes the closure body
   |
   = note: `FnMut` closures only have access to their captured variables while they are executing...
   = note: ...therefore, they cannot allow references to captured variables to escape
   = note: requirement occurs because of a mutable reference to `Context<'_>`
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/async-await/issue-70935-complex-spans.rs#drop_tracking stdout ----
diff of stderr:

18    |  ___________________________________________________________________^
19 LL | | }
20    | |_^
-    = note: required because it captures the following types: `ResumeTy`, `impl Future<Output = ()>`, `()`
+ note: required because it appears within the type `impl Future<Output = ()>`
+    |
+    |
+ LL | async fn baz<T>(_c: impl FnMut() -> T) where T: Future<Output=()> {
+    |                                        ^
+    = note: required because it captures the following types: `&mut Context<'_>`, `impl Future<Output = ()>`, `()`
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
   = note: required because it captures the following types: `&mut Context<'_>`, `impl Future<Output = ()>`, `()`
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
For more information about this error, try `rustc --explain E0277`.
------------------------------------------


---- [ui] src/test/ui/async-await/issues/issue-63388-3.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-63388-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-3/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
LL |       ) {
   |  _______^
LL | |     }
LL | |     }
   | |_____^
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#16r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
LL |       ) {
   |  _______^
LL | |     }
LL | |     }
   | |_____^
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#17r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/issues/issue-63388-4.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-63388-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-4" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-63388-4/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = &A>` captures lifetime that does not appear in bounds
   |
   |
LL |     async fn foo(&self, f: &u32) -> &A { self }
   |
   |
   = note: hidden type `impl Future<Output = &A>` captures lifetime '_#15r

error[E0700]: hidden type for `impl Future<Output = &A>` captures lifetime that does not appear in bounds
   |
   |
LL |     async fn foo(&self, f: &u32) -> &A { self }
   |
   |
   = note: hidden type `impl Future<Output = &A>` captures lifetime '_#16r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/issues/issue-64433.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-64433.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64433" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-64433/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = Result<(), String>>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn something_with_a(&mut self, a: A<'_>) -> Result<(), String> {
LL | |         println!("{:?}", a);
LL | |         Ok(())
LL | |     }
   | |_____^
   | |_____^
   |
   = note: hidden type `impl Future<Output = Result<(), String>>` captures lifetime '_#14r

error[E0700]: hidden type for `impl Future<Output = Result<(), String>>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn something_with_a(&mut self, a: A<'_>) -> Result<(), String> {
LL | |         println!("{:?}", a);
LL | |         Ok(())
LL | |     }
   | |_____^
   | |_____^
   |
   = note: hidden type `impl Future<Output = Result<(), String>>` captures lifetime '_#15r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/multiple-lifetimes/elided.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/multiple-lifetimes/elided.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/elided/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/elided/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#14r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#15r
error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args async-await/multiple-lifetimes/ret-impl-trait-one.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |   async fn async_ret_impl_trait3<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> + 'b {
   |  ________________________________--__--_______________________________________________^
   | |                                |   |
   | |                                |   lifetime `'b` defined here
   | |                                lifetime `'a` defined here
LL | |     //~^ ERROR lifetime may not live long enough
LL | |     (a, b)
LL | | }
   | |_^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`

error[E0700]: hidden type for `impl Future<Output = impl Trait<'a>>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> {
   |  ________________________________________________________________________________^
LL | |     //~^ ERROR captures lifetime that does not appear in bounds
LL | |     (a, b)
LL | | }
   |
   |
   = note: hidden type `impl Future<Output = (&'a u8, &'b u8)>` captures lifetime '_#16r

error[E0700]: hidden type for `impl Future<Output = impl Trait<'a>>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> {
   |  ________________________________________________________________________________^
LL | |     //~^ ERROR captures lifetime that does not appear in bounds
LL | |     (a, b)
LL | | }
   |
   |
   = note: hidden type `impl Future<Output = (&'a u8, &'b u8)>` captures lifetime '_#17r

error[E0700]: hidden type for `impl Trait<'a>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> {
   |  ____________________________________--__________________________________________^
   | |                                    |
   | |                                    hidden type `(&'a u8, &'b u8)` captures the lifetime `'b` as defined here
LL | |     //~^ ERROR captures lifetime that does not appear in bounds
LL | |     (a, b)
LL | | }
   |
   |
help: to declare that `impl Trait<'a>` captures `'b`, you can add an explicit `'b` lifetime bound
   |
LL | async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> + 'b {

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0700`.
For more information about this error, try `rustc --explain E0700`.
------------------------------------------


---- [ui] src/test/ui/async-await/multiple-lifetimes/partial-relation.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/multiple-lifetimes/partial-relation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/partial-relation/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/partial-relation/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = (&'a u32, &'b u32)>` captures lifetime that does not appear in bounds
   |
LL | / {
LL | / {
LL | |     drop((a, c));
LL | |     (b, b)
LL | | }
   |
   |
   = note: hidden type `impl Future<Output = (&'a u32, &'b u32)>` captures lifetime '_#29r

error[E0700]: hidden type for `impl Future<Output = (&'a u32, &'b u32)>` captures lifetime that does not appear in bounds
   |
LL | / {
LL | / {
LL | |     drop((a, c));
LL | |     (b, b)
LL | | }
   |
   |
   = note: hidden type `impl Future<Output = (&'a u32, &'b u32)>` captures lifetime '_#30r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-fg.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-fg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-fg/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-fg/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = impl Trait<'a, 'b>>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn async_ret_impl_trait<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a, 'b> {
LL | |     (a, b)
LL | | }
   | |_^
   |
   |
   = note: hidden type `impl Future<Output = (&'a u8, &'b u8)>` captures lifetime '_#16r

error[E0700]: hidden type for `impl Future<Output = impl Trait<'a, 'b>>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn async_ret_impl_trait<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a, 'b> {
LL | |     (a, b)
LL | | }
   | |_^
   |
   |
   = note: hidden type `impl Future<Output = (&'a u8, &'b u8)>` captures lifetime '_#17r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/multiple-lifetimes/variance.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/multiple-lifetimes/variance.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/variance/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/variance/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn lotsa_lifetimes<'a, 'b, 'c>(_: fn(&'a u8), _: fn(&'b u8) -> &'b u8, _: fn() -> &'c u8) { }
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#21r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn lotsa_lifetimes<'a, 'b, 'c>(_: fn(&'a u8), _: fn(&'b u8) -> &'b u8, _: fn() -> &'c u8) { }
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#22r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/multiple-lifetimes/ret-ref.rs stdout ----
diff of stderr:

+ error[E0700]: hidden type for `impl Future<Output = &'a u8>` captures lifetime that does not appear in bounds
+    |
+    |
+ LL |   async fn multiple_named_lifetimes<'a, 'b>(a: &'a u8, _: &'b u8) -> &'a u8 {
+ LL | |     a
+ LL | | }
+    | |_^
+    |
+    |
+    = note: hidden type `impl Future<Output = &'a u8>` captures lifetime '_#15r
+ 
+ error[E0700]: hidden type for `impl Future<Output = &'a u8>` captures lifetime that does not appear in bounds
+    |
+    |
+ LL |   async fn multiple_named_lifetimes<'a, 'b>(a: &'a u8, _: &'b u8) -> &'a u8 {
+ LL | |     a
+ LL | | }
+    | |_^
+    |
+    |
+    = note: hidden type `impl Future<Output = &'a u8>` captures lifetime '_#16r
+ 
1 error[E0506]: cannot assign to `a` because it is borrowed
3    |

32 LL |     drop(p);
33    |          - borrow later used here
---
To only update this specific test, also pass `--test-args async-await/multiple-lifetimes/ret-ref.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/multiple-lifetimes/ret-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-ref" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-ref/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = &'a u8>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn multiple_named_lifetimes<'a, 'b>(a: &'a u8, _: &'b u8) -> &'a u8 {
LL | |     a
LL | | }
   | |_^
   |
   |
   = note: hidden type `impl Future<Output = &'a u8>` captures lifetime '_#15r

error[E0700]: hidden type for `impl Future<Output = &'a u8>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn multiple_named_lifetimes<'a, 'b>(a: &'a u8, _: &'b u8) -> &'a u8 {
LL | |     a
LL | | }
   | |_^
   |
   |
   = note: hidden type `impl Future<Output = &'a u8>` captures lifetime '_#16r

error[E0506]: cannot assign to `a` because it is borrowed
   |
   |
LL |     let future = multiple_named_lifetimes(&a, &b);
   |                                           -- borrow of `a` occurs here
LL |     a += 1; //~ ERROR cannot assign
   |     ^^^^^^ assignment to borrowed `a` occurs here
LL |     b += 1; //~ ERROR cannot assign
LL |     let p = future.await;


error[E0506]: cannot assign to `b` because it is borrowed
   |
   |
LL |     let future = multiple_named_lifetimes(&a, &b);
   |                                               -- borrow of `b` occurs here
LL |     a += 1; //~ ERROR cannot assign
LL |     b += 1; //~ ERROR cannot assign
   |     ^^^^^^ assignment to borrowed `b` occurs here
LL |     let p = future.await;


error[E0506]: cannot assign to `a` because it is borrowed
   |
   |
LL |     let future = multiple_named_lifetimes(&a, &b);
   |                                           -- borrow of `a` occurs here
LL |     let p = future.await;
LL |     a += 1; //~ ERROR cannot assign
   |     ^^^^^^ assignment to borrowed `a` occurs here
LL |     b += 1;
LL |     drop(p);

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0506, E0700.
---
---- [ui] src/test/ui/async-await/partial-drop-partial-reinit.rs#drop_tracking stdout ----
diff of stderr:

11    |
12    = help: within `impl Future<Output = ()>`, the trait `Send` is not implemented for `NotSend`
13    = note: required because it appears within the type `(NotSend,)`
-    = note: required because it captures the following types: `ResumeTy`, `(NotSend,)`, `()`, `impl Future<Output = ()>`
+    = note: required because it captures the following types: `&mut Context<'_>`, `(NotSend,)`, `()`, `impl Future<Output = ()>`
15 note: required because it's used within this `async fn` body
17    |

24 LL | |     bar().await;
25 LL | | }
25 LL | | }
26    | |_^
+ note: required because it appears within the type `impl Future<Output = ()>`
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
   = note: required because it captures the following types: `&mut Context<'_>`, `(NotSend,)`, `()`, `impl Future<Output = ()>`
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


---- [ui] src/test/ui/async-await/partial-drop-partial-reinit.rs#no_drop_tracking stdout ----
diff of stderr:

11    |
12    = help: within `impl Future<Output = ()>`, the trait `Send` is not implemented for `NotSend`
13    = note: required because it appears within the type `(NotSend,)`
-    = note: required because it captures the following types: `ResumeTy`, `(NotSend,)`, `impl Future<Output = ()>`, `()`
+    = note: required because it captures the following types: `&mut Context<'_>`, `(NotSend,)`, `impl Future<Output = ()>`, `()`
15 note: required because it's used within this `async fn` body
17    |

24 LL | |     bar().await;
25 LL | | }
25 LL | | }
26    | |_^
+ note: required because it appears within the type `impl Future<Output = ()>`
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
   = note: required because it captures the following types: `&mut Context<'_>`, `(NotSend,)`, `impl Future<Output = ()>`, `()`
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


---- [ui] src/test/ui/async-await/unused-lifetime.rs stdout ----
diff of stderr:

44    |                        |
45    |                        help: elide the unused lifetime
- error: aborting due to 6 previous errors
- error: aborting due to 6 previous errors
+ error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
+   --> $DIR/unused-lifetime.rs:10:46
+    |
+ LL | async fn async_wrong_1_lifetime<'a>(_: &i32) {}
+    |
+    |
+    = note: hidden type `impl Future<Output = ()>` captures lifetime '_#16r
48 
+ error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
+   --> $DIR/unused-lifetime.rs:10:46
+    |
+ LL | async fn async_wrong_1_lifetime<'a>(_: &i32) {}
+    |
+    |
+    = note: hidden type `impl Future<Output = ()>` captures lifetime '_#17r
+ 
+ error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
+   --> $DIR/unused-lifetime.rs:12:63
+    |
+ LL | async fn async_wrong_2_lifetimes<'a, 'b>(_: &'a i32, _: &i32) {}
+    |
+    |
+    = note: hidden type `impl Future<Output = ()>` captures lifetime '_#21r
+ 
+ error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
+   --> $DIR/unused-lifetime.rs:12:63
+    |
+ LL | async fn async_wrong_2_lifetimes<'a, 'b>(_: &'a i32, _: &i32) {}
+    |
+    |
+    = note: hidden type `impl Future<Output = ()>` captures lifetime '_#22r
+ 
+ error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
+   --> $DIR/unused-lifetime.rs:16:66
+    |
+ LL | async fn async_right_2_lifetimes<'a, 'b>(_: &'a i32, _: &'b i32) {}
+    |
+    |
+    = note: hidden type `impl Future<Output = ()>` captures lifetime '_#14r
+ 
+ error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
+   --> $DIR/unused-lifetime.rs:16:66
+    |
+ LL | async fn async_right_2_lifetimes<'a, 'b>(_: &'a i32, _: &'b i32) {}
+    |
+    |
+    = note: hidden type `impl Future<Output = ()>` captures lifetime '_#15r
+ error: aborting due to 12 previous errors
+ 
+ For more information about this error, try `rustc --explain E0700`.
49 
---
To only update this specific test, also pass `--test-args async-await/unused-lifetime.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/unused-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unused-lifetime" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unused-lifetime/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: lifetime parameter `'a` never used
   |
   |
LL | async fn async_wrong_without_args<'a>() {} //~ ERROR lifetime parameter `'a` never used
   |                                  -^^- help: elide the unused lifetime
note: the lint level is defined here
  --> /checkout/src/test/ui/async-await/unused-lifetime.rs:6:9
   |
LL | #![deny(unused_lifetimes)]
LL | #![deny(unused_lifetimes)]
   |         ^^^^^^^^^^^^^^^^

error: lifetime parameter `'a` never used
   |
   |
LL | async fn async_wrong_1_lifetime<'a>(_: &i32) {} //~ ERROR lifetime parameter `'a` never used
   |                                -^^- help: elide the unused lifetime

error: lifetime parameter `'b` never used
   |
   |
LL | async fn async_wrong_2_lifetimes<'a, 'b>(_: &'a i32, _: &i32) {} //~ ERROR lifetime parameter `'b` never used
   |                                    --^^
   |                                    |
   |                                    help: elide the unused lifetime

error: lifetime parameter `'a` never used
   |
   |
LL | fn wrong_without_args<'a>() {} //~ ERROR lifetime parameter `'a` never used
   |                      -^^- help: elide the unused lifetime

error: lifetime parameter `'a` never used
   |
   |
LL | fn wrong_1_lifetime<'a>(_: &i32) {} //~ ERROR lifetime parameter `'a` never used
   |                    -^^- help: elide the unused lifetime

error: lifetime parameter `'b` never used
   |
   |
LL | fn wrong_2_lifetimes<'a, 'b>(_: &'a i32, _: &i32) {} //~ ERROR lifetime parameter `'b` never used
   |                        --^^
   |                        |
   |                        help: elide the unused lifetime

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn async_wrong_1_lifetime<'a>(_: &i32) {} //~ ERROR lifetime parameter `'a` never used
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#16r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn async_wrong_1_lifetime<'a>(_: &i32) {} //~ ERROR lifetime parameter `'a` never used
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#17r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn async_wrong_2_lifetimes<'a, 'b>(_: &'a i32, _: &i32) {} //~ ERROR lifetime parameter `'b` never used
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#21r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn async_wrong_2_lifetimes<'a, 'b>(_: &'a i32, _: &i32) {} //~ ERROR lifetime parameter `'b` never used
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#22r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn async_right_2_lifetimes<'a, 'b>(_: &'a i32, _: &'b i32) {}
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#14r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn async_right_2_lifetimes<'a, 'b>(_: &'a i32, _: &'b i32) {}
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#15r
error: aborting due to 12 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/closures/2229_closure_analysis/run_pass/issue-88431.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/run_pass/issue-88431.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/run_pass/issue-88431" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/run_pass/issue-88431/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = Box<(dyn GameState<'a> + 'a)>>` captures lifetime that does not appear in bounds
   |
   |
LL |   ) -> Box<dyn GameState<'a> + 'a> {
LL | |     unimplemented!()
LL | | }
   | |_^
   |
   |
   = note: hidden type `impl Future<Output = Box<(dyn GameState<'a> + 'a)>>` captures lifetime '_#21r

error[E0700]: hidden type for `impl Future<Output = Box<(dyn GameState<'a> + 'a)>>` captures lifetime that does not appear in bounds
   |
   |
LL |   ) -> Box<dyn GameState<'a> + 'a> {
LL | |     unimplemented!()
LL | | }
   | |_^
   |
   |
   = note: hidden type `impl Future<Output = Box<(dyn GameState<'a> + 'a)>>` captures lifetime '_#22r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
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



---- [ui] src/test/ui/generator/issue-93161.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/issue-93161.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-93161/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-93161/auxiliary" "--edition=2021" "-Zdrop-tracking"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = Option<()>>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn read_exact(_from: &mut &[u8], _to: &mut [u8]) -> Option<()> {
LL | |     Some(())
LL | | }
   | |_^
   |
   |
   = note: hidden type `impl Future<Output = Option<()>>` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = Option<()>>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn read_exact(_from: &mut &[u8], _to: &mut [u8]) -> Option<()> {
LL | |     Some(())
LL | | }
   | |_^
   |
   |
   = note: hidden type `impl Future<Output = Option<()>>` captures lifetime '_#20r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
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


---- [ui] src/test/ui/generic-associated-types/issue-90014.rs stdout ----
diff of stderr:

13 LL |     type Fut<'a> = impl Future<Output = ()>;
15 
- error: aborting due to previous error
+ error: non-defining opaque type use in defining scope
+   --> $DIR/issue-90014.rs:17:9
+   --> $DIR/issue-90014.rs:17:9
+    |
+ LL |         async { () }
+    |         ^^^^^^^^^^^^ lifetime `'a` is part of concrete type but not used in parameter list of the `impl Trait` type alias
+ error: non-defining opaque type use in defining scope
+   --> $DIR/issue-90014.rs:17:9
+    |
+    |
+ LL |         async { () }
+    |         ^^^^^^^^^^^^ lifetime `'a` is part of concrete type but not used in parameter list of the `impl Trait` type alias
+ error: aborting due to 3 previous errors
17 
18 For more information about this error, try `rustc --explain E0477`.
19 
---
To only update this specific test, also pass `--test-args generic-associated-types/issue-90014.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-90014.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-90014" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-90014/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0477]: the type `&mut ()` does not fulfill the required lifetime
   |
   |
LL |     type Fut<'a> where Self: 'a;
   |     ------------ definition of `Fut` from trait
...
LL |     type Fut<'a> = impl Future<Output = ()>;
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^- help: try copying this clause from the trait: `where Self: 'a`
   |
note: type must outlive the lifetime `'a` as defined here
   |
   |
LL |     type Fut<'a> = impl Future<Output = ()>;

error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/generic-associated-types/issue-90014.rs:17:9
   |
   |
LL |         async { () }
   |         ^^^^^^^^^^^^ lifetime `'a` is part of concrete type but not used in parameter list of the `impl Trait` type alias
error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/generic-associated-types/issue-90014.rs:17:9
   |
LL |         async { () }
LL |         async { () }
   |         ^^^^^^^^^^^^ lifetime `'a` is part of concrete type but not used in parameter list of the `impl Trait` type alias
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0477`.
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
---- [ui] src/test/ui/lifetimes/issue-77175.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/issue-77175.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-77175" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-77175/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = &'a str>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn bar<'a>(s1: String, s2: &'_ str, s3: &'a str) -> &'a str {
LL | |     s3
LL | | }
   | |_^
   |
   |
   = note: hidden type `impl Future<Output = &'a str>` captures lifetime '_#15r

error[E0700]: hidden type for `impl Future<Output = &'a str>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn bar<'a>(s1: String, s2: &'_ str, s3: &'a str) -> &'a str {
LL | |     s3
LL | | }
   | |_^
   |
   |
   = note: hidden type `impl Future<Output = &'a str>` captures lifetime '_#16r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
---
19 LL |     async {};

20    |     ^^^^^^^^
21    |
-    = note: futures do nothing unless you `.await` or poll them
+    = note: generators are lazy and do nothing unless resumed
24 error: unused closure that must be used
25   --> $DIR/unused-closure.rs:14:5



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-closure/unused-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/unused/unused-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/unused-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-closure/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/lint/unused/unused-closure.rs:9:5
   |
   |
LL | /     || { //~ ERROR unused closure that must be used
LL | |         println!("Hello!");
LL | |     };
   |
   |
   = note: closures are lazy and do nothing unless called
  --> /checkout/src/test/ui/lint/unused/unused-closure.rs:6:9
   |
LL | #![deny(unused_must_use)]
   |         ^^^^^^^^^^^^^^^
   |         ^^^^^^^^^^^^^^^

error: unused generator that must be used
  --> /checkout/src/test/ui/lint/unused/unused-closure.rs:13:5
   |
LL |     async {};    //~ ERROR unused implementer of `Future` that must be used
   |
   |
   = note: generators are lazy and do nothing unless resumed
error: unused closure that must be used
  --> /checkout/src/test/ui/lint/unused/unused-closure.rs:14:5
   |
   |
LL |     || async {}; //~ ERROR unused closure that must be used
   |
   |
   = note: closures are lazy and do nothing unless called
error: unused closure that must be used
  --> /checkout/src/test/ui/lint/unused/unused-closure.rs:15:5
   |
   |
LL |     async || {}; //~ ERROR unused closure that must be used
   |
   |
   = note: closures are lazy and do nothing unless called
error: unused array of boxed arrays of closures that must be used
  --> /checkout/src/test/ui/lint/unused/unused-closure.rs:18:5
   |
   |
LL |     [Box::new([|| {}; 10]); 1]; //~ ERROR unused array of boxed arrays of closures that must be used
   |
   |
   = note: closures are lazy and do nothing unless called
error: unused closure that must be used
  --> /checkout/src/test/ui/lint/unused/unused-closure.rs:20:5
   |
   |
LL |     vec![|| "a"].pop().unwrap(); //~ ERROR unused closure that must be used
   |
   |
   = note: closures are lazy and do nothing unless called
error: unused closure that must be used
  --> /checkout/src/test/ui/lint/unused/unused-closure.rs:23:9
   |
   |
LL |         || true; //~ ERROR unused closure that must be used
   |
   |
   = note: closures are lazy and do nothing unless called
error: aborting due to 7 previous errors
------------------------------------------



---- [ui] src/test/ui/regions/closure-in-projection-issue-97405.rs stdout ----
diff of stderr:

- error[E0310]: the associated type `<impl Iterator as Iterator>::Item` may not live long enough
+ error[E0310]: the parameter type `T` may not live long enough
3    |
3    |
4 LL |     assert_static(opaque(async move { t; }).next());
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
6    |
-    = help: consider adding an explicit lifetime bound `<impl Iterator as Iterator>::Item: 'static`...
-    = note: ...so that the type `<impl Iterator as Iterator>::Item` will meet its required lifetime bounds
+ help: consider adding an explicit lifetime bound...
+    |
+ LL | fn bad_generic_fn<T: Copy + 'static>(t: T) {
9 
9 
10 error[E0310]: the associated type `<impl Iterator as Iterator>::Item` may not live long enough


16    = help: consider adding an explicit lifetime bound `<impl Iterator as Iterator>::Item: 'static`...
17    = note: ...so that the type `<impl Iterator as Iterator>::Item` will meet its required lifetime bounds
18 
- error[E0310]: the associated type `<impl Iterator as Iterator>::Item` may not live long enough
+ error[E0310]: the parameter type `T` may not live long enough
21    |
21    |
22 LL |     assert_static(opaque(opaque(async move { t; }).next()).next());
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
24    |
-    = help: consider adding an explicit lifetime bound `<impl Iterator as Iterator>::Item: 'static`...
-    = note: ...so that the type `<impl Iterator as Iterator>::Item` will meet its required lifetime bounds
+ help: consider adding an explicit lifetime bound...
+    |
+ LL | fn bad_generic_fn<T: Copy + 'static>(t: T) {
27 
28 error: aborting due to 3 previous errors
29 

---
To only update this specific test, also pass `--test-args regions/closure-in-projection-issue-97405.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/closure-in-projection-issue-97405.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/closure-in-projection-issue-97405" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/closure-in-projection-issue-97405/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0310]: the parameter type `T` may not live long enough
   |
   |
LL |     assert_static(opaque(async move { t; }).next());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn bad_generic_fn<T: Copy + 'static>(t: T) {


error[E0310]: the associated type `<impl Iterator as Iterator>::Item` may not live long enough
   |
   |
LL |     assert_static(opaque(move || { t; }).next());
   |
   |
   = help: consider adding an explicit lifetime bound `<impl Iterator as Iterator>::Item: 'static`...
   = note: ...so that the type `<impl Iterator as Iterator>::Item` will meet its required lifetime bounds
error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/closure-in-projection-issue-97405.rs:28:5
   |
   |
LL |     assert_static(opaque(opaque(async move { t; }).next()).next());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn bad_generic_fn<T: Copy + 'static>(t: T) {

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0310`.
For more information about this error, try `rustc --explain E0310`.
------------------------------------------


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

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | pub async fn query<'a>(_: &(), _: &(), _: (&(dyn std::any::Any + 'a),) ) {}
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#24r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | pub async fn query<'a>(_: &(), _: &(), _: (&(dyn std::any::Any + 'a),) ) {}
   |
   |
   = note: hidden type `impl Future<Output = ()>` captures lifetime '_#25r
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/self/arbitrary_self_types_pin_lifetime-async.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime-async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_pin_lifetime-async/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = Pin<&Foo>>` captures lifetime that does not appear in bounds
   |
   |
LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> Alias<&Self> { self }
   |
   |
   = note: hidden type `impl Future<Output = Pin<&Foo>>` captures lifetime '_#15r

error[E0700]: hidden type for `impl Future<Output = Pin<&Foo>>` captures lifetime that does not appear in bounds
   |
   |
LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> Alias<&Self> { self }
   |
   |
   = note: hidden type `impl Future<Output = Pin<&Foo>>` captures lifetime '_#16r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/self/elision/lt-alias-async.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/lt-alias-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-alias-async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-alias-async/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_self(self, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_self(self, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Alias(self: Alias<'a>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Alias(self: Alias<'a>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Alias(self: Box<Alias<'a>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Alias(self: Box<Alias<'a>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Box_Alias(self: Box<Box<Alias<'a>>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Box_Alias(self: Box<Box<Alias<'a>>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Rc_Alias(self: Rc<Alias<'a>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Rc_Alias(self: Rc<Alias<'a>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Rc_Alias(self: Box<Rc<Alias<'a>>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Rc_Alias(self: Box<Rc<Alias<'a>>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#20r
error: aborting due to 12 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/self/elision/lt-assoc-async.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/lt-assoc-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-assoc-async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-assoc-async/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_self(self, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_self(self, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_AssocType(self: <Struct<'a> as Trait>::AssocType, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_AssocType(self: <Struct<'a> as Trait>::AssocType, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_AssocType(self: Box<<Struct<'a> as Trait>::AssocType>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_AssocType(self: Box<<Struct<'a> as Trait>::AssocType>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
LL |       ) -> &u32 {
   |  _______________^
LL | |         f
LL | |         f
LL | |     }
   | |_____^
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
LL |       ) -> &u32 {
   |  _______________^
LL | |         f
LL | |         f
LL | |     }
   | |_____^
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Rc_AssocType(self: Rc<<Struct<'a> as Trait>::AssocType>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Rc_AssocType(self: Rc<<Struct<'a> as Trait>::AssocType>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
LL |       ) -> &u32 {
   |  _______________^
LL | |         f
LL | |         f
LL | |     }
   | |_____^
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
LL |       ) -> &u32 {
   |  _______________^
LL | |         f
LL | |         f
LL | |     }
   | |_____^
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#20r
error: aborting due to 12 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/self/elision/lt-self-async.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/elision/lt-self-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-self-async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/elision/lt-self-async/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_self(self, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_self(self, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Self(self: Self, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Self(self: Self, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Self(self: Box<Self>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Self(self: Box<Self>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Box_Self(self: Box<Box<Self>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Box_Self(self: Box<Box<Self>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `impl Future<Output = &u32>` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Rc_Self(self: Rc<Self>, f: &u32) -> &u32 {
LL | |         f
