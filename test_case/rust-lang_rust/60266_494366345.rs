plain
travis_time:end:00fdb1b6:start=1558439894289278398,finish=1558439896523587357,duration=2234308959
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:16:38]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:16:42] error[E0107]: wrong number of type arguments: expected 3, found 4
[00:16:42]   --> src/librustc_mir/borrow_check/flows.rs:24:75
[00:16:42]    |
[00:16:42] 24 | crate type PoloniusOutput = Output<RegionVid, BorrowIndex, LocationIndex, Local>;
[00:16:42] 
[00:16:42] error[E0107]: wrong number of type arguments: expected 3, found 4
[00:16:42]   --> src/librustc_mir/borrow_check/nll/facts.rs:14:79
[00:16:42]    |
[00:16:42]    |
[00:16:42] 14 | crate type AllFacts = PoloniusAllFacts<RegionVid, BorrowIndex, LocationIndex, Local>;
[00:16:42] 
[00:16:42] error[E0107]: wrong number of type arguments: expected 3, found 4
[00:16:42]   --> src/librustc_mir/borrow_check/nll/mod.rs:86:61
[00:16:42]    |
[00:16:42]    |
[00:16:42] 86 |     Option<Rc<Output<RegionVid, BorrowIndex, LocationIndex, Local>>>,
[00:16:42] 
[00:16:42] error: aborting due to 3 previous errors
[00:16:42] 
[00:16:42] For more information about this error, try `rustc --explain E0107`.
