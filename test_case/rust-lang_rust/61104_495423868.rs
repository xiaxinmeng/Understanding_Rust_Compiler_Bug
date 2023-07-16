plain
travis_time:end:2b113177:start=1558654245412215864,finish=1558654246202213229,duration=789997365
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:15:39]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:15:52] error[E0282]: type annotations needed
[00:15:52]    --> <::rustc::mir::interpret::err macros>:2:1
[00:15:52]     |
[00:15:52] 1   | / ( $ ( $ tt : tt ) * ) => {
[00:15:52] 2   | | Err ( $ crate :: mir :: interpret :: InterpError :: $ ( $ tt ) * . into (  ) )
[00:15:52]     | | ^^^
[00:15:52]     | | cannot infer type for `E`
[00:15:52] 3   | | } ;
[00:15:52]     | |___- in this expansion of `err!`
[00:15:52]     | 
[00:15:52]     | 
[00:15:52]    ::: src/librustc_mir/interpret/operand.rs:474:63
[00:15:52]     |
[00:15:52] 474 |                   PlaceBase::Local(mir::RETURN_PLACE) => return err!(ReadFromReturnPointer),
[00:15:52] 
[00:15:55] error: aborting due to previous error
[00:15:55] 
[00:15:55] For more information about this error, try `rustc --explain E0282`.
