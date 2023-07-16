plain
---- [ui] src/test/ui/async-await/unnecessary-await.rs stdout ----
diff of stderr:

16    | 
17 help: alternatively, consider making `fn boo` asynchronous
18    |
- LL | async fn boo() {}
-    | +++++
+ LL |  asyncfn boo() {}
+    |  +++++
22 error: aborting due to previous error
23 


---
To only update this specific test, also pass `--test-args async-await/unnecessary-await.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/unnecessary-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unnecessary-await" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/unnecessary-await/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `()` is not a future
   |
   |
LL |     boo().await; //~ ERROR `()` is not a future
   |     -----^^^^^^ `()` is not a future
   |     this call returns `()`
   |
   = help: the trait `Future` is not implemented for `()`
   = help: the trait `Future` is not implemented for `()`
   = note: () must be a future or must implement `IntoFuture` to be awaited
   = note: required because of the requirements on the impl of `IntoFuture` for `()`
help: remove the `.await`
   |
LL -     boo().await; //~ ERROR `()` is not a future
LL +     boo(); //~ ERROR `()` is not a future
   | 
help: alternatively, consider making `fn boo` asynchronous
   |
LL |  asyncfn boo() {}
   |  +++++
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
