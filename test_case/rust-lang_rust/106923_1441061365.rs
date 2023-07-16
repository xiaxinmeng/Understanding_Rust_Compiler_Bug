plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
failures:

---- [ui] tests/ui/issues/issue-106755.rs stdout ----
normalized stderr:
error[E0751]: found both positive and negative implementation of trait `Send` for type `TestType<_>`:
   |
   |
LL | unsafe impl<T: MyTrait + 'static> Send for TestType<T> {}
   | ------------------------------------------------------ positive implementation here
LL |
LL | impl<T: MyTrait> !Send for TestType<T> {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ negative implementation here

error[E0119]: conflicting implementations of trait `Send` for type `TestType<_>`
   |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL | unsafe impl<T: MyTrait + 'static> Send for TestType<T> {}
...
...
LL | unsafe impl<T: 'static> Send for TestType<T> {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `TestType<_>`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0119, E0751.
For more information about an error, try `rustc --explain E0119`.
---
To only update this specific test, also pass `--test-args issues/issue-106755.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-106755.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-106755" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-106755/auxiliary" "-Ztranslate-lang=en_US"
stdout: none
--- stderr -------------------------------
error[E0751]: found both positive and negative implementation of trait `Send` for type `TestType<_>`:
  --> fake-test-src-base/issues/issue-106755.rs:13:1
   |
LL | unsafe impl<T: MyTrait + 'static> Send for TestType<T> {}
   | ------------------------------------------------------ positive implementation here
LL |
LL | impl<T: MyTrait> !Send for TestType<T> {} //~ ERROR found both positive and negative implementation
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ negative implementation here

error[E0119]: conflicting implementations of trait `Send` for type `TestType<_>`
  --> fake-test-src-base/issues/issue-106755.rs:15:1
   |
LL | unsafe impl<T: MyTrait + 'static> Send for TestType<T> {}
...
...
LL | unsafe impl<T: 'static> Send for TestType<T> {} //~ ERROR conflicting implementations
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `TestType<_>`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0119, E0751.
For more information about an error, try `rustc --explain E0119`.
