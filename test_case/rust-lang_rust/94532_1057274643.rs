plain
1 error[E0601]: `main` function not found in crate `continue_after_missing_main`
-   --> $DIR/continue-after-missing-main.rs:1:1
+   --> $DIR/continue-after-missing-main.rs:30:2
3    |
- LL | / #![allow(dead_code)]
- LL | |
- LL | | struct Tableau<'a, MP> {
- LL | |     provider: &'a MP,
- LL | |
- LL | | }
-    | |_^ consider adding a `main` function to `$DIR/continue-after-missing-main.rs`
+ LL | }
+ LL | }
+    |  ^ consider adding a `main` function to `$DIR/continue-after-missing-main.rs`
12 
13 error: aborting due to previous error
14 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/continue-after-missing-main.nll/continue-after-missing-main.nll.stderr
To only update this specific test, also pass `--test-args continue-after-missing-main.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/continue-after-missing-main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/continue-after-missing-main.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/continue-after-missing-main.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0601]: `main` function not found in crate `continue_after_missing_main`
   |
   |
LL | } //~ ERROR `main` function not found in crate
   |  ^ consider adding a `main` function to `/checkout/src/test/ui/continue-after-missing-main.rs`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0601`.
------------------------------------------
---
1 error: lifetime may not live long enough
-   --> $DIR/issue-75777.rs:13:5
+   --> $DIR/issue-75777.rs:11:5
3    |
4 LL | fn inject<'a, Env: 'a, A: 'a + Send>(v: A) -> Box<dyn FnOnce(&'a Env) -> BoxFuture<'a, A>> {
5    |           -- lifetime `'a` defined here

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-75777.nll/issue-75777.nll.stderr
To only update this specific test, also pass `--test-args issues/issue-75777.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-75777.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-75777.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-75777.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn inject<'a, Env: 'a, A: 'a + Send>(v: A) -> Box<dyn FnOnce(&'a Env) -> BoxFuture<'a, A>> {
   |           -- lifetime `'a` defined here
LL |     let fut: BoxFuture<'a, A> = Box::pin(future::ready(v));
LL |     Box::new(move |_| fut)
   |     ^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
error: aborting due to previous error
------------------------------------------



---- [ui (nll)] ui/object-lifetime/object-lifetime-default-from-rptr-struct-error.rs stdout ----
diff of stderr:

1 error: lifetime may not live long enough
-   --> $DIR/object-lifetime-default-from-rptr-struct-error.rs:21:5
+   --> $DIR/object-lifetime-default-from-rptr-struct-error.rs:20:5
3    |
4 LL | fn c<'a>(t: &'a MyBox<dyn Test+'a>, mut ss: SomeStruct<'a>) {
5    |      -- lifetime `'a` defined here

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-rptr-struct-error.nll/object-lifetime-default-from-rptr-struct-error.nll.stderr
To only update this specific test, also pass `--test-args object-lifetime/object-lifetime-default-from-rptr-struct-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-lifetime/object-lifetime-default-from-rptr-struct-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-rptr-struct-error.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-rptr-struct-error.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-from-rptr-struct-error.rs:20:5
   |
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL | fn c<'a>(t: &'a MyBox<dyn Test+'a>, mut ss: SomeStruct<'a>) {
   |      -- lifetime `'a` defined here
LL |     ss.t = t; //~ ERROR mismatched types
   |     ^^^^^^^^ assignment requires that `'a` must outlive `'static`
error: aborting due to previous error
------------------------------------------


