plain
travis_time:end:08211b00:start=1545857468389755734,finish=1545857469940376080,duration=1550620346
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:12:34]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:12:37] error[E0615]: attempted to take value of method `hir` on type `rustc::ty::TyCtxt<'_, '_, '_>`
[00:12:37]     --> src/librustc_typeck/check/mod.rs:1093:30
[00:12:37]      |
[00:12:37] 1093 |         let module = fcx.tcx.hir.get_module_parent(fn_id);
[00:12:37]      |
[00:12:37]      = help: maybe a `()` to call it is missing?
[00:12:37] 
[00:12:41] error: aborting due to previous error
