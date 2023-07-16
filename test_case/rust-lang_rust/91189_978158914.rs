plain
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  IMAGE: mingw-check
##[endgroup]
fatal: unknown commit 53fd98ca776cb875bc9e5514f56b52eb74f9e7a9
All commits in `HEAD` are present in `master`
src/ci/scripts/run-build-from-ci.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check
---
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: unused variable: `cgcx`
   --> compiler/rustc_codegen_llvm/src/back/write.rs:381:5
    |
381 |     cgcx: &CodegenContext<LlvmCodegenBackend>,
    |     ^^^^ help: if this is intentional, prefix it with an underscore: `_cgcx`
    |
    = note: `-D unused-variables` implied by `-D warnings`
    Checking rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
    Checking rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
