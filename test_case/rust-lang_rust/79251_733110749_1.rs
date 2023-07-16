
warning: the feature `const_generics` is incomplete and may not be safe to use and/or cause compiler crashes
 --> src/lib.rs:1:12
  |
1 | #![feature(const_generics)]
  |            ^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #44580 <https://github.com/rust-lang/rust/issues/44580> for more information
  = help: consider using `min_const_generics` instead, which is more stable and complete

warning: the feature `const_evaluatable_checked` is incomplete and may not be safe to use and/or cause compiler crashes
 --> src/lib.rs:2:12
  |
2 | #![feature(const_evaluatable_checked)]
  |            ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information

error: internal compiler error: compiler/rustc_middle/src/ich/impls_ty.rs:167:9: ty::TyKind::hash_stable() - can't hash a TyVid _#0t.

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-nightly (d9a105fdd 2020-11-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `std::pin::Pin<SmallVec<{ D * 2 }>>: std::ops::Deref`
#1 [method_autoderef_steps] computing autoderef types for `Canonical { max_universe: U0, variables: [CanonicalVarInfo { kind: Region(U0) }, CanonicalVarInfo { kind: Const(U0) }], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [ConstEvaluatable(WithOptConstParam { did: DefId(0:10 ~ btree_ice[5d5a]::{impl#0}::{constant#0}), const_param_did: None }, [Const { ty: usize, val: Param(D/#0) }]), OutlivesPredicate(SmallVec<{ D * 2 }>, ReLateBound(DebruijnIndex(0), BrAnon(0)))], reveal: UserFacing }, value: std::pin::Pin<SmallVec<{ D * 2 }>> } }`
end of query stack
error: aborting due to previous error; 2 warnings emitted

error: could not compile `btree_ice`
