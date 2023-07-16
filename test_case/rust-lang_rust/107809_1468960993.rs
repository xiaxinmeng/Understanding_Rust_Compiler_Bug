plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:067e5fb3939cbadf64f1a956c1ef134a6494dc9b)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
---- [codegen] tests/codegen/sanitizer-cfi-emit-type-metadata-id-itanium-cxx-abi.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/codegen/sanitizer-cfi-emit-type-metadata-id-itanium-cxx-abi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-cfi-emit-type-metadata-id-itanium-cxx-abi" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-cfi-emit-type-metadata-id-itanium-cxx-abi/auxiliary" "-Clto" "-Cno-prepopulate-passes" "-Ctarget-feature=-crt-static" "-Zsanitizer=cfi"
stdout: none
--- stderr -------------------------------
error: opaque type constrained in nested item
    |
107 |         fn foo() -> Type5 {
    |         ----------------- this item is not a sibling of the opaque type
108 |             Foo
