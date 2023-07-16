plain
travis_time:end:24f293fc:start=1541897565267259461,finish=1541897567950041315,duration=2682781854
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:38:33]    Compiling parking_lot_core v0.3.0
[00:38:33]    Compiling tempfile v3.0.3
[00:38:34]    Compiling parking_lot v0.6.4
[00:38:35]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:38:39] error[E0560]: struct `rustc::hir::AnonConst` has no field named `body_id`
[00:38:39]     |
[00:38:39] 239 |                             body_id: hir::BodyId {
[00:38:39]     |                             ^^^^^^^ `rustc::hir::AnonConst` does not have this field
[00:38:39]     |
[00:38:39]     |
[00:38:39]     = note: available fields are: `id`, `hir_id`, `body`
