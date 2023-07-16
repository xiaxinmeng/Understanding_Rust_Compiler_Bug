plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:46007752205b5430f5cabe1357251ea7621a9e98)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
.........................ii.i........................................................... 14520/14571
...................................................
failures:

---- [ui] tests/ui/suggestions/correct-binder-for-arbitrary-bound-sugg.rs stdout ----

8   --> $DIR/correct-binder-for-arbitrary-bound-sugg.rs:3:23
9    |
10 LL | trait Foo
10 LL | trait Foo
-    |       --- required by a bound in this
+    |       --- required by a bound in this trait
12 LL | where
13 LL |     for<'a> &'a Self: Bar,
14    |                       ^^^ required by this bound in `Foo`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/correct-binder-for-arbitrary-bound-sugg/correct-binder-for-arbitrary-bound-sugg.stderr
To only update this specific test, also pass `--test-args suggestions/correct-binder-for-arbitrary-bound-sugg.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/suggestions/correct-binder-for-arbitrary-bound-sugg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/correct-binder-for-arbitrary-bound-sugg" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/correct-binder-for-arbitrary-bound-sugg/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `for<'a> &'a T: Bar` is not satisfied
  --> fake-test-src-base/suggestions/correct-binder-for-arbitrary-bound-sugg.rs:13:11
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
LL | fn foo<T: Foo>() {}
   |           ^^^ the trait `for<'a> Bar` is not implemented for `&'a T`
note: required by a bound in `Foo`
  --> fake-test-src-base/suggestions/correct-binder-for-arbitrary-bound-sugg.rs:3:23
   |
LL | trait Foo
LL | trait Foo
   |       --- required by a bound in this trait
LL | where
LL |     for<'a> &'a Self: Bar,
   |                       ^^^ required by this bound in `Foo`
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | fn foo<T: Foo>() where for<'a> &'a T: Bar {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
