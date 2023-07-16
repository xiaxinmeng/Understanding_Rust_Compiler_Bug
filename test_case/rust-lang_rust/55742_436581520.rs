plain
travis_time:end:0825434e:start=1541584237071019649,finish=1541584238040638110,duration=969618461
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:50:11] diff of stderr:
[00:50:11] 
[00:50:11] 5    |         ^^^^^^^^^^^^^^^^ not a tuple variant or struct
[00:50:11] 6 
[00:50:11] 7 error[E0164]: expected tuple struct/variant, found method `<Path>::new`
[00:50:11] -   --> $DIR/match-fn-call.rs:7:9
[00:50:11] +   --> $DIR/match-fn-call.rs:8:9
[00:50:11] 9    |
[00:50:11] 10 LL |         Path::new("bar"ch foo {\n        Foo::B(i) => i, // error E0164\n    }\n}\n