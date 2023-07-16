`
---- [ui] tests/ui/suggestions/suggest-call-on-pat-mismatch.rs stdout ----
diff of stderr:

4	LL |     if let E::One(var1, var2) = var {
5	   |            ^^^^^^^^^^^^^^^^^^   --- this expression has type `fn(i32, i32) -> E {E::One}`
6	   |            |
-	   |            expected enum constructor, found enum `E`
+	   |            expected enum constructor, found `E`
8	   |
9	   = note: expected enum constructor `fn(i32, i32) -> E {E::One}`
10	                          found enum `E`

19	LL |     let Some(x) = Some;
20	   |         ^^^^^^^   ---- this expression has type `fn(_) -> Option<_> {Option::<_>::Some}`
21	   |         |
-	   |         expected enum constructor, found enum `Option`
+	   |         expected enum constructor, found `Option<_>`
23	   |
24	   = note: expected enum constructor `fn(_) -> Option<_> {Option::<_>::Some}`
25	                          found enum `Option<_>`


The actual stderr differed from the expected stderr.
Actual stderr saved to /home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-call-on-pat-mismatch/suggest-call-on-pat-mismatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/suggest-call-on-pat-mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/matthias/vcs/github/rust/tests/ui/suggestions/suggest-call-on-pat-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/home/matthias/vcs/github/rust/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-call-on-pat-mismatch" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/home/matthias/vcs/github/rust/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-call-on-pat-mismatch/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0308]: mismatched types
  --> fake-test-src-base/suggestions/suggest-call-on-pat-mismatch.rs:7:12
   |
LL |     if let E::One(var1, var2) = var {
   |            ^^^^^^^^^^^^^^^^^^   --- this expression has type `fn(i32, i32) -> E {E::One}`
   |            |
   |            expected enum constructor, found `E`
   |
   = note: expected enum constructor `fn(i32, i32) -> E {E::One}`
                          found enum `E`
help: use parentheses to construct this tuple variant
   |
LL |     if let E::One(var1, var2) = var(/* i32 */, /* i32 */) {
   |                                    ++++++++++++++++++++++

error[E0308]: mismatched types
  --> fake-test-src-base/suggestions/suggest-call-on-pat-mismatch.rs:13:9
   |
LL |     let Some(x) = Some;
   |         ^^^^^^^   ---- this expression has type `fn(_) -> Option<_> {Option::<_>::Some}`
   |         |
   |         expected enum constructor, found `Option<_>`
   |
   = note: expected enum constructor `fn(_) -> Option<_> {Option::<_>::Some}`
                          found enum `Option<_>`
help: use parentheses to construct this tuple variant
   |
LL |     let Some(x) = Some(/* value */);
   |                       +++++++++++++

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------



failures:
    [ui] tests/ui/suggestions/suggest-call-on-pat-mismatch.rs
