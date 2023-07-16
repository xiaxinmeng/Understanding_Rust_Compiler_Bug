plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:055e3b93d15803815fe6f9cbc1b02b11be094e54)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0599]: no method named `bound_fn_sig` found for struct `TyCtxt<'tcx>` in the current scope
   --> compiler/rustc_middle/src/ty/diagnostics.rs:537:45
    |
537 |                 self.tcx.mk_fn_ptr(self.tcx.bound_fn_sig(def_id).subst(self.tcx, substs))
    |                                             ^^^^^^^^^^^^ help: there is a method with a similar name: `fn_sig`
   ::: compiler/rustc_middle/src/ty/context.rs:397:1
    |
397 | pub struct TyCtxt<'tcx> {
397 | pub struct TyCtxt<'tcx> {
    | ----------------------- method `bound_fn_sig` not found for this struct
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_middle` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed successfully in 0:01:44
