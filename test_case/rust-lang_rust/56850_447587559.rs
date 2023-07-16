plain
travis_time:end:2514e170:start=1544896992779342396,finish=1544896995059318226,duration=2279975830
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:11:07]     --> src/librustc_typeck/check/mod.rs:5160:28
[00:11:07]      |
[00:11:07] 5160 |           let (def_id, ty) = match def {
[00:11:07]      |  ____________________________^
[00:11:07] 5161 | |             Def::SelfCtor(impl_def_id) => {
[00:11:07] 5162 | |                 let ty = self.impl_self_ty(span, impl_def_id).ty;
[00:11:07] 5163 | |                 let adt_def = ty.ty_adt_def();
[00:11:07] 5201 | |             }
[00:11:07] 5202 | |         };
[00:11:07]      | |_________^ expected (), found tuple
[00:11:07]      |
[00:11:07]      |
[00:11:07]      = note: expected type `()`
[00:11:07]                 found type `(rustc::hir::def_id::DefId, &rustc::ty::TyS<'_>)`
[00:11:07] note: match arm with an incompatible type
[00:11:07]     --> src/librustc_typeck/check/mod.rs:5194:18
[00:11:07] 5194 |               _ => {
[00:11:07]      |  __________________^
[00:11:07] 5195 | |                 let def_id = def.def_id();
[00:11:07] 5196 | |
[00:11:07] 5196 | |
[00:11:07] 5197 | |                 // The things we are substituting into the type should not contain
[00:11:07] 5200 | |                 (def_id, ty)
[00:11:07] 5201 | |             }
[00:11:07]      | |_____________^
[00:11:07] 
