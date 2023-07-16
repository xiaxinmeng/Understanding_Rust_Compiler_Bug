plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:c8a09107e1a7966f8c20565a263305ce8f62405f)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---

---- [ui] tests/ui/invalid_macro_export_argument.rs stdout ----
diff of stderr:

- error: `#[macro_export]` can only take 1 or 0 arguments.
+ error: `#[macro_export]` can only take 1 or 0 arguments
3    |
3    |
4 LL | #[macro_export(hello, world)]
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
6    |
7    = note: `#[deny(invalid_macro_export_arguments)]` on by default
8 
8 
- error: `not_local_inner_macros` isn't a valid `#[macro_export]` argument.
+ error: `not_local_inner_macros` isn't a valid `#[macro_export]` argument
11    |
12 LL | #[macro_export(not_local_inner_macros)]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid_macro_export_argument/invalid_macro_export_argument.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args invalid_macro_export_argument.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/invalid_macro_export_argument.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid_macro_export_argument" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid_macro_export_argument/auxiliary"
stdout: none
--- stderr -------------------------------
error: `#[macro_export]` can only take 1 or 0 arguments
  --> fake-test-src-base/invalid_macro_export_argument.rs:1:1
   |
LL | #[macro_export(hello, world)] //~ ERROR `#[macro_export]` can only take 1 or 0 arguments.
   |
   = note: `#[deny(invalid_macro_export_arguments)]` on by default


error: `not_local_inner_macros` isn't a valid `#[macro_export]` argument
  --> fake-test-src-base/invalid_macro_export_argument.rs:6:16
   |
LL | #[macro_export(not_local_inner_macros)] //~ ERROR `not_local_inner_macros` isn't a valid `#[macro_export]` argument.

error: aborting due to 2 previous errors
------------------------------------------

