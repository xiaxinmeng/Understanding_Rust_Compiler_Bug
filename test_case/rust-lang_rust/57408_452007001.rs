plain
travis_time:end:03ff83df:start=1546877652446898210,finish=1546877744167854714,duration=91720956504
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:51:46]    Compiling parking_lot_core v0.3.0
[00:51:48]    Compiling parking_lot v0.6.4
[00:51:50]    Compiling tempfile v3.0.5
[00:51:51]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:51:55] error[E0063]: missing field `hir_id` in initializer of `rustc::hir::Lifetime`
[00:51:55]     |
[00:51:55]     |
[00:51:55] 210 |                     args.push(hir::GenericArg::Lifetime(hir::Lifetime {
[00:51:55]     |                                                         ^^^^^^^^^^^^^ missing `hir_id`
