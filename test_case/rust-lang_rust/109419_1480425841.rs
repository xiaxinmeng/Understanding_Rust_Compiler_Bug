plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:5f3e9487b084c5235556ffd8baa8b183de9eb120)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
9 note: required by a bound in `call_mut`
10   --> $SRC_DIR/core/src/ops/function.rs:LL:COL
+ help: consider borrowing here
+    |
+ LL |     &mut handlers.unwrap().as_mut().call_mut(&mut ());
+    |     ++++
11 help: consider removing the leading `&`-reference
12    |
13 LL -     handlers.unwrap().as_mut().call_mut(&mut ());

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-57404/issue-57404.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/issue-57404.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/typeck/issue-57404.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-57404" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-57404/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `&mut ()` is not a tuple
  --> fake-test-src-base/typeck/issue-57404.rs:6:41
   |
LL |     handlers.unwrap().as_mut().call_mut(&mut ()); //~ ERROR: `&mut ()` is not a tuple
   |                                -------- ^^^^^^^ the trait `Tuple` is not implemented for `&mut ()`
   |                                required by a bound introduced by this call
   |
note: required by a bound in `call_mut`
  --> /rustc/FAKE_PREFIX/library/core/src/ops/function.rs:166:5
  --> /rustc/FAKE_PREFIX/library/core/src/ops/function.rs:166:5
help: consider borrowing here
   |
LL |     &mut handlers.unwrap().as_mut().call_mut(&mut ()); //~ ERROR: `&mut ()` is not a tuple
   |     ++++
help: consider removing the leading `&`-reference
   |
LL -     handlers.unwrap().as_mut().call_mut(&mut ()); //~ ERROR: `&mut ()` is not a tuple
LL +     handlers.unwrap().as_mut().call_mut(()); //~ ERROR: `&mut ()` is not a tuple

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
