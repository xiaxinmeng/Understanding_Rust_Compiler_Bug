plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:75573f9759179a720f4c3af6c9fb518ac0061dca)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
.............................................iii........................................ 14432/14501
.....................................................................
failures:

---- [ui] tests/ui/typeck/lazy-norm/cast-checks-handling-projections.rs stdout ----
normalized stderr:
error[E0271]: type mismatch resolving `char == <u8 as Add>::Output`
  --> $DIR/cast-checks-handling-projections.rs:5:5
   |
LL |     (0u8 + 0u8) as char;
   |     ^^^^^^^^^^^ types differ
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/lazy-norm/cast-checks-handling-projections/cast-checks-handling-projections.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/lazy-norm/cast-checks-handling-projections.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/typeck/lazy-norm/cast-checks-handling-projections.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/lazy-norm/cast-checks-handling-projections" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/lazy-norm/cast-checks-handling-projections/auxiliary" "-Ztrait-solver=next"
stdout: none
--- stderr -------------------------------
error[E0271]: type mismatch resolving `char == <u8 as Add>::Output`
  --> fake-test-src-base/typeck/lazy-norm/cast-checks-handling-projections.rs:5:5
   |
LL |     (0u8 + 0u8) as char;
   |     ^^^^^^^^^^^ types differ
error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.
------------------------------------------
