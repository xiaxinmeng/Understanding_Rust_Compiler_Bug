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
diff of stderr:

2   --> $DIR/bad-const-wf-doesnt-specialize.rs:8:29
3    |
4 LL | impl<const N: i32> Copy for S<N> {}
+    |                             ^^^^ expected `usize`, found `i32`
6    |
7 note: required by a bound in `S`
8   --> $DIR/bad-const-wf-doesnt-specialize.rs:6:10
---
To only update this specific test, also pass `--test-args specialization/min_specialization/bad-const-wf-doesnt-specialize.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/specialization/min_specialization/bad-const-wf-doesnt-specialize.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/min_specialization/bad-const-wf-doesnt-specialize" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/min_specialization/bad-const-wf-doesnt-specialize/auxiliary"
stdout: none
--- stderr -------------------------------
error: the constant `N` is not of type `usize`
  --> fake-test-src-base/specialization/min_specialization/bad-const-wf-doesnt-specialize.rs:8:29
   |
LL | impl<const N: i32> Copy for S<N> {}
   |                             ^^^^ expected `usize`, found `i32`
note: required by a bound in `S`
  --> fake-test-src-base/specialization/min_specialization/bad-const-wf-doesnt-specialize.rs:6:10
   |
   |
LL | struct S<const L: usize>;
   |          ^^^^^^^^^^^^^^ required by this bound in `S`
error: aborting due to previous error
------------------------------------------


