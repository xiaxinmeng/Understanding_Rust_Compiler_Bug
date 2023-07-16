plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:75573f9759179a720f4c3af6c9fb518ac0061dca)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
To only update this specific test, also pass `--test-args alloc-error/alloc-error-handler-bad-signature-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/alloc-error/alloc-error-handler-bad-signature-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/alloc-error/alloc-error-handler-bad-signature-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/alloc-error/alloc-error-handler-bad-signature-3/auxiliary" "-C" "panic=abort"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/alloc-error/alloc-error-handler-bad-signature-3.rs:10:1
   |
LL |   #[alloc_error_handler]
   |   ---------------------- in this procedural macro expansion
   |   ---------------------- in this procedural macro expansion
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |   fn oom() -> ! { //~ ERROR function takes 0 arguments but 1 argument was supplied
   |  _-^^^^^^^^^^^^
LL | |     loop {}
LL | | }
   | |_- unexpected argument of type `core::alloc::Layout`
note: function defined here
  --> fake-test-src-base/alloc-error/alloc-error-handler-bad-signature-3.rs:10:4
   |
   |
LL | fn oom() -> ! { //~ ERROR function takes 0 arguments but 1 argument was supplied
   = note: this error originates in the attribute macro `alloc_error_handler` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

