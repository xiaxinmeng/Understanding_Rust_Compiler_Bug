plain
travis_time:end:09473d36:start=1558083658540015162,finish=1558083659323885055,duration=783869893
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:09:07]    Compiling error_index_generator v0.0.0 (/checkout/src/tools/error_index_generator)
[01:09:08] error[E0061]: this function takes 2 parameters but 1 parameter was supplied
[01:09:08]    --> src/tools/error_index_generator/main.rs:267:18
[01:09:08]     |
[01:09:08] 267 |       let result = syntax::with_globals(move || {
[01:09:08]     |  __________________^
[01:09:08] 268 | |         main_with_result(format, &dst)
[01:09:08] 269 | |     });
[01:09:08] 
[01:09:08] error: aborting due to previous error
[01:09:08] 
[01:09:08] For more information about this error, try `rustc --explain E0061`.
