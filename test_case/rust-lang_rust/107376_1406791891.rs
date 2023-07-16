plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:ece894d15649f0f9d7884f915dc821f00bd0418b)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
   Compiling rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'tcx` due to conflicting requirements
   --> compiler/rustc_middle/src/traits/chalk.rs:237:14
    |
237 |         _ty: &Self::InternedType,
    |
note: first, the lifetime cannot outlive the lifetime `'tcx` as defined here...
   --> compiler/rustc_middle/src/traits/chalk.rs:60:6
    |
    |
60  | impl<'tcx> chalk_ir::interner::Interner for RustInterner<'tcx> {
note: ...so that the types are compatible
   --> compiler/rustc_middle/src/traits/chalk.rs:237:14
    |
    |
237 |         _ty: &Self::InternedType,
    |              ^^^^^^^^^^^^^^^^^^^
    = note: expected `<RustInterner<'tcx> as chalk_ir::interner::Interner>`
               found `<RustInterner<'_> as chalk_ir::interner::Interner>`
note: but, the lifetime must be valid for the anonymous lifetime defined here...
   --> compiler/rustc_middle/src/traits/chalk.rs:237:14
    |
237 |         _ty: &Self::InternedType,
    |              ^^^^^^^^^^^^^^^^^^^
note: ...so that the reference type `&std::boxed::Box<TyData<RustInterner<'_>>>` does not outlive the data it points at
   --> compiler/rustc_middle/src/traits/chalk.rs:237:14
    |
237 |         _ty: &Self::InternedType,

error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'tcx` due to conflicting requirements
   --> compiler/rustc_middle/src/traits/chalk.rs:238:13
    |
    |
238 |         c1: &Self::InternedConcreteConst,
    |
note: first, the lifetime cannot outlive the lifetime `'tcx` as defined here...
   --> compiler/rustc_middle/src/traits/chalk.rs:60:6
    |
    |
60  | impl<'tcx> chalk_ir::interner::Interner for RustInterner<'tcx> {
note: ...so that the types are compatible
   --> compiler/rustc_middle/src/traits/chalk.rs:238:13
    |
    |
238 |         c1: &Self::InternedConcreteConst,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: expected `<RustInterner<'tcx> as chalk_ir::interner::Interner>`
               found `<RustInterner<'_> as chalk_ir::interner::Interner>`
note: but, the lifetime must be valid for the anonymous lifetime defined here...
   --> compiler/rustc_middle/src/traits/chalk.rs:239:13
    |
239 |         c2: &Self::InternedConcreteConst,
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...so that the reference type `&valtree::ValTree<'_>` does not outlive the data it points at
   --> compiler/rustc_middle/src/traits/chalk.rs:239:13
    |
239 |         c2: &Self::InternedConcreteConst,

For more information about this error, try `rustc --explain E0495`.
error: could not compile `rustc_middle` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
