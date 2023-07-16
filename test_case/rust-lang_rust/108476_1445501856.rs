plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
........................................................................................ 14520/14531
...........
failures:

---- [ui] tests/ui/box/stack-usage.rs stdout ----
error: test compilation failed although it shouldn't!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/box/stack-usage.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/box/stack-usage/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/box/stack-usage/auxiliary" "-Copt-level=0" "-Zmir-opt-level=0"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `[Big; 128]: Default` is not satisfied
  --> fake-test-src-base/box/stack-usage.rs:22:31
   |
LL |     let _x: Box<[Big; 128]> = Box::default();
   |                               ^^^^^^^^^^^^ the trait `Default` is not implemented for `[Big; 128]`
   = help: the following other types implement trait `Default`:
             &[T]
             &mut [T]
             [T; 0]
             [T; 0]
             [T; 10]
             [T; 11]
             [T; 12]
             [T; 13]
             [T; 14]
           and 27 others
   = note: required for `Box<[Big; 128]>` to implement `Default`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
