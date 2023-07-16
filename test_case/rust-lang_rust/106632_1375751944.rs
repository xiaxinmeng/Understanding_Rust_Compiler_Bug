plain
.............................iii........................................................ 14080/14132
....................................................
failures:

---- [ui] src/test/ui/traits/ignore-err-impls.rs stdout ----

2   --> $DIR/ignore-err-impls.rs:6:14
3    |
3    |
4 LL | impl Generic<Type> for S {}
-    |     |
-    |     |
-    |     help: you might be missing a type parameter: `<Type>`
+    |
+ help: you might be missing a type parameter
+    |
+    |
+ LL | impl<Type> Generic<Type> for S {}
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
8 
9 error: aborting due to previous error
10 
---
To only update this specific test, also pass `--test-args traits/ignore-err-impls.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/ignore-err-impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/ignore-err-impls" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/ignore-err-impls/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0412]: cannot find type `Type` in this scope
  --> /checkout/src/test/ui/traits/ignore-err-impls.rs:6:14
   |
LL | impl Generic<Type> for S {}
   |
help: you might be missing a type parameter
   |
   |
LL | impl<Type> Generic<Type> for S {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0412`.
