plain
diff of stderr:

7    |     expected due to this
8    |
9    = note: expected closure signature `fn(char) -> _`
-               found closure signature `for<'r> fn(&'r char) -> _`
+               found closure signature `for<'a> fn(&'a char) -> _`
11 note: closure inferred to have a different signature due to this bound
13    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/multiple-fn-bounds/multiple-fn-bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/multiple-fn-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/checkout/src/test/ui/closures/multiple-fn-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/multiple-fn-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/multiple-fn-bounds/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/closures/multiple-fn-bounds.rs:10:5
   |
   |
LL |     foo(move |x| v);
   |     |
   |     expected due to this
   |
   |
   = note: expected closure signature `fn(char) -> _`
              found closure signature `for<'a> fn(&'a char) -> _`
note: closure inferred to have a different signature due to this bound
   |
   |
LL | fn foo<F: Fn(&char) -> bool + Fn(char) -> bool>(f: F) {
note: required by a bound in `foo`
  --> /checkout/src/test/ui/closures/multiple-fn-bounds.rs:1:31
   |
   |
LL | fn foo<F: Fn(&char) -> bool + Fn(char) -> bool>(f: F) {
   |                               ^^^^^^^^^^^^^^^^ required by this bound in `foo`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0631`.
------------------------------------------
