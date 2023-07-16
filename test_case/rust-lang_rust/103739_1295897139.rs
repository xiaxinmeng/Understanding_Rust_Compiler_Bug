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
    Checking rustc_lint v0.0.0 (/checkout/compiler/rustc_lint)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0614]: type `rustc_middle::ty::Const<'_>` cannot be dereferenced
    |
    |
602 |             mir::ConstantKind::Ty(ct) => self.const_to_op(*ct, layout),

error[E0614]: type `rustc_middle::mir::interpret::ConstValue<'_>` cannot be dereferenced
   --> compiler/rustc_const_eval/src/interpret/operand.rs:603:69
    |
    |
603 |             mir::ConstantKind::Val(val, ty) => self.const_val_to_op(*val, *ty, layout),

error[E0614]: type `rustc_middle::ty::Ty<'_>` cannot be dereferenced
   --> compiler/rustc_const_eval/src/interpret/operand.rs:603:75
    |
    |
603 |             mir::ConstantKind::Val(val, ty) => self.const_val_to_op(*val, *ty, layout),

For more information about this error, try `rustc --explain E0614`.
error: could not compile `rustc_const_eval` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
