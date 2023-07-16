plain
travis_time:end:0a9c67a6:start=1555817296937802560,finish=1555817381632706981,duration=84694904421
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:12:23]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:12:23] error: expected expression, found `>>`
[00:12:23]    --> src/librustc_mir/interpret/place.rs:549:1
[00:12:23]     |
[00:12:23] 549 | >>>>>>> parent of b62d9789da... rename EvalResult to InterpResult
[00:12:23]     | ^^ expected expression
[00:12:25] error[E0412]: cannot find type `InterpResult` in this scope
[00:12:25]    --> src/librustc_mir/interpret/place.rs:548:10
[00:12:25]     |
[00:12:25]     |
[00:12:25] 548 |     ) -> InterpResult<'tcx, PlaceTy<'tcx, M::PointerTag>> {
[00:12:25] 
[00:12:27] error[E0107]: wrong number of lifetime arguments: expected 0, found 1
[00:12:27]    --> src/librustc_mir/interpret/place.rs:547:41
[00:12:27]     |
[00:12:27]     |
[00:12:27] 547 |         proj_elem: &mir::ProjectionElem<'tcx, mir::Local, Ty<'tcx>>,
[00:12:27] 
[00:12:27] error: aborting due to 3 previous errors
[00:12:27] 
[00:12:27] Some errors occurred: E0107, E0412.
