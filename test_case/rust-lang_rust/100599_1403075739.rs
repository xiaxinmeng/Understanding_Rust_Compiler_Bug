plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:7873766cb4a6a0bb00c54496556b38db3fffa5d6)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---

---- [ui] tests/ui/error-codes/E0523.rs stdout ----
diff of stderr:

- error[E0425]: cannot find function `fifty` in crate `e0523_conflict`
-   --> $DIR/E0523.rs:7:29
+ error[E0464]: multiple candidates for `rlib` dependency `crateresolve1` found
+   --> $DIR/E0523.rs:11:1
3    |
- LL |     let x = e0523_conflict::fifty(); // from e0523_aux_1.rs
-    |                             ^^^^^ not found in `e0523_conflict`
+ LL | extern crate crateresolve1;
+    |
+    |
+    = note: candidate #1: $TEST_BUILD_DIR/error-codes/E0523/auxiliary/libcrateresolve1-1.somelib
+    = note: candidate #2: $TEST_BUILD_DIR/error-codes/E0523/auxiliary/libcrateresolve1-2.somelib
+    = note: candidate #3: $TEST_BUILD_DIR/error-codes/E0523/auxiliary/libcrateresolve1-3.somelib
7 error: aborting due to previous error
8 

- For more information about this error, try `rustc --explain E0425`.
---
To only update this specific test, also pass `--test-args error-codes/E0523.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/error-codes/E0523.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0523" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0523/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0464]: multiple candidates for `rlib` dependency `crateresolve1` found
  --> fake-test-src-base/error-codes/E0523.rs:11:1
LL | extern crate crateresolve1;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: candidate #1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0523/auxiliary/libcrateresolve1-1.rlib
   = note: candidate #2: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0523/auxiliary/libcrateresolve1-2.rlib
   = note: candidate #3: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0523/auxiliary/libcrateresolve1-3.rlib
error: aborting due to previous error

For more information about this error, try `rustc --explain E0464`.
------------------------------------------
