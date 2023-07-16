plain
travis_time:end:02c05ba8:start=1544896766438603167,finish=1544896824280414847,duration=57841811680
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:13:56]     --> src/librustc_typeck/check/mod.rs:5152:28
[00:13:56]      |
[00:13:56] 5152 |           let (def_id, ty) = match def {
[00:13:56]      |  ____________________________^
[00:13:56] 5153 | |             Def::SelfCtor(impl_def_id) => {
[00:13:56] 5154 | |                 let ty = self.impl_self_ty(span, impl_def_id).ty;
[00:13:56] 5155 | |                 let adt_def = ty.ty_adt_def();
[00:13:56] 5193 | |             }
[00:13:56] 5194 | |         };
[00:13:56]      | |_________^ expected (), found tuple
[00:13:56]      |
[00:13:56]      |
[00:13:56]      = note: expected type `()`
[00:13:56]                 found type `(rustc::hir::def_id::DefId, &rustc::ty::TyS<'_>)`
[00:13:56] note: match arm with an incompatible type
[00:13:56]     --> src/librustc_typeck/check/mod.rs:5186:18
[00:13:56] 5186 |               _ => {
[00:13:56]      |  __________________^
[00:13:56] 5187 | |                 let def_id = def.def_id();
[00:13:56] 5188 | |
[00:13:56] 5188 | |
[00:13:56] 5189 | |                 // The things we are substituting into the type should not contain
[00:13:56] 5192 | |                 (def_id, ty)
[00:13:56] 5193 | |             }
[00:13:56]      | |_____________^
[00:13:56] 
