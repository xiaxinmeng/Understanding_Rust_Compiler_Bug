plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
---- [ui] tests/ui/consts/const-external-macro-const-err.rs stdout ----
diff of stderr:

- error[E0080]: evaluation of constant value failed
+ error: expected one of `)`, `,`, `.`, `?`, or an operator, found `:`
3    |
3    |
4 LL |     static_assert!(2 + 2 == 5);
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ index out of bounds: the length is 1 but the index is 1
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ index out of bounds: the length is 1 but the index is 1
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected one of `)`, `,`, `.`, `?`, or an operator
7    = note: this error originates in the macro `static_assert` (in Nightly builds, run with -Z macro-backtrace for more info)
8 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---
To only update this specific test, also pass `--test-args consts/const-external-macro-const-err.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-external-macro-const-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-external-macro-const-err" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-external-macro-const-err/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: expected one of `)`, `,`, `.`, `?`, or an operator, found `:`
  --> fake-test-src-base/consts/const-external-macro-const-err.rs:12:5
   |
LL |     static_assert!(2 + 2 == 5); //~ ERROR constant
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected one of `)`, `,`, `.`, `?`, or an operator
   = note: this error originates in the macro `static_assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
------------------------------------------
