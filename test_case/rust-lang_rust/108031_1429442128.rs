plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:b9dd95b10bcfe24d57bf54db874f81a7c8315a80)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
- error: lifetimes cannot start with a number
+ error: lifetimes or labels cannot start with a number
10   --> $DIR/numeric-lifetime.rs:1:10
11    |
12 LL | struct S<'1> { s: &'1 usize }
13    |          ^^
14 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- error: lifetimes cannot start with a number
- error: lifetimes cannot start with a number
+ error: lifetimes or labels cannot start with a number
16   --> $DIR/numeric-lifetime.rs:1:20
17    |
18 LL | struct S<'1> { s: &'1 usize }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/numeric-lifetime/numeric-lifetime.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/numeric-lifetime.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/numeric-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/numeric-lifetime" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/numeric-lifetime/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/parser/numeric-lifetime.rs:6:20
   |
LL |     let x: usize = "";
   |            -----   ^^ expected `usize`, found `&str`
   |            -----   ^^ expected `usize`, found `&str`
   |            |
   |            expected due to this

error: lifetimes or labels cannot start with a number
  --> fake-test-src-base/parser/numeric-lifetime.rs:1:10
   |
LL | struct S<'1> { s: &'1 usize }

error: lifetimes or labels cannot start with a number
  --> fake-test-src-base/parser/numeric-lifetime.rs:1:20
   |
   |
LL | struct S<'1> { s: &'1 usize }

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.
