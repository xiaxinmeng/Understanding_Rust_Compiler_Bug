plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:cb2b9920774a63bd54b3676f2b669ea1e777a91e)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---

---- [ui] tests/ui/suggestions/suggest-call-on-pat-mismatch.rs stdout ----
diff of stderr:

4 LL |     if let E::One(var1, var2) = var {
5    |            ^^^^^^^^^^^^^^^^^^   --- this expression has type `fn(i32, i32) -> E {E::One}`
-    |            expected enum constructor, found enum `E`
+    |            expected enum constructor, found `E`
8    |
8    |
9    = note: expected enum constructor `fn(i32, i32) -> E {E::One}`
10                           found enum `E`

19 LL |     let Some(x) = Some;
20    |         ^^^^^^^   ---- this expression has type `fn(_) -> Option<_> {Option::<_>::Some}`
-    |         expected enum constructor, found enum `Option`
+    |         expected enum constructor, found `Option<_>`
23    |
23    |
24    = note: expected enum constructor `fn(_) -> Option<_> {Option::<_>::Some}`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-call-on-pat-mismatch/suggest-call-on-pat-mismatch.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-call-on-pat-mismatch/suggest-call-on-pat-mismatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/suggest-call-on-pat-mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/suggestions/suggest-call-on-pat-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-call-on-pat-mismatch" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-call-on-pat-mismatch/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/suggestions/suggest-call-on-pat-mismatch.rs:7:12
   |
   |
LL |     if let E::One(var1, var2) = var {
   |            ^^^^^^^^^^^^^^^^^^   --- this expression has type `fn(i32, i32) -> E {E::One}`
   |            expected enum constructor, found `E`
   |
   |
   = note: expected enum constructor `fn(i32, i32) -> E {E::One}`
                          found enum `E`
help: use parentheses to construct this tuple variant
   |
LL |     if let E::One(var1, var2) = var(/* i32 */, /* i32 */) {

error[E0308]: mismatched types
  --> fake-test-src-base/suggestions/suggest-call-on-pat-mismatch.rs:13:9
   |
   |
LL |     let Some(x) = Some;
   |         ^^^^^^^   ---- this expression has type `fn(_) -> Option<_> {Option::<_>::Some}`
   |         expected enum constructor, found `Option<_>`
   |
   |
   = note: expected enum constructor `fn(_) -> Option<_> {Option::<_>::Some}`
help: use parentheses to construct this tuple variant
   |
   |
LL |     let Some(x) = Some(/* value */);

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
