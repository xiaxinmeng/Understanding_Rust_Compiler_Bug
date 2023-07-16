plain

1 error[E0277]: the trait bound `String: Copy` is not satisfied
2   --> $DIR/assoc-type-eq-with-dyn-atb-fail.rs:32:18
3    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- <<<<<<< HEAD
5 LL |     fn func() -> Self::Out {
6    |                  ^^^^^^^^^ the trait `Copy` is not implemented for `String`
- =======
- LL |         Box::new(AssocNoCopy)
-    |         ^^^^^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `String`
-    |
-    = note: required for the cast to the object type `dyn Bar<Assoc = <AssocNoCopy as Thing>::Out::{opaque#0}>`
- >>>>>>> parent of 862873d20bb (Note concrete type being coerced into object)
14 error: aborting due to previous error
15 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/assoc-type-eq-with-dyn-atb-fail/assoc-type-eq-with-dyn-atb-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-type-bounds/assoc-type-eq-with-dyn-atb-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/assoc-type-eq-with-dyn-atb-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/assoc-type-eq-with-dyn-atb-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/assoc-type-eq-with-dyn-atb-fail/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `String: Copy` is not satisfied
   |
LL |     fn func() -> Self::Out {
   |                  ^^^^^^^^^ the trait `Copy` is not implemented for `String`

