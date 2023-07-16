plain
iiii.....................i..................i........................................... 264/13827
........................................................................................ 352/13827
........................................................................................ 440/13827
........................................................................................ 528/13827
.................................................................F..F.F..F.........F..F. 616/13827
.....F..................F...F............................................F...FF..FF..... 704/13827
.................F.......................F..F....F..F................F.F.FF.FF.FF.F..... 792/13827
..F.F.F..F......i............F.......................................................... 880/13827
........................................................................................ 1056/13827
........................................................................................ 1144/13827
........................................................................................ 1232/13827
........................................................................................ 1320/13827
---
........................................................................................ 4136/13827
...................iii.................................................................. 4224/13827
.........................................................................i.............. 4312/13827
........................................................................................ 4400/13827
F...............................................F....................................... 4488/13827
......................F................................................................. 4576/13827
.......................................FF......F........................................ 4664/13827
........................................................................................ 4840/13827
........................................................................................ 4928/13827
..........................................................F............................. 5016/13827
..........................................................F............................. 5016/13827
.....F..F................................F.............................................. 5104/13827
........................................................................................ 5280/13827
........................................................................................ 5368/13827
...i.....................................................................i.............. 5456/13827
........................................................................................ 5544/13827
---
........................................................................................ 10032/13827
........................................................................................ 10120/13827
...................................................ii...............i...iii............. 10208/13827
........................................................................................ 10296/13827
.....................................................................F..........F....... 10384/13827
........................................................................................ 10560/13827
........................................................................................ 10648/13827
........................................................................................ 10736/13827
........................................................................................ 10824/13827
........................................................................................ 10824/13827
........................................................................................ 10912/13827
...........iiiii...i....i.i............................................................. 11000/13827
........................................................................................ 11088/13827
..........i............................................................................. 11176/13827
....................iiiiii.i..iiiiiiiii.i...........................F................... 11264/13827
.F...FF...F..F..FF..F..............................F.................................... 11352/13827
........................................................................................ 11528/13827
........................................................................................ 11616/13827
........................................................................................ 11704/13827
........................................................................................ 11792/13827
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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/async-assoc-fn-anon-lifetimes.rs:12:39: 14:6]` captures lifetime '_#28r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn assoc(x: &u32, y: B<'_>) {
   |  _______________________________________^
LL | |         async fn nested(x: &u32, y: A<'_, '_>) {}
LL | |     }
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/async-assoc-fn-anon-lifetimes.rs:12:39: 14:6]` captures lifetime '_#29r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL |         async fn nested(x: &u32, y: A<'_, '_>) {}
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/async-assoc-fn-anon-lifetimes.rs:13:48: 13:50]` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL |         async fn nested(x: &u32, y: A<'_, '_>) {}
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/async-assoc-fn-anon-lifetimes.rs:13:48: 13:50]` captures lifetime '_#20r

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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/async-assoc-fn-anon-lifetimes.rs:16:44: 20:6]` captures lifetime '_#33r

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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/async-assoc-fn-anon-lifetimes.rs:16:44: 20:6]` captures lifetime '_#34r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL |             async fn nested_assoc(x: &u32, y: B<'_>) {}
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/async-assoc-fn-anon-lifetimes.rs:18:54: 18:56]` captures lifetime '_#28r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL |             async fn nested_assoc(x: &u32, y: B<'_>) {}
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/async-assoc-fn-anon-lifetimes.rs:18:54: 18:56]` captures lifetime '_#29r
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
---
31    |
32 LL |     async { *x }
-    |     ^^^^^^^^^^^^
+    |           ^^^^^^
34 help: to force the async block to take ownership of `x` (and any other referenced variables), use the `move` keyword
35    |
36 LL |     async move { *x }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-borrowck-escaping-block-error/async-borrowck-escaping-block-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/async-borrowck-escaping-block-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-borrowck-escaping-block-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-borrowck-escaping-block-error" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-borrowck-escaping-block-error/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0373]: async block may outlive the current function, but it borrows `x`, which is owned by the current function
   |
   |
LL |     Box::new(async { x } )
   |                    ^^-^^
   |                    | |
   |                    | `x` is borrowed here
   |                    may outlive borrowed value `x`
note: async block is returned here
  --> /checkout/src/test/ui/async-await/async-borrowck-escaping-block-error.rs:6:5
   |
   |
LL |     Box::new(async { x } )
   |     ^^^^^^^^^^^^^^^^^^^^^^
help: to force the async block to take ownership of `x` (and any other referenced variables), use the `move` keyword
   |
LL |     Box::new(async move { x } )


error[E0373]: async block may outlive the current function, but it borrows `x`, which is owned by the current function
   |
LL |     async { *x }
   |           ^^--^^
   |           | |
   |           | |
   |           | `x` is borrowed here
   |           may outlive borrowed value `x`
note: async block is returned here
  --> /checkout/src/test/ui/async-await/async-borrowck-escaping-block-error.rs:11:11
   |
LL |     async { *x }
LL |     async { *x }
   |           ^^^^^^
help: to force the async block to take ownership of `x` (and any other referenced variables), use the `move` keyword
   |
LL |     async move { *x }

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0373`.
---

31 LL | | }
32    | |_^ expected `u8`, found `()`
33 
- error[E0271]: expected `impl Future<Output = u8>` to be a future that resolves to `()`, but it resolves to `u8`
+ error[E0271]: expected `[static generator@$DIR/async-block-control-flow-static-semantics.rs:23:23: 25:6]` to be a future that resolves to `()`, but it resolves to `u8`
36    |
36    |
37 LL |     let _: &dyn Future<Output = ()> = &block;
38    |                                       ^^^^^^ expected `()`, found `u8`
39    |
39    |
-    = note: required for the cast from `impl Future<Output = u8>` to the object type `dyn Future<Output = ()>`
+    = note: required for the cast from `[static generator@$DIR/async-block-control-flow-static-semantics.rs:23:23: 25:6]` to the object type `dyn Future<Output = ()>`
42 error[E0308]: mismatched types
43   --> $DIR/async-block-control-flow-static-semantics.rs:12:43

47    |    |
47    |    |
48    |    implicitly returns `()` as its body has no tail or `return` expression
49 
- error[E0271]: expected `impl Future<Output = u8>` to be a future that resolves to `()`, but it resolves to `u8`
+ error[E0271]: expected `[static generator@$DIR/async-block-control-flow-static-semantics.rs:14:23: 16:6]` to be a future that resolves to `()`, but it resolves to `u8`
52    |
52    |
53 LL |     let _: &dyn Future<Output = ()> = &block;
54    |                                       ^^^^^^ expected `()`, found `u8`
55    |
55    |
-    = note: required for the cast from `impl Future<Output = u8>` to the object type `dyn Future<Output = ()>`
+    = note: required for the cast from `[static generator@$DIR/async-block-control-flow-static-semantics.rs:14:23: 16:6]` to the object type `dyn Future<Output = ()>`
58 error[E0308]: mismatched types
59   --> $DIR/async-block-control-flow-static-semantics.rs:49:44



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-block-control-flow-static-semantics/async-block-control-flow-static-semantics.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/async-block-control-flow-static-semantics.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-block-control-flow-static-semantics" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-block-control-flow-static-semantics/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0267]: `break` inside of an `async` block
   |
LL |       async {
   |  ___________-
   |  ___________-
LL | |         break 0u8; //~ ERROR `break` inside of an `async` block
   | |         ^^^^^^^^^ cannot `break` inside of an `async` block
LL | |     };
   | |_____- enclosing `async` block

error[E0267]: `break` inside of an `async` block
   |
LL |           async {
   |  _______________-
   |  _______________-
LL | |             break 0u8; //~ ERROR `break` inside of an `async` block
   | |             ^^^^^^^^^ cannot `break` inside of an `async` block
LL | |         };
   | |_________- enclosing `async` block
error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:21:58
   |
   |
LL |   async fn return_targets_async_block_not_async_fn() -> u8 {
   |  __________________________________________________________^
LL | |     //~^ ERROR mismatched types [E0308]
LL | |     let block = async {
LL | |         return 0u8;
...  |
LL | |     //~^ ERROR expected `impl Future<Output = u8>` to be a future that resolves to `()`, but it resolves to `u8`
LL | | }
   | |_^ expected `u8`, found `()`

error[E0271]: expected `[static generator@/checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:23:23: 25:6]` to be a future that resolves to `()`, but it resolves to `u8`
   |
   |
LL |     let _: &dyn Future<Output = ()> = &block;
   |                                       ^^^^^^ expected `()`, found `u8`
   |
   = note: required for the cast from `[static generator@/checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:23:23: 25:6]` to the object type `dyn Future<Output = ()>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:12:43
   |
   |
LL | fn return_targets_async_block_not_fn() -> u8 {
   |    ---------------------------------      ^^ expected `u8`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression

error[E0271]: expected `[static generator@/checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:14:23: 16:6]` to be a future that resolves to `()`, but it resolves to `u8`
   |
   |
LL |     let _: &dyn Future<Output = ()> = &block;
   |                                       ^^^^^^ expected `()`, found `u8`
   |
   = note: required for the cast from `[static generator@/checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:14:23: 16:6]` to the object type `dyn Future<Output = ()>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:49:44
   |
   |
LL | fn rethrow_targets_async_block_not_fn() -> Result<u8, MyErr> {
   |    |
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
   |
   = note:   expected enum `Result<u8, MyErr>`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/async-block-control-flow-static-semantics.rs:58:50
   |
   |
LL | fn rethrow_targets_async_block_not_async_fn() -> Result<u8, MyErr> {
   |    |
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
   |
   = note:   expected enum `Result<u8, MyErr>`

error: aborting due to 8 previous errors

Some errors have detailed explanations: E0267, E0271, E0308.
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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/async-await.rs:106:56: 109:2]` captures lifetime '_#14r

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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/async-await.rs:106:56: 109:2]` captures lifetime '_#15r
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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/async-await.rs:106:56: 109:2]` captures lifetime '_#14r

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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/async-await.rs:106:56: 109:2]` captures lifetime '_#15r
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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/async-await.rs:106:56: 109:2]` captures lifetime '_#14r

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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/async-await.rs:106:56: 109:2]` captures lifetime '_#15r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/generator-desc.rs stdout ----
diff of stderr:

2   --> $DIR/generator-desc.rs:10:25
3    |
4 LL |     fun(async {}, async {});
-    |               |         |
-    |               |         |
-    |               |         expected `async` block, found a different `async` block
-    |               |         arguments to this function are incorrect
-    |               the expected `async` block
+    |     ---       --        ^^ expected `async` block, found a different `async` block
+    |     |         |
+    |     |         the expected `async` block
10    |
10    |
11    = note: expected `async` block `[static generator@$DIR/generator-desc.rs:10:15: 10:17]`
12               found `async` block `[static generator@$DIR/generator-desc.rs:10:25: 10:27]`
13 note: function defined here
-   --> $SRC_DIR/core/src/future/mod.rs:LL:COL
+   --> $DIR/generator-desc.rs:8:4
15    |
15    |
- LL | pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
-    |              ^^^^^^^^^^^^^^
+ LL | fn fun<F: Future<Output = ()>>(f1: F, f2: F) {}
18 
19 error[E0308]: mismatched types
20   --> $DIR/generator-desc.rs:12:16


53    |     |             the expected `async` closure body
55    |
-   ::: $SRC_DIR/core/src/future/mod.rs:LL:COL
-    |
-    |
- LL | pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
-    |                                           |
-    |                                           the expected opaque type
-    |                                           the found opaque type
-    |
-    |
-    = note: expected opaque type `impl Future<Output = ()>` (`async` closure body)
-               found opaque type `impl Future<Output = ()>` (`async` closure body)
+    = note: expected `async` closure body `[static generator@$DIR/generator-desc.rs:14:19: 14:21]`
+               found `async` closure body `[static generator@$DIR/generator-desc.rs:14:36: 14:38]`
67   --> $DIR/generator-desc.rs:8:4
68    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generator-desc/generator-desc.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/generator-desc.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/generator-desc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generator-desc" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generator-desc/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/async-await/generator-desc.rs:10:25
   |
   |
LL |     fun(async {}, async {});
   |     ---       --        ^^ expected `async` block, found a different `async` block
   |     |         |
   |     |         the expected `async` block
   |
   |
   = note: expected `async` block `[static generator@/checkout/src/test/ui/async-await/generator-desc.rs:10:15: 10:17]`
              found `async` block `[static generator@/checkout/src/test/ui/async-await/generator-desc.rs:10:25: 10:27]`
  --> /checkout/src/test/ui/async-await/generator-desc.rs:8:4
   |
   |
LL | fn fun<F: Future<Output = ()>>(f1: F, f2: F) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/generator-desc.rs:12:16
   |
   |
LL |     fun(one(), two());
   |     |
   |     arguments to this function are incorrect
   |
note: while checking the return type of the `async fn`
note: while checking the return type of the `async fn`
  --> /checkout/src/test/ui/async-await/generator-desc.rs:5:16
   |
LL | async fn one() {}
   |                ^ checked the `Output` of this `async fn`, expected opaque type
note: while checking the return type of the `async fn`
   |
   |
LL | async fn two() {}
   |                ^ checked the `Output` of this `async fn`, found opaque type
   = note: expected opaque type `impl Future<Output = ()>` (opaque type at </checkout/src/test/ui/async-await/generator-desc.rs:5:16>)
              found opaque type `impl Future<Output = ()>` (opaque type at </checkout/src/test/ui/async-await/generator-desc.rs:6:16>)
   = help: consider `await`ing on both `Future`s
   = note: distinct uses of `impl Trait` result in different opaque types
  --> /checkout/src/test/ui/async-await/generator-desc.rs:8:4
   |
   |
LL | fn fun<F: Future<Output = ()>>(f1: F, f2: F) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/generator-desc.rs:14:26
   |
   |
LL |     fun((async || {})(), (async || {})());
   |     ---           --     ^^^^^^^^^^^^^^^ expected `async` closure body, found a different `async` closure body
   |     |             |
   |     |             the expected `async` closure body
   |
   |
   = note: expected `async` closure body `[static generator@/checkout/src/test/ui/async-await/generator-desc.rs:14:19: 14:21]`
              found `async` closure body `[static generator@/checkout/src/test/ui/async-await/generator-desc.rs:14:36: 14:38]`
  --> /checkout/src/test/ui/async-await/generator-desc.rs:8:4
   |
   |
LL | fn fun<F: Future<Output = ()>>(f1: F, f2: F) {}

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
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
+    = note: hidden type `[static generator@$DIR/async-associated-types.rs:19:58: 21:6]` captures lifetime '_#24r
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
+    = note: hidden type `[static generator@$DIR/async-associated-types.rs:19:58: 21:6]` captures lifetime '_#25r
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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/in-trait/async-associated-types.rs:19:58: 21:6]` captures lifetime '_#24r

error[E0700]: hidden type for `impl Future<Output = (&'a U, &'b T)>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn foo(&'a self, key: &'b T) -> (&'a U, &'b T) {
LL | |         (self, key)
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/in-trait/async-associated-types.rs:19:58: 21:6]` captures lifetime '_#25r
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0495, E0700.
For more information about an error, try `rustc --explain E0495`.
---
1 error: future cannot be sent between threads safely
-   --> $DIR/issue-67252-unnamed-future.rs:18:11
+   --> $DIR/issue-67252-unnamed-future.rs:18:17
3    |
4 LL |       spawn(async {
+    |  _________________^
+    |  _________________^
6 LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
7 LL | |         AFuture.await;
8 LL | |     });

9    | |_____^ future created by async block is not `Send`
10    |
-    = help: within `impl Future<Output = ()>`, the trait `Send` is not implemented for `*mut ()`
+    = help: within `[static generator@$DIR/issue-67252-unnamed-future.rs:18:17: 21:6]`, the trait `Send` is not implemented for `*mut ()`
12 note: future is not `Send` as this value is used across an await
14    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-67252-unnamed-future/issue-67252-unnamed-future.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issue-67252-unnamed-future.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-67252-unnamed-future" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-67252-unnamed-future/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: future cannot be sent between threads safely
   |
   |
LL |       spawn(async { //~ ERROR future cannot be sent between threads safely
   |  _________________^
LL | |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
LL | |         AFuture.await;
LL | |     });
   | |_____^ future created by async block is not `Send`
   |
   = help: within `[static generator@/checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs:18:17: 21:6]`, the trait `Send` is not implemented for `*mut ()`
note: future is not `Send` as this value is used across an await
   |
   |
LL |         let _a = std::ptr::null_mut::<()>(); // `*mut ()` is not `Send`
   |             -- has type `*mut ()` which is not `Send`
LL |         AFuture.await;
   |                ^^^^^^ await occurs here, with `_a` maybe used later
LL |     });
   |     - `_a` is later dropped here
note: required by a bound in `spawn`
   |
   |
LL | fn spawn<T: Send>(_: T) {}
   |             ^^^^ required by this bound in `spawn`
error: aborting due to previous error
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
63    |


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
   |  __________________________^
   |  __________________________^
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

1 error: captured variable cannot escape `FnMut` closure body
-   --> $DIR/issue-69446-fnmut-capture.rs:19:17
3    |
4 LL |       let mut x = Foo;
5    |           ----- variable defined here


6 LL |       bar(move || async {
-    |  _______________-_^
+    |  _______________-_______^
8    | |               |
9    | |               inferred to be a `FnMut` closure
10 LL | |         x.foo();
11    | |         - variable captured here
12 LL | |     });
12 LL | |     });
-    | |_____^ returns an `async` block that contains a reference to a captured variable, which then escapes the closure body
+    | |_____^ returns a reference to a captured variable which escapes the closure body
14    |
15    = note: `FnMut` closures only have access to their captured variables while they are executing...
16    = note: ...therefore, they cannot allow references to captured variables to escape
+    = note: requirement occurs because of a mutable reference to `Context<'_>`
+    = note: requirement occurs because of a mutable reference to `Context<'_>`
+    = note: mutable references are invariant over their type parameter
+    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
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
   |  _______________-_______^
   | |               |
   | |               inferred to be a `FnMut` closure
LL | |         x.foo();
LL | |     });
LL | |     });
   | |_____^ returns a reference to a captured variable which escapes the closure body
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
24    |


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
LL |       async move {
   |  ________________^
   |  ________________^
LL | |         baz(|| async{
LL | |             foo(tx.clone());
LL | |         }).await;
LL | |     }

error: aborting due to previous error

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
-    = note: required because it captures the following types: `ResumeTy`, `impl Future<Output = Arc<RefCell<i32>>>`, `()`, `i32`, `Ready<i32>`
+    = note: required because it captures the following types: `&mut Context<'_>`, `impl Future<Output = Arc<RefCell<i32>>>`, `()`, `i32`, `Ready<i32>`
61 note: required because it's used within this `async` block
63    |


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
   |  __________________________^
   |  __________________________^
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


---- [ui] src/test/ui/async-await/issue-86507.rs stdout ----
diff of stderr:

13    |
14 LL |                     let x = x;
15    |                             ^ has type `&T` which is not `Send`, because `T` is not `Sync`
-    = note: required for the cast from `impl Future<Output = ()>` to the object type `dyn Future<Output = ()> + Send`
+    = note: required for the cast from `[static generator@$DIR/issue-86507.rs:18:28: 20:18]` to the object type `dyn Future<Output = ()> + Send`
18    |
18    |
19 LL |     fn bar<'me, 'async_trait, T: Send + std::marker::Sync>(x: &'me T)

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-86507/issue-86507.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issue-86507.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-86507.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-86507" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-86507/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: future cannot be sent between threads safely
   |
   |
LL | /             Box::pin( //~ ERROR future cannot be sent between threads safely
LL | |                 async move {
LL | |                     let x = x;
LL | |                 }
LL | |             )
   | |_____________^ future created by async block is not `Send`
   |
note: captured value is not `Send` because `&` references cannot be sent unless their referent is `Sync`
   |
LL |                     let x = x;
LL |                     let x = x;
   |                             ^ has type `&T` which is not `Send`, because `T` is not `Sync`
   = note: required for the cast from `[static generator@/checkout/src/test/ui/async-await/issue-86507.rs:18:28: 20:18]` to the object type `dyn Future<Output = ()> + Send`
   |
   |
LL |     fn bar<'me, 'async_trait, T: Send + std::marker::Sync>(x: &'me T)

error: aborting due to previous error
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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/issues/issue-63388-3.rs:13:7: 14:6]` captures lifetime '_#16r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
LL |       ) {
   |  _______^
LL | |     }
LL | |     }
   | |_____^
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/issues/issue-63388-3.rs:13:7: 14:6]` captures lifetime '_#17r
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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/issues/issue-63388-4.rs:7:40: 7:48]` captures lifetime '_#15r

error[E0700]: hidden type for `impl Future<Output = &A>` captures lifetime that does not appear in bounds
   |
   |
LL |     async fn foo(&self, f: &u32) -> &A { self }
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/issues/issue-63388-4.rs:7:40: 7:48]` captures lifetime '_#16r
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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/issues/issue-64433.rs:17:74: 20:6]` captures lifetime '_#14r

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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/issues/issue-64433.rs:17:74: 20:6]` captures lifetime '_#15r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
---
3    |
4 LL |       assert_send(async {
-    |  _________________^
+    |  _______________________^
6 LL | |
7 LL | |         bar(Foo(std::ptr::null())).await;
8 LL | |     })

9    | |_____^ future created by async block is not `Send`
10    |
-    = help: within `impl Future<Output = ()>`, the trait `Send` is not implemented for `*const u8`
+    = help: within `[static generator@$DIR/issue-65436-raw-ptr-not-send.rs:16:23: 19:6]`, the trait `Send` is not implemented for `*const u8`
12 note: future is not `Send` as this value is used across an await
14    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.no_drop_tracking/issue-65436-raw-ptr-not-send.no_drop_tracking.stderr
To only update this specific test, also pass `--test-args async-await/issues/issue-65436-raw-ptr-not-send.rs`

error in revision `no_drop_tracking`: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "no_drop_tracking" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.no_drop_tracking" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.no_drop_tracking/auxiliary" "--edition=2018" "-Zdrop-tracking=no"
stdout: none
--- stderr -------------------------------
error: future cannot be sent between threads safely
   |
LL |       assert_send(async {
   |  _______________________^
   |  _______________________^
LL | |         //[no_drop_tracking]~^ ERROR future cannot be sent between threads safely
LL | |         bar(Foo(std::ptr::null())).await;
LL | |     })
   | |_____^ future created by async block is not `Send`
   |
   = help: within `[static generator@/checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:16:23: 19:6]`, the trait `Send` is not implemented for `*const u8`
note: future is not `Send` as this value is used across an await
   |
   |
LL |         bar(Foo(std::ptr::null())).await;
   |                 ----------------  ^^^^^^ await occurs here, with `std::ptr::null()` maybe used later
   |                 |
   |                 has type `*const u8` which is not `Send`
note: `std::ptr::null()` is later dropped here
   |
   |
LL |         bar(Foo(std::ptr::null())).await;
help: consider moving this into a `let` binding to create a shorter lived borrow
  --> /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:18:13
   |
   |
LL |         bar(Foo(std::ptr::null())).await;
note: required by a bound in `assert_send`
  --> /checkout/src/test/ui/async-await/issues/issue-65436-raw-ptr-not-send.rs:13:19
   |
   |
LL | fn assert_send<T: Send>(_: T) {}
   |                   ^^^^ required by this bound in `assert_send`
error: aborting due to previous error
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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/multiple-lifetimes/elided.rs:6:52: 6:54]` captures lifetime '_#14r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/multiple-lifetimes/elided.rs:6:52: 6:54]` captures lifetime '_#15r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/multiple-lifetimes/fn-ptr.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/multiple-lifetimes/fn-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/fn-ptr/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/fn-ptr/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn multiple_named_lifetimes<'a, 'b>(_: &'a u8, _: &'b u8, _: fn(&u8)) {}
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/multiple-lifetimes/fn-ptr.rs:6:77: 6:79]` captures lifetime '_#14r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn multiple_named_lifetimes<'a, 'b>(_: &'a u8, _: &'b u8, _: fn(&u8)) {}
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/multiple-lifetimes/fn-ptr.rs:6:77: 6:79]` captures lifetime '_#15r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/multiple-lifetimes/named.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/multiple-lifetimes/named.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/named/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/named/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn multiple_named_lifetimes<'a, 'b>(_: &'a u8, _: &'b u8) {}
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/multiple-lifetimes/named.rs:6:65: 6:67]` captures lifetime '_#14r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn multiple_named_lifetimes<'a, 'b>(_: &'a u8, _: &'b u8) {}
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/multiple-lifetimes/named.rs:6:65: 6:67]` captures lifetime '_#15r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/multiple-lifetimes/partial-relation.rs:6:1: 9:2]` captures lifetime '_#29r

error[E0700]: hidden type for `impl Future<Output = (&'a u32, &'b u32)>` captures lifetime that does not appear in bounds
   |
LL | / {
LL | / {
LL | |     drop((a, c));
LL | |     (b, b)
LL | | }
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/multiple-lifetimes/partial-relation.rs:6:1: 9:2]` captures lifetime '_#30r
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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-fg.rs:11:83: 13:2]` captures lifetime '_#16r

error[E0700]: hidden type for `impl Future<Output = impl Trait<'a, 'b>>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn async_ret_impl_trait<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a, 'b> {
LL | |     (a, b)
LL | | }
   | |_^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-fg.rs:11:83: 13:2]` captures lifetime '_#17r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.rs stdout ----
diff of stderr:

13    |
14    = help: consider adding the following bound: `'a: 'b`
15 
+ error[E0700]: hidden type for `impl Future<Output = impl Trait<'a>>` captures lifetime that does not appear in bounds
+    |
+    |
+ LL |   async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> {
+ LL | |
+ LL | |     (a, b)
+ LL | | }
+    | |_^
+    | |_^
+    |
+    = note: hidden type `[static generator@$DIR/ret-impl-trait-one.rs:16:80: 19:2]` captures lifetime '_#16r
+ 
+ error[E0700]: hidden type for `impl Future<Output = impl Trait<'a>>` captures lifetime that does not appear in bounds
+    |
+    |
+ LL |   async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> {
+ LL | |
+ LL | |     (a, b)
+ LL | | }
+    | |_^
+    | |_^
+    |
+    = note: hidden type `[static generator@$DIR/ret-impl-trait-one.rs:16:80: 19:2]` captures lifetime '_#17r
+ 
16 error[E0700]: hidden type for `impl Trait<'a>` captures lifetime that does not appear in bounds
18    |


30 LL | async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> + 'b {
32 
- error: aborting due to 2 previous errors
+ error: aborting due to 4 previous errors
34 
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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.rs:16:80: 19:2]` captures lifetime '_#16r

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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.rs:16:80: 19:2]` captures lifetime '_#17r

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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/multiple-lifetimes/variance.rs:7:97: 7:100]` captures lifetime '_#21r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn lotsa_lifetimes<'a, 'b, 'c>(_: fn(&'a u8), _: fn(&'b u8) -> &'b u8, _: fn() -> &'c u8) { }
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/multiple-lifetimes/variance.rs:7:97: 7:100]` captures lifetime '_#22r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/no-const-async.rs stdout ----
diff of stderr:

7    |     |     `async` because of this
8    |     `const` because of this
9 
- error[E0391]: cycle detected when computing type of `x::{opaque#0}`
-   --> $DIR/no-const-async.rs:4:24
-    |
- LL | pub const async fn x() {}
-    |
-    |
- note: ...which requires borrow-checking `x`...
-   --> $DIR/no-const-async.rs:4:1
-    |
- LL | pub const async fn x() {}
-    | ^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing MIR for `x`...
-   --> $DIR/no-const-async.rs:4:1
-    |
- LL | pub const async fn x() {}
-    | ^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires const checking `x`...
-   --> $DIR/no-const-async.rs:4:1
-    |
- LL | pub const async fn x() {}
-    | ^^^^^^^^^^^^^^^^^^^^^^
-    = note: ...which requires computing whether `impl core::future::future::Future<Output = ()>` is freeze...
-    = note: ...which requires evaluating trait selection obligation `impl core::future::future::Future<Output = ()>: core::marker::Freeze`...
-    = note: ...which again requires computing type of `x::{opaque#0}`, completing the cycle
- note: cycle used when checking item types in top-level module
-   --> $DIR/no-const-async.rs:4:1
-    |
- LL | pub const async fn x() {}
+ error: aborting due to previous error
39 
- error: aborting due to 2 previous errors
- 
---
To only update this specific test, also pass `--test-args async-await/no-const-async.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/no-const-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-const-async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/no-const-async/auxiliary" "--edition=2018" "--crate-type" "lib"
stdout: none
--- stderr -------------------------------
error: functions cannot be both `const` and `async`
   |
   |
LL | pub const async fn x() {}
   | ----^^^^^-^^^^^----------
   |     |     |
   |     |     `async` because of this
   |     `const` because of this
error: aborting due to previous error
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
+    = note: hidden type `[static generator@$DIR/ret-ref.rs:7:75: 9:2]` captures lifetime '_#15r
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
+    = note: hidden type `[static generator@$DIR/ret-ref.rs:7:75: 9:2]` captures lifetime '_#16r
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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/multiple-lifetimes/ret-ref.rs:7:75: 9:2]` captures lifetime '_#15r

error[E0700]: hidden type for `impl Future<Output = &'a u8>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn multiple_named_lifetimes<'a, 'b>(a: &'a u8, _: &'b u8) -> &'a u8 {
LL | |     a
LL | | }
   | |_^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/multiple-lifetimes/ret-ref.rs:7:75: 9:2]` captures lifetime '_#16r

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
+   --> $DIR/large_moves.rs:18:14
3    |
- LL |       let x = async {
-    |  _____________^
- LL | |         let y = [0; 9999];
- LL | |         dbg!(y);
- LL | |         thing(&y).await;
- LL | |         dbg!(y);
- LL | |     };
-    | |_____^ value moved from here
+ LL |     let z = (x, 42);
+    |              ^ value moved from here
12    |
13    = note: The current maximum size is 1000, but it can be customized with the move_size_limit attribute: `#![move_size_limit = "..."]`

18    |         ^^^^^^^^^^^^^^^^^
19 
20 error: moving 10024 bytes
20 error: moving 10024 bytes
-   --> $DIR/large_moves.rs:18:14
-    |
- LL |     let z = (x, 42);
-    |              ^ value moved from here
-    |
-    = note: The current maximum size is 1000, but it can be customized with the move_size_limit attribute: `#![move_size_limit = "..."]`
- error: moving 10024 bytes
29   --> $DIR/large_moves.rs:18:13
30    |
30    |
31 LL |     let z = (x, 42);
41    |
41    |
42    = note: The current maximum size is 1000, but it can be customized with the move_size_limit attribute: `#![move_size_limit = "..."]`
- error: aborting due to 4 previous errors
+ error: aborting due to 3 previous errors
45 
46 
46 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves.attribute/large_moves.attribute.stderr
To only update this specific test, also pass `--test-args async-await/large_moves.rs`


error in revision `attribute`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/large_moves.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "attribute" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves.attribute" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves.attribute/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: moving 10024 bytes
   |
   |
LL |     let z = (x, 42); //~ ERROR large_assignments
   |              ^ value moved from here
   |
   = note: The current maximum size is 1000, but it can be customized with the move_size_limit attribute: `#![move_size_limit = "..."]`
  --> /checkout/src/test/ui/async-await/large_moves.rs:1:9
   |
LL | #![deny(large_assignments)]
   |         ^^^^^^^^^^^^^^^^^
   |         ^^^^^^^^^^^^^^^^^

error: moving 10024 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:18:13
   |
LL |     let z = (x, 42); //~ ERROR large_assignments
   |
   |
   = note: The current maximum size is 1000, but it can be customized with the move_size_limit attribute: `#![move_size_limit = "..."]`
error: moving 10024 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:20:13
   |
   |
LL |     let a = z.0; //~ ERROR large_assignments
   |
   |
   = note: The current maximum size is 1000, but it can be customized with the move_size_limit attribute: `#![move_size_limit = "..."]`
error: aborting due to 3 previous errors
------------------------------------------



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
---
+   --> $DIR/large_moves.rs:18:14
3    |
- LL |       let x = async {
-    |  _____________^
- LL | |         let y = [0; 9999];
- LL | |         dbg!(y);
- LL | |         thing(&y).await;
- LL | |         dbg!(y);
- LL | |     };
-    | |_____^ value moved from here
+ LL |     let z = (x, 42);
+    |              ^ value moved from here
12    |
13    = note: The current maximum size is 1000, but it can be customized with the move_size_limit attribute: `#![move_size_limit = "..."]`

18    |         ^^^^^^^^^^^^^^^^^
19 
20 error: moving 10024 bytes
20 error: moving 10024 bytes
-   --> $DIR/large_moves.rs:18:14
-    |
- LL |     let z = (x, 42);
-    |              ^ value moved from here
-    |
-    = note: The current maximum size is 1000, but it can be customized with the move_size_limit attribute: `#![move_size_limit = "..."]`
- error: moving 10024 bytes
29   --> $DIR/large_moves.rs:18:13
30    |
30    |
31 LL |     let z = (x, 42);
41    |
41    |
42    = note: The current maximum size is 1000, but it can be customized with the move_size_limit attribute: `#![move_size_limit = "..."]`
- error: aborting due to 4 previous errors
+ error: aborting due to 3 previous errors
45 
46 
---
To only update this specific test, also pass `--test-args async-await/large_moves.rs`

error in revision `option`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/large_moves.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "option" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves.option" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves.option/auxiliary" "-Zmove-size-limit=1000" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: moving 10024 bytes
   |
   |
LL |     let z = (x, 42); //~ ERROR large_assignments
   |              ^ value moved from here
   |
   = note: The current maximum size is 1000, but it can be customized with the move_size_limit attribute: `#![move_size_limit = "..."]`
  --> /checkout/src/test/ui/async-await/large_moves.rs:1:9
   |
LL | #![deny(large_assignments)]
   |         ^^^^^^^^^^^^^^^^^
   |         ^^^^^^^^^^^^^^^^^

error: moving 10024 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:18:13
   |
LL |     let z = (x, 42); //~ ERROR large_assignments
   |
   |
   = note: The current maximum size is 1000, but it can be customized with the move_size_limit attribute: `#![move_size_limit = "..."]`
error: moving 10024 bytes
  --> /checkout/src/test/ui/async-await/large_moves.rs:20:13
   |
   |
LL |     let a = z.0; //~ ERROR large_assignments
   |
   |
   = note: The current maximum size is 1000, but it can be customized with the move_size_limit attribute: `#![move_size_limit = "..."]`
error: aborting due to 3 previous errors
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
+    = note: hidden type `[static generator@$DIR/unused-lifetime.rs:10:46: 10:48]` captures lifetime '_#16r
48 
+ error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
+   --> $DIR/unused-lifetime.rs:10:46
+    |
+ LL | async fn async_wrong_1_lifetime<'a>(_: &i32) {}
+    |
+    |
+    = note: hidden type `[static generator@$DIR/unused-lifetime.rs:10:46: 10:48]` captures lifetime '_#17r
+ 
+ error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
+   --> $DIR/unused-lifetime.rs:12:63
+    |
+ LL | async fn async_wrong_2_lifetimes<'a, 'b>(_: &'a i32, _: &i32) {}
+    |
+    |
+    = note: hidden type `[static generator@$DIR/unused-lifetime.rs:12:63: 12:65]` captures lifetime '_#21r
+ 
+ error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
+   --> $DIR/unused-lifetime.rs:12:63
+    |
+ LL | async fn async_wrong_2_lifetimes<'a, 'b>(_: &'a i32, _: &i32) {}
+    |
+    |
+    = note: hidden type `[static generator@$DIR/unused-lifetime.rs:12:63: 12:65]` captures lifetime '_#22r
+ 
+ error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
+   --> $DIR/unused-lifetime.rs:16:66
+    |
+ LL | async fn async_right_2_lifetimes<'a, 'b>(_: &'a i32, _: &'b i32) {}
+    |
+    |
+    = note: hidden type `[static generator@$DIR/unused-lifetime.rs:16:66: 16:68]` captures lifetime '_#14r
+ 
+ error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
+   --> $DIR/unused-lifetime.rs:16:66
+    |
+ LL | async fn async_right_2_lifetimes<'a, 'b>(_: &'a i32, _: &'b i32) {}
+    |
+    |
+    = note: hidden type `[static generator@$DIR/unused-lifetime.rs:16:66: 16:68]` captures lifetime '_#15r
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
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/unused-lifetime.rs:10:46: 10:48]` captures lifetime '_#16r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn async_wrong_1_lifetime<'a>(_: &i32) {} //~ ERROR lifetime parameter `'a` never used
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/unused-lifetime.rs:10:46: 10:48]` captures lifetime '_#17r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn async_wrong_2_lifetimes<'a, 'b>(_: &'a i32, _: &i32) {} //~ ERROR lifetime parameter `'b` never used
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/unused-lifetime.rs:12:63: 12:65]` captures lifetime '_#21r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn async_wrong_2_lifetimes<'a, 'b>(_: &'a i32, _: &i32) {} //~ ERROR lifetime parameter `'b` never used
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/unused-lifetime.rs:12:63: 12:65]` captures lifetime '_#22r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn async_right_2_lifetimes<'a, 'b>(_: &'a i32, _: &'b i32) {}
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/unused-lifetime.rs:16:66: 16:68]` captures lifetime '_#14r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | async fn async_right_2_lifetimes<'a, 'b>(_: &'a i32, _: &'b i32) {}
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/async-await/unused-lifetime.rs:16:66: 16:68]` captures lifetime '_#15r
error: aborting due to 12 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/chalkify/bugs/async.rs stdout ----
diff of stderr:

- error[E0277]: the trait bound `[static generator@$DIR/async.rs:7:29: 9:2]: Generator<ResumeTy>` is not satisfied
-   --> $DIR/async.rs:7:29
+ error[E0277]: `[static generator@$DIR/async.rs:7:29: 9:2]` is not a future
3    |
3    |
- LL |   async fn foo(x: u32) -> u32 {
- LL | |     x
- LL | | }
- LL | | }
-    | |_^ the trait `Generator<ResumeTy>` is not implemented for `[static generator@$DIR/async.rs:7:29: 9:2]`
+ LL | async fn foo(x: u32) -> u32 {
+    |                         ^^^ `[static generator@$DIR/async.rs:7:29: 9:2]` is not a future
- note: required by a bound in `std::future::from_generator`
-   --> $SRC_DIR/core/src/future/mod.rs:LL:COL
-    |
-    |
- LL |     T: Generator<ResumeTy, Yield = ()>,
+    = help: the trait `Future` is not implemented for `[static generator@$DIR/async.rs:7:29: 9:2]`
+    = help: the trait `Future` is not implemented for `[static generator@$DIR/async.rs:7:29: 9:2]`
+    = note: [static generator@$DIR/async.rs:7:29: 9:2] must be a future or must implement `IntoFuture` to be awaited
15 
- error[E0280]: the requirement `<[static generator@$DIR/async.rs:7:29: 9:2] as Generator<ResumeTy>>::Yield == ()` is not satisfied
-   --> $DIR/async.rs:7:29
-    |
- LL |   async fn foo(x: u32) -> u32 {
- LL | |     x
- LL | | }
-    | |_^
-    |
-    |
- note: required by a bound in `std::future::from_generator`
-   --> $SRC_DIR/core/src/future/mod.rs:LL:COL
-    |
- LL |     T: Generator<ResumeTy, Yield = ()>,
- 
- 
- error[E0280]: the requirement `<impl Future<Output = u32> as Future>::Output == u32` is not satisfied
+ error[E0280]: the requirement `<[static generator@$DIR/async.rs:7:29: 9:2] as Future>::Output == u32` is not satisfied
33    |
33    |
34 LL | async fn foo(x: u32) -> u32 {
35    |                         ^^^
36 
- error: aborting due to 3 previous errors
+ error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args chalkify/bugs/async.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/chalkify/bugs/async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/bugs/async/auxiliary" "-Z" "chalk" "--edition=2021"
stdout: none
--- stderr -------------------------------
error[E0277]: `[static generator@/checkout/src/test/ui/chalkify/bugs/async.rs:7:29: 9:2]` is not a future
   |
   |
LL | async fn foo(x: u32) -> u32 {
   |                         ^^^ `[static generator@/checkout/src/test/ui/chalkify/bugs/async.rs:7:29: 9:2]` is not a future
   |
   = help: the trait `Future` is not implemented for `[static generator@/checkout/src/test/ui/chalkify/bugs/async.rs:7:29: 9:2]`
   = note: [static generator@/checkout/src/test/ui/chalkify/bugs/async.rs:7:29: 9:2] must be a future or must implement `IntoFuture` to be awaited

error[E0280]: the requirement `<[static generator@/checkout/src/test/ui/chalkify/bugs/async.rs:7:29: 9:2] as Future>::Output == u32` is not satisfied
   |
   |
LL | async fn foo(x: u32) -> u32 {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
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
   = note: hidden type `[static generator@/checkout/src/test/ui/closures/2229_closure_analysis/run_pass/issue-88431.rs:19:34: 21:2]` captures lifetime '_#21r

error[E0700]: hidden type for `impl Future<Output = Box<(dyn GameState<'a> + 'a)>>` captures lifetime that does not appear in bounds
   |
   |
LL |   ) -> Box<dyn GameState<'a> + 'a> {
LL | |     unimplemented!()
LL | | }
   | |_^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/closures/2229_closure_analysis/run_pass/issue-88431.rs:19:34: 21:2]` captures lifetime '_#22r
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



---- [ui] src/test/ui/generator/clone-impl-async.rs stdout ----
diff of stderr:

- error[E0277]: the trait bound `impl Future<Output = ()>: Copy` is not satisfied
+ error[E0277]: the trait bound `[static generator@$DIR/clone-impl-async.rs:12:33: 16:6]: Copy` is not satisfied
3    |
3    |
4 LL |     check_copy(&inner_non_clone);
-    |     ---------- ^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `impl Future<Output = ()>`
+    |     ---------- ^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `[static generator@$DIR/clone-impl-async.rs:12:33: 16:6]`
6    |     |
7    |     required by a bound introduced by this call
7    |     required by a bound introduced by this call
8    |

12 LL | fn check_copy<T: Copy>(_x: &T) {}
13    |                  ^^^^ required by this bound in `check_copy`
14 
- error[E0277]: the trait bound `impl Future<Output = ()>: Clone` is not satisfied
+ error[E0277]: the trait bound `[static generator@$DIR/clone-impl-async.rs:12:33: 16:6]: Clone` is not satisfied
17    |
17    |
18 LL |     check_clone(&inner_non_clone);
-    |     ----------- ^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `impl Future<Output = ()>`
+    |     ----------- ^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `[static generator@$DIR/clone-impl-async.rs:12:33: 16:6]`
20    |     |
21    |     required by a bound introduced by this call
21    |     required by a bound introduced by this call
22    |

26 LL | fn check_clone<T: Clone>(_x: &T) {}
27    |                   ^^^^^ required by this bound in `check_clone`
28 
- error[E0277]: the trait bound `impl Future<Output = ()>: Copy` is not satisfied
+ error[E0277]: the trait bound `[static generator@$DIR/clone-impl-async.rs:23:38: 25:6]: Copy` is not satisfied
31    |
31    |
32 LL |     check_copy(&outer_non_clone);
-    |     ---------- ^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `impl Future<Output = ()>`
+    |     ---------- ^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `[static generator@$DIR/clone-impl-async.rs:23:38: 25:6]`
34    |     |
35    |     required by a bound introduced by this call
35    |     required by a bound introduced by this call
36    |

40 LL | fn check_copy<T: Copy>(_x: &T) {}
41    |                  ^^^^ required by this bound in `check_copy`
42 
- error[E0277]: the trait bound `impl Future<Output = ()>: Clone` is not satisfied
+ error[E0277]: the trait bound `[static generator@$DIR/clone-impl-async.rs:23:38: 25:6]: Clone` is not satisfied
45    |
45    |
46 LL |     check_clone(&outer_non_clone);
-    |     ----------- ^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `impl Future<Output = ()>`
+    |     ----------- ^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `[static generator@$DIR/clone-impl-async.rs:23:38: 25:6]`
48    |     |
49    |     required by a bound introduced by this call
49    |     required by a bound introduced by this call
50    |

54 LL | fn check_clone<T: Clone>(_x: &T) {}
55    |                   ^^^^^ required by this bound in `check_clone`
56 
- error[E0277]: the trait bound `impl Future<Output = ()>: Copy` is not satisfied
+ error[E0277]: the trait bound `[static generator@$DIR/clone-impl-async.rs:31:39: 31:41]: Copy` is not satisfied
59    |
59    |
60 LL |     check_copy(&maybe_copy_clone);
-    |     ---------- ^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `impl Future<Output = ()>`
+    |     ---------- ^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `[static generator@$DIR/clone-impl-async.rs:31:39: 31:41]`
62    |     |
63    |     required by a bound introduced by this call
63    |     required by a bound introduced by this call
64    |

68 LL | fn check_copy<T: Copy>(_x: &T) {}
69    |                  ^^^^ required by this bound in `check_copy`
70 
- error[E0277]: the trait bound `impl Future<Output = ()>: Clone` is not satisfied
+ error[E0277]: the trait bound `[static generator@$DIR/clone-impl-async.rs:31:39: 31:41]: Clone` is not satisfied
73    |
73    |
74 LL |     check_clone(&maybe_copy_clone);
-    |     ----------- ^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `impl Future<Output = ()>`
+    |     ----------- ^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `[static generator@$DIR/clone-impl-async.rs:31:39: 31:41]`
76    |     |
77    |     required by a bound introduced by this call
---
To only update this specific test, also pass `--test-args generator/clone-impl-async.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/clone-impl-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/clone-impl-async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/clone-impl-async/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `[static generator@/checkout/src/test/ui/generator/clone-impl-async.rs:12:33: 16:6]: Copy` is not satisfied
   |
   |
LL |     check_copy(&inner_non_clone);
   |     ---------- ^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `[static generator@/checkout/src/test/ui/generator/clone-impl-async.rs:12:33: 16:6]`
   |     required by a bound introduced by this call
   |
note: required by a bound in `check_copy`
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:70:18
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:70:18
   |
LL | fn check_copy<T: Copy>(_x: &T) {}
   |                  ^^^^ required by this bound in `check_copy`

error[E0277]: the trait bound `[static generator@/checkout/src/test/ui/generator/clone-impl-async.rs:12:33: 16:6]: Clone` is not satisfied
   |
   |
LL |     check_clone(&inner_non_clone);
   |     ----------- ^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `[static generator@/checkout/src/test/ui/generator/clone-impl-async.rs:12:33: 16:6]`
   |     required by a bound introduced by this call
   |
note: required by a bound in `check_clone`
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:71:19
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:71:19
   |
LL | fn check_clone<T: Clone>(_x: &T) {}
   |                   ^^^^^ required by this bound in `check_clone`

error[E0277]: the trait bound `[static generator@/checkout/src/test/ui/generator/clone-impl-async.rs:23:38: 25:6]: Copy` is not satisfied
   |
   |
LL |     check_copy(&outer_non_clone);
   |     ---------- ^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `[static generator@/checkout/src/test/ui/generator/clone-impl-async.rs:23:38: 25:6]`
   |     required by a bound introduced by this call
   |
note: required by a bound in `check_copy`
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:70:18
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:70:18
   |
LL | fn check_copy<T: Copy>(_x: &T) {}
   |                  ^^^^ required by this bound in `check_copy`

error[E0277]: the trait bound `[static generator@/checkout/src/test/ui/generator/clone-impl-async.rs:23:38: 25:6]: Clone` is not satisfied
   |
   |
LL |     check_clone(&outer_non_clone);
   |     ----------- ^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `[static generator@/checkout/src/test/ui/generator/clone-impl-async.rs:23:38: 25:6]`
   |     required by a bound introduced by this call
   |
note: required by a bound in `check_clone`
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:71:19
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:71:19
   |
LL | fn check_clone<T: Clone>(_x: &T) {}
   |                   ^^^^^ required by this bound in `check_clone`

error[E0277]: the trait bound `[static generator@/checkout/src/test/ui/generator/clone-impl-async.rs:31:39: 31:41]: Copy` is not satisfied
   |
   |
LL |     check_copy(&maybe_copy_clone);
   |     ---------- ^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `[static generator@/checkout/src/test/ui/generator/clone-impl-async.rs:31:39: 31:41]`
   |     required by a bound introduced by this call
   |
note: required by a bound in `check_copy`
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:70:18
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:70:18
   |
LL | fn check_copy<T: Copy>(_x: &T) {}
   |                  ^^^^ required by this bound in `check_copy`

error[E0277]: the trait bound `[static generator@/checkout/src/test/ui/generator/clone-impl-async.rs:31:39: 31:41]: Clone` is not satisfied
   |
   |
LL |     check_clone(&maybe_copy_clone);
   |     ----------- ^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `[static generator@/checkout/src/test/ui/generator/clone-impl-async.rs:31:39: 31:41]`
   |     required by a bound introduced by this call
   |
note: required by a bound in `check_clone`
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:71:19
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:71:19
   |
LL | fn check_clone<T: Clone>(_x: &T) {}
   |                   ^^^^^ required by this bound in `check_clone`

error[E0277]: the trait bound `impl Future<Output = ()>: Copy` is not satisfied
   |
   |
LL |     check_copy(&inner_non_clone_fn);
   |     ---------- ^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `impl Future<Output = ()>`
   |     required by a bound introduced by this call
   |
note: required by a bound in `check_copy`
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:70:18
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:70:18
   |
LL | fn check_copy<T: Copy>(_x: &T) {}
   |                  ^^^^ required by this bound in `check_copy`

error[E0277]: the trait bound `impl Future<Output = ()>: Clone` is not satisfied
   |
   |
LL |     check_clone(&inner_non_clone_fn);
   |     ----------- ^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `impl Future<Output = ()>`
   |     required by a bound introduced by this call
   |
note: required by a bound in `check_clone`
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:71:19
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:71:19
   |
LL | fn check_clone<T: Clone>(_x: &T) {}
   |                   ^^^^^ required by this bound in `check_clone`

error[E0277]: the trait bound `impl Future<Output = ()>: Copy` is not satisfied
   |
   |
LL |     check_copy(&outer_non_clone_fn);
   |     ---------- ^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `impl Future<Output = ()>`
   |     required by a bound introduced by this call
   |
note: required by a bound in `check_copy`
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:70:18
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:70:18
   |
LL | fn check_copy<T: Copy>(_x: &T) {}
   |                  ^^^^ required by this bound in `check_copy`

error[E0277]: the trait bound `impl Future<Output = ()>: Clone` is not satisfied
   |
   |
LL |     check_clone(&outer_non_clone_fn);
   |     ----------- ^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `impl Future<Output = ()>`
   |     required by a bound introduced by this call
   |
note: required by a bound in `check_clone`
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:71:19
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:71:19
   |
LL | fn check_clone<T: Clone>(_x: &T) {}
   |                   ^^^^^ required by this bound in `check_clone`

error[E0277]: the trait bound `impl Future<Output = ()>: Copy` is not satisfied
   |
   |
LL |     check_copy(&maybe_copy_clone_fn);
   |     ---------- ^^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `impl Future<Output = ()>`
   |     required by a bound introduced by this call
   |
note: required by a bound in `check_copy`
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:70:18
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:70:18
   |
LL | fn check_copy<T: Copy>(_x: &T) {}
   |                  ^^^^ required by this bound in `check_copy`

error[E0277]: the trait bound `impl Future<Output = ()>: Clone` is not satisfied
   |
   |
LL |     check_clone(&maybe_copy_clone_fn);
   |     ----------- ^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `impl Future<Output = ()>`
   |     required by a bound introduced by this call
   |
note: required by a bound in `check_clone`
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:71:19
  --> /checkout/src/test/ui/generator/clone-impl-async.rs:71:19
   |
LL | fn check_clone<T: Clone>(_x: &T) {}
   |                   ^^^^^ required by this bound in `check_clone`
error: aborting due to 12 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
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
   = note: hidden type `[static generator@/checkout/src/test/ui/generator/issue-93161.rs:52:70: 54:2]` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = Option<()>>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn read_exact(_from: &mut &[u8], _to: &mut [u8]) -> Option<()> {
LL | |     Some(())
LL | | }
   | |_^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/generator/issue-93161.rs:52:70: 54:2]` captures lifetime '_#20r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
---
1 error: lifetime bound not satisfied
-   --> $DIR/issue-100013.rs:15:5
+   --> $DIR/issue-100013.rs:15:11
3    |
- LL | /     async { // a generator checked for autotrait impl `Send`
+ LL |       async { // a generator checked for autotrait impl `Send`
5 LL | |
5 LL | |
6 LL | |         let x = None::<I::Future<'_, '_>>; // a type referencing GAT
7 LL | |         async {}.await; // a yield point
21    = note: this is a known limitation that will be removed in the future (see issue #100013 <https://github.com/rust-lang/rust/issues/100013> for more information)
22 
23 error: lifetime bound not satisfied
-   --> $DIR/issue-100013.rs:23:5
-   --> $DIR/issue-100013.rs:23:5
+   --> $DIR/issue-100013.rs:23:11
25    |
- LL | /     async { // a generator checked for autotrait impl `Send`
+ LL |       async { // a generator checked for autotrait impl `Send`
27 LL | |
27 LL | |
28 LL | |         let x = None::<I::Future<'a, 'b>>; // a type referencing GAT
29 LL | |

55    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'a` must outlive `'b`
56    |
57    = help: consider adding the following bound: `'a: 'b`
+ help: consider adding 'move' keyword before the nested closure
+    |
+ LL |     async move { // a generator checked for autotrait impl `Send`
58 
59 error: lifetime bound not satisfied
-   --> $DIR/issue-100013.rs:32:5
+   --> $DIR/issue-100013.rs:32:11
+   --> $DIR/issue-100013.rs:32:11
61    |
- LL | /     async { // a generator checked for autotrait impl `Send`
+ LL |       async { // a generator checked for autotrait impl `Send`
63 LL | |
63 LL | |
64 LL | |         let x = None::<I::Future<'a, 'b>>; // a type referencing GAT
65 LL | |         async {}.await; // a yield point

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-100013/issue-100013.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/bugs/issue-100013.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/bugs/issue-100013.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-100013" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-100013/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
error: lifetime bound not satisfied
   |
   |
LL |       async { // a generator checked for autotrait impl `Send`
   |  ___________^
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
  --> /checkout/src/test/ui/generic-associated-types/bugs/issue-100013.rs:23:11
  --> /checkout/src/test/ui/generic-associated-types/bugs/issue-100013.rs:23:11
   |
LL |       async { // a generator checked for autotrait impl `Send`
   |  ___________^
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
LL |     async move { // a generator checked for autotrait impl `Send`

error: lifetime bound not satisfied
  --> /checkout/src/test/ui/generic-associated-types/bugs/issue-100013.rs:32:11
   |
   |
LL |       async { // a generator checked for autotrait impl `Send`
   |  ___________^
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
+   --> $DIR/issue-90014.rs:17:15
+   --> $DIR/issue-90014.rs:17:15
+    |
+ LL |         async { () }
+    |               ^^^^^^ lifetime `'a` is part of concrete type but not used in parameter list of the `impl Trait` type alias
+ error: non-defining opaque type use in defining scope
+   --> $DIR/issue-90014.rs:17:15
+    |
+    |
+ LL |         async { () }
+    |               ^^^^^^ lifetime `'a` is part of concrete type but not used in parameter list of the `impl Trait` type alias
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
  --> /checkout/src/test/ui/generic-associated-types/issue-90014.rs:17:15
   |
   |
LL |         async { () }
   |               ^^^^^^ lifetime `'a` is part of concrete type but not used in parameter list of the `impl Trait` type alias
error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/generic-associated-types/issue-90014.rs:17:15
   |
LL |         async { () }
LL |         async { () }
   |               ^^^^^^ lifetime `'a` is part of concrete type but not used in parameter list of the `impl Trait` type alias
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0477`.
------------------------------------------
---
1 error[E0277]: the size for values of type `A` cannot be known at compilation time
-   --> $DIR/issue-88287.rs:34:9
+   --> $DIR/issue-88287.rs:34:20
3    |
4 LL | type SearchFutureTy<'f, A, B: 'f>
5    |                         - this type parameter needs to be `std::marker::Sized`
6 ...
6 ...
7 LL |         async move { todo!() }
+    |                    ^^^^^^^^^^^ doesn't have a size known at compile-time
9    |
9    |
10 note: required by a bound in `<T as SearchableResourceExt<Criteria>>`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-88287/issue-88287.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-88287/issue-88287.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/issue-88287.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-88287.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-88287" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-88287/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0277]: the size for values of type `A` cannot be known at compilation time
   |
   |
LL | type SearchFutureTy<'f, A, B: 'f>
   |                         - this type parameter needs to be `std::marker::Sized`
...
LL |         async move { todo!() }
   |
   |
note: required by a bound in `<T as SearchableResourceExt<Criteria>>`
   |
   |
LL | impl<T, Criteria> SearchableResourceExt<Criteria> for T
   |      ^ required by this bound in `<T as SearchableResourceExt<Criteria>>`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL -     A: SearchableResource<B> + ?Sized + 'f,
LL +     A: SearchableResource<B> + 'f,
   |
help: consider relaxing the implicit `Sized` restriction
   |
LL |     T: SearchableResource<Criteria> + ?Sized,

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
---
1 error: `C` does not live long enough
-   --> $DIR/issue-92096.rs:17:5
+   --> $DIR/issue-92096.rs:17:16
3    |
4 LL |     async move { c.connect().await }
+    |                ^^^^^^^^^^^^^^^^^^^^^
6 
7 error: aborting due to previous error
8 
---
To only update this specific test, also pass `--test-args generic-associated-types/issue-92096.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-92096.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-92096" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-92096/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: `C` does not live long enough
   |
   |
LL |     async move { c.connect().await }

error: aborting due to previous error
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


---- [ui] src/test/ui/impl-trait/issue-55872-2.rs stdout ----
diff of stderr:

1 error: type parameter `T` is part of concrete type but not used in parameter list for the `impl Trait` type alias
-   --> $DIR/issue-55872-2.rs:14:9
3    |
4 LL |         async {}
-    |         ^^^^^^^^
+    |               ^^
---
To only update this specific test, also pass `--test-args impl-trait/issue-55872-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issue-55872-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issue-55872-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issue-55872-2/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: type parameter `T` is part of concrete type but not used in parameter list for the `impl Trait` type alias
   |
LL |         async {}
   |               ^^


error: aborting due to previous error
------------------------------------------


---- [ui] src/test/ui/impl-trait/issue-55872-3.rs stdout ----
diff of stderr:

- error[E0277]: the trait bound `impl Future<Output = ()>: Copy` is not satisfied
+ error[E0277]: the trait bound `[static generator@$DIR/issue-55872-3.rs:16:15: 16:17]: Copy` is not satisfied
3    |
4 LL |     fn foo<T>() -> Self::E {

-    |                    ^^^^^^^ the trait `Copy` is not implemented for `impl Future<Output = ()>`
---
To only update this specific test, also pass `--test-args impl-trait/issue-55872-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issue-55872-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issue-55872-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issue-55872-3/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `[static generator@/checkout/src/test/ui/impl-trait/issue-55872-3.rs:16:15: 16:17]: Copy` is not satisfied
   |
LL |     fn foo<T>() -> Self::E {
LL |     fn foo<T>() -> Self::E {
   |                    ^^^^^^^ the trait `Copy` is not implemented for `[static generator@/checkout/src/test/ui/impl-trait/issue-55872-3.rs:16:15: 16:17]`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
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
- error[E0271]: expected `impl Future<Output = ()>` to be a future that resolves to `u8`, but it resolves to `()`
+ error[E0271]: expected `[static generator@$DIR/issue-78722.rs:11:19: 11:21]` to be a future that resolves to `u8`, but it resolves to `()`
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
error[E0271]: expected `[static generator@/checkout/src/test/ui/impl-trait/issues/issue-78722.rs:11:19: 11:21]` to be a future that resolves to `u8`, but it resolves to `()`
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
   = note: hidden type `[static generator@/checkout/src/test/ui/lifetimes/issue-77175.rs:11:67: 13:2]` captures lifetime '_#15r

error[E0700]: hidden type for `impl Future<Output = &'a str>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn bar<'a>(s1: String, s2: &'_ str, s3: &'a str) -> &'a str {
LL | |     s3
LL | | }
   | |_^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/lifetimes/issue-77175.rs:11:67: 13:2]` captures lifetime '_#16r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
---
19 LL |     async {};
-    |     ^^^^^^^^
+    |           ^^
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
  --> /checkout/src/test/ui/lint/unused/unused-closure.rs:13:11
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



---- [ui] src/test/ui/parser/fn-header-semantic-fail.rs stdout ----
diff of stderr:

188    = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
189    = help: add `#![feature(async_fn_in_trait)]` to the crate attributes to enable
190 
- error[E0391]: cycle detected when computing type of `main::ff5::{opaque#0}`
-   --> $DIR/fn-header-semantic-fail.rs:12:44
-    |
- LL |     const async unsafe extern "C" fn ff5() {}
-    |
-    |
- note: ...which requires borrow-checking `main::ff5`...
-   --> $DIR/fn-header-semantic-fail.rs:12:5
-    |
- LL |     const async unsafe extern "C" fn ff5() {}
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing MIR for `main::ff5`...
-   --> $DIR/fn-header-semantic-fail.rs:12:5
-    |
- LL |     const async unsafe extern "C" fn ff5() {}
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires const checking `main::ff5`...
-   --> $DIR/fn-header-semantic-fail.rs:12:5
-    |
- LL |     const async unsafe extern "C" fn ff5() {}
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    = note: ...which requires computing whether `impl core::future::future::Future<Output = ()>` is freeze...
-    = note: ...which requires evaluating trait selection obligation `impl core::future::future::Future<Output = ()>: core::marker::Freeze`...
-    = note: ...which again requires computing type of `main::ff5::{opaque#0}`, completing the cycle
- note: cycle used when checking item types in top-level module
-   --> $DIR/fn-header-semantic-fail.rs:5:1
- LL | / #![feature(const_extern_fn)]
- LL | |
- LL | | fn main() {
- LL | | fn main() {
- LL | |     async fn ff1() {} // OK.
- LL | |     }
- LL | | }
-    | |_^
+ error: aborting due to 18 previous errors
+ error: aborting due to 18 previous errors
226 
- error[E0391]: cycle detected when computing type of `main::<impl at $DIR/fn-header-semantic-fail.rs:28:5: 28:17>::ft5::{opaque#0}`
-   --> $DIR/fn-header-semantic-fail.rs:33:48
-    |
- LL |         const async unsafe extern "C" fn ft5() {}
-    |
-    |
- note: ...which requires borrow-checking `main::<impl at $DIR/fn-header-semantic-fail.rs:28:5: 28:17>::ft5`...
-   --> $DIR/fn-header-semantic-fail.rs:33:9
-    |
- LL |         const async unsafe extern "C" fn ft5() {}
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing MIR for `main::<impl at $DIR/fn-header-semantic-fail.rs:28:5: 28:17>::ft5`...
-   --> $DIR/fn-header-semantic-fail.rs:33:9
-    |
- LL |         const async unsafe extern "C" fn ft5() {}
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires const checking `main::<impl at $DIR/fn-header-semantic-fail.rs:28:5: 28:17>::ft5`...
-   --> $DIR/fn-header-semantic-fail.rs:33:9
-    |
- LL |         const async unsafe extern "C" fn ft5() {}
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    = note: ...which requires computing whether `impl core::future::future::Future<Output = ()>` is freeze...
-    = note: ...which requires evaluating trait selection obligation `impl core::future::future::Future<Output = ()>: core::marker::Freeze`...
-    = note: ...which again requires computing type of `main::<impl at $DIR/fn-header-semantic-fail.rs:28:5: 28:17>::ft5::{opaque#0}`, completing the cycle
- note: cycle used when checking item types in top-level module
-   --> $DIR/fn-header-semantic-fail.rs:5:1
- LL | / #![feature(const_extern_fn)]
- LL | |
- LL | | fn main() {
- LL | | fn main() {
- LL | |     async fn ff1() {} // OK.
- LL | |     }
- LL | | }
-    | |_^
- 
- 
- error[E0391]: cycle detected when computing type of `main::<impl at $DIR/fn-header-semantic-fail.rs:40:5: 40:11>::fi5::{opaque#0}`
-   --> $DIR/fn-header-semantic-fail.rs:45:48
-    |
- LL |         const async unsafe extern "C" fn fi5() {}
-    |
-    |
- note: ...which requires borrow-checking `main::<impl at $DIR/fn-header-semantic-fail.rs:40:5: 40:11>::fi5`...
-   --> $DIR/fn-header-semantic-fail.rs:45:9
-    |
- LL |         const async unsafe extern "C" fn fi5() {}
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires processing MIR for `main::<impl at $DIR/fn-header-semantic-fail.rs:40:5: 40:11>::fi5`...
-   --> $DIR/fn-header-semantic-fail.rs:45:9
-    |
- LL |         const async unsafe extern "C" fn fi5() {}
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: ...which requires const checking `main::<impl at $DIR/fn-header-semantic-fail.rs:40:5: 40:11>::fi5`...
-   --> $DIR/fn-header-semantic-fail.rs:45:9
-    |
- LL |         const async unsafe extern "C" fn fi5() {}
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    = note: ...which requires computing whether `impl core::future::future::Future<Output = ()>` is freeze...
-    = note: ...which requires evaluating trait selection obligation `impl core::future::future::Future<Output = ()>: core::marker::Freeze`...
-    = note: ...which again requires computing type of `main::<impl at $DIR/fn-header-semantic-fail.rs:40:5: 40:11>::fi5::{opaque#0}`, completing the cycle
- note: cycle used when checking item types in top-level module
-   --> $DIR/fn-header-semantic-fail.rs:5:1
- LL | / #![feature(const_extern_fn)]
- LL | |
- LL | | fn main() {
- LL | | fn main() {
- LL | |     async fn ff1() {} // OK.
- LL | |     }
- LL | | }
-    | |_^
- 
---
To only update this specific test, also pass `--test-args parser/fn-header-semantic-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/fn-header-semantic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-header-semantic-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/fn-header-semantic-fail/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: functions cannot be both `const` and `async`
   |
   |
LL |     const async unsafe extern "C" fn ff5() {}
   |     ^^^^^-^^^^^------------------------------
   |     |     |
   |     |     `async` because of this
   |     `const` because of this
error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:19:9
   |
   |
LL |         const fn ft3(); //~ ERROR functions in traits cannot be declared const

error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:21:9
   |
   |
LL |         const async unsafe extern "C" fn ft5();


error: functions cannot be both `const` and `async`
   |
   |
LL |         const async unsafe extern "C" fn ft5();
   |         ^^^^^-^^^^^----------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this
error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:31:9
   |
   |
LL |         const fn ft3() {} //~ ERROR functions in traits cannot be declared const

error[E0379]: functions in traits cannot be declared const
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:33:9
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}


error: functions cannot be both `const` and `async`
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |         ^^^^^-^^^^^------------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this

error: functions cannot be both `const` and `async`
   |
   |
LL |         const async unsafe extern "C" fn fi5() {}
   |         ^^^^^-^^^^^------------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this

error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
LL |         async fn fe1(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe1(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
LL |         async fn fe1(); //~ ERROR functions in `extern` blocks cannot have qualifiers
LL |         unsafe fn fe2(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe2(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
...
LL |         const fn fe3(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe3(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
...
LL |         extern "C" fn fe4(); //~ ERROR functions in `extern` blocks cannot have qualifiers
   |
help: remove the qualifiers
   |
   |
LL |         fn fe4(); //~ ERROR functions in `extern` blocks cannot have qualifiers


error: functions in `extern` blocks cannot have qualifiers
   |
LL |     extern "C" {
LL |     extern "C" {
   |     ---------- in this `extern` block
...
LL |         const async unsafe extern "C" fn fe5(); //~ ERROR functions in `extern` blocks
   |
help: remove the qualifiers
   |
   |
LL |         fn fe5(); //~ ERROR functions in `extern` blocks


error: functions cannot be both `const` and `async`
   |
   |
LL |         const async unsafe extern "C" fn fe5(); //~ ERROR functions in `extern` blocks
   |         ^^^^^-^^^^^----------------------------
   |         |     |
   |         |     `async` because of this
   |         `const` because of this
error[E0706]: functions in traits cannot be declared `async`
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:17:9
   |
   |
LL |         async fn ft1(); //~ ERROR functions in traits cannot be declared `async`
   |         |
   |         |
   |         `async` because of this
   = note: `async` trait functions are not currently supported
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
   = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
   = help: add `#![feature(async_fn_in_trait)]` to the crate attributes to enable
error[E0706]: functions in traits cannot be declared `async`
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:21:9
   |
   |
LL |         const async unsafe extern "C" fn ft5();
   |               |
   |               |
   |               `async` because of this
   = note: `async` trait functions are not currently supported
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
   = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
   = help: add `#![feature(async_fn_in_trait)]` to the crate attributes to enable
error[E0706]: functions in traits cannot be declared `async`
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:29:9
   |
   |
LL |         async fn ft1() {} //~ ERROR functions in traits cannot be declared `async`
   |         |
   |         |
   |         `async` because of this
   = note: `async` trait functions are not currently supported
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
   = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
   = help: add `#![feature(async_fn_in_trait)]` to the crate attributes to enable
error[E0706]: functions in traits cannot be declared `async`
  --> /checkout/src/test/ui/parser/fn-header-semantic-fail.rs:33:9
   |
   |
LL |         const async unsafe extern "C" fn ft5() {}
   |               |
   |               |
   |               `async` because of this
   = note: `async` trait functions are not currently supported
   = note: `async` trait functions are not currently supported
   = note: consider using the `async-trait` crate: https://crates.io/crates/async-trait
   = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
   = help: add `#![feature(async_fn_in_trait)]` to the crate attributes to enable
error: aborting due to 18 previous errors

Some errors have detailed explanations: E0379, E0706.
For more information about an error, try `rustc --explain E0379`.
For more information about an error, try `rustc --explain E0379`.
------------------------------------------


---- [ui] src/test/ui/pattern/non-structural-match-types.rs stdout ----
diff of stderr:

4 LL |         const { || {} } => {},
6 
6 
- error: `impl Future<Output = ()>` cannot be used in patterns
+ error: `[static generator@$DIR/non-structural-match-types.rs:12:23: 12:25]` cannot be used in patterns
9    |
9    |
10 LL |         const { async {} } => {},

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/non-structural-match-types/non-structural-match-types.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/non-structural-match-types.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/non-structural-match-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/non-structural-match-types" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/non-structural-match-types/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
error: `[closure@/checkout/src/test/ui/pattern/non-structural-match-types.rs:9:17: 9:19]` cannot be used in patterns
   |
   |
LL |         const { || {} } => {}, //~ ERROR cannot be used in patterns


error: `[static generator@/checkout/src/test/ui/pattern/non-structural-match-types.rs:12:23: 12:25]` cannot be used in patterns
   |
   |
LL |         const { async {} } => {}, //~ ERROR cannot be used in patterns

error: aborting due to 2 previous errors
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
   |                    hidden type `[static generator@/checkout/src/test/ui/regions/issue-72051-member-region-hang.rs:6:74: 6:76]` captures the lifetime `'a` as defined here

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | pub async fn query<'a>(_: &(), _: &(), _: (&(dyn std::any::Any + 'a),) ) {}
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/regions/issue-72051-member-region-hang.rs:6:74: 6:76]` captures lifetime '_#24r

error[E0700]: hidden type for `impl Future<Output = ()>` captures lifetime that does not appear in bounds
   |
   |
LL | pub async fn query<'a>(_: &(), _: &(), _: (&(dyn std::any::Any + 'a),) ) {}
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/regions/issue-72051-member-region-hang.rs:6:74: 6:76]` captures lifetime '_#25r
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
   = note: hidden type `[static generator@/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime-async.rs:23:71: 23:79]` captures lifetime '_#15r

error[E0700]: hidden type for `impl Future<Output = Pin<&Foo>>` captures lifetime that does not appear in bounds
   |
   |
LL |     async fn bar<'a>(self: Alias<&Self>, arg: &'a ()) -> Alias<&Self> { self }
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/arbitrary_self_types_pin_lifetime-async.rs:23:71: 23:79]` captures lifetime '_#16r
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
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-alias-async.rs:13:47: 15:6]` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_self(self, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-alias-async.rs:13:47: 15:6]` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Alias(self: Alias<'a>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-alias-async.rs:17:59: 19:6]` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Alias(self: Alias<'a>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-alias-async.rs:17:59: 19:6]` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Alias(self: Box<Alias<'a>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-alias-async.rs:21:68: 23:6]` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Alias(self: Box<Alias<'a>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-alias-async.rs:21:68: 23:6]` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Box_Alias(self: Box<Box<Alias<'a>>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-alias-async.rs:25:77: 27:6]` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Box_Alias(self: Box<Box<Alias<'a>>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-alias-async.rs:25:77: 27:6]` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Rc_Alias(self: Rc<Alias<'a>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-alias-async.rs:29:66: 31:6]` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Rc_Alias(self: Rc<Alias<'a>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-alias-async.rs:29:66: 31:6]` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Rc_Alias(self: Box<Rc<Alias<'a>>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-alias-async.rs:33:75: 35:6]` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Rc_Alias(self: Box<Rc<Alias<'a>>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-alias-async.rs:33:75: 35:6]` captures lifetime '_#20r
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
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-assoc-async.rs:19:47: 21:6]` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_self(self, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-assoc-async.rs:19:47: 21:6]` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_AssocType(self: <Struct<'a> as Trait>::AssocType, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-assoc-async.rs:23:86: 25:6]` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_AssocType(self: <Struct<'a> as Trait>::AssocType, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-assoc-async.rs:23:86: 25:6]` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_AssocType(self: Box<<Struct<'a> as Trait>::AssocType>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-assoc-async.rs:27:95: 29:6]` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_AssocType(self: Box<<Struct<'a> as Trait>::AssocType>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-assoc-async.rs:27:95: 29:6]` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
LL |       ) -> &u32 {
   |  _______________^
LL | |         f
LL | |         f
LL | |     }
   | |_____^
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-assoc-async.rs:34:15: 36:6]` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
LL |       ) -> &u32 {
   |  _______________^
LL | |         f
LL | |         f
LL | |     }
   | |_____^
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-assoc-async.rs:34:15: 36:6]` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Rc_AssocType(self: Rc<<Struct<'a> as Trait>::AssocType>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-assoc-async.rs:38:93: 40:6]` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Rc_AssocType(self: Rc<<Struct<'a> as Trait>::AssocType>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-assoc-async.rs:38:93: 40:6]` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
LL |       ) -> &u32 {
   |  _______________^
LL | |         f
LL | |         f
LL | |     }
   | |_____^
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-assoc-async.rs:45:15: 47:6]` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
LL |       ) -> &u32 {
   |  _______________^
LL | |         f
LL | |         f
LL | |     }
   | |_____^
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-assoc-async.rs:45:15: 47:6]` captures lifetime '_#20r
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
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-self-async.rs:14:47: 16:6]` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_self(self, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-self-async.rs:14:47: 16:6]` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Self(self: Self, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-self-async.rs:18:53: 20:6]` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Self(self: Self, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-self-async.rs:18:53: 20:6]` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Self(self: Box<Self>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-self-async.rs:22:62: 24:6]` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Self(self: Box<Self>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-self-async.rs:22:62: 24:6]` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Box_Self(self: Box<Box<Self>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-self-async.rs:26:71: 28:6]` captures lifetime '_#19r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Box_Box_Self(self: Box<Box<Self>>, f: &u32) -> &u32 {
LL | |         f
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/self/elision/lt-self-async.rs:26:71: 28:6]` captures lifetime '_#20r

error[E0700]: hidden type for `impl Future<Output = &u32>` captures lifetime that does not appear in bounds
   |
   |
LL |       async fn take_Rc_Self(self: Rc<Self>, f: &u32) -> &u32 {
LL | |         f
---


---- [ui] src/test/ui/suggestions/issue-81839.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui/suggestions/auxiliary/issue-81839.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/auxiliary/issue-81839.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-81839/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-81839/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Future<Output = Test>` captures lifetime that does not appear in bounds
   |
   |
LL |       pub async fn answer_str(&self, _s: &str) -> Test {
LL | |         Test {}
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/suggestions/auxiliary/issue-81839.rs:6:54: 8:6]` captures lifetime '_#14r

error[E0700]: hidden type for `impl Future<Output = Test>` captures lifetime that does not appear in bounds
   |
   |
LL |       pub async fn answer_str(&self, _s: &str) -> Test {
LL | |         Test {}
LL | |     }
   | |_____^
   |
   |
   = note: hidden type `[static generator@/checkout/src/test/ui/suggestions/auxiliary/issue-81839.rs:6:54: 8:6]` captures lifetime '_#15r
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
------------------------------------------
---
18    |                     ^^--
19    |                     |
20    |                     call expression requires function
-    |
- help: if you meant to create this closure and immediately call it, surround the closure with parentheses
-    |
- LL |     let _ = (async ||{})();
26 
27 error: aborting due to 2 previous errors
28 

---
To only update this specific test, also pass `--test-args suggestions/suggest-on-bare-closure-call.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-on-bare-closure-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-on-bare-closure-call" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-on-bare-closure-call/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/suggestions/suggest-on-bare-closure-call.rs:6:15
   |
   |
LL |     let _ = ||{}();
   |               ^^--
   |               call expression requires function
   |
   |
help: if you meant to create this closure and immediately call it, surround the closure with parentheses
   |
LL |     let _ = (||{})();

error[E0618]: expected function, found `()`
  --> /checkout/src/test/ui/suggestions/suggest-on-bare-closure-call.rs:9:21
   |
   |
LL |     let _ = async ||{}();
   |                     ^^--
   |                     call expression requires function

error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0618`.
------------------------------------------


---- [ui] src/test/ui/type-alias-impl-trait/future.rs stdout ----
diff of stderr:

1 error[E0277]: the trait bound `B: Bar` is not satisfied
-   --> $DIR/future.rs:15:5
3    |
3    |
4 LL |     async move { bar.bar() }
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Bar` is not implemented for `B`
+    |                ^^^^^^^^^^^^^ the trait `Bar` is not implemented for `B`
7 note: required by a bound in `foo`
8   --> $DIR/future.rs:14:11



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/future/future.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/future.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/future.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/future" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/future/auxiliary" "--edition=2021" "--crate-type=lib"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `B: Bar` is not satisfied
   |
   |
LL |     async move { bar.bar() }
   |                ^^^^^^^^^^^^^ the trait `Bar` is not implemented for `B`
note: required by a bound in `foo`
  --> /checkout/src/test/ui/type-alias-impl-trait/future.rs:14:11
   |
   |
LL | fn foo<B: Bar>(bar: B) -> FooFuture<B> {
   |           ^^^ required by this bound in `foo`
help: consider restricting type parameter `B`
   |
LL | type FooFuture<B: Bar> = impl Future<Output = ()>;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
---
1 error[E0277]: the trait bound `T: Trait` is not satisfied
-   --> $DIR/issue-89686.rs:18:9
+   --> $DIR/issue-89686.rs:18:20
3    |
4 LL |         async move { self.f().await }
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Trait` is not implemented for `T`
+    |                    ^^^^^^^^^^^^^^^^^^ the trait `Trait` is not implemented for `T`
7 help: consider restricting type parameter `T`
8    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-89686/issue-89686.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-89686.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-89686.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-89686" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-89686/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `T: Trait` is not satisfied
   |
   |
LL |         async move { self.f().await }
   |                    ^^^^^^^^^^^^^^^^^^ the trait `Trait` is not implemented for `T`
help: consider restricting type parameter `T`
   |
   |
LL | type G<'a, T: Trait> = impl Future<Output = ()> + 'a;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
