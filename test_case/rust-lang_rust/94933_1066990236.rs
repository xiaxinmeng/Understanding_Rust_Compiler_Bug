plain
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  IMAGE: mingw-check
##[endgroup]
fatal: unknown commit 53fd98ca776cb875bc9e5514f56b52eb74f9e7a9
All commits in `HEAD` are present in `master`
src/ci/scripts/verify-stable-version-number.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check
---
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0599]: no method named `forget_guarantee` found for struct `DiagnosticBuilder<'_>` in the current scope
    |
    |
117 |             _ => diag.struct_span_err(attr_sp, msg).forget_guarantee(),
    |                                                     ^^^^^^^^^^^^^^^^ method not found in `DiagnosticBuilder<'_>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_builtin_macros` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
