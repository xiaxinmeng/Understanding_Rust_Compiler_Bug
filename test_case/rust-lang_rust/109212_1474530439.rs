plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

---- [ui] tests/ui/stability-attribute/issue-109177.rs stdout ----
diff of stderr:

1 error[E0425]: cannot find function `foo1` in crate `similar_unstable_method`
-   --> $DIR/issue-109177.rs:6:30
3    |
4 LL |     similar_unstable_method::foo1();
5    |                              ^^^^ help: a function with a similar name exists: `foo`


10    | ------------ similarly named function `foo` defined here
11 
12 error[E0599]: no method named `foo1` found for struct `Foo` in the current scope
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   --> $DIR/issue-109177.rs:10:9
+   --> $DIR/issue-109177.rs:11:9
14    |
15 LL |     foo.foo1();
16    |         ^^^^ method not found in `Foo`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/issue-109177/issue-109177.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args stability-attribute/issue-109177.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/stability-attribute/issue-109177.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/issue-109177" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/issue-109177/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find function `foo1` in crate `similar_unstable_method`
  --> fake-test-src-base/stability-attribute/issue-109177.rs:7:30
LL |     similar_unstable_method::foo1();
   |                              ^^^^ help: a function with a similar name exists: `foo`
   |
   |
  ::: fake-test-src-base/stability-attribute/auxiliary/similar-unstable-method.rs:5:1
LL | pub fn foo() {}
   | ------------ similarly named function `foo` defined here

error[E0599]: no method named `foo1` found for struct `Foo` in the current scope
error[E0599]: no method named `foo1` found for struct `Foo` in the current scope
  --> fake-test-src-base/stability-attribute/issue-109177.rs:11:9
   |
LL |     foo.foo1();
   |         ^^^^ method not found in `Foo`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0599.
For more information about an error, try `rustc --explain E0425`.
