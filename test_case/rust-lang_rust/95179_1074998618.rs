plain

---- [ui] ui/const-generics/issues/issue-83765.rs stdout ----
diff of stderr:

- error[E0391]: cycle detected when resolving instance `<LazyUpdim<T, {T::DIM}, DIM> as TensorDimension>::DIM`
+ error[E0391]: cycle detected when resolving instance `<LazyUpdim<T, { T::DIM }, DIM> as TensorDimension>::DIM`
3    |
3    |
- LL |     const DIM : usize;
-    |     ^^^^^^^^^^^^^^^^^^
+ LL |     const DIM: usize;
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
6    |
6    |
7 note: ...which requires checking if `TensorDimension` fulfills its obligations...

9    |
9    |
10 LL | trait TensorDimension {
11    | ^^^^^^^^^^^^^^^^^^^^^
-    = note: ...which again requires resolving instance `<LazyUpdim<T, {T::DIM}, DIM> as TensorDimension>::DIM`, completing the cycle
-    = note: cycle used when normalizing `<LazyUpdim<T, {T::DIM}, DIM> as TensorDimension>::DIM`
+    = note: ...which again requires resolving instance `<LazyUpdim<T, { T::DIM }, DIM> as TensorDimension>::DIM`, completing the cycle
+    = note: cycle used when normalizing `<LazyUpdim<T, { T::DIM }, DIM> as TensorDimension>::DIM`
15 error: aborting due to previous error
16 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83765/issue-83765.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/issues/issue-83765.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-83765.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83765" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-83765/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when resolving instance `<LazyUpdim<T, { T::DIM }, DIM> as TensorDimension>::DIM`
   |
   |
LL |     const DIM: usize;
   |
   |
note: ...which requires checking if `TensorDimension` fulfills its obligations...
   |
   |
LL | trait TensorDimension {
   | ^^^^^^^^^^^^^^^^^^^^^
   = note: ...which again requires resolving instance `<LazyUpdim<T, { T::DIM }, DIM> as TensorDimension>::DIM`, completing the cycle
   = note: cycle used when normalizing `<LazyUpdim<T, { T::DIM }, DIM> as TensorDimension>::DIM`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
------------------------------------------
