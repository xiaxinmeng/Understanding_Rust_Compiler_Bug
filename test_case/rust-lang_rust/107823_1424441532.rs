plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:ca7cab66e2f838703fe12775fbabb05754421ad8)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
.................................................iii.................................... 14344/14417
.........................................................................
failures:

---- [ui] tests/ui/traits/new-solver/provisional-result-done.rs stdout ----

error: Error: expected failure status (Some(101)) but received status Some(1).
status: exit status: 1
command: RUST_BACKTRACE="0" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/new-solver/provisional-result-done.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/provisional-result-done" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/provisional-result-done/auxiliary" "-Ztrait-solver=next"
stdout: none
--- stderr -------------------------------
error[E0283]: type annotations needed: cannot satisfy `Bar<T>: Coinductive`
  --> fake-test-src-base/traits/new-solver/provisional-result-done.rs:20:25
   |
LL | impl<T> Coinductive for Bar<T>
   |
   |
   = note: cannot satisfy `Bar<T>: Coinductive`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.
------------------------------------------
