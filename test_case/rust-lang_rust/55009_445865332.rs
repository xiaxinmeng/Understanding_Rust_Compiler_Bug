plain
travis_time:end:2767eafe:start=1544456333888661105,finish=1544456334914581118,duration=1025920013
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:13:15]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:25] error[E0615]: attempted to take value of method `hir` on type `rustc::ty::TyCtxt<'a, 'tcx, 'tcx>`
[00:13:25]    --> src/librustc_mir/transform/check_unsafety.rs:564:18
[00:13:25]     |
[00:13:25] 564 |     let id = tcx.hir.as_local_node_id(def_id).unwrap();
[00:13:25]     |
[00:13:25]     = help: maybe a `()` to call it is missing?
[00:13:25] 
[00:13:25] error[E0615]: attempted to take value of method `hir` on type `rustc::ty::TyCtxt<'a, 'tcx, 'tcx>`
[00:13:25] error[E0615]: attempted to take value of method `hir` on type `rustc::ty::TyCtxt<'a, 'tcx, 'tcx>`
[00:13:25]    --> src/librustc_mir/transform/check_unsafety.rs:565:51
[00:13:25]     |
[00:13:25] 565 |     let (const_context, min_const_fn) = match tcx.hir.body_owner_kind(id) {
[00:13:25]     |
[00:13:25]     = help: maybe a `()` to call it is missing?
[00:13:25] 
[00:13:30] error: aborting due to 2 previous errors
