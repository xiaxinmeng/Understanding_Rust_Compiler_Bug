plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:13c1b4e09b845ddb9664cee13d03879444a1054d)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
failures:

---- [ui] tests/ui/coherence/coherence-overlap-negative-cycles.rs stdout ----
normalized stderr:
error[E0119]: conflicting implementations of trait `MyTrait`
   |
   |
LL | impl MyTrait for Foo where Foo: MyTrait {}
   | |
   | first implementation here
   | conflicting implementation

---
To only update this specific test, also pass `--test-args coherence/coherence-overlap-negative-cycles.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/coherence/coherence-overlap-negative-cycles.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-overlap-negative-cycles" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/coherence/coherence-overlap-negative-cycles/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0119]: conflicting implementations of trait `MyTrait`
  --> fake-test-src-base/coherence/coherence-overlap-negative-cycles.rs:14:1
   |
LL | impl MyTrait for Foo where Foo: MyTrait {}
   | |
   | first implementation here
   | conflicting implementation

