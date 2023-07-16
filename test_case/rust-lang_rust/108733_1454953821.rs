plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:0fdabd83e1d3faaa8e9cfd7c00031e3a92997344)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

---- [ui] tests/ui/unsized-locals/issue-105753.rs stdout ----
diff of stderr:

1 error[E0277]: the size for values of type `[i32]` cannot be known at compilation time
-   --> $DIR/issue-105753-01.rs:1:13
3    |
3    |
4 LL | fn foo() -> [i32] {

8    = note: the return type of a function must have a statically known size
9 
9 
10 error[E0277]: the size for values of type `[i32]` cannot be known at compilation time
-   --> $DIR/issue-105753-01.rs:7:9
12    |
13 LL |     let x = foo();
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
14    |         ^ doesn't have a size known at compile-time
---
To only update this specific test, also pass `--test-args unsized-locals/issue-105753.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/unsized-locals/issue-105753.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/issue-105753" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/issue-105753/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the size for values of type `[i32]` cannot be known at compilation time
  --> fake-test-src-base/unsized-locals/issue-105753.rs:1:13
   |
LL | fn foo() -> [i32] {
   |
   = help: the trait `Sized` is not implemented for `[i32]`
   = note: the return type of a function must have a statically known size


error[E0277]: the size for values of type `[i32]` cannot be known at compilation time
  --> fake-test-src-base/unsized-locals/issue-105753.rs:7:9
LL |     let x = foo();
   |         ^ doesn't have a size known at compile-time
   |
   = help: the trait `Sized` is not implemented for `[i32]`
