plain
travis_time:end:15c28e3f:start=1560619892371217892,finish=1560619980965539479,duration=88594321587
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:14:01] 
[00:14:01] error[E0107]: wrong number of lifetime arguments: expected 1, found 2
[00:14:01]    --> src/librustc_typeck/check/wfcheck.rs:369:47
[00:14:01]     |
[00:14:01] 369 |         impl<'tcx> ty::fold::TypeFolder<'tcx, 'tcx> for DefaultNormalizer<'tcx> {
[00:14:01] 
[00:14:01] error[E0107]: wrong number of lifetime arguments: expected 1, found 2
[00:14:01]    --> src/librustc_typeck/check/wfcheck.rs:370:50
[00:14:01]     |
[00:14:01]     |
[00:14:01] 370 |             fn tcx<'a>(&'a self) -> TyCtxt<'tcx, 'tcx> {
[00:14:01] 
[00:14:01] error: aborting due to 3 previous errors
[00:14:01] 
[00:14:01] For more information about this error, try `rustc --explain E0107`.
