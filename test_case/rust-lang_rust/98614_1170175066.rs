plain
---- [ui] src/test/ui/type-alias-impl-trait/issue-98608.rs stdout ----
diff of stderr:

9    |
10    = note:   expected struct `Box<u8>`
-    = note: required for the cast to the object type `dyn Fn() -> Box<u8>`
-    = note: required for the cast to the object type `dyn Fn() -> Box<u8>`
+    = note: required for the cast from `fn() -> impl Sized {hi}` to the object type `dyn Fn() -> Box<u8>`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
14 error: aborting due to previous error
15 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-98608/issue-98608.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-98608.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-98608.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-98608" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-98608/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0271]: type mismatch resolving `<fn() -> impl Sized {hi} as FnOnce<()>>::Output == Box<u8>`
   |
   |
LL | fn hi() -> impl Sized { std::ptr::null::<u8>() }
...
...
LL |     let b: Box<dyn Fn() -> Box<u8>> = Box::new(hi);
   |                                       ^^^^^^^^^^^^ expected struct `Box`, found opaque type
   |
   = note:   expected struct `Box<u8>`
           found opaque type `impl Sized`
   = note: required for the cast from `fn() -> impl Sized {hi}` to the object type `dyn Fn() -> Box<u8>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/type-alias-impl-trait/issue-98604.rs stdout ----
diff of stderr:

11    |                 ^ checked the `Output` of this `async fn`, found opaque type
12    = note:   expected struct `Pin<Box<(dyn Future<Output = ()> + 'static)>>`
13            found opaque type `impl Future<Output = ()>`
-    = note: required for the cast to the object type `dyn Fn() -> Pin<Box<(dyn Future<Output = ()> + 'static)>>`
+    = note: required for the cast from `fn() -> impl Future<Output = ()> {test}` to the object type `dyn Fn() -> Pin<Box<(dyn Future<Output = ()> + 'static)>>`
16 error: aborting due to previous error
17 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-98604/issue-98604.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type-alias-impl-trait/issue-98604.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/issue-98604.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-98604" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/issue-98604/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0271]: type mismatch resolving `<fn() -> impl Future<Output = ()> {test} as FnOnce<()>>::Output == Pin<Box<(dyn Future<Output = ()> + 'static)>>`
   |
   |
LL |     Box::new(test) as AsyncFnPtr;
   |     ^^^^^^^^^^^^^^ expected struct `Pin`, found opaque type
note: while checking the return type of the `async fn`
  --> /checkout/src/test/ui/type-alias-impl-trait/issue-98604.rs:7:17
   |
LL | async fn test() {}
LL | async fn test() {}
   |                 ^ checked the `Output` of this `async fn`, found opaque type
   = note:   expected struct `Pin<Box<(dyn Future<Output = ()> + 'static)>>`
           found opaque type `impl Future<Output = ()>`
   = note: required for the cast from `fn() -> impl Future<Output = ()> {test}` to the object type `dyn Fn() -> Pin<Box<(dyn Future<Output = ()> + 'static)>>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.
------------------------------------------
